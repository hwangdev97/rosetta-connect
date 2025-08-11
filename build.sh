#!/bin/bash

# Rosetta Connect Build Script with Automatic Version Bumping
# Automatically increments the patch version (0.1.0 -> 0.1.1) before each build

echo "🚀 Rosetta Connect Build Script"
echo "================================"
echo

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to extract version from Cargo.toml
get_current_version() {
    grep "^version" crates/rc-cli/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/'
}

# Function to bump patch version (0.1.0 -> 0.1.1)
bump_patch_version() {
    local version="$1"
    local major=$(echo "$version" | cut -d. -f1)
    local minor=$(echo "$version" | cut -d. -f2)
    local patch=$(echo "$version" | cut -d. -f3)
    local new_patch=$((patch + 1))
    echo "${major}.${minor}.${new_patch}"
}

# Function to update version in Cargo.toml
update_cargo_version() {
    local file="$1"
    local new_version="$2"
    sed -i.bak "s/^version = \".*\"/version = \"${new_version}\"/" "$file"
    rm -f "${file}.bak"
}

# Function to update version in package.json
update_package_version() {
    local file="$1"
    local new_version="$2"
    sed -i.bak "s/\"version\": \".*\"/\"version\": \"${new_version}\"/" "$file"
    rm -f "${file}.bak"
}

# Step 1: Get current version
echo -e "${BLUE}📋 Step 1: Version Management${NC}"
current_version=$(get_current_version)
echo "Current version: $current_version"

# Step 2: Bump version
new_version=$(bump_patch_version "$current_version")
echo -e "${GREEN}Bumping version: $current_version → $new_version${NC}"

# Step 3: Update version in all relevant files
echo -e "${YELLOW}📝 Updating version in project files...${NC}"

# Update Rust CLI Cargo.toml
update_cargo_version "crates/rc-cli/Cargo.toml" "$new_version"
echo "✅ Updated crates/rc-cli/Cargo.toml"

# Update Node.js package.json
update_package_version "js/package.json" "$new_version"
echo "✅ Updated js/package.json"

# Update root Node.js bridge Cargo.toml if it has a version
if grep -q "^version" crates/rc-node/Cargo.toml; then
    update_cargo_version "crates/rc-node/Cargo.toml" "$new_version"
    echo "✅ Updated crates/rc-node/Cargo.toml"
fi

echo

# Step 4: Build TypeScript components
echo -e "${BLUE}📋 Step 2: Building TypeScript Components${NC}"
echo "Building TypeScript..."
cd js
npm run build
build_status=$?

if [ $build_status -ne 0 ]; then
    echo -e "${RED}❌ TypeScript build failed${NC}"
    exit 1
fi

cd ..
echo -e "${GREEN}✅ TypeScript build successful${NC}"
echo

# Step 5: Build Rust components
echo -e "${BLUE}📋 Step 3: Building Rust Components${NC}"
echo "Building Rust release binary..."
cargo build --release

build_status=$?

if [ $build_status -ne 0 ]; then
    echo -e "${RED}❌ Rust build failed${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Rust build successful${NC}"
echo

# Step 6: Verify build
echo -e "${BLUE}📋 Step 4: Build Verification${NC}"
if [ -f "./target/release/rosetta-connect" ]; then
    file_size=$(ls -lh ./target/release/rosetta-connect | awk '{print $5}')
    echo -e "${GREEN}✅ Binary created successfully${NC}"
    echo "   📁 Location: ./target/release/rosetta-connect"
    echo "   📏 Size: $file_size"
    
    # Test version output
    version_output=$(./target/release/rosetta-connect --version)
    echo "   📋 Version: $version_output"
else
    echo -e "${RED}❌ Binary not found${NC}"
    exit 1
fi

echo

# Step 7: Show build summary
echo -e "${BLUE}📊 Build Summary${NC}"
echo "================================"
echo -e "🏷️  Version: ${GREEN}$current_version → $new_version${NC}"
echo -e "📦 TypeScript: ${GREEN}Built${NC}"
echo -e "🦀 Rust: ${GREEN}Built${NC}"
echo -e "🎯 Binary: ${GREEN}Ready${NC}"
echo
echo -e "${GREEN}🎉 Build completed successfully!${NC}"
echo
echo "You can now run:"
echo "  ./target/release/rosetta-connect --version"
echo "  ./target/release/rosetta-connect --help"
echo

# Step 8: Optional - show git status
if git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${YELLOW}💡 Git Status:${NC}"
    echo "Modified files with version updates:"
    git status --porcelain | grep -E "(Cargo\.toml|package\.json)" || echo "  (No version files modified)"
    echo
    echo "To commit version bump:"
    echo "  git add ."
    echo "  git commit -m \"Bump version to $new_version\""
fi