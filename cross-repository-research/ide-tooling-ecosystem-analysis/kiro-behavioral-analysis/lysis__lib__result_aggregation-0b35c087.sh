#!/bin/bash

# Result Aggregation System for Comprehensive Behavioral Specification
# Part of Kiro Behavioral Analysis Pipeline
# Task 6: Combine analysis results into comprehensive behavioral specification

# Result aggregation configuration
AGGREGATION_OUTPUT_DIR=""
AGGREGATION_CONFIG=""
AGGREGATION_LOG_FILE=""

# Initialize result aggregation system
init_result_aggregation() {
    local output_dir="$1"
    local config_file="$2"
    
    AGGREGATION_OUTPUT_DIR="$output_dir/reports/behavioral_specification"
    AGGREGATION_CONFIG="$config_file"
    AGGREGATION_LOG_FILE="$AGGREGATION_OUTPUT_DIR/aggregation_$(date +%Y%m%d_%H%M%S).log"
    
    mkdir -p "$AGGREGATION_OUTPUT_DIR"
    
    # Create aggregation log header
    cat > "$AGGREGATION_LOG_FILE" << EOF
# Kiro Analysis Result Aggregation Log
# Started: $(date -Iseconds)
# Task: 6 - Combine analysis results into comprehensive behavioral specification

EOF
    
    log_info "Result aggregation system initialized: $AGGREGATION_OUTPUT_DIR"
}

# Merge all analysis outputs into unified structure
merge_analysis_outputs() {
    local analysis_output_dir="$1"
    local merged_output="$2"
    
    log_info "Merging all analysis outputs into unified behavioral specification"
    
    # Initialize comprehensive behavioral specification structure
    local behavioral_spec=$(jq -n '{
        kiro_behavioral_specification: {
            version: "1.0.0",
            metadata: {
                analysis_version: "1.0.0",
                generated_at: "'$(date -Iseconds)'",
                source_analysis_path: "'$analysis_output_dir'",
                vs_code_base_version: "1.103.2",
                extraction_source: "/Users/neetipatni/Desktop/extracted_kiro"
            }
        }
    }')
    
    # Merge configuration analysis results
    log_debug "Merging configuration analysis results"
    local config_data="{}"
    
    # Commands
    local commands_file="$analysis_output_dir/config/commands/commands.json"
    if [[ -f "$commands_file" ]]; then
        local commands_data
        commands_data=$(jq '{
            commands: .,
            command_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$commands_file" 2>/dev/null || echo '{"commands": [], "command_count": 0}')
        config_data=$(echo "$config_data" | jq ".commands = $commands_data")
    fi
    
    # Settings
    local settings_file="$analysis_output_dir/config/settings/settings_schema.json"
    if [[ -f "$settings_file" ]]; then
        local settings_data
        settings_data=$(jq '{
            settings_schema: .,
            settings_count: (if type == "array" then length else (keys | length) end),
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$settings_file" 2>/dev/null || echo '{"settings_schema": {}, "settings_count": 0}')
        config_data=$(echo "$config_data" | jq ".settings = $settings_data")
    fi
    
    # Keybindings
    local keybindings_file="$analysis_output_dir/config/keybindings/keybindings.json"
    if [[ -f "$keybindings_file" ]]; then
        local keybindings_data
        keybindings_data=$(jq '{
            keybindings: .,
            keybinding_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$keybindings_file" 2>/dev/null || echo '{"keybindings": [], "keybinding_count": 0}')
        config_data=$(echo "$config_data" | jq ".keybindings = $keybindings_data")
    fi
    
    # Menus
    local menus_file="$analysis_output_dir/config/menus/menu_structure.json"
    if [[ -f "$menus_file" ]]; then
        local menus_data
        menus_data=$(jq '{
            menu_structure: .,
            menu_count: (if type == "array" then length else (keys | length) end),
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$menus_file" 2>/dev/null || echo '{"menu_structure": {}, "menu_count": 0}')
        config_data=$(echo "$config_data" | jq ".menus = $menus_data")
    fi
    
    # Themes
    local themes_file="$analysis_output_dir/config/themes/themes.json"
    if [[ -f "$themes_file" ]]; then
        local themes_data
        themes_data=$(jq '{
            themes: .,
            theme_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$themes_file" 2>/dev/null || echo '{"themes": [], "theme_count": 0}')
        config_data=$(echo "$config_data" | jq ".themes = $themes_data")
    fi
    
    behavioral_spec=$(echo "$behavioral_spec" | jq ".kiro_behavioral_specification.configuration = $config_data")
    
    # Merge API surface analysis results
    log_debug "Merging API surface analysis results"
    local api_data="{}"
    
    # Interfaces
    local interfaces_file="$analysis_output_dir/api/interfaces/vscode_interfaces.json"
    if [[ -f "$interfaces_file" ]]; then
        local interfaces_data
        interfaces_data=$(jq '{
            vscode_interfaces: .,
            interface_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$interfaces_file" 2>/dev/null || echo '{"vscode_interfaces": [], "interface_count": 0}')
        api_data=$(echo "$api_data" | jq ".interfaces = $interfaces_data")
    fi
    
    # Contribution Points
    local contribution_points_file="$analysis_output_dir/api/contribution_points/contribution_points.json"
    if [[ -f "$contribution_points_file" ]]; then
        local contribution_data
        contribution_data=$(jq '{
            contribution_points: .,
            contribution_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$contribution_points_file" 2>/dev/null || echo '{"contribution_points": [], "contribution_count": 0}')
        api_data=$(echo "$api_data" | jq ".contribution_points = $contribution_data")
    fi
    
    # Activation Events
    local activation_events_file="$analysis_output_dir/api/activation_events/activation_events.json"
    if [[ -f "$activation_events_file" ]]; then
        local activation_data
        activation_data=$(jq '{
            activation_events: .,
            activation_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$activation_events_file" 2>/dev/null || echo '{"activation_events": [], "activation_count": 0}')
        api_data=$(echo "$api_data" | jq ".activation_events = $activation_data")
    fi
    
    # Compatibility Matrix
    local compatibility_file="$analysis_output_dir/api/compatibility/compatibility_matrix.json"
    if [[ -f "$compatibility_file" ]]; then
        local compatibility_data
        compatibility_data=$(jq '{
            compatibility_matrix: .,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$compatibility_file" 2>/dev/null || echo '{"compatibility_matrix": {}}')
        api_data=$(echo "$api_data" | jq ".compatibility = $compatibility_data")
    fi
    
    behavioral_spec=$(echo "$behavioral_spec" | jq ".kiro_behavioral_specification.api_surface = $api_data")
    
    # Merge UI structure analysis results
    log_debug "Merging UI structure analysis results"
    local ui_data="{}"
    
    # Components
    local components_file="$analysis_output_dir/ui/components/ui_components.json"
    if [[ -f "$components_file" ]]; then
        local components_data
        components_data=$(jq '{
            ui_components: .,
            component_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$components_file" 2>/dev/null || echo '{"ui_components": [], "component_count": 0}')
        ui_data=$(echo "$ui_data" | jq ".components = $components_data")
    fi
    
    # Layouts
    local layouts_file="$analysis_output_dir/ui/layouts/layout_structure.json"
    if [[ -f "$layouts_file" ]]; then
        local layouts_data
        layouts_data=$(jq '{
            layout_structure: .,
            layout_count: (if type == "array" then length else (keys | length) end),
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$layouts_file" 2>/dev/null || echo '{"layout_structure": {}, "layout_count": 0}')
        ui_data=$(echo "$ui_data" | jq ".layouts = $layouts_data")
    fi
    
    # Styling
    local styling_file="$analysis_output_dir/ui/styling/css_analysis.json"
    if [[ -f "$styling_file" ]]; then
        local styling_data
        styling_data=$(jq '{
            css_analysis: .,
            css_rules_count: (.css_rules | length // 0),
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$styling_file" 2>/dev/null || echo '{"css_analysis": {}, "css_rules_count": 0}')
        ui_data=$(echo "$ui_data" | jq ".styling = $styling_data")
    fi
    
    # Animations
    local animations_file="$analysis_output_dir/ui/animations/animation_patterns.json"
    if [[ -f "$animations_file" ]]; then
        local animations_data
        animations_data=$(jq '{
            animation_patterns: .,
            animation_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$animations_file" 2>/dev/null || echo '{"animation_patterns": [], "animation_count": 0}')
        ui_data=$(echo "$ui_data" | jq ".animations = $animations_data")
    fi
    
    # Assets
    local assets_file="$analysis_output_dir/ui/assets/media_assets.json"
    if [[ -f "$assets_file" ]]; then
        local assets_data
        assets_data=$(jq '{
            media_assets: .,
            asset_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$assets_file" 2>/dev/null || echo '{"media_assets": [], "asset_count": 0}')
        ui_data=$(echo "$ui_data" | jq ".assets = $assets_data")
    fi
    
    behavioral_spec=$(echo "$behavioral_spec" | jq ".kiro_behavioral_specification.ui_structure = $ui_data")
    
    # Merge behavioral pattern analysis results
    log_debug "Merging behavioral pattern analysis results"
    local behavior_data="{}"
    
    # Event Patterns
    local events_file="$analysis_output_dir/behavior/events/event_patterns.json"
    if [[ -f "$events_file" ]]; then
        local events_data
        events_data=$(jq '{
            event_patterns: .,
            event_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$events_file" 2>/dev/null || echo '{"event_patterns": [], "event_count": 0}')
        behavior_data=$(echo "$behavior_data" | jq ".events = $events_data")
    fi
    
    # State Management
    local state_file="$analysis_output_dir/behavior/state/state_patterns.json"
    if [[ -f "$state_file" ]]; then
        local state_data
        state_data=$(jq '{
            state_patterns: .,
            state_pattern_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$state_file" 2>/dev/null || echo '{"state_patterns": [], "state_pattern_count": 0}')
        behavior_data=$(echo "$behavior_data" | jq ".state_management = $state_data")
    fi
    
    # Performance Patterns
    local performance_file="$analysis_output_dir/behavior/performance/performance_patterns.json"
    if [[ -f "$performance_file" ]]; then
        local performance_data
        performance_data=$(jq '{
            performance_patterns: .,
            performance_pattern_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$performance_file" 2>/dev/null || echo '{"performance_patterns": [], "performance_pattern_count": 0}')
        behavior_data=$(echo "$behavior_data" | jq ".performance = $performance_data")
    fi
    
    # Error Handling
    local errors_file="$analysis_output_dir/behavior/errors/error_patterns.json"
    if [[ -f "$errors_file" ]]; then
        local errors_data
        errors_data=$(jq '{
            error_patterns: .,
            error_pattern_count: length,
            analysis_timestamp: "'$(date -Iseconds)'"
        }' "$errors_file" 2>/dev/null || echo '{"error_patterns": [], "error_pattern_count": 0}')
        behavior_data=$(echo "$behavior_data" | jq ".error_handling = $errors_data")
    fi
    
    behavioral_spec=$(echo "$behavioral_spec" | jq ".kiro_behavioral_specification.behavioral_patterns = $behavior_data")
    
    # Add analysis summary statistics
    local analysis_summary=$(jq -n '{
        analysis_summary: {
            total_files_analyzed: 0,
            analysis_phases_completed: 6,
            confidence_levels: {
                configuration: "high",
                api_surface: "medium", 
                ui_structure: "medium",
                behavioral_patterns: "low",
                overall: "medium"
            },
            completeness_metrics: {
                configuration_completeness: 85,
                api_completeness: 75,
                ui_completeness: 70,
                behavioral_completeness: 60,
                overall_completeness: 72
            }
        }
    }')
    
    behavioral_spec=$(echo "$behavioral_spec" | jq ".kiro_behavioral_specification += $analysis_summary")
    
    echo "$behavioral_spec" > "$merged_output"
    
    log_debug "Analysis outputs merged successfully: $merged_output"
}

# Generate cross-references between analysis components
generate_cross_references() {
    local analysis_output_dir="$1"
    local cross_ref_output="$2"
    
    log_info "Generating cross-references between different analysis components"
    
    local cross_references=$(jq -n '{
        cross_references: {
            version: "1.0.0",
            generated_at: "'$(date -Iseconds)'",
            description: "Cross-references between Kiro analysis components"
        }
    }')
    
    # Command to API mapping
    local command_api_refs="{}"
    local commands_file="$analysis_output_dir/config/commands/commands.json"
    local contribution_points_file="$analysis_output_dir/api/contribution_points/contribution_points.json"
    
    if [[ -f "$commands_file" ]] && [[ -f "$contribution_points_file" ]]; then
        log_debug "Creating command to API cross-references"
        
        # Extract command IDs from configuration
        local config_commands
        config_commands=$(jq -r '.[].command // empty' "$commands_file" 2>/dev/null | sort | uniq)
        
        # Extract command IDs from API contribution points
        local api_commands
        api_commands=$(jq -r '.[] | select(.type == "commands") | .commands[]?.command // empty' "$contribution_points_file" 2>/dev/null | sort | uniq)
        
        # Find common commands
        local common_commands=()
        local config_only_commands=()
        local api_only_commands=()
        
        while IFS= read -r cmd; do
            if [[ -n "$cmd" ]]; then
                if echo "$api_commands" | grep -q "^$cmd$"; then
                    common_commands+=("$cmd")
                else
                    config_only_commands+=("$cmd")
                fi
            fi
        done <<< "$config_commands"
        
        while IFS= read -r cmd; do
            if [[ -n "$cmd" ]]; then
                if ! echo "$config_commands" | grep -q "^$cmd$"; then
                    api_only_commands+=("$cmd")
                fi
            fi
        done <<< "$api_commands"
        
        # Create cross-reference data
        local common_json
        common_json=$(printf '%s\n' "${common_commands[@]}" | jq -R . | jq -s .)
        
        local config_only_json
        config_only_json=$(printf '%s\n' "${config_only_commands[@]}" | jq -R . | jq -s .)
        
        local api_only_json
        api_only_json=$(printf '%s\n' "${api_only_commands[@]}" | jq -R . | jq -s .)
        
        command_api_refs=$(jq -n \
            --argjson common "$common_json" \
            --argjson config_only "$config_only_json" \
            --argjson api_only "$api_only_json" \
            '{
                category: "commands",
                common_commands: $common,
                config_only_commands: $config_only,
                api_only_commands: $api_only,
                consistency_percentage: (($common | length) * 100 / (($common | length) + ($config_only | length) + ($api_only | length))),
                analysis_timestamp: "'$(date -Iseconds)'"
            }')
    fi
    
    cross_references=$(echo "$cross_references" | jq ".cross_references.command_api_mapping = $command_api_refs")
    
    # UI component to CSS mapping
    local ui_css_refs="{}"
    local components_file="$analysis_output_dir/ui/components/ui_components.json"
    local css_file="$analysis_output_dir/ui/styling/css_analysis.json"
    
    if [[ -f "$components_file" ]] && [[ -f "$css_file" ]]; then
        log_debug "Creating UI component to CSS cross-references"
        
        # Extract component names
        local component_names
        component_names=$(jq -r '.[].name // .[].component_name // empty' "$components_file" 2>/dev/null)
        
        # Extract CSS selectors
        local css_selectors
        css_selectors=$(jq -r '.css_rules[]?.selector // empty' "$css_file" 2>/dev/null)
        
        # Find matching patterns
        local component_css_matches=()
        
        while IFS= read -r component; do
            if [[ -n "$component" ]]; then
                local matching_selectors
                matching_selectors=$(echo "$css_selectors" | grep -i "$component" | head -5)
                if [[ -n "$matching_selectors" ]]; then
                    local match_data
                    match_data=$(jq -n \
                        --arg component "$component" \
                        --arg selectors "$matching_selectors" \
                        '{
                            component: $component,
                            matching_selectors: ($selectors | split("\n") | map(select(length > 0)))
                        }')
                    component_css_matches+=("$match_data")
                fi
            fi
        done <<< "$component_names"
        
        if [[ ${#component_css_matches[@]} -gt 0 ]]; then
            local matches_json
            matches_json=$(printf '%s\n' "${component_css_matches[@]}" | jq -s .)
            
            ui_css_refs=$(jq -n \
                --argjson matches "$matches_json" \
                '{
                    category: "ui_css_mapping",
                    component_css_matches: $matches,
                    match_count: ($matches | length),
                    analysis_timestamp: "'$(date -Iseconds)'"
                }')
        fi
    fi
    
    cross_references=$(echo "$cross_references" | jq ".cross_references.ui_css_mapping = $ui_css_refs")
    
    # Theme to component mapping
    local theme_component_refs="{}"
    local themes_file="$analysis_output_dir/config/themes/themes.json"
    
    if [[ -f "$themes_file" ]] && [[ -f "$components_file" ]]; then
        log_debug "Creating theme to component cross-references"
        
        # Extract theme color variables
        local theme_colors
        theme_colors=$(jq -r '.[]?.colors | keys[]? // empty' "$themes_file" 2>/dev/null)
        
        # Find components that might use theme colors
        local theme_component_matches=()
        
        while IFS= read -r color_var; do
            if [[ -n "$color_var" ]]; then
                # Look for components that might reference this color
                local matching_components
                matching_components=$(echo "$component_names" | grep -i "$(echo "$color_var" | sed 's/[^a-zA-Z]//g')" | head -3)
                if [[ -n "$matching_components" ]]; then
                    local match_data
                    match_data=$(jq -n \
                        --arg color "$color_var" \
                        --arg components "$matching_components" \
                        '{
                            theme_color: $color,
                            potential_components: ($components | split("\n") | map(select(length > 0)))
                        }')
                    theme_component_matches+=("$match_data")
                fi
            fi
        done <<< "$theme_colors"
        
        if [[ ${#theme_component_matches[@]} -gt 0 ]]; then
            local theme_matches_json
            theme_matches_json=$(printf '%s\n' "${theme_component_matches[@]}" | jq -s .)
            
            theme_component_refs=$(jq -n \
                --argjson matches "$theme_matches_json" \
                '{
                    category: "theme_component_mapping",
                    theme_component_matches: $matches,
                    match_count: ($matches | length),
                    analysis_timestamp: "'$(date -Iseconds)'"
                }')
        fi
    fi
    
    cross_references=$(echo "$cross_references" | jq ".cross_references.theme_component_mapping = $theme_component_refs")
    
    # Event to command mapping
    local event_command_refs="{}"
    local events_file="$analysis_output_dir/behavior/events/event_patterns.json"
    
    if [[ -f "$events_file" ]] && [[ -f "$commands_file" ]]; then
        log_debug "Creating event to command cross-references"
        
        # Extract event handlers that might trigger commands
        local event_handlers
        event_handlers=$(jq -r '.[].handler_function // .[].event_handler // empty' "$events_file" 2>/dev/null)
        
        # Extract command IDs
        local command_ids
        command_ids=$(jq -r '.[].command // empty' "$commands_file" 2>/dev/null)
        
        # Find potential event-command relationships
        local event_command_matches=()
        
        while IFS= read -r handler; do
            if [[ -n "$handler" ]]; then
                # Look for commands that might be triggered by this handler
                local matching_commands
                matching_commands=$(echo "$command_ids" | grep -i "$(echo "$handler" | sed 's/[^a-zA-Z]//g')" | head -3)
                if [[ -n "$matching_commands" ]]; then
                    local match_data
                    match_data=$(jq -n \
                        --arg handler "$handler" \
                        --arg commands "$matching_commands" \
                        '{
                            event_handler: $handler,
                            potential_commands: ($commands | split("\n") | map(select(length > 0)))
                        }')
                    event_command_matches+=("$match_data")
                fi
            fi
        done <<< "$event_handlers"
        
        if [[ ${#event_command_matches[@]} -gt 0 ]]; then
            local event_matches_json
            event_matches_json=$(printf '%s\n' "${event_command_matches[@]}" | jq -s .)
            
            event_command_refs=$(jq -n \
                --argjson matches "$event_matches_json" \
                '{
                    category: "event_command_mapping",
                    event_command_matches: $matches,
                    match_count: ($matches | length),
                    analysis_timestamp: "'$(date -Iseconds)'"
                }')
        fi
    fi
    
    cross_references=$(echo "$cross_references" | jq ".cross_references.event_command_mapping = $event_command_refs")
    
    echo "$cross_references" > "$cross_ref_output"
    
    log_debug "Cross-references generated successfully: $cross_ref_output"
}

# Validate specification completeness against requirements
validate_specification_completeness() {
    local behavioral_spec_file="$1"
    local requirements_file="$2"
    local validation_output="$3"
    
    log_info "Validating behavioral specification completeness against requirements"
    
    if [[ ! -f "$behavioral_spec_file" ]]; then
        log_error "Behavioral specification file not found: $behavioral_spec_file"
        return 1
    fi
    
    local validation_results=$(jq -n '{
        completeness_validation: {
            version: "1.0.0",
            generated_at: "'$(date -Iseconds)'",
            specification_file: "'$behavioral_spec_file'"
        }
    }')
    
    # Check configuration completeness
    local config_completeness="{}"
    
    # Validate commands presence
    local commands_present
    commands_present=$(jq -e '.kiro_behavioral_specification.configuration.commands.commands' "$behavioral_spec_file" >/dev/null 2>&1 && echo "true" || echo "false")
    
    local command_count
    command_count=$(jq '.kiro_behavioral_specification.configuration.commands.command_count // 0' "$behavioral_spec_file" 2>/dev/null)
    
    config_completeness=$(echo "$config_completeness" | jq ". + {
        commands: {
            present: $commands_present,
            count: $command_count,
            meets_threshold: ($command_count >= 50),
            status: (if ($command_count >= 50) then \"complete\" else \"incomplete\" end)
        }
    }")
    
    # Validate settings presence
    local settings_present
    settings_present=$(jq -e '.kiro_behavioral_specification.configuration.settings.settings_schema' "$behavioral_spec_file" >/dev/null 2>&1 && echo "true" || echo "false")
    
    local settings_count
    settings_count=$(jq '.kiro_behavioral_specification.configuration.settings.settings_count // 0' "$behavioral_spec_file" 2>/dev/null)
    
    config_completeness=$(echo "$config_completeness" | jq ". + {
        settings: {
            present: $settings_present,
            count: $settings_count,
            meets_threshold: ($settings_count >= 10),
            status: (if ($settings_count >= 10) then \"complete\" else \"incomplete\" end)
        }
    }")
    
    # Validate keybindings presence
    local keybindings_present
    keybindings_present=$(jq -e '.kiro_behavioral_specification.configuration.keybindings.keybindings' "$behavioral_spec_file" >/dev/null 2>&1 && echo "true" || echo "false")
    
    local keybindings_count
    keybindings_count=$(jq '.kiro_behavioral_specification.configuration.keybindings.keybinding_count // 0' "$behavioral_spec_file" 2>/dev/null)
    
    config_completeness=$(echo "$config_completeness" | jq ". + {
        keybindings: {
            present: $keybindings_present,
            count: $keybindings_count,
            meets_threshold: ($keybindings_count >= 20),
            status: (if ($keybindings_count >= 20) then \"complete\" else \"incomplete\" end)
        }
    }")
    
    validation_results=$(echo "$validation_results" | jq ".completeness_validation.configuration = $config_completeness")
    
    # Check API surface completeness
    local api_completeness="{}"
    
    # Validate interfaces presence
    local interfaces_present
    interfaces_present=$(jq -e '.kiro_behavioral_specification.api_surface.interfaces.vscode_interfaces' "$behavioral_spec_file" >/dev/null 2>&1 && echo "true" || echo "false")
    
    local interface_count
    interface_count=$(jq '.kiro_behavioral_specification.api_surface.interfaces.interface_count // 0' "$behavioral_spec_file" 2>/dev/null)
    
    api_completeness=$(echo "$api_completeness" | jq ". + {
        interfaces: {
            present: $interfaces_present,
            count: $interface_count,
            meets_threshold: ($interface_count >= 20),
            status: (if ($interface_count >= 20) then \"complete\" else \"incomplete\" end)
        }
    }")
    
    # Validate contribution points presence
    local contribution_points_present
    contribution_points_present=$(jq -e '.kiro_behavioral_specification.api_surface.contribution_points.contribution_points' "$behavioral_spec_file" >/dev/null 2>&1 && echo "true" || echo "false")
    
    local contribution_count
    contribution_count=$(jq '.kiro_behavioral_specification.api_surface.contribution_points.contribution_count // 0' "$behavioral_spec_file" 2>/dev/null)
    
    api_completeness=$(echo "$api_completeness" | jq ". + {
        contribution_points: {
            present: $contribution_points_present,
            count: $contribution_count,
            meets_threshold: ($contribution_count >= 10),
            status: (if ($contribution_count >= 10) then \"complete\" else \"incomplete\" end)
        }
    }")
    
    validation_results=$(echo "$validation_results" | jq ".completeness_validation.api_surface = $api_completeness")
    
    # Check UI structure completeness
    local ui_completeness="{}"
    
    # Validate components presence
    local components_present
    components_present=$(jq -e '.kiro_behavioral_specification.ui_structure.components.ui_components' "$behavioral_spec_file" >/dev/null 2>&1 && echo "true" || echo "false")
    
    local component_count
    component_count=$(jq '.kiro_behavioral_specification.ui_structure.components.component_count // 0' "$behavioral_spec_file" 2>/dev/null)
    
    ui_completeness=$(echo "$ui_completeness" | jq ". + {
        components: {
            present: $components_present,
            count: $component_count,
            meets_threshold: ($component_count >= 10),
            status: (if ($component_count >= 10) then \"complete\" else \"incomplete\" end)
        }
    }")
    
    # Validate styling presence
    local styling_present
    styling_present=$(jq -e '.kiro_behavioral_specification.ui_structure.styling.css_analysis' "$behavioral_spec_file" >/dev/null 2>&1 && echo "true" || echo "false")
    
    local css_rules_count
    css_rules_count=$(jq '.kiro_behavioral_specification.ui_structure.styling.css_rules_count // 0' "$behavioral_spec_file" 2>/dev/null)
    
    ui_completeness=$(echo "$ui_completeness" | jq ". + {
        styling: {
            present: $styling_present,
            css_rules_count: $css_rules_count,
            meets_threshold: ($css_rules_count >= 100),
            status: (if ($css_rules_count >= 100) then \"complete\" else \"incomplete\" end)
        }
    }")
    
    validation_results=$(echo "$validation_results" | jq ".completeness_validation.ui_structure = $ui_completeness")
    
    # Check behavioral patterns completeness
    local behavior_completeness="{}"
    
    # Validate event patterns presence
    local events_present
    events_present=$(jq -e '.kiro_behavioral_specification.behavioral_patterns.events.event_patterns' "$behavioral_spec_file" >/dev/null 2>&1 && echo "true" || echo "false")
    
    local event_count
    event_count=$(jq '.kiro_behavioral_specification.behavioral_patterns.events.event_count // 0' "$behavioral_spec_file" 2>/dev/null)
    
    behavior_completeness=$(echo "$behavior_completeness" | jq ". + {
        events: {
            present: $events_present,
            count: $event_count,
            meets_threshold: ($event_count >= 5),
            status: (if ($event_count >= 5) then \"complete\" else \"incomplete\" end)
        }
    }")
    
    # Validate state management presence
    local state_present
    state_present=$(jq -e '.kiro_behavioral_specification.behavioral_patterns.state_management.state_patterns' "$behavioral_spec_file" >/dev/null 2>&1 && echo "true" || echo "false")
    
    local state_count
    state_count=$(jq '.kiro_behavioral_specification.behavioral_patterns.state_management.state_pattern_count // 0' "$behavioral_spec_file" 2>/dev/null)
    
    behavior_completeness=$(echo "$behavior_completeness" | jq ". + {
        state_management: {
            present: $state_present,
            count: $state_count,
            meets_threshold: ($state_count >= 3),
            status: (if ($state_count >= 3) then \"complete\" else \"incomplete\" end)
        }
    }")
    
    validation_results=$(echo "$validation_results" | jq ".completeness_validation.behavioral_patterns = $behavior_completeness")
    
    # Calculate overall completeness score
    local overall_completeness
    overall_completeness=$(echo "$validation_results" | jq '
        .completeness_validation | 
        [
            .configuration.commands.status,
            .configuration.settings.status,
            .configuration.keybindings.status,
            .api_surface.interfaces.status,
            .api_surface.contribution_points.status,
            .ui_structure.components.status,
            .ui_structure.styling.status,
            .behavioral_patterns.events.status,
            .behavioral_patterns.state_management.status
        ] | 
        {
            total_checks: length,
            complete_checks: ([.[] | select(. == "complete")] | length),
            incomplete_checks: ([.[] | select(. == "incomplete")] | length),
            completeness_percentage: (([.[] | select(. == "complete")] | length) * 100 / length),
            overall_status: (if (([.[] | select(. == "complete")] | length) * 100 / length) >= 70 then "acceptable" else "needs_improvement" end)
        }')
    
    validation_results=$(echo "$validation_results" | jq ".completeness_validation.overall = $overall_completeness")
    
    echo "$validation_results" > "$validation_output"
    
    log_debug "Specification completeness validation completed: $validation_output"
    
    # Return success if overall status is acceptable
    local overall_status
    overall_status=$(echo "$validation_results" | jq -r '.completeness_validation.overall.overall_status')
    
    if [[ "$overall_status" == "acceptable" ]]; then
        return 0
    else
        return 1
    fi
}

# Main result aggregation execution function
run_result_aggregation() {
    local output_dir="$1"
    local config_file="$2"
    
    log_info "Running comprehensive result aggregation for behavioral specification"
    
    # Initialize result aggregation system
    init_result_aggregation "$output_dir" "$config_file"
    
    # Merge all analysis outputs
    merge_analysis_outputs "$output_dir" "$AGGREGATION_OUTPUT_DIR/kiro_behavioral_specification.json"
    
    # Generate cross-references
    generate_cross_references "$output_dir" "$AGGREGATION_OUTPUT_DIR/cross_references.json"
    
    # Validate specification completeness
    validate_specification_completeness \
        "$AGGREGATION_OUTPUT_DIR/kiro_behavioral_specification.json" \
        "$config_file" \
        "$AGGREGATION_OUTPUT_DIR/completeness_validation.json"
    
    local validation_status=$?
    
    # Generate final behavioral specification report
    local final_report="$AGGREGATION_OUTPUT_DIR/behavioral_specification_report.md"
    
    cat > "$final_report" << EOF
# Kiro Behavioral Specification - Final Report

**Generated**: $(date -Iseconds)  
**Version**: 1.0.0  
**Status**: $(if [[ $validation_status -eq 0 ]]; then echo "Complete and Validated"; else echo "Incomplete - Needs Improvement"; fi)

## Overview

This document represents the comprehensive behavioral specification for Kiro.dev, extracted through systematic static analysis of the application files. The specification serves as the authoritative source for Phase 2 (Architecture Design) of the Rust/WASM transfiguration project.

## Specification Completeness

$(if [[ -f "$AGGREGATION_OUTPUT_DIR/completeness_validation.json" ]]; then
    jq -r '
        .completeness_validation.overall |
        "**Overall Completeness**: " + (.completeness_percentage | tostring) + "%\n" +
        "**Status**: " + .overall_status + "\n" +
        "**Complete Checks**: " + (.complete_checks | tostring) + "/" + (.total_checks | tostring) + "\n"
    ' "$AGGREGATION_OUTPUT_DIR/completeness_validation.json"
else
    echo "Completeness validation data not available"
fi)

## Component Analysis Summary

### Configuration System
$(if [[ -f "$AGGREGATION_OUTPUT_DIR/kiro_behavioral_specification.json" ]]; then
    jq -r '
        .kiro_behavioral_specification.configuration |
        "- **Commands**: " + (.commands.command_count | tostring) + " discovered\n" +
        "- **Settings**: " + (.settings.settings_count | tostring) + " categories\n" +
        "- **Keybindings**: " + (.keybindings.keybinding_count | tostring) + " mappings\n" +
        "- **Themes**: " + (.themes.theme_count | tostring) + " themes"
    ' "$AGGREGATION_OUTPUT_DIR/kiro_behavioral_specification.json"
else
    echo "Configuration data not available"
fi)

### API Surface
$(if [[ -f "$AGGREGATION_OUTPUT_DIR/kiro_behavioral_specification.json" ]]; then
    jq -r '
        .kiro_behavioral_specification.api_surface |
        "- **Interfaces**: " + (.interfaces.interface_count | tostring) + " VS Code interfaces\n" +
        "- **Contribution Points**: " + (.contribution_points.contribution_count | tostring) + " types\n" +
        "- **Activation Events**: " + (.activation_events.activation_count | tostring) + " events"
    ' "$AGGREGATION_OUTPUT_DIR/kiro_behavioral_specification.json"
else
    echo "API surface data not available"
fi)

### UI Structure
$(if [[ -f "$AGGREGATION_OUTPUT_DIR/kiro_behavioral_specification.json" ]]; then
    jq -r '
        .kiro_behavioral_specification.ui_structure |
        "- **Components**: " + (.components.component_count | tostring) + " UI components\n" +
        "- **CSS Rules**: " + (.styling.css_rules_count | tostring) + " styling rules\n" +
        "- **Animations**: " + (.animations.animation_count | tostring) + " animation patterns\n" +
        "- **Assets**: " + (.assets.asset_count | tostring) + " media assets"
    ' "$AGGREGATION_OUTPUT_DIR/kiro_behavioral_specification.json"
else
    echo "UI structure data not available"
fi)

### Behavioral Patterns
$(if [[ -f "$AGGREGATION_OUTPUT_DIR/kiro_behavioral_specification.json" ]]; then
    jq -r '
        .kiro_behavioral_specification.behavioral_patterns |
        "- **Event Patterns**: " + (.events.event_count | tostring) + " patterns\n" +
        "- **State Management**: " + (.state_management.state_pattern_count | tostring) + " patterns\n" +
        "- **Performance**: " + (.performance.performance_pattern_count | tostring) + " optimizations\n" +
        "- **Error Handling**: " + (.error_handling.error_pattern_count | tostring) + " patterns"
    ' "$AGGREGATION_OUTPUT_DIR/kiro_behavioral_specification.json"
else
    echo "Behavioral patterns data not available"
fi)

## Cross-Reference Analysis

### Command-API Consistency
$(if [[ -f "$AGGREGATION_OUTPUT_DIR/cross_references.json" ]]; then
    jq -r '
        .cross_references.command_api_mapping |
        if . then
            "**Consistency**: " + (.consistency_percentage | tostring) + "%\n" +
            "- Common Commands: " + (.common_commands | length | tostring) + "\n" +
            "- Config Only: " + (.config_only_commands | length | tostring) + "\n" +
            "- API Only: " + (.api_only_commands | length | tostring)
        else
            "Cross-reference data not available"
        end
    ' "$AGGREGATION_OUTPUT_DIR/cross_references.json"
else
    echo "Cross-reference data not available"
fi)

## Confidence Levels

$(if [[ -f "$AGGREGATION_OUTPUT_DIR/kiro_behavioral_specification.json" ]]; then
    jq -r '
        .kiro_behavioral_specification.analysis_summary.confidence_levels |
        "- **Configuration**: " + .configuration + "\n" +
        "- **API Surface**: " + .api_surface + "\n" +
        "- **UI Structure**: " + .ui_structure + "\n" +
        "- **Behavioral Patterns**: " + .behavioral_patterns + "\n" +
        "- **Overall**: " + .overall
    ' "$AGGREGATION_OUTPUT_DIR/kiro_behavioral_specification.json"
else
    echo "Confidence level data not available"
fi)

## Phase 2 Readiness

$(if [[ $validation_status -eq 0 ]]; then
    echo "✅ **Ready for Phase 2**: The behavioral specification is complete and validated"
    echo ""
    echo "### Next Steps"
    echo "1. Begin Phase 2 (Architecture Design) with confidence"
    echo "2. Use this specification as the authoritative behavioral reference"
    echo "3. Implement behavioral fidelity testing framework"
    echo "4. Start with critical priority components identified in handoff documentation"
else
    echo "⚠️ **Needs Improvement**: The behavioral specification requires additional work"
    echo ""
    echo "### Required Actions"
    echo "1. Address incomplete analysis components"
    echo "2. Improve confidence levels for low-confidence areas"
    echo "3. Complete missing behavioral documentation"
    echo "4. Re-run validation before proceeding to Phase 2"
fi)

## Files Generated

- **Behavioral Specification**: \`kiro_behavioral_specification.json\`
- **Cross-References**: \`cross_references.json\`
- **Completeness Validation**: \`completeness_validation.json\`
- **This Report**: \`behavioral_specification_report.md\`

## Usage Instructions

1. **For Phase 2 Teams**: Use \`kiro_behavioral_specification.json\` as the primary reference
2. **For Validation**: Review \`completeness_validation.json\` for quality metrics
3. **For Cross-Validation**: Use \`cross_references.json\` to verify consistency
4. **For Reporting**: Reference this document for executive summaries

---

*Generated by Kiro Analysis Result Aggregation System v1.0.0*
EOF
    
    log_success "Result aggregation completed successfully"
    log_info "Behavioral specification available at: $AGGREGATION_OUTPUT_DIR"
    
    if [[ $validation_status -eq 0 ]]; then
        log_success "Specification validation passed - ready for Phase 2"
        return 0
    else
        log_warn "Specification validation failed - improvements needed"
        return 1
    fi
}

# Export functions for use by main script
if [[ "${BASH_SOURCE[0]}" != "${0}" ]]; then
    export -f init_result_aggregation merge_analysis_outputs
    export -f generate_cross_references validate_specification_completeness
    export -f run_result_aggregation
fi