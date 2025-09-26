#!/bin/bash

# Debug TypeScript parser
set -euo pipefail

# Source the TypeScript parser
source lib/typescript_parser.sh

# Test file
TEST_FILE="test_extracted_kiro/resources/app/out/vs/vscode.d.ts"
OUTPUT_DIR="debug_output"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Initialize output files
echo "[]" > "$OUTPUT_DIR/all_interfaces.json"
echo "[]" > "$OUTPUT_DIR/all_classes.json"
echo "[]" > "$OUTPUT_DIR/all_enums.json"
echo "[]" > "$OUTPUT_DIR/all_types.json"
echo "[]" > "$OUTPUT_DIR/all_modules.json"

echo "=== Testing TypeScript Parser ==="
echo "File: $TEST_FILE"
echo "Content:"
cat "$TEST_FILE"
echo ""

echo "=== Testing grep patterns ==="
echo "Interface pattern:"
grep -n "interface[[:space:]]\+[A-Za-z_][A-Za-z0-9_]*" "$TEST_FILE" || echo "No matches"
echo ""

echo "Module pattern:"
grep -n "\(module\|namespace\)[[:space:]]\+\|declare[[:space:]]\+module" "$TEST_FILE" || echo "No matches"
echo ""

echo "=== Testing extraction functions ==="
echo "Extracting interfaces..."
interface_count=$(extract_typescript_interfaces "$TEST_FILE" "test/vscode.d.ts" "$OUTPUT_DIR")
echo "Found $interface_count interfaces"

echo "Extracting modules..."
extract_typescript_modules "$TEST_FILE" "test/vscode.d.ts" "$OUTPUT_DIR"

echo ""
echo "=== Results ==="
echo "Interfaces:"
cat "$OUTPUT_DIR/all_interfaces.json"
echo ""
echo "Modules:"
cat "$OUTPUT_DIR/all_modules.json"