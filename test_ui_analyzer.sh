#!/bin/bash

# Test script for UI Structure Analyzer
# This script tests the UI structure analysis functionality

set -e

# Source required modules
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/error_handling.sh"
source "$SCRIPT_DIR/lib/output_management.sh"
source "$SCRIPT_DIR/lib/ui_structure_analyzer.sh"

# Simple logging functions for testing
log_info() {
    echo "[INFO] $1"
}

log_debug() {
    echo "[DEBUG] $1"
}

log_success() {
    echo "[SUCCESS] $1"
}

log_warning() {
    echo "[WARNING] $1"
}

log_error() {
    echo "[ERROR] $1"
}

# Test configuration
TEST_INPUT_PATH="/Users/neetipatni/Desktop/extracted_kiro"
TEST_OUTPUT_DIR="./test_ui_analysis_output"
TEST_CONFIG_FILE="./kiro_analysis_config.json"

# Main test function
main() {
    echo "=== UI Structure Analyzer Test ==="
    echo "Input path: $TEST_INPUT_PATH"
    echo "Output directory: $TEST_OUTPUT_DIR"
    echo ""
    
    # Check if input path exists
    if [[ ! -d "$TEST_INPUT_PATH" ]]; then
        echo "ERROR: Input path does not exist: $TEST_INPUT_PATH"
        echo "Please ensure the Kiro files are extracted to this location."
        exit 1
    fi
    
    # Initialize output management
    init_output_management "$TEST_OUTPUT_DIR"
    
    # Initialize error handling
    init_error_handling "$TEST_OUTPUT_DIR"
    
    echo "Starting UI structure analysis test..."
    
    # Run the UI structure analysis
    if run_ui_structure_analysis "$TEST_INPUT_PATH" "$TEST_OUTPUT_DIR" "$TEST_CONFIG_FILE"; then
        echo ""
        echo "=== Test Results ==="
        echo "✅ UI structure analysis completed successfully"
        
        # Check if output files were created
        echo ""
        echo "Generated files:"
        find "$TEST_OUTPUT_DIR" -name "*.json" -o -name "*.md" | sort | while read -r file; do
            local file_size
            file_size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null)
            echo "  - $(basename "$file"): $file_size bytes"
        done
        
        # Show summary statistics
        echo ""
        echo "=== Analysis Summary ==="
        
        # CSS Analysis Summary
        if [[ -f "$TEST_OUTPUT_DIR/ui/styling/css_analysis.json" ]]; then
            local css_summary
            css_summary=$(jq -r '.summary | "CSS Files: \(.total_files), Rules: \(.total_rules), Selectors: \(.total_selectors)"' "$TEST_OUTPUT_DIR/ui/styling/css_analysis.json" 2>/dev/null || echo "CSS analysis data not available")
            echo "CSS: $css_summary"
        fi
        
        # HTML Analysis Summary
        if [[ -f "$TEST_OUTPUT_DIR/ui/components/html_analysis.json" ]]; then
            local html_summary
            html_summary=$(jq -r '.summary | "HTML Files: \(.total_files), Elements: \(.total_elements), Components: \(.total_components)"' "$TEST_OUTPUT_DIR/ui/components/html_analysis.json" 2>/dev/null || echo "HTML analysis data not available")
            echo "HTML: $html_summary"
        fi
        
        # Theme Analysis Summary
        if [[ -f "$TEST_OUTPUT_DIR/ui/themes/theme_analysis.json" ]]; then
            local theme_summary
            theme_summary=$(jq -r '.summary | "Theme Files: \(.total_files), Color Themes: \(.color_themes), Icon Themes: \(.icon_themes)"' "$TEST_OUTPUT_DIR/ui/themes/theme_analysis.json" 2>/dev/null || echo "Theme analysis data not available")
            echo "Themes: $theme_summary"
        fi
        
        # Media Assets Summary
        if [[ -f "$TEST_OUTPUT_DIR/ui/assets/media_analysis.json" ]]; then
            local media_summary
            media_summary=$(jq -r '.summary | "Media Files: \(.total_files), Total Size: \(.total_size_human)"' "$TEST_OUTPUT_DIR/ui/assets/media_analysis.json" 2>/dev/null || echo "Media analysis data not available")
            echo "Media: $media_summary"
        fi
        
        echo ""
        echo "Test completed successfully! Check the output directory for detailed results:"
        echo "$TEST_OUTPUT_DIR"
        
    else
        echo "❌ UI structure analysis test failed"
        exit 1
    fi
}

# Run the test
main "$@"