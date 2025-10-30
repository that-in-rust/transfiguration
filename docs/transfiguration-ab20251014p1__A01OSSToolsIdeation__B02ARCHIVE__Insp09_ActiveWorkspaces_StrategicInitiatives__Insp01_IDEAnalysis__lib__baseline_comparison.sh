#!/bin/bash

# VS Code OSS Baseline Comparison Documentation
# Part of Kiro Behavioral Analysis Pipeline
# Task 6.2: Generate VS Code OSS baseline comparison documentation

# Baseline comparison configuration
BASELINE_OUTPUT_DIR=""
BASELINE_CONFIG=""
BASELINE_LOG_FILE=""
VS_CODE_VERSION="1.103.2"

# Comparison categories
COMPARISON_CATEGORIES="configuration api ui behavior extensions"

# Initialize baseline comparison system
init_baseline_comparison() {
    local output_dir="$1"
    local config_file="$2"
    
    BASELINE_OUTPUT_DIR="$output_dir/reports/baseline_comparison"
    BASELINE_CONFIG="$config_file"
    BASELINE_LOG_FILE="$BASELINE_OUTPUT_DIR/baseline_comparison_$(date +%Y%m%d_%H%M%S).log"
    
    mkdir -p "$BASELINE_OUTPUT_DIR"
    
    # Create baseline comparison log header
    cat > "$BASELINE_LOG_FILE" << EOF
# Kiro vs VS Code OSS Baseline Comparison Log
# Started: $(date -Iseconds)
# VS Code OSS Version: $VS_CODE_VERSION
# Task: 6.2 - Generate VS Code OSS baseline comparison documentation

EOF
    
    log_info "Baseline comparison system initialized: $BASELINE_OUTPUT_DIR"
}

# VS Code OSS baseline data (known characteristics)
get_vscode_baseline_data() {
    local category="$1"
    
    case "$category" in
        "configuration")
            cat << 'EOF'
{
    "vs_code_oss_baseline": {
        "version": "1.103.2",
        "configuration": {
            "core_commands": {
                "expected_count": 500,
                "essential_commands": [
                    "workbench.action.openSettings",
                    "workbench.action.showCommands",
                    "workbench.action.quickOpen",
                    "workbench.action.files.save",
                    "workbench.action.files.saveAll",
                    "editor.action.formatDocument",
                    "workbench.action.terminal.new"
                ]
            },
            "settings_schema": {
                "expected_categories": [
                    "editor",
                    "workbench",
                    "files",
                    "search",
                    "git",
                    "terminal",
                    "debug",
                    "extensions"
                ]
            },
            "keybindings": {
                "platform_specific": true,
                "customizable": true,
                "essential_shortcuts": {
                    "Ctrl+P": "workbench.action.quickOpen",
                    "Ctrl+Shift+P": "workbench.action.showCommands",
                    "Ctrl+S": "workbench.action.files.save",
                    "Ctrl+`": "workbench.action.terminal.toggleTerminal"
                }
            }
        }
    }
}
EOF
            ;;
        "api")
            cat << 'EOF'
{
    "vs_code_oss_baseline": {
        "version": "1.103.2",
        "api_surface": {
            "extension_api": {
                "namespaces": [
                    "vscode.commands",
                    "vscode.window",
                    "vscode.workspace",
                    "vscode.languages",
                    "vscode.debug",
                    "vscode.extensions",
                    "vscode.env"
                ]
            },
            "contribution_points": [
                "commands",
                "keybindings",
                "languages",
                "grammars",
                "themes",
                "iconThemes",
                "snippets",
                "debuggers",
                "breakpoints",
                "views",
                "viewsContainers",
                "menus",
                "configuration",
                "configurationDefaults"
            ],
            "activation_events": [
                "onLanguage",
                "onCommand",
                "onDebug",
                "workspaceContains",
                "onFileSystem",
                "onView",
                "onUri",
                "onWebviewPanel",
                "onCustomEditor",
                "onAuthenticationRequest",
                "onStartupFinished"
            ]
        }
    }
}
EOF
            ;;
        "ui")
            cat << 'EOF'
{
    "vs_code_oss_baseline": {
        "version": "1.103.2",
        "ui_structure": {
            "core_components": [
                "ActivityBar",
                "SideBar",
                "EditorGroup",
                "Panel",
                "StatusBar",
                "MenuBar",
                "TitleBar"
            ],
            "themes": {
                "default_themes": [
                    "Default Dark+",
                    "Default Light+",
                    "Default High Contrast"
                ],
                "customizable": true
            },
            "layout": {
                "resizable_panels": true,
                "dockable_panels": true,
                "tabbed_interface": true,
                "split_editors": true
            }
        }
    }
}
EOF
            ;;
        "behavior")
            cat << 'EOF'
{
    "vs_code_oss_baseline": {
        "version": "1.103.2",
        "behavioral_patterns": {
            "file_operations": {
                "auto_save": "configurable",
                "file_watching": "enabled",
                "backup_files": "enabled"
            },
            "editor_behavior": {
                "syntax_highlighting": "language_specific",
                "code_completion": "intellisense",
                "error_detection": "real_time",
                "formatting": "on_demand"
            },
            "workspace_management": {
                "multi_root_workspaces": true,
                "workspace_settings": true,
                "project_specific_config": true
            }
        }
    }
}
EOF
            ;;
        "extensions")
            cat << 'EOF'
{
    "vs_code_oss_baseline": {
        "version": "1.103.2",
        "extension_system": {
            "marketplace": "Microsoft Marketplace",
            "extension_host": "separate_process",
            "api_version": "1.103.2",
            "built_in_extensions": [
                "ms-vscode.references-view",
                "ms-vscode.search-result",
                "vscode.builtin-notebook-renderers",
                "vscode.git",
                "vscode.github",
                "vscode.microsoft-authentication"
            ]
        }
    }
}
EOF
            ;;
        *)
            echo "{}"
            ;;
    esac
}

# Compare Kiro configuration against VS Code baseline
compare_configuration_differences() {
    local kiro_output_dir="$1"
    local comparison_output="$2"
    
    log_info "Comparing Kiro configuration against VS Code OSS baseline"
    
    local kiro_commands_file="$kiro_output_dir/config/commands/commands.json"
    local kiro_settings_file="$kiro_output_dir/config/settings/settings_schema.json"
    
    # Get baseline data
    local baseline_data
    baseline_data=$(get_vscode_baseline_data "configuration")
    
    local comparison_result="{}"
    
    # Compare commands
    if [[ -f "$kiro_commands_file" ]]; then
        local kiro_command_count
        kiro_command_count=$(jq 'length' "$kiro_commands_file" 2>/dev/null || echo "0")
        
        local baseline_expected_count
        baseline_expected_count=$(echo "$baseline_data" | jq -r '.vs_code_oss_baseline.configuration.core_commands.expected_count')
        
        local command_comparison
        command_comparison=$(jq -n \
            --argjson kiro_count "$kiro_command_count" \
            --argjson baseline_count "$baseline_expected_count" \
            '{
                category: "commands",
                kiro_count: $kiro_count,
                baseline_expected: $baseline_count,
                difference: ($kiro_count - $baseline_count),
                percentage_difference: ((($kiro_count - $baseline_count) * 100) / $baseline_count),
                status: (if ($kiro_count >= ($baseline_count * 0.8)) then "acceptable" else "concerning" end)
            }')
        
        comparison_result=$(echo "$comparison_result" | jq ".commands = $command_comparison")
    fi
    
    # Check for essential commands
    if [[ -f "$kiro_commands_file" ]]; then
        local essential_commands
        essential_commands=$(echo "$baseline_data" | jq -r '.vs_code_oss_baseline.configuration.core_commands.essential_commands[]')
        
        local missing_essential=()
        while IFS= read -r essential_cmd; do
            if [[ -n "$essential_cmd" ]] && ! jq -e ".[] | select(.command == \"$essential_cmd\")" "$kiro_commands_file" >/dev/null 2>&1; then
                missing_essential+=("$essential_cmd")
            fi
        done <<< "$essential_commands"
        
        local essential_commands_json
        essential_commands_json=$(printf '%s\n' "${missing_essential[@]}" | jq -R . | jq -s .)
        
        comparison_result=$(echo "$comparison_result" | jq ".essential_commands_missing = $essential_commands_json")
    fi
    
    echo "$comparison_result" > "$comparison_output"
    
    log_debug "Configuration comparison completed: $comparison_output"
}

# Compare API surface differences
compare_api_differences() {
    local kiro_output_dir="$1"
    local comparison_output="$2"
    
    log_info "Comparing Kiro API surface against VS Code OSS baseline"
    
    local kiro_interfaces_file="$kiro_output_dir/api/interfaces/vscode_interfaces.json"
    local kiro_contribution_points_file="$kiro_output_dir/api/contribution_points/contribution_points.json"
    
    # Get baseline data
    local baseline_data
    baseline_data=$(get_vscode_baseline_data "api")
    
    local comparison_result="{}"
    
    # Compare contribution points
    if [[ -f "$kiro_contribution_points_file" ]]; then
        local baseline_contribution_points
        baseline_contribution_points=$(echo "$baseline_data" | jq -r '.vs_code_oss_baseline.api_surface.contribution_points[]')
        
        local missing_contribution_points=()
        local found_contribution_points=()
        
        while IFS= read -r contrib_point; do
            if [[ -n "$contrib_point" ]]; then
                if jq -e ".[] | select(.type == \"$contrib_point\")" "$kiro_contribution_points_file" >/dev/null 2>&1; then
                    found_contribution_points+=("$contrib_point")
                else
                    missing_contribution_points+=("$contrib_point")
                fi
            fi
        done <<< "$baseline_contribution_points"
        
        local found_json
        found_json=$(printf '%s\n' "${found_contribution_points[@]}" | jq -R . | jq -s .)
        
        local missing_json
        missing_json=$(printf '%s\n' "${missing_contribution_points[@]}" | jq -R . | jq -s .)
        
        local contribution_comparison
        contribution_comparison=$(jq -n \
            --argjson found "$found_json" \
            --argjson missing "$missing_json" \
            '{
                category: "contribution_points",
                found_points: $found,
                missing_points: $missing,
                coverage_percentage: (($found | length) * 100 / (($found | length) + ($missing | length))),
                status: (if (($found | length) >= (($found | length) + ($missing | length)) * 0.9) then "excellent" elif (($found | length) >= (($found | length) + ($missing | length)) * 0.7) then "good" else "needs_improvement" end)
            }')
        
        comparison_result=$(echo "$comparison_result" | jq ".contribution_points = $contribution_comparison")
    fi
    
    echo "$comparison_result" > "$comparison_output"
    
    log_debug "API comparison completed: $comparison_output"
}

# Compare UI structure differences
compare_ui_differences() {
    local kiro_output_dir="$1"
    local comparison_output="$2"
    
    log_info "Comparing Kiro UI structure against VS Code OSS baseline"
    
    local kiro_components_file="$kiro_output_dir/ui/components/ui_components.json"
    local kiro_themes_file="$kiro_output_dir/ui/styling/themes.json"
    
    # Get baseline data
    local baseline_data
    baseline_data=$(get_vscode_baseline_data "ui")
    
    local comparison_result="{}"
    
    # Compare core UI components
    if [[ -f "$kiro_components_file" ]]; then
        local baseline_components
        baseline_components=$(echo "$baseline_data" | jq -r '.vs_code_oss_baseline.ui_structure.core_components[]')
        
        local missing_components=()
        local found_components=()
        
        while IFS= read -r component; do
            if [[ -n "$component" ]]; then
                if jq -e ".[] | select(.name == \"$component\" or .component_name == \"$component\")" "$kiro_components_file" >/dev/null 2>&1; then
                    found_components+=("$component")
                else
                    missing_components+=("$component")
                fi
            fi
        done <<< "$baseline_components"
        
        local found_json
        found_json=$(printf '%s\n' "${found_components[@]}" | jq -R . | jq -s .)
        
        local missing_json
        missing_json=$(printf '%s\n' "${missing_components[@]}" | jq -R . | jq -s .)
        
        local ui_comparison
        ui_comparison=$(jq -n \
            --argjson found "$found_json" \
            --argjson missing "$missing_json" \
            '{
                category: "ui_components",
                found_components: $found,
                missing_components: $missing,
                coverage_percentage: (($found | length) * 100 / (($found | length) + ($missing | length))),
                status: (if (($found | length) >= (($found | length) + ($missing | length)) * 0.8) then "good" else "needs_improvement" end)
            }')
        
        comparison_result=$(echo "$comparison_result" | jq ".ui_components = $ui_comparison")
    fi
    
    echo "$comparison_result" > "$comparison_output"
    
    log_debug "UI comparison completed: $comparison_output"
}

# Identify Kiro-specific customizations
identify_kiro_customizations() {
    local kiro_output_dir="$1"
    local customizations_output="$2"
    
    log_info "Identifying Kiro-specific customizations and additions"
    
    local customizations="{}"
    
    # Check for Kiro-specific branding
    local product_file="$kiro_output_dir/config/product.json"
    if [[ -f "$product_file" ]]; then
        local branding_info
        branding_info=$(jq '{
            product_name: .nameLong,
            application_name: .applicationName,
            data_folder_name: .dataFolderName,
            server_data_folder_name: .serverDataFolderName,
            quality: .quality,
            commit: .commit
        }' "$product_file" 2>/dev/null || echo "{}")
        
        customizations=$(echo "$customizations" | jq ".branding = $branding_info")
    fi
    
    # Check for Kiro Agent extension
    local extensions_dir="$kiro_output_dir/extensions"
    if [[ -d "$extensions_dir" ]]; then
        local kiro_extensions=()
        
        # Look for Kiro-specific extensions
        while IFS= read -r ext_dir; do
            if [[ -n "$ext_dir" ]] && [[ "$ext_dir" == *"kiro"* ]] || [[ "$ext_dir" == *"agent"* ]]; then
                local ext_name
                ext_name=$(basename "$ext_dir")
                kiro_extensions+=("$ext_name")
            fi
        done < <(find "$extensions_dir" -type d -maxdepth 1 2>/dev/null)
        
        local kiro_extensions_json
        kiro_extensions_json=$(printf '%s\n' "${kiro_extensions[@]}" | jq -R . | jq -s .)
        
        customizations=$(echo "$customizations" | jq ".kiro_specific_extensions = $kiro_extensions_json")
    fi
    
    # Check for AWS integration
    local aws_integration_indicators=()
    
    # Look for AWS-related configuration
    if grep -r "aws\|AWS" "$kiro_output_dir/config" 2>/dev/null | head -5; then
        aws_integration_indicators+=("AWS configuration found")
    fi
    
    # Look for AWS-related commands
    local commands_file="$kiro_output_dir/config/commands/commands.json"
    if [[ -f "$commands_file" ]] && jq -e '.[] | select(.command | contains("aws") or contains("AWS"))' "$commands_file" >/dev/null 2>&1; then
        aws_integration_indicators+=("AWS commands found")
    fi
    
    local aws_integration_json
    aws_integration_json=$(printf '%s\n' "${aws_integration_indicators[@]}" | jq -R . | jq -s .)
    
    customizations=$(echo "$customizations" | jq ".aws_integration = $aws_integration_json")
    
    # Check for Open VSX Registry usage
    if grep -r "open-vsx\|openvsx" "$kiro_output_dir" 2>/dev/null | head -3; then
        customizations=$(echo "$customizations" | jq '.marketplace = "Open VSX Registry"')
    else
        customizations=$(echo "$customizations" | jq '.marketplace = "Unknown or Microsoft Marketplace"')
    fi
    
    echo "$customizations" > "$customizations_output"
    
    log_debug "Kiro customizations identified: $customizations_output"
}

# Generate migration complexity assessment
generate_migration_complexity_assessment() {
    local comparison_results_dir="$1"
    local assessment_output="$2"
    
    log_info "Generating migration complexity assessment"
    
    local complexity_assessment="{}"
    
    # Analyze configuration migration complexity
    local config_comparison="$comparison_results_dir/configuration_comparison.json"
    if [[ -f "$config_comparison" ]]; then
        local config_complexity
        config_complexity=$(jq '{
            category: "configuration",
            complexity_level: (
                if (.commands.status == "acceptable" and (.essential_commands_missing | length) == 0) then "low"
                elif (.commands.status == "acceptable" or (.essential_commands_missing | length) <= 2) then "medium"
                else "high"
                end
            ),
            migration_effort: (
                if (.commands.status == "acceptable" and (.essential_commands_missing | length) == 0) then "1-2 weeks"
                elif (.commands.status == "acceptable" or (.essential_commands_missing | length) <= 2) then "2-4 weeks"
                else "4-8 weeks"
                end
            ),
            risk_factors: [
                (if (.essential_commands_missing | length) > 0 then "Missing essential commands" else empty end),
                (if (.commands.status != "acceptable") then "Command count significantly different" else empty end)
            ]
        }' "$config_comparison" 2>/dev/null || echo '{"category": "configuration", "complexity_level": "unknown"}')
        
        complexity_assessment=$(echo "$complexity_assessment" | jq ".configuration = $config_complexity")
    fi
    
    # Analyze API migration complexity
    local api_comparison="$comparison_results_dir/api_comparison.json"
    if [[ -f "$api_comparison" ]]; then
        local api_complexity
        api_complexity=$(jq '{
            category: "api_surface",
            complexity_level: (
                if (.contribution_points.status == "excellent") then "low"
                elif (.contribution_points.status == "good") then "medium"
                else "high"
                end
            ),
            migration_effort: (
                if (.contribution_points.status == "excellent") then "2-3 weeks"
                elif (.contribution_points.status == "good") then "3-6 weeks"
                else "6-12 weeks"
                end
            ),
            risk_factors: [
                (if (.contribution_points.coverage_percentage < 80) then "Low contribution point coverage" else empty end),
                (if (.contribution_points.missing_points | length) > 3 then "Many missing contribution points" else empty end)
            ]
        }' "$api_comparison" 2>/dev/null || echo '{"category": "api_surface", "complexity_level": "unknown"}')
        
        complexity_assessment=$(echo "$complexity_assessment" | jq ".api_surface = $api_complexity")
    fi
    
    # Analyze UI migration complexity
    local ui_comparison="$comparison_results_dir/ui_comparison.json"
    if [[ -f "$ui_comparison" ]]; then
        local ui_complexity
        ui_complexity=$(jq '{
            category: "ui_structure",
            complexity_level: (
                if (.ui_components.status == "good" and (.ui_components.coverage_percentage >= 80)) then "medium"
                else "high"
                end
            ),
            migration_effort: (
                if (.ui_components.status == "good" and (.ui_components.coverage_percentage >= 80)) then "4-8 weeks"
                else "8-16 weeks"
                end
            ),
            risk_factors: [
                (if (.ui_components.coverage_percentage < 70) then "Low UI component coverage" else empty end),
                "UI requires pixel-perfect replication",
                "Theme system needs careful migration"
            ]
        }' "$ui_comparison" 2>/dev/null || echo '{"category": "ui_structure", "complexity_level": "high"}')
        
        complexity_assessment=$(echo "$complexity_assessment" | jq ".ui_structure = $ui_complexity")
    fi
    
    # Overall complexity assessment
    local overall_complexity
    overall_complexity=$(echo "$complexity_assessment" | jq '{
        overall_complexity: (
            if ((.configuration.complexity_level == "low") and (.api_surface.complexity_level == "low")) then "medium"
            elif ((.configuration.complexity_level == "high") or (.api_surface.complexity_level == "high") or (.ui_structure.complexity_level == "high")) then "high"
            else "medium"
            end
        ),
        total_estimated_effort: "6-24 months",
        critical_path: [
            "API surface compatibility",
            "UI component replication",
            "Extension ecosystem support",
            "Performance optimization"
        ],
        success_factors: [
            "Comprehensive behavioral specification",
            "Incremental migration approach",
            "Extensive testing framework",
            "Performance benchmarking"
        ]
    }')
    
    complexity_assessment=$(echo "$complexity_assessment" | jq ". + $overall_complexity")
    
    echo "$complexity_assessment" > "$assessment_output"
    
    log_debug "Migration complexity assessment generated: $assessment_output"
}

# Generate comprehensive baseline comparison report
generate_baseline_comparison_report() {
    local output_dir="$1"
    
    log_info "Generating comprehensive VS Code OSS baseline comparison report"
    
    local report_file="$BASELINE_OUTPUT_DIR/vs_code_baseline_comparison.md"
    local json_report="$BASELINE_OUTPUT_DIR/vs_code_baseline_comparison.json"
    
    # Collect all comparison results
    local comparison_data="{}"
    
    # Load individual comparison results
    local config_comparison="$BASELINE_OUTPUT_DIR/configuration_comparison.json"
    local api_comparison="$BASELINE_OUTPUT_DIR/api_comparison.json"
    local ui_comparison="$BASELINE_OUTPUT_DIR/ui_comparison.json"
    local customizations="$BASELINE_OUTPUT_DIR/kiro_customizations.json"
    local complexity_assessment="$BASELINE_OUTPUT_DIR/migration_complexity.json"
    
    if [[ -f "$config_comparison" ]]; then
        comparison_data=$(echo "$comparison_data" | jq ".configuration = $(cat "$config_comparison")")
    fi
    
    if [[ -f "$api_comparison" ]]; then
        comparison_data=$(echo "$comparison_data" | jq ".api_surface = $(cat "$api_comparison")")
    fi
    
    if [[ -f "$ui_comparison" ]]; then
        comparison_data=$(echo "$comparison_data" | jq ".ui_structure = $(cat "$ui_comparison")")
    fi
    
    if [[ -f "$customizations" ]]; then
        comparison_data=$(echo "$comparison_data" | jq ".kiro_customizations = $(cat "$customizations")")
    fi
    
    if [[ -f "$complexity_assessment" ]]; then
        comparison_data=$(echo "$comparison_data" | jq ".migration_complexity = $(cat "$complexity_assessment")")
    fi
    
    # Add metadata
    comparison_data=$(echo "$comparison_data" | jq ". + {
        comparison_metadata: {
            vs_code_baseline_version: \"$VS_CODE_VERSION\",
            comparison_date: \"$(date -Iseconds)\",
            analysis_version: \"1.0.0\"
        }
    }")
    
    echo "$comparison_data" > "$json_report"
    
    # Generate markdown report
    cat > "$report_file" << EOF
# Kiro vs VS Code OSS Baseline Comparison

**Generated**: $(date -Iseconds)  
**VS Code OSS Baseline Version**: $VS_CODE_VERSION  
**Analysis Version**: 1.0.0

## Executive Summary

This document provides a comprehensive comparison between Kiro.dev and VS Code OSS $VS_CODE_VERSION, documenting differences, customizations, and migration complexity for the Rust/WASM transfiguration project.

## Configuration Comparison

### Commands Analysis
$(if [[ -f "$config_comparison" ]]; then
    jq -r '
        if .commands then
            "- **Kiro Commands**: " + (.commands.kiro_count | tostring) + "\n" +
            "- **VS Code Baseline Expected**: " + (.commands.baseline_expected | tostring) + "\n" +
            "- **Difference**: " + (.commands.difference | tostring) + " (" + (.commands.percentage_difference | tostring) + "%)\n" +
            "- **Status**: " + .commands.status
        else
            "Configuration comparison data not available"
        end
    ' "$config_comparison"
else
    echo "Configuration comparison not completed"
fi)

### Essential Commands Status
$(if [[ -f "$config_comparison" ]]; then
    jq -r '
        if .essential_commands_missing then
            if (.essential_commands_missing | length) == 0 then
                "✅ All essential VS Code commands are present in Kiro"
            else
                "⚠️ Missing essential commands:\n" + (.essential_commands_missing | map("  - " + .) | join("\n"))
            end
        else
            "Essential commands analysis not available"
        end
    ' "$config_comparison"
else
    echo "Essential commands analysis not completed"
fi)

## API Surface Comparison

### Contribution Points Coverage
$(if [[ -f "$api_comparison" ]]; then
    jq -r '
        if .contribution_points then
            "- **Coverage**: " + (.contribution_points.coverage_percentage | tostring) + "%\n" +
            "- **Status**: " + .contribution_points.status + "\n" +
            "- **Found Points**: " + (.contribution_points.found_points | length | tostring) + "\n" +
            "- **Missing Points**: " + (.contribution_points.missing_points | length | tostring)
        else
            "API comparison data not available"
        end
    ' "$api_comparison"
else
    echo "API comparison not completed"
fi)

## UI Structure Comparison

### Core Components Coverage
$(if [[ -f "$ui_comparison" ]]; then
    jq -r '
        if .ui_components then
            "- **Coverage**: " + (.ui_components.coverage_percentage | tostring) + "%\n" +
            "- **Status**: " + .ui_components.status + "\n" +
            "- **Found Components**: " + (.ui_components.found_components | join(", ")) + "\n" +
            if (.ui_components.missing_components | length) > 0 then
                "- **Missing Components**: " + (.ui_components.missing_components | join(", "))
            else
                "- **Missing Components**: None"
            end
        else
            "UI comparison data not available"
        end
    ' "$ui_comparison"
else
    echo "UI comparison not completed"
fi)

## Kiro-Specific Customizations

### Branding and Identity
$(if [[ -f "$customizations" ]]; then
    jq -r '
        if .branding then
            "- **Product Name**: " + (.branding.product_name // "N/A") + "\n" +
            "- **Application Name**: " + (.branding.application_name // "N/A") + "\n" +
            "- **Data Folder**: " + (.branding.data_folder_name // "N/A")
        else
            "Branding information not available"
        end
    ' "$customizations"
else
    echo "Customizations analysis not completed"
fi)

### Extension Marketplace
$(if [[ -f "$customizations" ]]; then
    jq -r '
        if .marketplace then
            "- **Marketplace**: " + .marketplace
        else
            "Marketplace information not available"
        end
    ' "$customizations"
else
    echo "Marketplace analysis not completed"
fi)

### AWS Integration
$(if [[ -f "$customizations" ]]; then
    jq -r '
        if .aws_integration then
            if (.aws_integration | length) > 0 then
                "✅ AWS integration detected:\n" + (.aws_integration | map("  - " + .) | join("\n"))
            else
                "❌ No AWS integration indicators found"
            end
        else
            "AWS integration analysis not available"
        end
    ' "$customizations"
else
    echo "AWS integration analysis not completed"
fi)

### Kiro-Specific Extensions
$(if [[ -f "$customizations" ]]; then
    jq -r '
        if .kiro_specific_extensions then
            if (.kiro_specific_extensions | length) > 0 then
                "Kiro-specific extensions found:\n" + (.kiro_specific_extensions | map("  - " + .) | join("\n"))
            else
                "No Kiro-specific extensions identified"
            end
        else
            "Extension analysis not available"
        end
    ' "$customizations"
else
    echo "Extension analysis not completed"
fi)

## Migration Complexity Assessment

### Overall Complexity
$(if [[ -f "$complexity_assessment" ]]; then
    jq -r '
        if .overall_complexity then
            "**Complexity Level**: " + .overall_complexity + "\n" +
            "**Estimated Effort**: " + .total_estimated_effort
        else
            "Overall complexity assessment not available"
        end
    ' "$complexity_assessment"
else
    echo "Complexity assessment not completed"
fi)

### Component-Specific Complexity

#### Configuration Migration
$(if [[ -f "$complexity_assessment" ]]; then
    jq -r '
        if .configuration then
            "- **Complexity**: " + .configuration.complexity_level + "\n" +
            "- **Effort**: " + .configuration.migration_effort + "\n" +
            if (.configuration.risk_factors | length) > 0 then
                "- **Risk Factors**: " + (.configuration.risk_factors | join(", "))
            else
                "- **Risk Factors**: None identified"
            end
        else
            "Configuration complexity assessment not available"
        end
    ' "$complexity_assessment"
else
    echo "Configuration complexity not assessed"
fi)

#### API Surface Migration
$(if [[ -f "$complexity_assessment" ]]; then
    jq -r '
        if .api_surface then
            "- **Complexity**: " + .api_surface.complexity_level + "\n" +
            "- **Effort**: " + .api_surface.migration_effort + "\n" +
            if (.api_surface.risk_factors | length) > 0 then
                "- **Risk Factors**: " + (.api_surface.risk_factors | join(", "))
            else
                "- **Risk Factors**: None identified"
            end
        else
            "API surface complexity assessment not available"
        end
    ' "$complexity_assessment"
else
    echo "API surface complexity not assessed"
fi)

#### UI Structure Migration
$(if [[ -f "$complexity_assessment" ]]; then
    jq -r '
        if .ui_structure then
            "- **Complexity**: " + .ui_structure.complexity_level + "\n" +
            "- **Effort**: " + .ui_structure.migration_effort + "\n" +
            if (.ui_structure.risk_factors | length) > 0 then
                "- **Risk Factors**: " + (.ui_structure.risk_factors | join(", "))
            else
                "- **Risk Factors**: None identified"
            end
        else
            "UI structure complexity assessment not available"
        end
    ' "$complexity_assessment"
else
    echo "UI structure complexity not assessed"
fi)

## Critical Path for Rust/WASM Implementation

$(if [[ -f "$complexity_assessment" ]]; then
    jq -r '
        if .critical_path then
            (.critical_path | map("1. " + .) | join("\n"))
        else
            "Critical path not defined"
        end
    ' "$complexity_assessment"
else
    echo "Critical path analysis not completed"
fi)

## Success Factors

$(if [[ -f "$complexity_assessment" ]]; then
    jq -r '
        if .success_factors then
            (.success_factors | map("- " + .) | join("\n"))
        else
            "Success factors not defined"
        end
    ' "$complexity_assessment"
else
    echo "Success factors analysis not completed"
fi)

## Recommendations for Phase 2

### High Priority
1. **API Compatibility Layer**: Ensure 100% VS Code extension API compatibility
2. **UI Component Replication**: Pixel-perfect recreation of core UI components
3. **Configuration Migration**: Seamless import of existing Kiro configurations

### Medium Priority
1. **Performance Optimization**: Leverage Rust/WASM for superior performance
2. **Extension Ecosystem**: Maintain compatibility with existing extensions
3. **Theme System**: Replicate theming and customization capabilities

### Low Priority
1. **Advanced Features**: Kiro-specific enhancements and AWS integration
2. **Marketplace Integration**: Open VSX Registry support
3. **Developer Tools**: Enhanced debugging and development features

## Compatibility Impact Analysis

### Breaking Changes Risk: $(if [[ -f "$complexity_assessment" ]]; then echo "$(jq -r '.overall_complexity' "$complexity_assessment" | tr '[:lower:]' '[:upper:]')"; else echo "UNKNOWN"; fi)

The migration to Rust/WASM architecture presents $(if [[ -f "$complexity_assessment" ]]; then jq -r '.overall_complexity' "$complexity_assessment"; else echo "unknown"; fi) complexity with careful attention required for:

- Extension API surface compatibility
- UI/UX behavioral fidelity  
- Configuration and settings migration
- Performance characteristic preservation

---

*Generated by Kiro Analysis Baseline Comparison v1.0.0*
EOF
    
    log_success "Baseline comparison report generated: $report_file"
}

# Main baseline comparison execution function
run_baseline_comparison() {
    local output_dir="$1"
    local config_file="$2"
    
    log_info "Running VS Code OSS baseline comparison analysis"
    
    # Initialize baseline comparison system
    init_baseline_comparison "$output_dir" "$config_file"
    
    # Run individual comparisons
    compare_configuration_differences "$output_dir" "$BASELINE_OUTPUT_DIR/configuration_comparison.json"
    compare_api_differences "$output_dir" "$BASELINE_OUTPUT_DIR/api_comparison.json"
    compare_ui_differences "$output_dir" "$BASELINE_OUTPUT_DIR/ui_comparison.json"
    
    # Identify Kiro customizations
    identify_kiro_customizations "$output_dir" "$BASELINE_OUTPUT_DIR/kiro_customizations.json"
    
    # Generate migration complexity assessment
    generate_migration_complexity_assessment "$BASELINE_OUTPUT_DIR" "$BASELINE_OUTPUT_DIR/migration_complexity.json"
    
    # Generate comprehensive report
    generate_baseline_comparison_report "$output_dir"
    
    log_success "VS Code OSS baseline comparison completed successfully"
    log_info "Results available at: $BASELINE_OUTPUT_DIR"
    
    return 0
}

# Export functions for use by main script
if [[ "${BASH_SOURCE[0]}" != "${0}" ]]; then
    export -f init_baseline_comparison get_vscode_baseline_data
    export -f compare_configuration_differences compare_api_differences compare_ui_differences
    export -f identify_kiro_customizations generate_migration_complexity_assessment
    export -f generate_baseline_comparison_report run_baseline_comparison
fi