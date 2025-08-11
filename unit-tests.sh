#!/bin/bash

# Rosetta Connect Unit Tests
# Tests individual components and functions

echo "🔬 Rosetta Connect Unit Tests"
echo "=============================="
echo

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

TESTS_PASSED=0
TESTS_FAILED=0

test_result() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ PASSED${NC}: $1"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}❌ FAILED${NC}: $1"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

# Test 1: Binary exists and is executable
echo -e "${BLUE}🧪 Testing Binary Availability${NC}"
[ -f "./target/release/rosetta-connect" ] && [ -x "./target/release/rosetta-connect" ]
test_result "Binary exists and is executable"

# Test 2: Version output format
echo -e "${BLUE}🧪 Testing Version Output${NC}"
VERSION_OUTPUT=$(./target/release/rosetta-connect --version)
echo "$VERSION_OUTPUT" | grep -q "rosetta-connect"
test_result "Version contains 'rosetta-connect'"

# Test 3: Help command provides usage info
echo -e "${BLUE}🧪 Testing Help Command${NC}"
HELP_OUTPUT=$(./target/release/rosetta-connect --help)
echo "$HELP_OUTPUT" | grep -q "Usage:"
test_result "Help command shows usage"

echo "$HELP_OUTPUT" | grep -q "Commands:"
test_result "Help command shows commands"

# Test 4: Available commands
echo -e "${BLUE}🧪 Testing Available Commands${NC}"
for cmd in "init" "pull" "translate" "cost" "push" "validate" "preview" "rollback" "template"; do
    echo "$HELP_OUTPUT" | grep -q "$cmd"
    test_result "Command '$cmd' available in help"
done

# Test 5: Config file creation
echo -e "${BLUE}🧪 Testing Config File Creation${NC}"
# Create a temporary directory for testing
TEST_DIR="test-temp-$$"
mkdir "$TEST_DIR"
cd "$TEST_DIR"

../target/release/rosetta-connect init --bundle-id com.test.example >/dev/null 2>&1
[ -f "rosetta.toml" ]
test_result "Config file created by init command"

if [ -f "rosetta.toml" ]; then
    grep -q "com.test.example" rosetta.toml
    test_result "Bundle ID correctly set in config"
    
    grep -q "en-US" rosetta.toml
    test_result "Default locale set in config"
fi

# Test 6: Environment file creation
[ -f ".env" ]
test_result "Environment file created by init command"

# Test 7: Configuration validation
echo -e "${BLUE}🧪 Testing Configuration Structure${NC}"
if [ -f "rosetta.toml" ]; then
    grep -q "\\[app\\]" rosetta.toml
    test_result "App section exists in config"
    
    grep -q "\\[ai\\]" rosetta.toml
    test_result "AI section exists in config"
    
    grep -q "bundle_id" rosetta.toml
    test_result "Bundle ID field exists"
    
    grep -q "default_locale" rosetta.toml
    test_result "Default locale field exists"
fi

# Test 8: TypeScript compilation
echo -e "${BLUE}🧪 Testing TypeScript Components${NC}"
cd ..
[ -d "js/dist" ]
test_result "TypeScript dist directory exists"

[ -f "js/dist/asc.js" ]
test_result "ASC TypeScript compiled"

[ -f "js/dist/openai-service.js" ]
test_result "OpenAI service TypeScript compiled"

# Test 9: Node.js dependencies
echo -e "${BLUE}🧪 Testing Node.js Dependencies${NC}"
[ -d "js/node_modules" ]
test_result "Node.js dependencies installed"

[ -f "js/node_modules/appstore-connect-sdk/package.json" ]
test_result "App Store Connect SDK installed"

[ -f "js/node_modules/openai/package.json" ]
test_result "OpenAI SDK installed"

# Test 10: Error handling for invalid inputs
echo -e "${BLUE}🧪 Testing Error Handling${NC}"
./target/release/rosetta-connect init --bundle-id "" >/dev/null 2>&1
[ $? -ne 0 ]
test_result "Rejects empty bundle ID"

./target/release/rosetta-connect translate --locales "" >/dev/null 2>&1
[ $? -ne 0 ]
test_result "Rejects empty locales"

# Cleanup
cd "$TEST_DIR" && rm -f rosetta.toml .env
cd .. && rmdir "$TEST_DIR"

# Summary
echo
echo "=============================="
echo -e "${BLUE}📊 Unit Test Summary${NC}"
echo "=============================="
echo -e "Total tests: $((TESTS_PASSED + TESTS_FAILED))"
echo -e "${GREEN}Passed: ${TESTS_PASSED}${NC}"
echo -e "${RED}Failed: ${TESTS_FAILED}${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All unit tests passed!${NC}"
    exit 0
else
    echo -e "${RED}💥 Some unit tests failed${NC}"
    exit 1
fi