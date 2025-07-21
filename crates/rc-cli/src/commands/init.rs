use crate::config::Config;
use anyhow::{Context, Result};
use std::path::Path;

pub async fn run(bundle_id: Option<String>, default_locale: String, config_path: &Path) -> Result<()> {
    println!("Initializing new rosetta-connect project...");
    
    if config_path.exists() {
        anyhow::bail!("Configuration file already exists: {}", config_path.display());
    }
    
    let bundle_id = match bundle_id {
        Some(id) => id,
        None => {
            println!("Please enter your app's bundle ID (e.g., com.example.myapp):");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)
                .context("Failed to read bundle ID")?;
            input.trim().to_string()
        }
    };
    
    if bundle_id.is_empty() {
        anyhow::bail!("Bundle ID cannot be empty");
    }
    
    let config = Config::create_default(bundle_id.clone(), default_locale.clone());
    config.save(config_path)
        .context("Failed to save configuration file")?;
    
    std::fs::create_dir_all("screenshots/en")
        .context("Failed to create screenshots directory")?;
    std::fs::create_dir_all("screenshots/zh")
        .context("Failed to create screenshots directory")?;
    
    let env_content = r#"# App Store Connect API credentials
ISSUER_ID=your_issuer_id_here
KEY_ID=your_key_id_here
PRIVATE_KEY_PATH=./AuthKey_YOUR_KEY_ID.p8

# OpenAI API key (optional, for AI translations)
OPENAI_API_KEY=your_openai_api_key_here
"#;
    
    if !Path::new(".env").exists() {
        std::fs::write(".env", env_content)
            .context("Failed to create .env file")?;
        println!("Created .env file - please fill in your API credentials");
    }
    
    println!("✅ Successfully initialized rosetta-connect project");
    println!("📱 Bundle ID: {}", bundle_id);
    println!("🌍 Default locale: {}", default_locale);
    println!("📝 Configuration saved to: {}", config_path.display());
    println!("🔑 Please edit .env file with your API credentials");
    
    Ok(())
}