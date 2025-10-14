#!/bin/bash

# Test Script for Kiro Analysis Pipeline
# This script tests the analysis pipeline with a mock directory structure

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_DIR="$SCRIPT_DIR/test_extracted_kiro"
TEST_OUTPUT_DIR="$SCRIPT_DIR/test_output"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_test() {
    echo -e "${BLUE}[TEST]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

# Create mock Kiro directory structure
create_mock_kiro_structure() {
    log_test "Creating mock Kiro directory structure"
    
    rm -rf "$TEST_DIR"
    mkdir -p "$TEST_DIR"
    
    # Create mock package.json files
    mkdir -p "$TEST_DIR/resources/app"
    cat > "$TEST_DIR/resources/app/package.json" << 'EOF'
{
  "name": "kiro",
  "version": "1.103.2",
  "description": "Kiro - AI-powered development environment",
  "main": "./out/main.js",
  "contributes": {
    "commands": [
      {
        "command": "kiro.openFile",
        "title": "Open File",
        "category": "File"
      },
      {
        "command": "kiro.ai.askQuestion",
        "title": "Ask AI",
        "category": "AI"
      }
    ],
    "keybindings": [
      {
        "command": "kiro.openFile",
        "key": "ctrl+o",
        "mac": "cmd+o"
      }
    ],
    "configuration": {
      "title": "Kiro",
      "properties": {
        "kiro.ai.enabled": {
          "type": "boolean",
          "default": true,
          "description": "Enable AI assistance"
        }
      }
    }
  }
}
EOF

    # Create mock product.json
    cat > "$TEST_DIR/resources/app/product.json" << 'EOF'
{
  "nameShort": "Kiro",
  "nameLong": "Kiro",
  "applicationName": "kiro",
  "dataFolderName": ".kiro",
  "version": "1.103.2",
  "quality": "stable",
  "commit": "abc123def456",
  "date": "2024-01-15T10:00:00.000Z",
  "extensionsGallery": {
    "serviceUrl": "https://open-vsx.org/vscode/gallery",
    "itemUrl": "https://open-vsx.org/vscode/item"
  }
}
EOF

    # Create mock TypeScript definition files
    mkdir -p "$TEST_DIR/resources/app/out/vs"
    cat > "$TEST_DIR/resources/app/out/vs/vscode.d.ts" << 'EOF'
declare module 'vscode' {
    export interface TextDocument {
        readonly uri: Uri;
        readonly fileName: string;
        readonly languageId: string;
    }
    
    export interface Window {
        showInformationMessage(message: string): Thenable<string | undefined>;
        showErrorMessage(message: string): Thenable<string | undefined>;
    }
    
    export namespace commands {
        export function registerCommand(command: string, callback: (...args: any[]) => any): Disposable;
        export function executeCommand<T>(command: string, ...rest: any[]): Thenable<T>;
    }
}
EOF

    # Create mock HTML files
    mkdir -p "$TEST_DIR/resources/app/out/vs/workbench"
    cat > "$TEST_DIR/resources/app/out/vs/workbench/workbench.html" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>Kiro</title>
    <link rel="stylesheet" href="workbench.css">
</head>
<body>
    <div id="workbench" class="kiro-workbench">
        <div class="titlebar">
            <div class="window-title">Kiro</div>
        </div>
        <div class="editor-container">
            <div id="monaco-editor"></div>
        </div>
    </div>
</body>
</html>
EOF

    # Create mock CSS files
    cat > "$TEST_DIR/resources/app/out/vs/workbench/workbench.css" << 'EOF'
.kiro-workbench {
    display: flex;
    flex-direction: column;
    height: 100vh;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.titlebar {
    height: 30px;
    background-color: #2d2d30;
    color: #cccccc;
    display: flex;
    align-items: center;
    padding: 0 10px;
}

.editor-container {
    flex: 1;
    background-color: #1e1e1e;
}

#monaco-editor {
    width: 100%;
    height: 100%;
}
EOF

    # Create mock JavaScript files
    mkdir -p "$TEST_DIR/resources/app/out/vs/workbench/services"
    cat > "$TEST_DIR/resources/app/out/vs/workbench/services/aiService.js" << 'EOF'
class AIService {
    constructor() {
        this.apiEndpoint = 'https://api.anthropic.com/v1/messages';
        this.eventHandlers = new Map();
    }
    
    async sendRequest(prompt) {
        // AI request handling logic
        const response = await fetch(this.apiEndpoint, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ prompt })
        });
        return response.json();
    }
    
    addEventListener(event, handler) {
        this.eventHandlers.set(event, handler);
    }
    
    setState(newState) {
        this.state = { ...this.state, ...newState };
        this.notifyStateChange();
    }
}
EOF

    # Create some image files (empty for testing)
    mkdir -p "$TEST_DIR/resources/app/resources/icons"
    touch "$TEST_DIR/resources/app/resources/icons/kiro-icon.png"
    touch "$TEST_DIR/resources/app/resources/icons/file-icon.svg"
    
    # Create some log files (should be excluded)
    mkdir -p "$TEST_DIR/logs"
    echo "Log entry 1" > "$TEST_DIR/logs/app.log"
    echo "Debug info" > "$TEST_DIR/logs/debug.log"
    
    # Create node_modules directory (should be excluded)
    mkdir -p "$TEST_DIR/node_modules/some-package"
    echo '{"name": "some-package"}' > "$TEST_DIR/node_modules/some-package/package.json"
    
    log_success "Mock Kiro directory structure created at: $TEST_DIR"
}

# Test the analysis pipeline
test_analysis_pipeline() {
    log_test "Testing analysis pipeline"
    
    # Clean up previous test output
    rm -rf "$TEST_OUTPUT_DIR"
    
    # Run the analysis pipeline
    if "$SCRIPT_DIR/analyze_kiro.sh" \
        --input "$TEST_DIR" \
        --output "$TEST_OUTPUT_DIR" \
        --config "$SCRIPT_DIR/kiro_analysis_config.json" \
        --log-level debug; then
        log_success "Analysis pipeline completed successfully"
    else
        log_error "Analysis pipeline failed"
        return 1
    fi
}

# Verify output structure
verify_output_structure() {
    log_test "Verifying output structure"
    
    local expected_dirs=(
        "config"
        "api" 
        "ui"
        "behavior"
        "reports"
        "logs"
        "temp"
    )
    
    for dir in "${expected_dirs[@]}"; do
        if [[ -d "$TEST_OUTPUT_DIR/$dir" ]]; then
            log_success "Directory exists: $dir"
        else
            log_error "Missing directory: $dir"
            return 1
        fi
    done
    
    # Check for key files
    local expected_files=(
        "reports/file_inventory.json"
        "reports/file_inventory_report.md"
        "reports/file_validation.json"
        "reports/summary/behavioral_specification.md"
        "progress.json"
        "status.json"
        "README.md"
    )
    
    for file in "${expected_files[@]}"; do
        if [[ -f "$TEST_OUTPUT_DIR/$file" ]]; then
            log_success "File exists: $file"
        else
            log_error "Missing file: $file"
            return 1
        fi
    done
}

# Verify file inventory results
verify_file_inventory() {
    log_test "Verifying file inventory results"
    
    local inventory_file="$TEST_OUTPUT_DIR/reports/file_inventory.json"
    
    if [[ ! -f "$inventory_file" ]]; then
        log_error "File inventory not found: $inventory_file"
        return 1
    fi
    
    # Check that files were discovered
    local file_count
    file_count=$(jq 'length' "$inventory_file")
    
    if [[ $file_count -gt 0 ]]; then
        log_success "Discovered $file_count files"
    else
        log_error "No files discovered"
        return 1
    fi
    
    # Check that excluded directories were ignored
    local node_modules_count
    node_modules_count=$(jq '[.[] | select(.path | contains("node_modules"))] | length' "$inventory_file")
    
    if [[ $node_modules_count -eq 0 ]]; then
        log_success "node_modules directory properly excluded"
    else
        log_warn "Found $node_modules_count files in node_modules (should be excluded)"
    fi
    
    # Check that log files were excluded
    local log_files_count
    log_files_count=$(jq '[.[] | select(.extension == "log")] | length' "$inventory_file")
    
    if [[ $log_files_count -eq 0 ]]; then
        log_success "Log files properly excluded"
    else
        log_warn "Found $log_files_count log files (should be excluded)"
    fi
    
    # Check file categorization
    local json_files_count
    json_files_count=$(jq '[.[] | select(.category == "json")] | length' "$inventory_file")
    
    if [[ $json_files_count -gt 0 ]]; then
        log_success "Found $json_files_count JSON files (categorization working)"
    else
        log_warn "No JSON files found (categorization may not be working)"
    fi
}

# Test error handling
test_error_handling() {
    log_test "Testing error handling with invalid input"
    
    # Test with non-existent directory
    if "$SCRIPT_DIR/analyze_kiro.sh" \
        --input "/nonexistent/directory" \
        --output "$TEST_OUTPUT_DIR/error_test" \
        --log-level debug 2>/dev/null; then
        log_error "Pipeline should have failed with non-existent input directory"
        return 1
    else
        log_success "Pipeline correctly failed with non-existent input directory"
    fi
}

# Test help and version options
test_help_and_version() {
    log_test "Testing help and version options"
    
    # Test help option
    if "$SCRIPT_DIR/analyze_kiro.sh" --help >/dev/null 2>&1; then
        log_success "Help option works"
    else
        log_error "Help option failed"
        return 1
    fi
    
    # Test version option
    if "$SCRIPT_DIR/analyze_kiro.sh" --version >/dev/null 2>&1; then
        log_success "Version option works"
    else
        log_error "Version option failed"
        return 1
    fi
}

# Main test execution
main() {
    log_test "Starting Kiro Analysis Pipeline Test Suite"
    
    # Check dependencies
    local missing_deps=()
    command -v jq >/dev/null 2>&1 || missing_deps+=("jq")
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        log_error "Missing required dependencies: ${missing_deps[*]}"
        log_error "Please install missing tools and try again."
        exit 1
    fi
    
    # Run tests
    create_mock_kiro_structure
    test_help_and_version
    test_error_handling
    test_analysis_pipeline
    verify_output_structure
    verify_file_inventory
    
    log_success "All tests passed! Analysis pipeline is working correctly."
    log_test "Test output available at: $TEST_OUTPUT_DIR"
    
    # Cleanup
    read -p "Remove test directories? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf "$TEST_DIR" "$TEST_OUTPUT_DIR"
        log_success "Test directories cleaned up"
    fi
}

# Execute main function
main "$@"