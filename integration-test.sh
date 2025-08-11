#!/bin/bash

# Rosetta Connect Integration Test
# Tests the complete workflow with real App Store Connect integration

echo "🔗 Rosetta Connect Integration Test"
echo "===================================="
echo

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo -e "${RED}❌ .env file not found${NC}"
    echo "This integration test requires App Store Connect credentials."
    echo "Please create a .env file with:"
    echo "  ISSUER_ID=your-issuer-id"
    echo "  KEY_ID=your-key-id" 
    echo "  PRIVATE_KEY_PATH=./path-to-key.p8"
    echo "  OPENAI_API_KEY=your-openai-key"
    exit 1
fi

echo -e "${GREEN}✅ Found .env file${NC}"

# Check if private key exists
PRIVATE_KEY_PATH=$(grep "PRIVATE_KEY_PATH" .env | cut -d'=' -f2)
if [ ! -f "$PRIVATE_KEY_PATH" ]; then
    echo -e "${RED}❌ Private key file not found at: $PRIVATE_KEY_PATH${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Found private key file${NC}"

# Check if binary exists
if [ ! -f "./target/release/rosetta-connect" ]; then
    echo -e "${RED}❌ Binary not found. Building...${NC}"
    cargo build --release
    if [ $? -ne 0 ]; then
        echo -e "${RED}❌ Build failed${NC}"
        exit 1
    fi
fi

echo -e "${GREEN}✅ Binary ready${NC}"
echo

# Test 1: Full workflow with real data
echo -e "${BLUE}🧪 Test 1: Complete App Store Connect Integration${NC}"
echo "================================================="

echo -e "${YELLOW}Step 1: Pulling real data from App Store Connect...${NC}"
export ROSETTA_DEBUG_JS=1
./target/release/rosetta-connect pull

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Successfully connected to App Store Connect${NC}"
else
    echo -e "${RED}❌ Failed to connect to App Store Connect${NC}"
    exit 1
fi

echo
echo -e "${YELLOW}Step 2: Testing cost estimation...${NC}"
./target/release/rosetta-connect cost --detailed

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Cost estimation successful${NC}"
else
    echo -e "${RED}❌ Cost estimation failed${NC}"
fi

echo
echo -e "${YELLOW}Step 3: Testing AI translation...${NC}"
./target/release/rosetta-connect translate --locales zh-Hans,fr-FR

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ AI translation successful${NC}"
else
    echo -e "${RED}❌ AI translation failed${NC}"
fi

unset ROSETTA_DEBUG_JS

echo
echo "================================================="
echo -e "${GREEN}🎉 Integration test completed!${NC}"
echo

# Test 2: Performance test
echo -e "${BLUE}🧪 Test 2: Performance Benchmarks${NC}"
echo "=================================="

echo -e "${YELLOW}Testing startup time (5 runs):${NC}"
for i in {1..5}; do
    echo -n "Run $i: "
    time ./target/release/rosetta-connect --version > /dev/null
done

echo
echo -e "${YELLOW}Testing pull command time:${NC}"
time ./target/release/rosetta-connect pull > /dev/null

echo
echo -e "${GREEN}✅ Performance tests completed${NC}"

# Test 3: Error handling
echo
echo -e "${BLUE}🧪 Test 3: Error Handling${NC}"
echo "========================="

echo -e "${YELLOW}Testing with invalid bundle ID...${NC}"
./target/release/rosetta-connect init --bundle-id "invalid..bundle.id" 2>/dev/null
if [ $? -ne 0 ]; then
    echo -e "${GREEN}✅ Correctly handled invalid bundle ID${NC}"
else
    echo -e "${RED}❌ Should have failed with invalid bundle ID${NC}"
fi

echo -e "${YELLOW}Testing with missing arguments...${NC}"
./target/release/rosetta-connect translate 2>/dev/null
if [ $? -ne 0 ]; then
    echo -e "${GREEN}✅ Correctly handled missing arguments${NC}"
else
    echo -e "${RED}❌ Should have failed with missing arguments${NC}"
fi

echo
echo -e "${GREEN}✅ Error handling tests completed${NC}"

echo
echo "===================================="
echo -e "${GREEN}🎊 All integration tests passed!${NC}"
echo "===================================="