#!/bin/bash

# Quick Build Script (without version bump)
# For development builds when you don't want to increment version

echo "⚡ Quick Build (no version bump)"
echo "==============================="

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

# Build TypeScript
echo -e "${BLUE}🔨 Building TypeScript...${NC}"
cd js && npm run build && cd ..

# Build Rust
echo -e "${BLUE}🦀 Building Rust...${NC}" 
cargo build --release

# Verify
if [ -f "./target/release/rosetta-connect" ]; then
    version_output=$(./target/release/rosetta-connect --version)
    echo -e "${GREEN}✅ Build complete: $version_output${NC}"
else
    echo "❌ Build failed"
    exit 1
fi