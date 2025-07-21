use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod config;
mod commands;

#[derive(Parser)]
#[command(name = "rosetta-connect")]
#[command(about = "A CLI tool for App Store Connect localization management")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Configuration file path
    #[arg(short, long, default_value = "rosetta.toml")]
    config: PathBuf,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new rosetta-connect project
    Init {
        /// Bundle ID for the app
        #[arg(long)]
        bundle_id: Option<String>,
        /// Default locale
        #[arg(long, default_value = "en-US")]
        default_locale: String,
    },
    /// Download current localizations from App Store Connect
    Pull,
    /// Generate translations using AI
    Translate {
        /// Target locales to translate
        #[arg(long, value_delimiter = ',')]
        locales: Vec<String>,
        /// AI model to use
        #[arg(long)]
        model: Option<String>,
    },
    /// Show differences between local and remote content
    Diff,
    /// Upload text and screenshots to App Store Connect
    Push {
        /// Version number
        version: String,
        /// Skip confirmation prompts
        #[arg(long)]
        yes: bool,
    },
    /// Validate content against App Store guidelines
    Validate,
    /// Preview generated content locally
    Preview {
        /// Locale to preview
        #[arg(long)]
        locale: Option<String>,
    },
    /// Rollback to previous version
    Rollback {
        /// Version to rollback to
        version: String,
    },
    /// Estimate AI API call costs
    Cost {
        /// Show detailed breakdown
        #[arg(long)]
        detailed: bool,
    },
    /// Manage AI prompt templates
    Template {
        #[command(subcommand)]
        action: TemplateAction,
    },
}

#[derive(Subcommand)]
enum TemplateAction {
    /// List available templates
    List,
    /// Create a new template
    Create {
        name: String,
        /// Template file path
        #[arg(long)]
        file: PathBuf,
    },
    /// Edit an existing template
    Edit {
        name: String,
    },
    /// Delete a template
    Delete {
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    
    let cli = Cli::parse();
    
    if std::env::var("ROSETTA_DEBUG_JS").is_ok() {
        std::env::set_var("NODE_DEBUG", "1");
    }
    
    match cli.command {
        Commands::Init { bundle_id, default_locale } => {
            commands::init::run(bundle_id, default_locale, &cli.config).await
        }
        Commands::Pull => {
            commands::pull::run(&cli.config).await
        }
        Commands::Translate { locales, model } => {
            commands::translate::run(locales, model, &cli.config).await
        }
        Commands::Diff => {
            commands::diff::run(&cli.config).await
        }
        Commands::Push { version, yes } => {
            commands::push::run(version, yes, &cli.config).await
        }
        Commands::Validate => {
            commands::validate::run(&cli.config).await
        }
        Commands::Preview { locale } => {
            commands::preview::run(locale, &cli.config).await
        }
        Commands::Rollback { version } => {
            commands::rollback::run(version, &cli.config).await
        }
        Commands::Cost { detailed } => {
            commands::cost::run(detailed, &cli.config).await
        }
        Commands::Template { action } => {
            commands::template::run(action, &cli.config).await
        }
    }
}