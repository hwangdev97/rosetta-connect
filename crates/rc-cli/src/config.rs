use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub assets: Option<AssetsConfig>,
    pub ai: Option<AiConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub bundle_id: String,
    pub default_locale: String,
    pub target_locales: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsConfig {
    #[serde(flatten)]
    pub screenshots: HashMap<String, PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            model: "gpt-4o-mini".to_string(),
            temperature: 0.7,
            max_tokens: 1024,
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        
        toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))
    }
    
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        std::fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;
        
        Ok(())
    }
    
    pub fn create_default(bundle_id: String, default_locale: String) -> Self {
        Self {
            app: AppConfig {
                bundle_id,
                default_locale: default_locale.clone(),
                target_locales: vec!["zh-Hans".to_string(), "fr-FR".to_string(), "de-DE".to_string()],
            },
            assets: Some(AssetsConfig {
                screenshots: {
                    let mut map = HashMap::new();
                    map.insert("default".to_string(), PathBuf::from("./screenshots/en"));
                    map.insert("zh-Hans".to_string(), PathBuf::from("./screenshots/zh"));
                    map
                },
            }),
            ai: Some(AiConfig::default()),
        }
    }
}