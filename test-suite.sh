#!/bin/bash

# Rosetta Connect Test Suite
# Tests all major functionality of the CLI tool

echo "🧪 Rosetta Connect Test Suite"
echo "============================="
echo

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0
TOTAL_TESTS=0

# Function to run a test
run_test() {
    local test_name="$1"
    local command="$2"
    local expected_exit_code="${3:-0}"
    
    echo -e "${BLUE}🔍 Testing: ${test_name}${NC}"
    echo "Command: $command"
    echo "----------------------------------------"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Run the command and capture exit code
    eval "$command"
    local actual_exit_code=$?
    
    if [ $actual_exit_code -eq $expected_exit_code ]; then
        echo -e "${GREEN}✅ PASSED${NC}: $test_name"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}❌ FAILED${NC}: $test_name (expected exit code $expected_exit_code, got $actual_exit_code)"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    echo
    echo "========================================="
    echo
}

# Function to check if binary exists
check_binary() {
    if [ ! -f "./target/release/rosetta-connect" ]; then
        echo -e "${RED}❌ Error: Binary not found at ./target/release/rosetta-connect${NC}"
        echo "Please run 'cargo build --release' first"
        exit 1
    fi
}

# Function to backup and create test config
setup_test_config() {
    if [ -f "rosetta.toml" ]; then
        cp rosetta.toml rosetta.toml.backup
        echo -e "${YELLOW}📋 Backed up existing rosetta.toml${NC}"
    fi
}

# Function to restore config
cleanup_test_config() {
    if [ -f "rosetta.toml.backup" ]; then
        mv rosetta.toml.backup rosetta.toml
        echo -e "${YELLOW}📋 Restored original rosetta.toml${NC}"
    fi
}

# Start testing
echo -e "${BLUE}🚀 Starting Rosetta Connect Test Suite${NC}"
echo

# Check prerequisites
check_binary

# Setup
setup_test_config

# Test 1: Version check
run_test "Version Check" "./target/release/rosetta-connect --version"

# Test 2: Help command
run_test "Help Command" "./target/release/rosetta-connect --help"

# Test 3: Initialize new project
run_test "Project Initialization" "./target/release/rosetta-connect init --bundle-id com.test.app"

# Test 4: Cost estimation (should work without real API calls)
run_test "Cost Estimation" "./target/release/rosetta-connect cost"

# Test 5: Detailed cost estimation
run_test "Detailed Cost Estimation" "./target/release/rosetta-connect cost --detailed"

# Test 6: App Store Connect pull (with real credentials if available)
if [ -f ".env" ]; then
    echo -e "${YELLOW}📋 Found .env file, testing with real App Store Connect${NC}"
    run_test "App Store Connect Pull" "./target/release/rosetta-connect pull"
else
    echo -e "${YELLOW}📋 No .env file found, testing with mock data${NC}"
    run_test "App Store Connect Pull (Mock)" "./target/release/rosetta-connect pull"
fi

# Test 7: Translation with specific locales
run_test "AI Translation" "./target/release/rosetta-connect translate --locales zh-Hans,fr-FR"

# Test 8: Translation with single locale
run_test "AI Translation (Single Locale)" "./target/release/rosetta-connect translate --locales zh-Hans"

# Test 9: Invalid command (should fail)
run_test "Invalid Command" "./target/release/rosetta-connect invalid-command" 1

# Test 10: Missing required arguments (should fail)
run_test "Missing Bundle ID" "./target/release/rosetta-connect init" 1

# Performance tests
echo -e "${BLUE}⚡ Performance Tests${NC}"
echo "----------------------------------------"

# Test startup time
echo -e "${BLUE}🔍 Testing startup time${NC}"
time ./target/release/rosetta-connect --version > /dev/null
echo

# Test with debug mode
if [ -f ".env" ]; then
    echo -e "${BLUE}🔍 Testing with debug mode${NC}"
    export ROSETTA_DEBUG_JS=1
    ./target/release/rosetta-connect pull > /dev/null 2>&1
    unset ROSETTA_DEBUG_JS
    echo -e "${GREEN}✅ Debug mode test completed${NC}"
    echo
fi

# Cleanup
cleanup_test_config

# Summary
echo "========================================="
echo -e "${BLUE}📊 Test Summary${NC}"
echo "========================================="
echo -e "Total tests: ${TOTAL_TESTS}"
echo -e "${GREEN}Passed: ${TESTS_PASSED}${NC}"
echo -e "${RED}Failed: ${TESTS_FAILED}${NC}"
echo

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}💥 Some tests failed. Please check the output above.${NC}"
    exit 1
fi