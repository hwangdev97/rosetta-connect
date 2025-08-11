use crate::config::Config;
use crate::formatter::DataFormatter;
use anyhow::{Context, Result};
use serde_json::json;
use std::path::Path;

pub async fn run(detailed: bool, config_path: &Path) -> Result<()> {
    println!("Estimating AI API call costs...");
    
    let config = Config::load(config_path)
        .context("Failed to load configuration")?;
    
    // Initialize Node.js runtime
    rc_node::init_node_runtime()
        .context("Failed to initialize Node.js runtime")?;
    
    // Get current app data for accurate cost estimation
    println!("📥 Fetching app metadata for cost calculation...");
    let app_data = rc_node::asc_download(config.app.bundle_id.clone()).await
        .context("Failed to download app data")?;
    
    // Extract source metadata 
    let source_locale = &config.app.default_locale;
    let source_metadata = app_data
        .get("metadata")
        .and_then(|m| m.get(source_locale))
        .context("No source metadata found for cost estimation")?
        .clone();
    
    // Prepare estimation request
    let estimation_request = json!({
        "metadata": source_metadata,
        "sourceLocale": source_locale,
        "targetLocales": config.app.target_locales,
        "context": format!("App Store metadata for {}", config.app.bundle_id)
    });
    
    // Get accurate cost estimate
    let cost_estimate = rc_node::ai_estimate_cost(estimation_request).await
        .context("Failed to estimate translation cost")?;
    
    println!("💰 Cost Estimation");
    println!("{}", "═".repeat(50));
    println!();
    
    let estimated_cost = cost_estimate.get("estimatedCost").and_then(|c| c.as_f64()).unwrap_or(0.0);
    let token_estimate = cost_estimate.get("tokenEstimate").and_then(|t| t.as_u64()).unwrap_or(0);
    
    if detailed {
        println!("📊 Content Analysis");
        println!("{}", "─".repeat(30));
        
        // Analyze each field for detailed breakdown
        if let Some(metadata) = source_metadata.as_object() {
            for (field, value) in metadata {
                if let Some(text) = value.as_str() {
                    let field_tokens = (text.len() as f64 / 4.0).ceil() as u64;
                    let field_cost = (field_tokens as f64 / 1000.0) * 0.0006;
                    
                    let (icon, label) = match field.as_str() {
                        "name" => ("📱", "App Name"),
                        "description" => ("📝", "Description"),
                        "keywords" => ("🔍", "Keywords"),
                        "whatsNew" => ("📢", "What's New"),
                        _ => ("📄", field.as_str()),
                    };
                    
                    println!("   {} {}: ${:.4} ({} chars, ~{} tokens)", 
                        icon, label, field_cost, text.len(), field_tokens);
                }
            }
        }
        println!();
        
        println!("🌍 Locale Breakdown");
        println!("{}", "─".repeat(30));
        let cost_per_locale = estimated_cost / (config.app.target_locales.len() as f64).max(1.0);
        for locale in &config.app.target_locales {
            let locale_flag = match locale.as_str() {
                "en-US" => "🇺🇸",
                "zh-Hans" => "🇨🇳", 
                "fr-FR" => "🇫🇷",
                "de-DE" => "🇩🇪",
                _ => "🌐",
            };
            println!("   {} {}: ${:.4}", locale_flag, locale, cost_per_locale);
        }
        println!();
    }
    
    // Use formatter for cost display
    let cost_info = DataFormatter::format_cost_info(&cost_estimate);
    print!("{}", cost_info);
    
    println!("📈 Project Summary");
    println!("{}", "─".repeat(30));
    println!("🤖 Model: {}", config.ai.as_ref().map(|ai| &ai.model).unwrap_or(&"gpt-4o-mini".to_string()));
    println!("🔤 Source Locale: {}", source_locale);
    println!("🌍 Target Locales: {}", config.app.target_locales.len());
    println!("💰 Max Cost (with retries): ${:.4}", estimated_cost * 1.5);
    
    if estimated_cost > 5.0 {
        println!("\n⚠️  High Cost Alert!");
        println!("   • Consider using gpt-4o-mini instead");
        println!("   • Reduce content length if possible");
        println!("   • Translate fewer locales per batch");
    } else if estimated_cost < 0.01 {
        println!("\n✨ Very affordable translation! Safe to proceed.");
    } else {
        println!("\n✅ Reasonable cost for quality translation.");
    }
    
    Ok(())
}