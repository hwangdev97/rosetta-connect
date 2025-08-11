use crate::config::Config;
use crate::formatter::DataFormatter;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;
use serde_json::Value;
use std::fs;
use tokio::time::{sleep, Instant};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct PullOptions {
    pub force_refresh: bool,
    pub retry_count: u32,
    pub output_format: String,
    pub export_file: Option<PathBuf>,
    pub filter_locales: Vec<String>,
}

impl Default for PullOptions {
    fn default() -> Self {
        Self {
            force_refresh: false,
            retry_count: 3,
            output_format: "table".to_string(),
            export_file: None,
            filter_locales: Vec::new(),
        }
    }
}

pub async fn run(config_path: &Path, options: PullOptions) -> Result<()> {
    println!("🚀 Pulling current localizations from App Store Connect...");
    
    // Step 1: Validate configuration
    let config = validate_and_load_config(config_path)
        .context("Failed to validate and load configuration")?;
    
    // Step 2: Check cache unless force refresh
    if !options.force_refresh {
        if let Some(cached_data) = check_cache(&config).await? {
            println!("📋 Using cached data (use --force-refresh to update)");
            display_results(&cached_data, &options)?;
            return Ok(());
        }
    }
    
    // Step 3: Verify API access
    verify_api_access(&config).await
        .context("Failed to verify API access")?;
    
    // Step 4: Download with retry and progress tracking
    let result = download_with_retry(&config, &options).await
        .context("Failed to download app data after retries")?;
    
    // Step 5: Save to cache and persistent storage
    save_to_cache(&result, &config).await
        .context("Failed to save data to cache")?;
    save_to_files(&result, &config).await
        .context("Failed to save data to files")?;
    
    // Step 6: Display and export results
    display_results(&result, &options)?;
    
    if let Some(export_path) = &options.export_file {
        export_data(&result, export_path, &options.output_format)
            .context("Failed to export data")?;
        println!("📁 Data exported to: {}", export_path.display());
    }
    
    println!("✅ Pull completed successfully!");
    Ok(())
}

fn validate_and_load_config(config_path: &Path) -> Result<Config> {
    let config = Config::load(config_path)
        .context("Failed to load configuration")?;
    
    // Validate bundle ID format
    validate_bundle_id(&config.app.bundle_id)
        .context("Invalid bundle ID format")?;
    
    Ok(config)
}

fn validate_bundle_id(bundle_id: &str) -> Result<()> {
    let bundle_id_regex = Regex::new(r"^[a-zA-Z0-9-]+\.[a-zA-Z0-9.-]+$")
        .context("Failed to compile bundle ID regex")?;
    
    if !bundle_id_regex.is_match(bundle_id) {
        return Err(anyhow::anyhow!(
            "Bundle ID '{}' does not match expected format (e.g., com.company.app)",
            bundle_id
        ));
    }
    
    Ok(())
}

async fn verify_api_access(config: &Config) -> Result<()> {
    println!("🔑 Verifying API access...");
    
    // Initialize Node.js runtime
    rc_node::init_node_runtime()
        .context("Failed to initialize Node.js runtime")?;
    
    // Test API connection with a lightweight call
    let test_result = rc_node::asc_validate(serde_json::json!({
        "bundle_id": config.app.bundle_id
    })).await;
    
    match test_result {
        Ok(_) => {
            println!("✅ API access verified");
            Ok(())
        }
        Err(e) => {
            Err(anyhow::anyhow!("API access verification failed: {}", e))
        }
    }
}

async fn check_cache(config: &Config) -> Result<Option<Value>> {
    let cache_dir = get_cache_dir(config)?;
    let cache_file = cache_dir.join("pull_cache.json");
    
    if !cache_file.exists() {
        return Ok(None);
    }
    
    // Check if cache is fresh (less than 1 hour old)
    let metadata = fs::metadata(&cache_file)
        .context("Failed to read cache file metadata")?;
    let cache_age = metadata.modified()
        .context("Failed to get cache modification time")?
        .elapsed()
        .unwrap_or(Duration::from_secs(0));
    
    if cache_age > Duration::from_secs(3600) { // 1 hour
        println!("⏰ Cache expired, will fetch fresh data");
        return Ok(None);
    }
    
    let cache_content = fs::read_to_string(&cache_file)
        .context("Failed to read cache file")?;
    let cached_data: Value = serde_json::from_str(&cache_content)
        .context("Failed to parse cached data")?;
    
    Ok(Some(cached_data))
}

async fn download_with_retry(config: &Config, options: &PullOptions) -> Result<Value> {
    let m = MultiProgress::new();
    let main_pb = m.add(ProgressBar::new_spinner());
    main_pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .context("Failed to create progress bar style")?
    );
    main_pb.enable_steady_tick(Duration::from_millis(80));
    
    let mut last_error = None;
    
    for attempt in 1..=options.retry_count {
        main_pb.set_message(format!(
            "Connecting to App Store Connect API... (attempt {}/{})",
            attempt, options.retry_count
        ));
        
        let start_time = Instant::now();
        
        match rc_node::asc_download(config.app.bundle_id.clone()).await {
            Ok(mut result) => {
                let duration = start_time.elapsed();
                main_pb.finish_with_message(format!(
                    "✅ Downloaded app metadata and screenshots ({:.1}s)",
                    duration.as_secs_f32()
                ));
                
                // Filter locales if specified
                if !options.filter_locales.is_empty() {
                    result = filter_locales_data(result, &options.filter_locales)?;
                }
                
                return Ok(result);
            }
            Err(e) => {
                last_error = Some(e);
                if attempt < options.retry_count {
                    let wait_time = Duration::from_secs(2_u64.pow(attempt - 1)); // Exponential backoff
                    main_pb.set_message(format!(
                        "⚠️  Attempt {} failed, retrying in {}s...",
                        attempt, wait_time.as_secs()
                    ));
                    sleep(wait_time).await;
                }
            }
        }
    }
    
    main_pb.finish_with_message("❌ All retry attempts failed");
    Err(anyhow::anyhow!(
        "Failed to download after {} attempts. Last error: {}",
        options.retry_count,
        last_error.unwrap()
    ))
}

fn filter_locales_data(mut data: Value, filter_locales: &[String]) -> Result<Value> {
    if let Some(metadata) = data.get_mut("metadata").and_then(|v| v.as_object_mut()) {
        let mut filtered_metadata = serde_json::Map::new();
        for locale in filter_locales {
            if let Some(locale_data) = metadata.remove(locale) {
                filtered_metadata.insert(locale.clone(), locale_data);
            }
        }
        *metadata = filtered_metadata;
    }
    
    if let Some(locales) = data.get_mut("locales").and_then(|v| v.as_array_mut()) {
        locales.retain(|locale| {
            if let Some(locale_str) = locale.as_str() {
                filter_locales.contains(&locale_str.to_string())
            } else {
                false
            }
        });
    }
    
    Ok(data)
}

async fn save_to_cache(data: &Value, config: &Config) -> Result<()> {
    let cache_dir = get_cache_dir(config)?;
    fs::create_dir_all(&cache_dir)
        .context("Failed to create cache directory")?;
    
    let cache_file = cache_dir.join("pull_cache.json");
    let cache_content = serde_json::to_string_pretty(data)
        .context("Failed to serialize cache data")?;
    
    fs::write(&cache_file, cache_content)
        .context("Failed to write cache file")?;
    
    Ok(())
}

async fn save_to_files(data: &Value, config: &Config) -> Result<()> {
    let data_dir = get_data_dir(config)?;
    fs::create_dir_all(&data_dir)
        .context("Failed to create data directory")?;
    
    // Save metadata for each locale
    if let Some(metadata) = data.get("metadata").and_then(|v| v.as_object()) {
        for (locale, locale_data) in metadata {
            let locale_dir = data_dir.join(locale);
            fs::create_dir_all(&locale_dir)
                .context(format!("Failed to create locale directory for {}", locale))?;
            
            let metadata_file = locale_dir.join("metadata.json");
            let metadata_content = serde_json::to_string_pretty(locale_data)
                .context("Failed to serialize locale metadata")?;
            
            fs::write(&metadata_file, metadata_content)
                .context(format!("Failed to write metadata for locale {}", locale))?;
        }
    }
    
    // Save summary data
    let summary_file = data_dir.join("summary.json");
    let summary_content = serde_json::to_string_pretty(data)
        .context("Failed to serialize summary data")?;
    
    fs::write(&summary_file, summary_content)
        .context("Failed to write summary file")?;
    
    Ok(())
}

fn display_results(data: &Value, options: &PullOptions) -> Result<()> {
    match options.output_format.as_str() {
        "json" => {
            let json_output = serde_json::to_string_pretty(data)
                .context("Failed to serialize data as JSON")?;
            println!("{}", json_output);
        }
        "csv" => {
            let csv_output = format_as_csv(data)
                .context("Failed to format data as CSV")?;
            println!("{}", csv_output);
        }
        "table" | _ => {
            // Show default locale content
            let default_only = DataFormatter::format_default_locale(data);
            println!("{}", default_only);
            
            // Show multi-locale compact status table
            let locales_table = DataFormatter::format_locales_status_table(data);
            println!("{}", locales_table);
        }
    }
    
    Ok(())
}

fn format_as_csv(data: &Value) -> Result<String> {
    let mut csv_lines = vec!["Locale,Name,Description,Keywords,WhatsNew,Status".to_string()];
    
    if let Some(metadata) = data.get("metadata").and_then(|v| v.as_object()) {
        for (locale, locale_data) in metadata {
            if let Some(obj) = locale_data.as_object() {
                let name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let description = obj.get("description").and_then(|v| v.as_str()).unwrap_or("");
                let keywords = obj.get("keywords").and_then(|v| v.as_str()).unwrap_or("");
                let whats_new = obj.get("whatsNew").and_then(|v| v.as_str()).unwrap_or("");
                
                // Simple status logic
                let status = if !name.is_empty() && !description.is_empty() { "Complete" } else { "Incomplete" };
                
                csv_lines.push(format!(
                    "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
                    locale,
                    name.replace('"', "\"\""),
                    description.replace('"', "\"\""),
                    keywords.replace('"', "\"\""),
                    whats_new.replace('"', "\"\""),
                    status
                ));
            }
        }
    }
    
    Ok(csv_lines.join("\n"))
}

fn export_data(data: &Value, export_path: &Path, format: &str) -> Result<()> {
    let content = match format {
        "json" => serde_json::to_string_pretty(data)
            .context("Failed to serialize data as JSON")?,
        "csv" => format_as_csv(data)
            .context("Failed to format data as CSV")?,
        _ => return Err(anyhow::anyhow!("Unsupported export format: {}", format)),
    };
    
    // Create parent directories if they don't exist
    if let Some(parent) = export_path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create export directory")?;
    }
    
    fs::write(export_path, content)
        .context("Failed to write export file")?;
    
    Ok(())
}

fn get_cache_dir(config: &Config) -> Result<PathBuf> {
    let mut cache_dir = std::env::current_dir()
        .context("Failed to get current directory")?;
    cache_dir.push(".rosetta-cache");
    cache_dir.push(&config.app.bundle_id);
    Ok(cache_dir)
}

fn get_data_dir(config: &Config) -> Result<PathBuf> {
    let mut data_dir = std::env::current_dir()
        .context("Failed to get current directory")?;
    data_dir.push(&config.app.bundle_id);
    data_dir.push("current");
    Ok(data_dir)
}