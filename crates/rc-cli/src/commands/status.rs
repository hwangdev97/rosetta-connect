use crate::config::Config;
use anyhow::{Context, Result};
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use serde_json::Value;
use owo_colors::OwoColorize;

pub async fn run(config_path: &Path, all_versions: bool, detailed: bool) -> Result<()> {
    println!("🔍 Checking App Store Connect version status...");
    
    let config = Config::load(config_path)
        .context("Failed to load configuration")?;
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message("Fetching version information...");
    
    // Initialize Node.js runtime
    rc_node::init_node_runtime()
        .context("Failed to initialize Node.js runtime")?;
    
    // Get version status information
    let version_info = rc_node::asc_get_version_status(config.app.bundle_id.clone()).await
        .context("Failed to get version status")?;
    
    pb.finish_with_message("✅ Version information retrieved");
    
    // Display version status
    display_version_status(&version_info, all_versions, detailed)?;
    
    // Provide workflow recommendations
    provide_workflow_recommendations(&version_info)?;
    
    Ok(())
}

fn display_version_status(version_info: &Value, all_versions: bool, detailed: bool) -> Result<()> {
    println!("\n📱 {} App Status", "App Store Connect".bold());
    println!("{}", "━".repeat(50));
    
    if let Some(app_name) = version_info.get("appName").and_then(|v| v.as_str()) {
        println!("📱 App: {}", app_name.bright_blue());
    }
    
    if let Some(bundle_id) = version_info.get("bundleId").and_then(|v| v.as_str()) {
        println!("🔗 Bundle ID: {}", bundle_id.bright_cyan());
    }
    
    // Display current version status
    if let Some(current_version) = version_info.get("currentVersion").and_then(|v| v.as_object()) {
        display_single_version(current_version, true, detailed)?;
    }
    
    // Display all versions if requested
    if all_versions {
        if let Some(versions) = version_info.get("allVersions").and_then(|v| v.as_array()) {
            println!("\n📚 {} Version History", "All".bold());
            println!("{}", "─".repeat(30));
            
            for version in versions {
                if let Some(version_obj) = version.as_object() {
                    display_single_version(version_obj, false, false)?;
                }
            }
        }
    }
    
    Ok(())
}

fn display_single_version(version: &serde_json::Map<String, Value>, is_current: bool, detailed: bool) -> Result<()> {
    let version_string = version.get("versionString").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let state = version.get("appStoreState").and_then(|v| v.as_str()).unwrap_or("Unknown");
    
    let prefix = if is_current { "🎯 Current Version:" } else { "📦 Version:" };
    
    println!("\n{} {} {}", prefix, version_string.bright_yellow(), format!("({})", state).dimmed());
    
    // Display status with color coding
    let (status_icon, status_text, can_edit) = match state {
        "PREPARE_FOR_SUBMISSION" => ("✅", "Ready for editing".green().to_string(), true),
        "DEVELOPER_REJECTED" | "METADATA_REJECTED" | "REJECTED" => ("⚠️", "Can be edited (rejected)".yellow().to_string(), true),
        "INVALID_BINARY" => ("❌", "Can be edited (invalid binary)".yellow().to_string(), true),
        "WAITING_FOR_REVIEW" => ("⏳", "Waiting for review - DO NOT EDIT".red().to_string(), false),
        "IN_REVIEW" => ("🔍", "In review - DO NOT EDIT".red().to_string(), false),
        "PENDING_DEVELOPER_RELEASE" => ("🚀", "Pending release - DO NOT EDIT".red().to_string(), false),
        "READY_FOR_SALE" => ("🌟", "Published - DO NOT EDIT".red().to_string(), false),
        _ => ("❓", format!("Unknown status: {}", state).bright_red().to_string(), false),
    };
    
    println!("   {} Status: {}", status_icon, status_text);
    
    if detailed {
        // Show additional details if available
        if let Some(created_date) = version.get("createdDate").and_then(|v| v.as_str()) {
            println!("   📅 Created: {}", created_date.bright_blue());
        }
        
        if let Some(review_type) = version.get("reviewType").and_then(|v| v.as_str()) {
            println!("   📋 Review Type: {}", review_type.bright_cyan());
        }
        
        if let Some(downloadable) = version.get("downloadable").and_then(|v| v.as_bool()) {
            let status = if downloadable { "Yes".green().to_string() } else { "No".red().to_string() };
            println!("   📥 Downloadable: {}", status);
        }
    }
    
    // Show what user can do
    if can_edit {
        println!("   {} {}", "✅".green(), "Safe to proceed with localization work".green());
    } else {
        println!("   {} {}", "⚠️".red(), "Localization editing NOT recommended".red());
    }
    
    Ok(())
}

fn provide_workflow_recommendations(version_info: &Value) -> Result<()> {
    println!("\n💡 {} Workflow Recommendations", "Smart".bold());
    println!("{}", "━".repeat(40));
    
    if let Some(current_version) = version_info.get("currentVersion").and_then(|v| v.as_object()) {
        let state = current_version.get("appStoreState").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let version_string = current_version.get("versionString").and_then(|v| v.as_str()).unwrap_or("Unknown");
        
        match state {
            "PREPARE_FOR_SUBMISSION" => {
                println!("🎯 {} Perfect! Version {} is ready for localization work.", "Excellent:".green().bold(), version_string.bright_yellow());
                println!("   📋 Recommended workflow:");
                println!("   1️⃣  {} {}", "rosetta-connect pull".bright_cyan(), "# Get current content");
                println!("   2️⃣  {} {}", "rosetta-connect translate --locales <your-locales>".bright_cyan(), "# Generate translations");
                println!("   3️⃣  {} {}", "rosetta-connect push".bright_cyan(), "# Upload when ready");
            },
            
            "DEVELOPER_REJECTED" | "METADATA_REJECTED" | "REJECTED" => {
                println!("⚠️  {} Version {} was rejected but can be edited.", "Attention:".yellow().bold(), version_string.bright_yellow());
                println!("   📋 Recommended actions:");
                println!("   1️⃣  Review rejection reasons in App Store Connect");
                println!("   2️⃣  Fix issues and update localizations if needed");
                println!("   3️⃣  Use normal workflow: pull → translate → push");
            },
            
            "WAITING_FOR_REVIEW" | "IN_REVIEW" => {
                println!("🛑 {} Version {} is in review process.", "Stop:".red().bold(), version_string.bright_yellow());
                println!("   ❌ Do NOT modify localizations now");
                println!("   💡 Options:");
                println!("   • Wait for review completion");
                println!("   • Create a new version if urgent changes needed");
            },
            
            "READY_FOR_SALE" => {
                println!("🌟 {} Version {} is live in App Store.", "Published:".green().bold(), version_string.bright_yellow());
                println!("   📋 For new localization work:");
                println!("   1️⃣  Create a new app version in App Store Connect");
                println!("   2️⃣  Run {} to work with new version", "rosetta-connect status".bright_cyan());
                println!("   3️⃣  Proceed with normal localization workflow");
            },
            
            _ => {
                println!("❓ {} Unknown status for version {}.", "Unknown:".yellow().bold(), version_string.bright_yellow());
                println!("   📋 Please check App Store Connect manually");
                println!("   💡 Proceed with caution");
            }
        }
    }
    
    // Additional general recommendations
    println!("\n📚 {} General Tips", "Pro".bold());
    println!("   • Always run {} before starting localization work", "rosetta-connect status".bright_cyan());
    println!("   • Use {} to see version changes", "rosetta-connect status --detailed".bright_cyan());
    println!("   • Check {} for historical context", "rosetta-connect status --all-versions".bright_cyan());
    
    Ok(())
}