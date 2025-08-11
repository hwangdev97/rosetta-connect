# Pull Command Documentation

The `pull` command is the core feature of Rosetta Connect for downloading and managing App Store Connect localization data. This document provides comprehensive documentation for all available features and options.

## 🚀 Quick Start

```bash
# Basic pull - downloads all locales from App Store Connect
./target/release/rosetta-connect pull

# Pull with force refresh - bypasses cache
./target/release/rosetta-connect pull --force-refresh
```

## 📋 Command Reference

### Basic Syntax
```bash
rosetta-connect pull [OPTIONS]
```

### Available Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--force-refresh` | | Force refresh, skip cache | `false` |
| `--retry-count <COUNT>` | | Number of retry attempts for network failures | `3` |
| `--format <FORMAT>` | | Output format: table, json, csv | `table` |
| `--export <FILE>` | | Export to file | None |
| `--locales <LOCALES>` | | Filter specific locales (comma-separated) | All |

## 🎯 Usage Examples

### Basic Operations
```bash
# Simple pull (uses cache if available)
rosetta-connect pull

# Force refresh from API
rosetta-connect pull --force-refresh

# Pull with custom retry count
rosetta-connect pull --retry-count 5
```

### Output Formats
```bash
# Human-readable table (default)
rosetta-connect pull --format table

# JSON output for scripts/automation
rosetta-connect pull --format json

# CSV output for spreadsheets
rosetta-connect pull --format csv
```

### Locale Filtering
```bash
# Pull only English locales
rosetta-connect pull --locales en-US,en-GB,en-AU

# Pull Asian locales
rosetta-connect pull --locales zh-Hans,zh-Hant,ja,ko

# Pull European locales
rosetta-connect pull --locales de-DE,fr-FR,es-ES,it-IT
```

### Export Options
```bash
# Export to JSON file
rosetta-connect pull --export ./app-locales.json --format json

# Export to CSV file
rosetta-connect pull --export ./app-locales.csv --format csv

# Export filtered data
rosetta-connect pull --locales zh-Hans,ja --export ./asian-locales.csv --format csv
```

### Advanced Combinations
```bash
# Complete workflow: fresh data, specific locales, export to CSV
rosetta-connect pull --force-refresh --locales en-US,zh-Hans,fr-FR --export ./main-locales.csv --format csv

# Robust network handling: high retry count with JSON export
rosetta-connect pull --retry-count 10 --export ./backup-data.json --format json
```

## 🏗️ Features Overview

### 🚀 Smart Caching System
- **Automatic caching**: Downloaded data is cached for 1 hour
- **Cache location**: `.rosetta-cache/{bundle_id}/pull_cache.json`
- **Cache validation**: Automatic expiration after 1 hour
- **Force refresh**: `--force-refresh` bypasses cache completely

### 🔄 Retry Logic
- **Exponential backoff**: 2^attempt seconds wait time
- **Configurable retries**: Use `--retry-count` to adjust
- **Network resilience**: Handles temporary network failures gracefully
- **Error reporting**: Detailed failure information on final failure

### 🔐 Validation & Security
- **Bundle ID validation**: Regex validation of bundle ID format
- **API access verification**: Pre-flight checks before downloading
- **Configuration validation**: Ensures all required config is present
- **Error prevention**: Catches issues early to avoid wasted API calls

### 📊 Output Formats

#### Table Format (Default)
- **Human-readable**: Beautifully formatted tables with colors
- **Comprehensive**: Shows default locale details and multi-locale status
- **Status indicators**: Visual status for each locale (上线/未上线/有错误)
- **Screenshot counts**: Shows available screenshots per locale

#### JSON Format
- **Machine-readable**: Perfect for scripts and automation
- **Complete data**: All metadata, screenshots, and status information
- **Structured**: Organized by locale with consistent schema
- **Filterable**: Works with jq and other JSON tools

#### CSV Format
- **Spreadsheet-friendly**: Opens in Excel, Google Sheets, etc.
- **Flat structure**: One row per locale with key information
- **Exportable**: Easy to import into other systems
- **Status summary**: Includes completion status for each locale

### 🌍 Locale Filtering
- **Selective downloads**: Only download specific locales
- **Bandwidth savings**: Reduces API calls and storage
- **Workflow optimization**: Focus on languages you need
- **Comma-separated**: Simple syntax for multiple locales

### 📤 Export System
- **File output**: Save data to disk in any format
- **Path creation**: Automatically creates parent directories
- **Format matching**: Export format matches `--format` option
- **Integration ready**: Perfect for CI/CD pipelines

### 📈 Progress Reporting
- **Real-time progress**: Live progress bars and status updates
- **Time tracking**: Shows download duration
- **Status emojis**: Visual indicators for different states
- **Network feedback**: Shows retry attempts and backoff

## 💾 Data Storage

### Cache Structure
```
.rosetta-cache/
└── {bundle_id}/
    └── pull_cache.json          # Cached API response
```

### Persistent Storage
```
{bundle_id}/
├── current/                     # Latest data for workflows
│   ├── summary.json            # App-level summary
│   └── {locale}/
│       └── metadata.json       # Locale-specific data
└── {version}/                  # Version-specific data
    └── {locale}/
        ├── metadata.json       # App Store metadata
        ├── screenshots.json    # Screenshot manifest
        └── screenshots/        # Downloaded screenshot files
            └── {device_type}/
                ├── 01-{name}.png
                ├── 02-{name}.png
                └── 03-{name}.png
```

## 🎨 Status Indicators

### Console Output
- 🚀 **Starting operations**
- 🔑 **Authentication/verification**
- ✅ **Success states**
- ⚠️ **Warnings/retries**
- ❌ **Errors/failures**
- 📋 **Cache operations**
- 📊 **Data processing**

### Locale Status
- **上线** (Online): Complete and ready for App Store
- **未上线** (Offline): Missing required content
- **有错误** (Has Errors): Content validation errors

## 🔧 Configuration

### Required Configuration
The pull command requires a valid `rosetta.toml` configuration file:

```toml
[app]
bundle_id = "com.example.myapp"
default_locale = "en-US"

[locales]
supported = ["en-US", "zh-Hans", "fr-FR", "de-DE"]
```

### Environment Variables
- `ROSETTA_DEBUG_JS=1`: Enable JavaScript debugging output
- API credentials should be in `.env` file or environment

## 🚨 Error Handling

### Common Errors
1. **Invalid Bundle ID**: Check format (com.company.app)
2. **API Authentication**: Verify App Store Connect credentials
3. **Network Issues**: Use `--retry-count` for unstable connections
4. **Disk Space**: Ensure sufficient space for downloads

### Debug Mode
```bash
export ROSETTA_DEBUG_JS=1
rosetta-connect pull
```

## 🎯 Best Practices

### Development Workflow
```bash
# Daily development: use cache
rosetta-connect pull

# Before translation work: fresh data
rosetta-connect pull --force-refresh

# For specific language work: filter locales
rosetta-connect pull --locales zh-Hans,zh-Hant
```

### CI/CD Integration
```bash
# Automated data export for builds
rosetta-connect pull --format json --export ./locales.json

# Backup workflow with retries
rosetta-connect pull --retry-count 5 --export ./backup.json --format json
```

### Performance Optimization
- Use cache for repeated operations
- Filter locales when working on specific languages
- Use `--retry-count` for unstable network environments
- Export data once and reuse for multiple operations

## 📊 Performance Metrics

### Typical Performance
- **Cache hit**: < 1 second
- **API download**: 10-30 seconds (depends on locale count)
- **With screenshots**: 30-60 seconds (depends on image count)
- **Export operations**: 1-3 seconds

### Network Usage
- **Metadata only**: ~50-100KB per locale
- **With screenshots**: ~500KB-2MB per locale
- **Cache efficiency**: 95%+ cache hit rate in development

This comprehensive pull command provides enterprise-grade reliability and flexibility for managing App Store Connect localization workflows.