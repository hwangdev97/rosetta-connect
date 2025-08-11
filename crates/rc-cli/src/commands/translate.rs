use crate::config::Config;
use crate::formatter::DataFormatter;
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::path::Path;

pub async fn run(locales: Vec<String>, model: Option<String>, config_path: &Path) -> Result<()> {
    println!("Generating translations using AI...");
    
    let config = Config::load(config_path)
        .context("Failed to load configuration")?;
    
    let target_locales = if locales.is_empty() {
        config.app.target_locales.clone()
    } else {
        locales
    };
    
    let ai_model = model.unwrap_or_else(|| {
        config.ai.as_ref()
            .map(|ai| ai.model.clone())
            .unwrap_or_else(|| "gpt-4o-mini".to_string())
    });
    
    println!("🤖 Using AI model: {}", ai_model);
    println!("🌍 Target locales: {}", target_locales.join(", "));
    
    // Initialize Node.js runtime
    rc_node::init_node_runtime()
        .context("Failed to initialize Node.js runtime")?;
    
    // First, get current app data to translate
    println!("📥 Fetching current app metadata...");
    let app_data = rc_node::asc_download(config.app.bundle_id.clone()).await
        .context("Failed to download app data")?;
    
    // Extract source metadata (usually English)
    let source_locale = &config.app.default_locale;
    let source_metadata = app_data
        .get("metadata")
        .and_then(|m| m.get(source_locale))
        .context("No source metadata found for translation")?
        .clone();
    
    // Prepare translation request
    let translation_request = json!({
        "metadata": source_metadata,
        "sourceLocale": source_locale,
        "targetLocales": target_locales,
        "context": format!("App Store metadata for {}", config.app.bundle_id)
    });
    
    // Estimate cost first
    println!("💰 Estimating translation cost...");
    let cost_estimate = rc_node::ai_estimate_cost(translation_request.clone()).await
        .context("Failed to estimate translation cost")?;
    
    let cost_info = DataFormatter::format_cost_info(&cost_estimate);
    print!("{}", cost_info);
    
    // Perform the actual translation
    println!("🔄 Starting AI translation...");
    let translation_result = rc_node::ai_translate(translation_request).await
        .context("Failed to translate metadata")?;
    
    // Process and display results
    if let Some(translations) = translation_result.get("translations") {
        let formatted_results = DataFormatter::format_translation_results(translations, source_locale);
        println!("{}", formatted_results);
    }
    
    // Display final cost information
    let final_cost_info = DataFormatter::format_cost_info(&translation_result);
    print!("{}", final_cost_info);
    
    println!("✅ Translation completed for {} locales", target_locales.len());
    
    // TODO: Save translated metadata to local files for review before upload
    println!("📝 Review translated content before uploading with 'rosetta-connect push'");
    
    Ok(())
}