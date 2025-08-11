# Status Command Documentation

The `status` command is a critical component of Rosetta Connect that checks App Store Connect version status and provides workflow guidance. This prevents wasted effort on versions that cannot or should not be modified.

## 🎯 Purpose

Before starting any localization work, you need to ensure:
1. **Version is editable** - Not published or under review
2. **Safe to modify** - Won't interfere with App Store processes  
3. **Workflow guidance** - Know exactly what to do next

## 🚀 Quick Start

```bash
# Always start with a status check
rosetta-connect status

# If version is ready (PREPARE_FOR_SUBMISSION), proceed with:
rosetta-connect pull
rosetta-connect translate --locales your-locales
```

## 📋 Command Reference

### Basic Syntax
```bash
rosetta-connect status [OPTIONS]
```

### Available Options

| Option | Description | Default |
|--------|-------------|---------|
| `--detailed` | Show detailed version information | `false` |
| `--all-versions` | Show complete version history | `false` |

## 🎨 Usage Examples

### Daily Workflow Check
```bash
# Quick daily check before starting work
rosetta-connect status

# Detailed view for planning
rosetta-connect status --detailed
```

### Project Management
```bash
# Review all versions for project status
rosetta-connect status --all-versions

# Detailed history analysis
rosetta-connect status --detailed --all-versions
```

## 📊 Version States Explained

### ✅ **Safe to Edit States**

#### `PREPARE_FOR_SUBMISSION`
- **Status**: 🎯 **Perfect for localization work**
- **Description**: Version is being prepared, not yet submitted
- **Action**: Full workflow available (pull → translate → push)
- **Risk Level**: ✅ No risk

#### `DEVELOPER_REJECTED` / `METADATA_REJECTED` / `REJECTED`
- **Status**: ⚠️ **Can be edited with caution**  
- **Description**: Apple rejected the version, needs fixes
- **Action**: Fix rejection issues, then normal workflow
- **Risk Level**: ⚠️ Low risk after addressing rejections

#### `INVALID_BINARY`
- **Status**: ⚠️ **Can be edited**
- **Description**: Binary issues, metadata can still be updated
- **Action**: Normal localization workflow while binary is fixed
- **Risk Level**: ⚠️ Low risk for metadata changes

### 🛑 **DO NOT EDIT States**

#### `WAITING_FOR_REVIEW`
- **Status**: 🛑 **DO NOT TOUCH**
- **Description**: Submitted to Apple, waiting in review queue
- **Action**: Wait for review completion or create new version
- **Risk Level**: 🚨 High risk - will disrupt review process

#### `IN_REVIEW`
- **Status**: 🛑 **DO NOT TOUCH**
- **Description**: Apple is actively reviewing
- **Action**: Wait for review completion
- **Risk Level**: 🚨 Very high risk - will cancel review

#### `PENDING_DEVELOPER_RELEASE`
- **Status**: 🛑 **DO NOT TOUCH**
- **Description**: Approved by Apple, waiting for developer release
- **Action**: Release current version or wait
- **Risk Level**: 🚨 High risk - will affect approved version

#### `READY_FOR_SALE`
- **Status**: 🛑 **DO NOT TOUCH**
- **Description**: Live in App Store
- **Action**: Create new version for any changes
- **Risk Level**: 🚨 Critical - will affect live app

## 🎨 Command Output Examples

### Ready for Work Example
```
🎯 Current Version: 2.15.0 (PREPARE_FOR_SUBMISSION)
   ✅ Status: Ready for editing
   ✅ Safe to proceed with localization work

💡 Smart Workflow Recommendations
🎯 Excellent: Perfect! Version 2.15.0 is ready for localization work.
   📋 Recommended workflow:
   1️⃣  rosetta-connect pull # Get current content  
   2️⃣  rosetta-connect translate --locales <your-locales> # Generate translations
   3️⃣  rosetta-connect push # Upload when ready
```

### Published Version Warning
```
🎯 Current Version: 2.14.1 (READY_FOR_SALE)
   🌟 Status: Published - DO NOT EDIT
   ⚠️ Localization editing NOT recommended

💡 Smart Workflow Recommendations
🌟 Published: Version 2.14.1 is live in App Store.
   📋 For new localization work:
   1️⃣  Create a new app version in App Store Connect
   2️⃣  Run rosetta-connect status to work with new version
   3️⃣  Proceed with normal localization workflow
```

## 🔄 Workflow Integration

### Complete Safe Workflow
```bash
# Step 1: Always check status first
rosetta-connect status
# ✅ Verify version is PREPARE_FOR_SUBMISSION

# Step 2: If ready, proceed with normal workflow  
rosetta-connect pull
rosetta-connect translate --locales zh-Hans,fr-FR,de-DE
rosetta-connect push
```

### Pre-Translation Validation
```bash
# Before any localization sprint
rosetta-connect status --detailed

# Before major translation projects
rosetta-connect status --all-versions
```

### CI/CD Integration
```bash
# In automated pipelines
if rosetta-connect status | grep -q "Ready for editing"; then
  echo "✅ Safe to proceed with localization"
  rosetta-connect pull
  rosetta-connect translate --locales $TARGET_LOCALES
else
  echo "⚠️ Version not ready for localization work"
  exit 1
fi
```

## 🛡️ Safety Features

### Automatic Warnings
- **Red status indicators** for dangerous states
- **Clear "DO NOT EDIT"** messages for risky versions
- **Specific recommendations** for each state

### Smart Guidance
- **Step-by-step workflows** for ready versions
- **Alternative approaches** for blocked states
- **Historical context** with --all-versions

### Risk Prevention
- **Clear visual cues** (✅ ⚠️ 🛑) for quick decision making
- **Detailed explanations** of why each state is safe/unsafe
- **Specific next steps** to avoid workflow mistakes

## 📈 Best Practices

### Daily Workflow
1. **Always start with status check**
2. **Only proceed if version is green** (✅)
3. **Check detailed info** if state is unclear
4. **Review all versions** for project planning

### Team Collaboration
- **Share status output** in team channels
- **Document version states** in project logs
- **Coordinate releases** using status information

### Emergency Situations
```bash
# If you accidentally worked on wrong version
rosetta-connect status --all-versions  # Find correct version
# Switch to correct version in App Store Connect
rosetta-connect status  # Verify new version state
```

## 🔧 Troubleshooting

### Common Issues

**"No versions found"**
- App might not exist in App Store Connect
- Check bundle ID in rosetta.toml
- Verify API credentials

**"Unknown status"**
- New App Store Connect state not recognized
- Check App Store Connect web interface
- Contact support with the unknown state

**"API access failed"**
- Verify .env file configuration
- Check App Store Connect API key permissions
- Ensure network connectivity

### Debug Mode
```bash
export ROSETTA_DEBUG_JS=1
rosetta-connect status
```

## 📊 Performance

### Typical Response Times
- **Status check**: 3-5 seconds
- **Detailed view**: 4-6 seconds  
- **All versions**: 5-8 seconds (depending on version count)

### Network Usage
- **Basic status**: ~10KB API calls
- **Version history**: ~20KB API calls
- **Cached results**: Near instant after first call

## 🎯 Integration with Other Commands

### Pull Command Enhancement
The pull command can integrate status checking:
```bash
# Future enhancement idea
rosetta-connect pull --check-status  # Verify version before pulling
```

### Translate Command Safety
The translate command can verify safety:
```bash  
# Future enhancement idea
rosetta-connect translate --verify-version  # Check status before translating
```

This status command provides the foundation for safe, intelligent localization workflows by ensuring users always work on the correct, editable versions.