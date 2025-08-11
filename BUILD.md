# Build Instructions

This document explains how to build Rosetta Connect with automatic version management.

## 🚀 Build Scripts

### 1. Full Build with Version Bump
```bash
./build.sh
```

**What it does:**
- ✅ Automatically bumps patch version (e.g., 0.1.0 → 0.1.1)
- ✅ Updates version in all relevant files:
  - `crates/rc-cli/Cargo.toml`
  - `crates/rc-node/Cargo.toml` 
  - `js/package.json`
- ✅ Builds TypeScript components
- ✅ Builds Rust release binary
- ✅ Verifies build and shows summary
- ✅ Shows git status with modified files

**Use this for:** Production builds, releases, official versions

### 2. Quick Build (No Version Bump)
```bash
./quick-build.sh
```

**What it does:**
- ✅ Builds TypeScript components
- ✅ Builds Rust release binary
- ✅ Verifies build
- ❌ Does NOT change version numbers

**Use this for:** Development builds, testing, debugging

## 📋 Version Management

### Current Version Scheme
- **Format**: `MAJOR.MINOR.PATCH` (following semantic versioning)
- **Auto-increment**: Patch version only (last number)
- **Examples**: 
  - `0.1.0` → `0.1.1` → `0.1.2` → `0.1.3`

### Manual Version Changes
If you need to bump major or minor versions manually:

1. Edit `crates/rc-cli/Cargo.toml`:
   ```toml
   version = "0.2.0"  # New minor version
   ```

2. Edit `js/package.json`:
   ```json
   "version": "0.2.0"
   ```

3. Edit `crates/rc-node/Cargo.toml`:
   ```toml
   version = "0.2.0"
   ```

4. Run `./quick-build.sh` to build without auto-increment

## 🎯 Build Output

After a successful build:
- **Binary location**: `./target/release/rosetta-connect`
- **Binary size**: ~4.5MB (includes all optimized commands)
- **TypeScript dist**: `js/dist/`
- **Latest features**: 
  - Smart status command with workflow guidance
  - Enhanced pull command with caching, retry logic, and export options
  - Complete localization workflow validation

## 🧪 Verification

Check the build was successful:
```bash
# Check version
./target/release/rosetta-connect --version

# Test basic functionality
./target/release/rosetta-connect --help

# Test complete workflow
./target/release/rosetta-connect status
./target/release/rosetta-connect pull

# Test advanced features
./target/release/rosetta-connect status --detailed --all-versions
./target/release/rosetta-connect pull --format json
./target/release/rosetta-connect pull --force-refresh --retry-count 3
```

## 🔄 Git Workflow

After building with `./build.sh`:

```bash
# Review changes
git status
git diff

# Commit version bump
git add .
git commit -m "Bump version to 0.1.x"

# Tag the release (optional)
git tag v0.1.x
git push origin v0.1.x
```

## 🛠 Prerequisites

Make sure you have:
- ✅ Rust (latest stable)
- ✅ Node.js (v18+)
- ✅ npm dependencies installed (`cd js && npm install`)

## 🚨 Troubleshooting

### Build fails with "command not found"
```bash
chmod +x build.sh
chmod +x quick-build.sh
```

### TypeScript build fails
```bash
cd js
npm install
npm run build
```

### Rust build fails
```bash
cargo clean
cargo build --release
```

### Version not updating correctly
Check that files have write permissions:
```bash
ls -la crates/rc-cli/Cargo.toml
ls -la js/package.json
```

## 📊 Build Performance

Typical build times:
- **TypeScript**: ~2-3 seconds
- **Rust (clean)**: ~30-60 seconds  
- **Rust (incremental)**: ~5-10 seconds
- **Total**: ~10-65 seconds depending on changes