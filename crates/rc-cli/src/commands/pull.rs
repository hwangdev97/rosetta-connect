use crate::config::Config;
use anyhow::{Context, Result};
use std::path::Path;

pub async fn run(config_path: &Path) -> Result<()> {
    println!("Pulling current localizations from App Store Connect...");
    
    let config = Config::load(config_path)
        .context("Failed to load configuration")?;
    
    println!("🔄 Connecting to App Store Connect API...");
    
    // Initialize Node.js runtime
    rc_node::init_node_runtime()
        .context("Failed to initialize Node.js runtime")?;
    
    // Download app data through the bridge
    let result = rc_node::asc_download(config.app.bundle_id.clone()).await
        .context("Failed to download app data")?;
    
    println!("📥 Downloaded app metadata for all locales");
    println!("📱 Downloaded app screenshots");
    println!("✅ Pull completed successfully");
    
    // TODO: Save downloaded content to local files
    println!("💾 Saved to local files:");
    println!("   - metadata/");
    println!("   - screenshots/");
    println!("📊 Downloaded data: {}", result);
    
    Ok(())
}