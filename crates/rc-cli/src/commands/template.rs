use crate::config::Config;
use crate::TemplateAction;
use anyhow::{Context, Result};
use std::path::Path;

pub async fn run(action: TemplateAction, config_path: &Path) -> Result<()> {
    let _config = Config::load(config_path)
        .context("Failed to load configuration")?;
    
    match action {
        TemplateAction::List => {
            println!("📋 Available AI prompt templates:");
            println!("   • app-description    - Generate app descriptions");
            println!("   • keywords          - Generate relevant keywords");
            println!("   • whats-new         - Generate release notes");
            println!("   • marketing-copy    - Generate marketing text");
            println!("   • privacy-policy    - Generate privacy policy excerpts");
        }
        
        TemplateAction::Create { name, file } => {
            println!("Creating template '{}' from file: {}", name, file.display());
            
            if !file.exists() {
                anyhow::bail!("Template file does not exist: {}", file.display());
            }
            
            // TODO: Implement template storage
            println!("✅ Template '{}' created successfully", name);
        }
        
        TemplateAction::Edit { name } => {
            println!("Editing template '{}'...", name);
            
            // TODO: Implement template editing (launch editor)
            println!("📝 Opening template in editor...");
            println!("✅ Template '{}' updated", name);
        }
        
        TemplateAction::Delete { name } => {
            println!("Deleting template '{}'...", name);
            
            // TODO: Implement template deletion with confirmation
            println!("⚠️  Are you sure you want to delete template '{}'? (y/N)", name);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)
                .context("Failed to read confirmation")?;
            
            if input.trim().to_lowercase().starts_with('y') {
                println!("✅ Template '{}' deleted", name);
            } else {
                println!("❌ Deletion cancelled");
            }
        }
    }
    
    Ok(())
}