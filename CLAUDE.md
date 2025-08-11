# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rosetta Connect is a Rust-based CLI tool for automating App Store Connect localization workflows with AI-powered translation. It uses a hybrid architecture combining Rust for performance with Node.js/TypeScript for App Store Connect SDK integration.

## Build Commands

```bash
# Build the release binary
cargo build --release

# The main executable will be at:
./target/release/rosetta-connect

# Build TypeScript components
cd js && npm run build

# Watch TypeScript development
cd js && npm run watch
```

## Development Commands

```bash
# Run the CLI tool (after building)
./target/release/rosetta-connect --help

# Complete localization workflow:
./target/release/rosetta-connect init --bundle-id com.demo.testapp
./target/release/rosetta-connect status                                    # Check version readiness
./target/release/rosetta-connect pull
./target/release/rosetta-connect translate --locales zh-Hans,fr-FR
./target/release/rosetta-connect cost --detailed

# Advanced status command options:
./target/release/rosetta-connect status --detailed --all-versions
./target/release/rosetta-connect status                                    # Quick status check

# Advanced pull command options:
./target/release/rosetta-connect pull --force-refresh --format json
./target/release/rosetta-connect pull --locales en-US,zh-Hans --export ./data.csv
./target/release/rosetta-connect pull --retry-count 5 --format csv

# Test with the demo project
cd test-demo && ../target/release/rosetta-connect pull
```

## Code Architecture

### Workspace Structure
- `crates/rc-cli/`: Main CLI application with command implementations
- `crates/rc-node/`: Node.js bridge layer (currently using mock implementations for MVP)
- `js/`: TypeScript wrapper for App Store Connect SDK integration

### Key Components
- **Configuration System**: TOML-based config in `rosetta.toml` with environment variable support
- **Command Structure**: Each command is implemented in `crates/rc-cli/src/commands/`
- **Bridge Pattern**: Rust calls into Node.js/TypeScript for external API integrations
- **Progress UI**: Uses `indicatif` for progress bars and status displays
- **Caching System**: Smart caching in `.rosetta-cache/` directory with 1-hour expiration
- **Data Persistence**: Structured data storage in `{bundle_id}/current/` and `{bundle_id}/{version}/`
- **Export System**: Multi-format export support (JSON, CSV, table)

### Command Implementation Pattern
Commands follow a consistent async pattern in `crates/rc-cli/src/commands/`:
- Each command has its own module (e.g., `init.rs`, `translate.rs`, `pull.rs`)
- Commands receive config and parameters, return `anyhow::Result<()>`
- Configuration is loaded from `rosetta.toml` using the config system in `config.rs`

### Current Status
- CLI framework and UI are complete and functional
- **Enhanced Pull Command**: Full-featured with caching, retry logic, validation, and export options
- **Smart Status Command**: Version status checking with workflow guidance
- Real API integrations with App Store Connect are active and working
- Node.js bridge provides real data from App Store Connect API
- OpenAI integration ready for translation workflows
- Robust error handling, progress reporting, and data persistence
- Complete workflow validation and safety checks

### Configuration File
The main configuration is in `rosetta.toml`:
- `[app]` section: bundle_id, locales
- `[assets]` section: screenshot paths
- `[ai]` section: OpenAI/translation settings

### Development Environment
- Enable debug mode with: `export ROSETTA_DEBUG_JS=1`
- Node.js components are in early development stage
- Focus on Rust CLI code for current development work

## Status Command Documentation

The `status` command is essential for checking version readiness before starting localization work. It prevents working on published or in-review versions.

### Basic Usage
```bash
# Quick status check
./target/release/rosetta-connect status

# Detailed version information
./target/release/rosetta-connect status --detailed

# View all version history
./target/release/rosetta-connect status --all-versions
```

### Version States & Recommendations

**✅ Safe to Edit:**
- `PREPARE_FOR_SUBMISSION` - Ready for localization work
- `DEVELOPER_REJECTED` - Can be edited after fixing issues
- `METADATA_REJECTED` - Can be edited with corrections
- `INVALID_BINARY` - Can be edited while fixing binary

**⚠️ DO NOT EDIT:**
- `WAITING_FOR_REVIEW` - In Apple's review queue
- `IN_REVIEW` - Currently being reviewed
- `PENDING_DEVELOPER_RELEASE` - Approved, waiting for release
- `READY_FOR_SALE` - Published to App Store

### Smart Workflow Guidance
The status command provides specific recommendations:
- **Ready versions**: Complete step-by-step workflow
- **Rejected versions**: Fix issues then proceed
- **In-review versions**: Wait or create new version
- **Published versions**: Create new version for changes

## Pull Command Documentation

The `pull` command is the core feature for downloading App Store Connect localization data. It has been fully optimized with enterprise-grade features:

### Basic Usage
```bash
# Simple pull (uses cache if available)
./target/release/rosetta-connect pull

# Force refresh from API
./target/release/rosetta-connect pull --force-refresh

# Pull specific locales only
./target/release/rosetta-connect pull --locales en-US,zh-Hans,fr-FR
```

### Output Formats
```bash
# Table format (default, human-readable)
./target/release/rosetta-connect pull --format table

# JSON format (for automation/scripts)
./target/release/rosetta-connect pull --format json

# CSV format (for spreadsheets)
./target/release/rosetta-connect pull --format csv
```

### Export Options
```bash
# Export to file
./target/release/rosetta-connect pull --export ./locales.json --format json
./target/release/rosetta-connect pull --export ./locales.csv --format csv

# Combined: filter locales and export
./target/release/rosetta-connect pull --locales zh-Hans,ja --export ./asian-locales.csv --format csv
```

### Advanced Options
```bash
# Custom retry count for network issues
./target/release/rosetta-connect pull --retry-count 5

# Combine multiple options
./target/release/rosetta-connect pull --force-refresh --retry-count 3 --format json --export ./fresh-data.json
```

### Features
- **🚀 Smart Caching**: 1-hour cache prevents unnecessary API calls
- **🔄 Retry Logic**: Exponential backoff for network failures  
- **🔐 Validation**: Bundle ID format validation and API access verification
- **📊 Multiple Formats**: Table, JSON, and CSV output
- **📤 Export Support**: Save data to files in any format
- **🌍 Locale Filtering**: Pull only the languages you need
- **📈 Progress Tracking**: Real-time progress bars and status updates
- **💾 Data Persistence**: Structured local storage for translation workflows

### Data Storage Structure
```
.rosetta-cache/
└── {bundle_id}/
    └── pull_cache.json

{bundle_id}/
├── current/
│   ├── summary.json
│   └── {locale}/
│       └── metadata.json
└── {version}/
    └── {locale}/
        ├── metadata.json
        ├── screenshots.json
        └── screenshots/
```

## Important Notes

- Real API integrations with App Store Connect are active and working
- Configuration follows a cascading priority: CLI args > env vars > config file > defaults
- All commands are async and use proper error handling with `anyhow` and `color-eyre`
- Caching significantly improves performance and reduces API usage