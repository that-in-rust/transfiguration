#!/bin/bash

# Configuration Analyzer Module
# Part of Kiro Behavioral Analysis Pipeline
#
# This module implements comprehensive configuration analysis for package.json 
# and product.json files, extracting commands, settings, keybindings, and menus.

# Configuration analysis functions

# Extract command definitions from package.json files
extract_command_definitions() {
    local input_path="$1"
    local output_dir="$2"
    
    log_info "Extracting command definitions from package.json files"
    
    local commands_output="$output_dir/config/commands/command_definitions.json"
    local commands_report="$output_dir/config/commands/command_analysis.md"
    
    mkdir -p "$(dirname "$commands_output")"
    
    # Find all package.json files
    local package_files=()
    while IFS= read -r -d '' file; do
        package_files+=("$file")
    done < <(find "$input_path" -name "package.json" -type f -print0 2>/dev/null)
    
    log_info "Found ${#package_files[@]} package.json files"
    
    # Initialize results structure
    local all_commands='[]'
    local command_count=0
    
    # Process each package.json file
    for package_file in "${package_files[@]}"; do
        log_debug "Processing package.json: $package_file"
        
        # Validate JSON syntax
        if ! jq empty "$package_file" 2>/dev/null; then
            log_warn "Invalid JSON in package.json: $package_file"
            continue
        fi
        
        # Extract commands from contributes.commands
        local commands
        commands=$(jq -r '.contributes.commands // []' "$package_file" 2>/dev/null)
        
        if [[ "$commands" != "[]" && "$commands" != "null" ]]; then
            # Add source file information to each command
            local commands_with_source
            commands_with_source=$(echo "$commands" | jq --arg source "$package_file" '
                map(. + {
                    "source_file": $source,
                    "extracted_at": now
                })
            ')
            
            # Merge with all commands
            all_commands=$(echo "$all_commands" | jq ". + $commands_with_source")
            
            local file_command_count
            file_command_count=$(echo "$commands" | jq 'length')
            command_count=$((command_count + file_command_count))
            
            log_debug "Extracted $file_command_count commands from $package_file"
        fi
    done
    
    # Save command definitions
    echo "$all_commands" | jq '.' > "$commands_output"
    
    # Generate command analysis report
    generate_command_analysis_report "$all_commands" "$commands_report"
    
    log_success "Command extraction completed: $command_count commands found"
    return 0
}

# Extract menu structures and hierarchies
extract_menu_structures() {
    local input_path="$1"
    local output_dir="$2"
    
    log_info "Extracting menu structures and hierarchies"
    
    local menus_output="$output_dir/config/menus/menu_structures.json"
    local menus_report="$output_dir/config/menus/menu_analysis.md"
    
    mkdir -p "$(dirname "$menus_output")"
    
    # Find all package.json files
    local package_files=()
    while IFS= read -r -d '' file; do
        package_files+=("$file")
    done < <(find "$input_path" -name "package.json" -type f -print0 2>/dev/null)
    
    # Initialize results structure
    local all_menus='{
        "command_palette": [],
        "context_menus": [],
        "menu_bar": [],
        "view_menus": [],
        "editor_menus": []
    }'
    
    # Process each package.json file
    for package_file in "${package_files[@]}"; do
        log_debug "Processing menus in: $package_file"
        
        # Validate JSON syntax
        if ! jq empty "$package_file" 2>/dev/null; then
            continue
        fi
        
        # Extract different menu types
        local menus
        menus=$(jq -r '.contributes.menus // {}' "$package_file" 2>/dev/null)
        
        if [[ "$menus" != "{}" && "$menus" != "null" ]]; then
            # Process each menu type
            local menu_types=("commandPalette" "editor/context" "view/title" "view/item/context" "explorer/context")
            
            for menu_type in "${menu_types[@]}"; do
                local menu_items
                menu_items=$(echo "$menus" | jq --arg type "$menu_type" '.[$type] // []')
                
                if [[ "$menu_items" != "[]" && "$menu_items" != "null" ]]; then
                    # Add source information
                    local items_with_source
                    items_with_source=$(echo "$menu_items" | jq --arg source "$package_file" --arg type "$menu_type" '
                        map(. + {
                            "source_file": $source,
                            "menu_type": $type,
                            "extracted_at": now
                        })
                    ')
                    
                    # Categorize menu items
                    case "$menu_type" in
                        "commandPalette")
                            all_menus=$(echo "$all_menus" | jq ".command_palette += $items_with_source")
                            ;;
                        "editor/context")
                            all_menus=$(echo "$all_menus" | jq ".context_menus += $items_with_source")
                            ;;
                        *)
                            all_menus=$(echo "$all_menus" | jq ".view_menus += $items_with_source")
                            ;;
                    esac
                fi
            done
        fi
    done
    
    # Save menu structures
    echo "$all_menus" | jq '.' > "$menus_output"
    
    # Generate menu analysis report
    generate_menu_analysis_report "$all_menus" "$menus_report"
    
    log_success "Menu structure extraction completed"
    return 0
}

# Extract keybinding definitions
extract_keybinding_definitions() {
    local input_path="$1"
    local output_dir="$2"
    
    log_info "Extracting keybinding definitions and shortcuts"
    
    local keybindings_output="$output_dir/config/keybindings/keybinding_definitions.json"
    local keybindings_report="$output_dir/config/keybindings/keybinding_analysis.md"
    
    mkdir -p "$(dirname "$keybindings_output")"
    
    # Find all package.json files
    local package_files=()
    while IFS= read -r -d '' file; do
        package_files+=("$file")
    done < <(find "$input_path" -name "package.json" -type f -print0 2>/dev/null)
    
    # Initialize results structure
    local all_keybindings='[]'
    local keybinding_count=0
    
    # Process each package.json file
    for package_file in "${package_files[@]}"; do
        log_debug "Processing keybindings in: $package_file"
        
        # Validate JSON syntax
        if ! jq empty "$package_file" 2>/dev/null; then
            continue
        fi
        
        # Extract keybindings from contributes.keybindings
        local keybindings
        keybindings=$(jq -r '.contributes.keybindings // []' "$package_file" 2>/dev/null)
        
        if [[ "$keybindings" != "[]" && "$keybindings" != "null" ]]; then
            # Add source file information and platform detection
            local keybindings_with_source
            keybindings_with_source=$(echo "$keybindings" | jq --arg source "$package_file" '
                map(. + {
                    "source_file": $source,
                    "extracted_at": now,
                    "platforms": {
                        "mac": (.mac // .key),
                        "win": (.win // .key),
                        "linux": (.linux // .key)
                    }
                })
            ')
            
            # Merge with all keybindings
            all_keybindings=$(echo "$all_keybindings" | jq ". + $keybindings_with_source")
            
            local file_keybinding_count
            file_keybinding_count=$(echo "$keybindings" | jq 'length')
            keybinding_count=$((keybinding_count + file_keybinding_count))
            
            log_debug "Extracted $file_keybinding_count keybindings from $package_file"
        fi
    done
    
    # Save keybinding definitions
    echo "$all_keybindings" | jq '.' > "$keybindings_output"
    
    # Generate keybinding analysis report
    generate_keybinding_analysis_report "$all_keybindings" "$keybindings_report"
    
    log_success "Keybinding extraction completed: $keybinding_count keybindings found"
    return 0
}

# Extract settings and preferences schemas
extract_settings_schemas() {
    local input_path="$1"
    local output_dir="$2"
    
    log_info "Extracting settings and preferences schemas"
    
    local settings_output="$output_dir/config/settings/settings_schemas.json"
    local settings_report="$output_dir/config/settings/settings_analysis.md"
    
    mkdir -p "$(dirname "$settings_output")"
    
    # Find all package.json files
    local package_files=()
    while IFS= read -r -d '' file; do
        package_files+=("$file")
    done < <(find "$input_path" -name "package.json" -type f -print0 2>/dev/null)
    
    # Initialize results structure
    local all_settings='[]'
    local settings_count=0
    
    # Process each package.json file
    for package_file in "${package_files[@]}"; do
        log_debug "Processing settings in: $package_file"
        
        # Validate JSON syntax
        if ! jq empty "$package_file" 2>/dev/null; then
            continue
        fi
        
        # Extract configuration from contributes.configuration
        local configuration
        configuration=$(jq -r '.contributes.configuration // {}' "$package_file" 2>/dev/null)
        
        if [[ "$configuration" != "{}" && "$configuration" != "null" ]]; then
            # Extract properties (settings definitions)
            local properties
            properties=$(echo "$configuration" | jq -r '.properties // {}')
            
            if [[ "$properties" != "{}" && "$properties" != "null" ]]; then
                # Convert properties object to array with setting names
                local settings_array
                settings_array=$(echo "$properties" | jq --arg source "$package_file" '
                    to_entries | map({
                        "setting_name": .key,
                        "schema": .value,
                        "source_file": $source,
                        "extracted_at": now,
                        "type": (.value.type // "unknown"),
                        "default": (.value.default // null),
                        "description": (.value.description // ""),
                        "enum_values": (.value.enum // null),
                        "scope": (.value.scope // "window")
                    })
                ')
                
                # Merge with all settings
                all_settings=$(echo "$all_settings" | jq ". + $settings_array")
                
                local file_settings_count
                file_settings_count=$(echo "$settings_array" | jq 'length')
                settings_count=$((settings_count + file_settings_count))
                
                log_debug "Extracted $file_settings_count settings from $package_file"
            fi
        fi
    done
    
    # Save settings schemas
    echo "$all_settings" | jq '.' > "$settings_output"
    
    # Generate settings analysis report
    generate_settings_analysis_report "$all_settings" "$settings_report"
    
    log_success "Settings extraction completed: $settings_count settings found"
    return 0
}

# Parse product.json for branding and feature flags
parse_product_json() {
    local input_path="$1"
    local output_dir="$2"
    
    log_info "Parsing product.json for branding and feature flags"
    
    local product_output="$output_dir/config/product_configuration.json"
    local product_report="$output_dir/config/product_analysis.md"
    
    mkdir -p "$(dirname "$product_output")"
    
    # Find product.json file
    local product_file
    product_file=$(find "$input_path" -name "product.json" -type f | head -1)
    
    if [[ -z "$product_file" ]]; then
        log_warn "No product.json file found in $input_path"
        echo '{"error": "product.json not found"}' > "$product_output"
        return 1
    fi
    
    log_info "Found product.json: $product_file"
    
    # Validate JSON syntax
    if ! jq empty "$product_file" 2>/dev/null; then
        log_error "Invalid JSON in product.json: $product_file"
        echo '{"error": "invalid JSON syntax"}' > "$product_output"
        return 1
    fi
    
    # Extract comprehensive product configuration
    local product_config
    product_config=$(jq '{
        "branding": {
            "name": (.name // .nameLong // "Unknown"),
            "nameLong": (.nameLong // .name // "Unknown"),
            "applicationName": (.applicationName // .name // "Unknown"),
            "version": (.version // "Unknown"),
            "commit": (.commit // "Unknown"),
            "quality": (.quality // "Unknown"),
            "dataFolderName": (.dataFolderName // "Unknown")
        },
        "urls": {
            "webEndpointUrl": (.webEndpointUrl // null),
            "webEndpointUrlTemplate": (.webEndpointUrlTemplate // null),
            "webviewContentExternalBaseUrlTemplate": (.webviewContentExternalBaseUrlTemplate // null),
            "updateUrl": (.updateUrl // null),
            "releaseNotesUrl": (.releaseNotesUrl // null),
            "keyboardShortcutsUrlMac": (.keyboardShortcutsUrlMac // null),
            "keyboardShortcutsUrlLinux": (.keyboardShortcutsUrlLinux // null),
            "keyboardShortcutsUrlWin": (.keyboardShortcutsUrlWin // null),
            "introductoryVideosUrl": (.introductoryVideosUrl // null),
            "tipsAndTricksUrl": (.tipsAndTricksUrl // null),
            "documentationUrl": (.documentationUrl // null),
            "requestFeatureUrl": (.requestFeatureUrl // null),
            "reportIssueUrl": (.reportIssueUrl // null),
            "licenseUrl": (.licenseUrl // null),
            "privacyStatementUrl": (.privacyStatementUrl // null)
        },
        "feature_flags": {
            "enableTelemetry": (.enableTelemetry // false),
            "enableCrashReporter": (.enableCrashReporter // false),
            "sendASmile": (.sendASmile // {}),
            "extensionsGallery": (.extensionsGallery // {}),
            "extensionTips": (.extensionTips // {}),
            "extensionImportantTips": (.extensionImportantTips // {}),
            "configurationSync": (.configurationSync // {}),
            "surveys": (.surveys // [])
        },
        "built_in_extensions": (.builtInExtensions // []),
        "extension_gallery": (.extensionsGallery // {}),
        "telemetry": {
            "enableTelemetry": (.enableTelemetry // false),
            "aiConfig": (.aiConfig // {}),
            "msftInternalDomains": (.msftInternalDomains // [])
        },
        "ai_configuration": (.aiConfig // {}),
        "extracted_at": now,
        "source_file": "'$product_file'"
    }' "$product_file")
    
    # Save product configuration
    echo "$product_config" > "$product_output"
    
    # Generate product analysis report
    generate_product_analysis_report "$product_config" "$product_report"
    
    log_success "Product.json parsing completed"
    return 0
}

# Report generation functions

generate_command_analysis_report() {
    local commands_data="$1"
    local report_file="$2"
    
    local total_commands
    total_commands=$(echo "$commands_data" | jq 'length')
    
    local command_categories
    command_categories=$(echo "$commands_data" | jq -r '
        group_by(.category // "uncategorized") | 
        map({
            category: (.[0].category // "uncategorized"),
            count: length,
            commands: [.[].command]
        }) | 
        sort_by(-.count)
    ')
    
    cat > "$report_file" << EOF
# Command Definitions Analysis

**Generated**: $(date -Iseconds)  
**Total Commands**: $total_commands

## Command Categories

$(echo "$command_categories" | jq -r '.[] | "### \(.category | ascii_upcase)\n\n- **Count**: \(.count)\n- **Commands**: \(.commands | join(", "))\n"')

## Command Details

$(echo "$commands_data" | jq -r '.[] | "### \(.title // .command)\n\n- **Command**: `\(.command)`\n- **Category**: \(.category // "uncategorized")\n- **When**: \(.when // "always")\n- **Source**: \(.source_file)\n"')

---
*Generated by Kiro Configuration Analyzer*
EOF
}

generate_menu_analysis_report() {
    local menus_data="$1"
    local report_file="$2"
    
    local command_palette_count
    command_palette_count=$(echo "$menus_data" | jq '.command_palette | length')
    
    local context_menus_count
    context_menus_count=$(echo "$menus_data" | jq '.context_menus | length')
    
    local view_menus_count
    view_menus_count=$(echo "$menus_data" | jq '.view_menus | length')
    
    cat > "$report_file" << EOF
# Menu Structures Analysis

**Generated**: $(date -Iseconds)

## Menu Summary

- **Command Palette Items**: $command_palette_count
- **Context Menu Items**: $context_menus_count  
- **View Menu Items**: $view_menus_count

## Command Palette

$(echo "$menus_data" | jq -r '.command_palette[] | "- **\(.command)** (when: \(.when // "always"))"')

## Context Menus

$(echo "$menus_data" | jq -r '.context_menus[] | "- **\(.command)** (group: \(.group // "navigation"))"')

## View Menus

$(echo "$menus_data" | jq -r '.view_menus[] | "- **\(.command)** (when: \(.when // "always"))"')

---
*Generated by Kiro Configuration Analyzer*
EOF
}

generate_keybinding_analysis_report() {
    local keybindings_data="$1"
    local report_file="$2"
    
    local total_keybindings
    total_keybindings=$(echo "$keybindings_data" | jq 'length')
    
    local platform_specific
    platform_specific=$(echo "$keybindings_data" | jq '[.[] | select(.mac != .win or .win != .linux)] | length')
    
    cat > "$report_file" << EOF
# Keybinding Definitions Analysis

**Generated**: $(date -Iseconds)  
**Total Keybindings**: $total_keybindings  
**Platform-Specific**: $platform_specific

## Platform Variations

$(echo "$keybindings_data" | jq -r '.[] | select(.platforms.mac != .platforms.win or .platforms.win != .platforms.linux) | "### \(.command)\n\n- **macOS**: \(.platforms.mac // "none")\n- **Windows**: \(.platforms.win // "none")\n- **Linux**: \(.platforms.linux // "none")\n- **When**: \(.when // "always")\n"')

## All Keybindings

$(echo "$keybindings_data" | jq -r '.[] | "- **\(.command)**: \(.key) (when: \(.when // "always"))"')

---
*Generated by Kiro Configuration Analyzer*
EOF
}

generate_settings_analysis_report() {
    local settings_data="$1"
    local report_file="$2"
    
    local total_settings
    total_settings=$(echo "$settings_data" | jq 'length')
    
    local settings_by_type
    settings_by_type=$(echo "$settings_data" | jq -r '
        group_by(.type) | 
        map({
            type: .[0].type,
            count: length
        }) | 
        sort_by(-.count)
    ')
    
    cat > "$report_file" << EOF
# Settings Schema Analysis

**Generated**: $(date -Iseconds)  
**Total Settings**: $total_settings

## Settings by Type

$(echo "$settings_by_type" | jq -r '.[] | "- **\(.type)**: \(.count) settings"')

## Setting Categories

$(echo "$settings_data" | jq -r 'group_by(.setting_name | split(".")[0]) | map({category: (.[0].setting_name | split(".")[0]), count: length}) | sort_by(-.count) | .[] | "### \(.category | ascii_upcase)\n\n**Count**: \(.count) settings\n"')

## All Settings

$(echo "$settings_data" | jq -r '.[] | "### \(.setting_name)\n\n- **Type**: \(.type)\n- **Default**: \(.default // "none")\n- **Scope**: \(.scope)\n- **Description**: \(.description)\n"')

---
*Generated by Kiro Configuration Analyzer*
EOF
}

generate_product_analysis_report() {
    local product_data="$1"
    local report_file="$2"
    
    cat > "$report_file" << EOF
# Product Configuration Analysis

**Generated**: $(date -Iseconds)

## Branding Information

$(echo "$product_data" | jq -r '.branding | to_entries | .[] | "- **\(.key | ascii_upcase)**: \(.value)"')

## URLs and Endpoints

$(echo "$product_data" | jq -r '.urls | to_entries | map(select(.value != null)) | .[] | "- **\(.key)**: \(.value)"')

## Feature Flags

$(echo "$product_data" | jq -r '.feature_flags | to_entries | .[] | "- **\(.key)**: \(.value)"')

## Built-in Extensions

$(echo "$product_data" | jq -r '.built_in_extensions[] | "- \(.)"')

## AI Configuration

$(echo "$product_data" | jq -r '.ai_configuration | to_entries | .[] | "- **\(.key)**: \(.value)"')

---
*Generated by Kiro Configuration Analyzer*
EOF
}

# Main configuration analysis function
run_configuration_analysis() {
    local input_path="$1"
    local output_dir="$2"
    local config_file="$3"
    
    log_info "Running comprehensive configuration analysis"
    
    # Check if configuration analysis is enabled
    local enabled
    enabled=$(jq -r '.analysis.phases.configuration_analysis.enabled' "$config_file" 2>/dev/null || echo "true")
    
    if [[ "$enabled" != "true" ]]; then
        log_info "Configuration analysis is disabled in config"
        return 0
    fi
    
    # Run all configuration analysis subtasks
    extract_command_definitions "$input_path" "$output_dir"
    extract_menu_structures "$input_path" "$output_dir"
    extract_keybinding_definitions "$input_path" "$output_dir"
    extract_settings_schemas "$input_path" "$output_dir"
    parse_product_json "$input_path" "$output_dir"
    
    # Generate comprehensive configuration summary
    generate_configuration_summary "$output_dir"
    
    log_success "Configuration analysis completed successfully"
    return 0
}

# Generate comprehensive configuration summary
generate_configuration_summary() {
    local output_dir="$1"
    
    local summary_file="$output_dir/config/configuration_summary.json"
    local summary_report="$output_dir/config/configuration_summary.md"
    
    log_info "Generating configuration analysis summary"
    
    # Collect all configuration data
    local commands_file="$output_dir/config/commands/command_definitions.json"
    local menus_file="$output_dir/config/menus/menu_structures.json"
    local keybindings_file="$output_dir/config/keybindings/keybinding_definitions.json"
    local settings_file="$output_dir/config/settings/settings_schemas.json"
    local product_file="$output_dir/config/product_configuration.json"
    
    # Create comprehensive summary
    local summary='{
        "analysis_metadata": {
            "generated_at": "'$(date -Iseconds)'",
            "version": "1.0.0"
        },
        "statistics": {},
        "files_analyzed": []
    }'
    
    # Add statistics for each component
    if [[ -f "$commands_file" ]]; then
        local command_count
        command_count=$(jq 'length' "$commands_file" 2>/dev/null || echo "0")
        summary=$(echo "$summary" | jq ".statistics.commands = $command_count")
        summary=$(echo "$summary" | jq '.files_analyzed += ["commands"]')
    fi
    
    if [[ -f "$keybindings_file" ]]; then
        local keybinding_count
        keybinding_count=$(jq 'length' "$keybindings_file" 2>/dev/null || echo "0")
        summary=$(echo "$summary" | jq ".statistics.keybindings = $keybinding_count")
        summary=$(echo "$summary" | jq '.files_analyzed += ["keybindings"]')
    fi
    
    if [[ -f "$settings_file" ]]; then
        local settings_count
        settings_count=$(jq 'length' "$settings_file" 2>/dev/null || echo "0")
        summary=$(echo "$summary" | jq ".statistics.settings = $settings_count")
        summary=$(echo "$summary" | jq '.files_analyzed += ["settings"]')
    fi
    
    # Save summary
    echo "$summary" > "$summary_file"
    
    # Generate markdown summary
    cat > "$summary_report" << EOF
# Configuration Analysis Summary

**Generated**: $(date -Iseconds)

## Analysis Statistics

$(echo "$summary" | jq -r '.statistics | to_entries | .[] | "- **\(.key | ascii_upcase)**: \(.value)"')

## Components Analyzed

$(echo "$summary" | jq -r '.files_analyzed[] | "- \(. | ascii_upcase)"')

## Files Generated

- Command definitions: \`config/commands/command_definitions.json\`
- Menu structures: \`config/menus/menu_structures.json\`
- Keybinding definitions: \`config/keybindings/keybinding_definitions.json\`
- Settings schemas: \`config/settings/settings_schemas.json\`
- Product configuration: \`config/product_configuration.json\`

---
*Generated by Kiro Configuration Analyzer v1.0.0*
EOF
    
    log_success "Configuration summary generated: $summary_report"
}

# Export functions for use by main script
if [[ "${BASH_SOURCE[0]}" != "${0}" ]]; then
    export -f extract_command_definitions extract_menu_structures
    export -f extract_keybinding_definitions extract_settings_schemas
    export -f parse_product_json run_configuration_analysis
    export -f generate_configuration_summary
fi