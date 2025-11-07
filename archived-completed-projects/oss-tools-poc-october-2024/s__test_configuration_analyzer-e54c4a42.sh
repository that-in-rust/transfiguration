#!/bin/bash

# Test Configuration Analyzer
# This script tests the configuration analyzer with sample data

set -euo pipefail

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test configuration
TEST_DIR="./test_config_analysis"
TEST_INPUT_DIR="$TEST_DIR/sample_kiro"
TEST_OUTPUT_DIR="$TEST_DIR/output"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
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

log_debug() {
    echo -e "${BLUE}[DEBUG]${NC} $*"
}

# Create sample test data
create_sample_data() {
    log_info "Creating sample test data"
    
    mkdir -p "$TEST_INPUT_DIR"
    
    # Create sample package.json with VS Code extension structure
    cat > "$TEST_INPUT_DIR/package.json" << 'EOF'
{
    "name": "kiro-sample-extension",
    "displayName": "Kiro Sample Extension",
    "description": "Sample extension for testing configuration analysis",
    "version": "1.0.0",
    "engines": {
        "vscode": "^1.103.0"
    },
    "categories": ["Other"],
    "contributes": {
        "commands": [
            {
                "command": "kiro.openFile",
                "title": "Open File",
                "category": "File",
                "icon": "$(file)"
            },
            {
                "command": "kiro.saveAll",
                "title": "Save All Files",
                "category": "File"
            },
            {
                "command": "kiro.ai.askQuestion",
                "title": "Ask AI Question",
                "category": "AI Assistant"
            }
        ],
        "keybindings": [
            {
                "command": "kiro.openFile",
                "key": "ctrl+o",
                "mac": "cmd+o",
                "when": "editorTextFocus"
            },
            {
                "command": "kiro.saveAll",
                "key": "ctrl+k s",
                "mac": "cmd+k s"
            },
            {
                "command": "kiro.ai.askQuestion",
                "key": "ctrl+shift+a",
                "mac": "cmd+shift+a",
                "when": "editorTextFocus"
            }
        ],
        "menus": {
            "commandPalette": [
                {
                    "command": "kiro.openFile",
                    "when": "true"
                },
                {
                    "command": "kiro.ai.askQuestion",
                    "when": "editorTextFocus"
                }
            ],
            "editor/context": [
                {
                    "command": "kiro.ai.askQuestion",
                    "group": "ai@1",
                    "when": "editorTextFocus"
                }
            ]
        },
        "configuration": {
            "title": "Kiro Configuration",
            "properties": {
                "kiro.ai.enabled": {
                    "type": "boolean",
                    "default": true,
                    "description": "Enable AI assistance features",
                    "scope": "window"
                },
                "kiro.ai.model": {
                    "type": "string",
                    "default": "claude-3-sonnet",
                    "enum": ["claude-3-sonnet", "claude-3-haiku", "gpt-4"],
                    "description": "AI model to use for assistance",
                    "scope": "window"
                },
                "kiro.editor.fontSize": {
                    "type": "number",
                    "default": 14,
                    "minimum": 8,
                    "maximum": 72,
                    "description": "Font size for the editor",
                    "scope": "window"
                },
                "kiro.files.autoSave": {
                    "type": "string",
                    "default": "afterDelay",
                    "enum": ["off", "afterDelay", "onFocusChange", "onWindowChange"],
                    "description": "Auto save configuration",
                    "scope": "window"
                }
            }
        }
    }
}
EOF

    # Create sample product.json
    cat > "$TEST_INPUT_DIR/product.json" << 'EOF'
{
    "name": "kiro",
    "nameLong": "Kiro IDE",
    "applicationName": "kiro",
    "version": "1.103.2",
    "commit": "abc123def456",
    "quality": "stable",
    "dataFolderName": ".kiro",
    "webEndpointUrl": "https://api.kiro.dev",
    "updateUrl": "https://update.kiro.dev",
    "releaseNotesUrl": "https://kiro.dev/releases",
    "documentationUrl": "https://docs.kiro.dev",
    "requestFeatureUrl": "https://github.com/kiro-dev/kiro/issues/new?template=feature_request.md",
    "reportIssueUrl": "https://github.com/kiro-dev/kiro/issues/new?template=bug_report.md",
    "licenseUrl": "https://kiro.dev/license",
    "privacyStatementUrl": "https://kiro.dev/privacy",
    "keyboardShortcutsUrlMac": "https://docs.kiro.dev/shortcuts/mac",
    "keyboardShortcutsUrlWin": "https://docs.kiro.dev/shortcuts/windows",
    "keyboardShortcutsUrlLinux": "https://docs.kiro.dev/shortcuts/linux",
    "enableTelemetry": true,
    "enableCrashReporter": true,
    "extensionsGallery": {
        "serviceUrl": "https://open-vsx.org/vscode/gallery",
        "itemUrl": "https://open-vsx.org/vscode/item"
    },
    "aiConfig": {
        "anthropicApiUrl": "https://api.anthropic.com",
        "defaultModel": "claude-3-sonnet-20240229",
        "maxTokens": 4096,
        "temperature": 0.7
    },
    "builtInExtensions": [
        {
            "name": "kiro-agent",
            "version": "1.0.0",
            "path": "./extensions/kiro-agent"
        },
        {
            "name": "ms-vscode.vscode-typescript-next",
            "version": "5.4.0",
            "path": "./extensions/typescript"
        }
    ]
}
EOF

    # Create additional package.json in subdirectory (simulating built-in extension)
    mkdir -p "$TEST_INPUT_DIR/extensions/kiro-agent"
    cat > "$TEST_INPUT_DIR/extensions/kiro-agent/package.json" << 'EOF'
{
    "name": "kiro-agent",
    "displayName": "Kiro AI Agent",
    "description": "AI-powered development assistant",
    "version": "1.0.0",
    "engines": {
        "vscode": "^1.103.0"
    },
    "categories": ["AI", "Other"],
    "contributes": {
        "commands": [
            {
                "command": "kiro-agent.chat",
                "title": "Open AI Chat",
                "category": "Kiro Agent",
                "icon": "$(comment-discussion)"
            },
            {
                "command": "kiro-agent.explainCode",
                "title": "Explain Selected Code",
                "category": "Kiro Agent"
            },
            {
                "command": "kiro-agent.generateTests",
                "title": "Generate Tests",
                "category": "Kiro Agent"
            }
        ],
        "keybindings": [
            {
                "command": "kiro-agent.chat",
                "key": "ctrl+shift+k",
                "mac": "cmd+shift+k"
            },
            {
                "command": "kiro-agent.explainCode",
                "key": "ctrl+shift+e",
                "mac": "cmd+shift+e",
                "when": "editorHasSelection"
            }
        ],
        "menus": {
            "editor/context": [
                {
                    "command": "kiro-agent.explainCode",
                    "group": "kiro@1",
                    "when": "editorHasSelection"
                },
                {
                    "command": "kiro-agent.generateTests",
                    "group": "kiro@2",
                    "when": "editorTextFocus"
                }
            ],
            "view/title": [
                {
                    "command": "kiro-agent.chat",
                    "when": "view == kiro-agent-chat",
                    "group": "navigation"
                }
            ]
        },
        "configuration": {
            "title": "Kiro Agent",
            "properties": {
                "kiro-agent.apiKey": {
                    "type": "string",
                    "default": "",
                    "description": "API key for AI services",
                    "scope": "machine"
                },
                "kiro-agent.autoExplain": {
                    "type": "boolean",
                    "default": false,
                    "description": "Automatically explain code on selection",
                    "scope": "window"
                },
                "kiro-agent.contextLines": {
                    "type": "number",
                    "default": 10,
                    "minimum": 0,
                    "maximum": 100,
                    "description": "Number of context lines to include",
                    "scope": "window"
                }
            }
        }
    }
}
EOF

    log_success "Sample test data created in $TEST_INPUT_DIR"
}

# Run configuration analysis test
run_test() {
    log_info "Running configuration analyzer test"
    
    # Clean up previous test
    rm -rf "$TEST_DIR"
    
    # Create sample data
    create_sample_data
    
    # Create test configuration
    cp kiro_analysis_config.json "$TEST_DIR/test_config.json"
    
    # Source the required modules
    source "$SCRIPT_DIR/lib/error_handling.sh"
    source "$SCRIPT_DIR/lib/output_management.sh"
    source "$SCRIPT_DIR/lib/configuration_analyzer.sh"
    
    # Initialize systems
    init_output_management "$TEST_OUTPUT_DIR"
    init_error_handling "$TEST_OUTPUT_DIR"
    
    # Run configuration analysis
    log_info "Running configuration analysis on sample data"
    
    if run_configuration_analysis "$TEST_INPUT_DIR" "$TEST_OUTPUT_DIR" "$TEST_DIR/test_config.json"; then
        log_success "Configuration analysis completed successfully"
    else
        log_error "Configuration analysis failed"
        return 1
    fi
    
    # Validate results
    validate_results
}

# Validate test results
validate_results() {
    log_info "Validating analysis results"
    
    local errors=0
    
    # Check if output files were created
    local expected_files=(
        "config/commands/command_definitions.json"
        "config/commands/command_analysis.md"
        "config/menus/menu_structures.json"
        "config/menus/menu_analysis.md"
        "config/keybindings/keybinding_definitions.json"
        "config/keybindings/keybinding_analysis.md"
        "config/settings/settings_schemas.json"
        "config/settings/settings_analysis.md"
        "config/product_configuration.json"
        "config/product_analysis.md"
        "config/configuration_summary.json"
        "config/configuration_summary.md"
    )
    
    for file in "${expected_files[@]}"; do
        if [[ -f "$TEST_OUTPUT_DIR/$file" ]]; then
            log_success "✓ Generated: $file"
        else
            log_error "✗ Missing: $file"
            ((errors++))
        fi
    done
    
    # Validate JSON files
    local json_files=(
        "config/commands/command_definitions.json"
        "config/menus/menu_structures.json"
        "config/keybindings/keybinding_definitions.json"
        "config/settings/settings_schemas.json"
        "config/product_configuration.json"
        "config/configuration_summary.json"
    )
    
    for json_file in "${json_files[@]}"; do
        if [[ -f "$TEST_OUTPUT_DIR/$json_file" ]]; then
            if jq empty "$TEST_OUTPUT_DIR/$json_file" 2>/dev/null; then
                log_success "✓ Valid JSON: $json_file"
            else
                log_error "✗ Invalid JSON: $json_file"
                ((errors++))
            fi
        fi
    done
    
    # Check content validation
    log_info "Validating content..."
    
    # Check commands
    local command_count
    command_count=$(jq 'length' "$TEST_OUTPUT_DIR/config/commands/command_definitions.json" 2>/dev/null || echo "0")
    if [[ "$command_count" -gt 0 ]]; then
        log_success "✓ Found $command_count commands"
    else
        log_error "✗ No commands found"
        ((errors++))
    fi
    
    # Check keybindings
    local keybinding_count
    keybinding_count=$(jq 'length' "$TEST_OUTPUT_DIR/config/keybindings/keybinding_definitions.json" 2>/dev/null || echo "0")
    if [[ "$keybinding_count" -gt 0 ]]; then
        log_success "✓ Found $keybinding_count keybindings"
    else
        log_error "✗ No keybindings found"
        ((errors++))
    fi
    
    # Check settings
    local settings_count
    settings_count=$(jq 'length' "$TEST_OUTPUT_DIR/config/settings/settings_schemas.json" 2>/dev/null || echo "0")
    if [[ "$settings_count" -gt 0 ]]; then
        log_success "✓ Found $settings_count settings"
    else
        log_error "✗ No settings found"
        ((errors++))
    fi
    
    # Check product configuration
    local product_name
    product_name=$(jq -r '.branding.name' "$TEST_OUTPUT_DIR/config/product_configuration.json" 2>/dev/null || echo "")
    if [[ "$product_name" == "kiro" ]]; then
        log_success "✓ Product configuration parsed correctly"
    else
        log_error "✗ Product configuration parsing failed"
        ((errors++))
    fi
    
    # Summary
    if [[ $errors -eq 0 ]]; then
        log_success "All validation tests passed!"
        log_info "Test results available in: $TEST_OUTPUT_DIR"
        return 0
    else
        log_error "$errors validation errors found"
        return 1
    fi
}

# Show test results
show_results() {
    log_info "Configuration Analysis Test Results"
    echo
    echo "Generated files:"
    find "$TEST_OUTPUT_DIR" -type f -name "*.json" -o -name "*.md" | sort
    echo
    echo "Sample command from results:"
    jq -r '.[] | select(.command == "kiro.ai.askQuestion") | "Command: \(.command)\nTitle: \(.title)\nCategory: \(.category)"' "$TEST_OUTPUT_DIR/config/commands/command_definitions.json" 2>/dev/null || echo "No commands found"
    echo
    echo "Sample setting from results:"
    jq -r '.[] | select(.setting_name == "kiro.ai.enabled") | "Setting: \(.setting_name)\nType: \(.type)\nDefault: \(.default)\nDescription: \(.description)"' "$TEST_OUTPUT_DIR/config/settings/settings_schemas.json" 2>/dev/null || echo "No settings found"
}

# Main execution
main() {
    log_info "Starting Configuration Analyzer Test"
    
    if run_test; then
        show_results
        log_success "Configuration analyzer test completed successfully"
    else
        log_error "Configuration analyzer test failed"
        exit 1
    fi
}

# Run the test
main "$@"