#!/bin/bash

# Output Management and Documentation System
# Part of Kiro Behavioral Analysis Pipeline
#
# This module implements structured output management, JSON schema validation,
# progress tracking, and comprehensive documentation generation.

# Output format schemas
declare -A OUTPUT_SCHEMAS=(
    ["file_inventory"]='{"type":"array","items":{"type":"object","properties":{"path":{"type":"string"},"filename":{"type":"string"},"dirname":{"type":"string"},"extension":{"type":"string"},"category":{"type":"string"},"size_bytes":{"type":"number"},"modified_timestamp":{"type":"number"},"size_human":{"type":"string"}},"required":["path","filename","category","size_bytes"]}}'
    
    ["configuration_analysis"]='{"type":"object","properties":{"package_json":{"type":"object"},"product_json":{"type":"object"},"commands":{"type":"array"},"settings":{"type":"array"},"keybindings":{"type":"array"},"menus":{"type":"array"}}}'
    
    ["api_surface"]='{"type":"object","properties":{"interfaces":{"type":"array"},"contribution_points":{"type":"array"},"activation_events":{"type":"array"},"compatibility_matrix":{"type":"object"}}}'
    
    ["ui_structure"]='{"type":"object","properties":{"components":{"type":"array"},"layouts":{"type":"array"},"styling":{"type":"object"},"animations":{"type":"array"},"assets":{"type":"array"}}}'
    
    ["behavioral_patterns"]='{"type":"object","properties":{"event_handling":{"type":"array"},"state_management":{"type":"array"},"performance_patterns":{"type":"array"},"error_handling":{"type":"array"}}}'
    
    ["analysis_summary"]='{"type":"object","properties":{"metadata":{"type":"object"},"statistics":{"type":"object"},"phases":{"type":"object"},"validation":{"type":"object"},"recommendations":{"type":"array"}}}'
)

# Progress tracking
declare -A PROGRESS_STATE=(
    ["total_phases"]=6
    ["completed_phases"]=0
    ["current_phase"]=""
    ["phase_start_time"]=""
    ["total_files"]=0
    ["processed_files"]=0
    ["analysis_start_time"]=""
)

# Output directory structure
OUTPUT_BASE_DIR=""
OUTPUT_STRUCTURE=(
    "config/commands"
    "config/settings" 
    "config/keybindings"
    "config/menus"
    "config/themes"
    "api/interfaces"
    "api/contribution_points"
    "api/activation_events"
    "api/compatibility"
    "ui/components"
    "ui/layouts"
    "ui/styling"
    "ui/animations"
    "ui/assets"
    "behavior/events"
    "behavior/state"
    "behavior/performance"
    "behavior/errors"
    "reports/summary"
    "reports/cross_references"
    "reports/validation"
    "logs/analysis"
    "logs/errors"
    "logs/debug"
    "temp"
)

# Initialize output management system
init_output_management() {
    local output_dir="$1"
    
    OUTPUT_BASE_DIR="$output_dir"
    PROGRESS_STATE["analysis_start_time"]=$(date +%s)
    
    log_info "Initializing output management system: $output_dir"
    
    # Create directory structure
    create_output_directories "$output_dir"
    
    # Initialize progress tracking
    init_progress_tracking "$output_dir"
    
    # Create initial status file
    update_analysis_status "initialized" "Output management system initialized"
    
    log_success "Output management system initialized"
}

# Create structured output directories
create_output_directories() {
    local base_dir="$1"
    
    log_debug "Creating output directory structure"
    
    # Create base directory
    mkdir -p "$base_dir"
    
    # Create all subdirectories
    for dir_path in "${OUTPUT_STRUCTURE[@]}"; do
        mkdir -p "$base_dir/$dir_path"
        log_debug "Created directory: $base_dir/$dir_path"
    done
    
    # Create README files for each major section
    create_section_readmes "$base_dir"
    
    log_debug "Output directory structure created successfully"
}

# Create README files for documentation
create_section_readmes() {
    local base_dir="$1"
    
    # Main README
    cat > "$base_dir/README.md" << 'EOF'
# Kiro Behavioral Analysis Results

This directory contains the complete analysis results from the Kiro behavioral discovery pipeline.

## Directory Structure

- `config/` - Configuration analysis results (package.json, settings, commands, etc.)
- `api/` - API surface mapping results (interfaces, contribution points, compatibility)
- `ui/` - UI structure analysis results (components, styling, themes, assets)
- `behavior/` - Behavioral pattern inference results (events, state, performance)
- `reports/` - Summary reports and cross-reference documentation
- `logs/` - Analysis execution logs and debug information
- `temp/` - Temporary files and intermediate processing results

## Analysis Phases

1. **File Discovery & Validation** - Inventory and validate all extracted files
2. **Configuration Analysis** - Extract commands, settings, keybindings, menus
3. **API Surface Mapping** - Document extension APIs and contribution points
4. **UI Structure Analysis** - Analyze components, styling, and visual assets
5. **Behavioral Pattern Inference** - Infer runtime behaviors from code patterns
6. **Integration & Documentation** - Generate comprehensive behavioral specification

## Usage

Each directory contains JSON data files and corresponding markdown reports. The `reports/summary/` directory contains the final behavioral specification document.

Generated by Kiro Behavioral Analysis Pipeline v1.0.0
EOF

    # Config section README
    cat > "$base_dir/config/README.md" << 'EOF'
# Configuration Analysis Results

This directory contains analysis results for Kiro's configuration structure.

## Files

- `commands/` - Command definitions and mappings
- `settings/` - Settings schemas and default values
- `keybindings/` - Keyboard shortcut mappings
- `menus/` - Menu hierarchies and organization
- `themes/` - Theme definitions and customization options

## Data Format

All configuration data is stored in JSON format with corresponding markdown reports for human readability.
EOF

    # API section README
    cat > "$base_dir/api/README.md" << 'EOF'
# API Surface Analysis Results

This directory contains analysis results for Kiro's extension API surface.

## Files

- `interfaces/` - TypeScript interface definitions and method signatures
- `contribution_points/` - Extension contribution point mappings
- `activation_events/` - Extension activation event definitions
- `compatibility/` - VS Code API compatibility matrix

## Purpose

This analysis ensures 100% compatibility with existing VS Code extensions in the Rust/WASM implementation.
EOF

    # UI section README
    cat > "$base_dir/ui/README.md" << 'EOF'
# UI Structure Analysis Results

This directory contains analysis results for Kiro's user interface structure.

## Files

- `components/` - UI component hierarchies and structures
- `layouts/` - Layout systems and panel arrangements
- `styling/` - CSS analysis and styling patterns
- `animations/` - Animation definitions and transitions
- `assets/` - Media assets inventory and usage patterns

## Purpose

This analysis enables pixel-perfect UI replication in the Rust/WASM implementation.
EOF

    # Behavior section README
    cat > "$base_dir/behavior/README.md" << 'EOF'
# Behavioral Pattern Analysis Results

This directory contains inferred behavioral patterns from Kiro's codebase.

## Files

- `events/` - Event handling patterns and user interactions
- `state/` - State management and data flow patterns
- `performance/` - Performance optimization patterns
- `errors/` - Error handling and recovery mechanisms

## Purpose

This analysis captures runtime behaviors that must be replicated in the Rust/WASM implementation.
EOF

    log_debug "Section README files created"
}

# Progress tracking functions
init_progress_tracking() {
    local output_dir="$1"
    
    local progress_file="$output_dir/progress.json"
    
    # Initialize progress tracking file
    jq -n \
        --arg start_time "$(date -Iseconds)" \
        --argjson total_phases "${PROGRESS_STATE["total_phases"]}" \
        '{
            analysis_id: ("kiro_analysis_" + (now | tostring)),
            start_time: $start_time,
            status: "initialized",
            total_phases: $total_phases,
            completed_phases: 0,
            current_phase: null,
            phases: {},
            statistics: {
                total_files: 0,
                processed_files: 0,
                errors: 0,
                warnings: 0
            },
            last_updated: $start_time
        }' > "$progress_file"
    
    log_debug "Progress tracking initialized: $progress_file"
}

update_progress() {
    local phase_name="$1"
    local status="$2"
    local details="$3"
    
    local progress_file="$OUTPUT_BASE_DIR/progress.json"
    local timestamp
    timestamp=$(date -Iseconds)
    
    # Update progress file
    jq \
        --arg phase "$phase_name" \
        --arg status "$status" \
        --arg details "$details" \
        --arg timestamp "$timestamp" \
        '.current_phase = $phase |
         .phases[$phase] = {
             status: $status,
             details: $details,
             timestamp: $timestamp
         } |
         .last_updated = $timestamp |
         if $status == "completed" then
             .completed_phases += 1
         else . end' \
        "$progress_file" > "$progress_file.tmp" && mv "$progress_file.tmp" "$progress_file"
    
    # Update global progress state
    PROGRESS_STATE["current_phase"]="$phase_name"
    if [[ "$status" == "started" ]]; then
        PROGRESS_STATE["phase_start_time"]=$(date +%s)
    elif [[ "$status" == "completed" ]]; then
        PROGRESS_STATE["completed_phases"]=$((PROGRESS_STATE["completed_phases"] + 1))
    fi
    
    log_debug "Progress updated: $phase_name -> $status"
}

update_file_progress() {
    local processed_count="$1"
    local total_count="$2"
    
    local progress_file="$OUTPUT_BASE_DIR/progress.json"
    
    # Update file processing statistics
    jq \
        --argjson processed "$processed_count" \
        --argjson total "$total_count" \
        --arg timestamp "$(date -Iseconds)" \
        '.statistics.processed_files = $processed |
         .statistics.total_files = $total |
         .last_updated = $timestamp' \
        "$progress_file" > "$progress_file.tmp" && mv "$progress_file.tmp" "$progress_file"
    
    PROGRESS_STATE["processed_files"]="$processed_count"
    PROGRESS_STATE["total_files"]="$total_count"
}

# Status reporting
update_analysis_status() {
    local status="$1"
    local message="$2"
    
    local status_file="$OUTPUT_BASE_DIR/status.json"
    local timestamp
    timestamp=$(date -Iseconds)
    
    jq -n \
        --arg status "$status" \
        --arg message "$message" \
        --arg timestamp "$timestamp" \
        --argjson completed "${PROGRESS_STATE["completed_phases"]}" \
        --argjson total "${PROGRESS_STATE["total_phases"]}" \
        --arg current_phase "${PROGRESS_STATE["current_phase"]}" \
        '{
            status: $status,
            message: $message,
            timestamp: $timestamp,
            progress: {
                completed_phases: $completed,
                total_phases: $total,
                current_phase: $current_phase,
                percentage: (($completed / $total) * 100 | floor)
            }
        }' > "$status_file"
    
    log_info "Status: $status - $message"
}

# JSON schema validation
validate_output_schema() {
    local output_file="$1"
    local schema_name="$2"
    
    if [[ ! -f "$output_file" ]]; then
        log_error "Output file not found for validation: $output_file"
        return 1
    fi
    
    if [[ -z "${OUTPUT_SCHEMAS[$schema_name]:-}" ]]; then
        log_warn "No schema defined for: $schema_name"
        return 0  # Skip validation if no schema
    fi
    
    local schema="${OUTPUT_SCHEMAS[$schema_name]}"
    local temp_schema_file="/tmp/schema_${schema_name}_$$.json"
    
    # Write schema to temporary file
    echo "$schema" > "$temp_schema_file"
    
    # Validate using ajv-cli if available, otherwise use basic jq validation
    if command -v ajv >/dev/null 2>&1; then
        if ajv validate -s "$temp_schema_file" -d "$output_file" 2>/dev/null; then
            log_debug "Schema validation passed: $output_file"
            rm -f "$temp_schema_file"
            return 0
        else
            log_error "Schema validation failed: $output_file"
            rm -f "$temp_schema_file"
            return 1
        fi
    else
        # Basic JSON syntax validation with jq
        if jq empty "$output_file" 2>/dev/null; then
            log_debug "Basic JSON validation passed: $output_file"
            rm -f "$temp_schema_file"
            return 0
        else
            log_error "JSON syntax validation failed: $output_file"
            rm -f "$temp_schema_file"
            return 1
        fi
    fi
}

# Safe output writing with validation
safe_write_output() {
    local output_file="$1"
    local data="$2"
    local schema_name="$3"
    local backup_existing="${4:-true}"
    
    # Create directory if it doesn't exist
    mkdir -p "$(dirname "$output_file")"
    
    # Backup existing file if requested
    if [[ "$backup_existing" == "true" ]] && [[ -f "$output_file" ]]; then
        local backup_file="${output_file}.backup.$(date +%s)"
        cp "$output_file" "$backup_file"
        log_debug "Backed up existing file: $backup_file"
    fi
    
    # Write data to temporary file first
    local temp_file="${output_file}.tmp.$$"
    echo "$data" > "$temp_file"
    
    # Validate the output
    if [[ -n "$schema_name" ]]; then
        if ! validate_output_schema "$temp_file" "$schema_name"; then
            log_error "Output validation failed, not writing: $output_file"
            rm -f "$temp_file"
            return 1
        fi
    fi
    
    # Move temporary file to final location
    mv "$temp_file" "$output_file"
    
    log_debug "Output written successfully: $output_file"
    return 0
}

# Cross-reference generation
generate_cross_references() {
    local output_dir="$1"
    
    log_info "Generating cross-reference documentation"
    
    local cross_ref_file="$output_dir/reports/cross_references/index.json"
    local cross_ref_md="$output_dir/reports/cross_references/index.md"
    
    # Initialize cross-reference structure
    local cross_refs='{
        "commands_to_keybindings": {},
        "settings_to_ui": {},
        "apis_to_extensions": {},
        "components_to_styling": {},
        "events_to_handlers": {},
        "files_to_categories": {}
    }'
    
    # Generate file category cross-references
    if [[ -f "$output_dir/reports/file_inventory.json" ]]; then
        local file_categories
        file_categories=$(jq -r '
            group_by(.category) | 
            map({
                category: .[0].category,
                files: [.[].path],
                count: length
            }) | 
            from_entries
        ' "$output_dir/reports/file_inventory.json")
        
        cross_refs=$(echo "$cross_refs" | jq --argjson categories "$file_categories" '.files_to_categories = $categories')
    fi
    
    # Save cross-references
    safe_write_output "$cross_ref_file" "$cross_refs" ""
    
    # Generate markdown report
    cat > "$cross_ref_md" << 'EOF'
# Cross-Reference Index

This document provides cross-references between different analysis components.

## File Categories

EOF
    
    if [[ -f "$cross_ref_file" ]]; then
        jq -r '.files_to_categories | to_entries[] | "### \(.key | ascii_upcase)\n\n- **Count**: \(.value.count) files\n- **Files**: \(.value.files[0:5] | join(", "))\(.value.files | if length > 5 then " (and \(length - 5) more)" else "" end)\n"' "$cross_ref_file" >> "$cross_ref_md"
    fi
    
    log_success "Cross-reference documentation generated"
}

# Summary report generation
generate_summary_report() {
    local output_dir="$1"
    
    log_info "Generating comprehensive summary report"
    
    local summary_file="$output_dir/reports/summary/behavioral_specification.json"
    local summary_md="$output_dir/reports/summary/behavioral_specification.md"
    
    # Collect data from all analysis phases
    local metadata='{
        "analysis_version": "1.0.0",
        "generated_at": "'$(date -Iseconds)'",
        "source_path": "'${EXTRACTED_KIRO_PATH:-}'",
        "vs_code_base_version": "1.103.2"
    }'
    
    # Collect statistics
    local statistics='{}'
    
    if [[ -f "$output_dir/reports/file_inventory.json" ]]; then
        local file_stats
        file_stats=$(jq '{
            total_files: length,
            total_size: ([.[].size_bytes] | add),
            categories: (group_by(.category) | map({category: .[0].category, count: length}) | from_entries)
        }' "$output_dir/reports/file_inventory.json")
        
        statistics=$(echo "$statistics" | jq --argjson files "$file_stats" '.files = $files')
    fi
    
    # Collect phase results
    local phases='{}'
    
    # Add configuration analysis results if available
    if [[ -f "$output_dir/config/analysis_summary.json" ]]; then
        local config_summary
        config_summary=$(jq '.' "$output_dir/config/analysis_summary.json")
        phases=$(echo "$phases" | jq --argjson config "$config_summary" '.configuration = $config')
    fi
    
    # Create comprehensive summary
    local behavioral_spec
    behavioral_spec=$(jq -n \
        --argjson metadata "$metadata" \
        --argjson statistics "$statistics" \
        --argjson phases "$phases" \
        '{
            kiro_behavioral_spec: {
                version: "1.0.0",
                metadata: $metadata,
                statistics: $statistics,
                phases: $phases,
                validation: {
                    completed_at: "'$(date -Iseconds)'",
                    status: "completed"
                }
            }
        }')
    
    # Save behavioral specification
    safe_write_output "$summary_file" "$behavioral_spec" "analysis_summary"
    
    # Generate markdown summary
    cat > "$summary_md" << EOF
# Kiro Behavioral Specification

**Generated**: $(date -Iseconds)  
**Version**: 1.0.0  
**Source**: ${EXTRACTED_KIRO_PATH:-"Unknown"}

## Overview

This document contains the complete behavioral specification for Kiro.dev extracted through static analysis of the application files. This specification serves as the foundation for the Rust/WASM transfiguration project.

## Analysis Summary

### File Statistics

$(if [[ -f "$output_dir/reports/file_inventory.json" ]]; then
    echo "- **Total Files**: $(jq 'length' "$output_dir/reports/file_inventory.json")"
    echo "- **Total Size**: $(jq -r '[.[].size_bytes] | add | . / 1048576 | floor | tostring + "MB"' "$output_dir/reports/file_inventory.json")"
    echo ""
    echo "### File Categories"
    echo ""
    jq -r 'group_by(.category) | map("- **\(.[0].category | ascii_upcase)**: \(length) files") | .[]' "$output_dir/reports/file_inventory.json"
fi)

## Analysis Phases

1. ✅ **File Discovery & Validation** - Completed
2. 🔄 **Configuration Analysis** - In Progress  
3. ⏳ **API Surface Mapping** - Pending
4. ⏳ **UI Structure Analysis** - Pending
5. ⏳ **Behavioral Pattern Inference** - Pending
6. ⏳ **Integration & Documentation** - Pending

## Next Steps

This behavioral specification will be used in Phase 2: Architecture Design to create the Rust/WASM implementation plan.

---

*Generated by Kiro Behavioral Analysis Pipeline v1.0.0*
EOF
    
    log_success "Summary report generated: $summary_md"
}

# Analysis statistics collection
collect_analysis_statistics() {
    local output_dir="$1"
    
    local stats_file="$output_dir/reports/analysis_statistics.json"
    
    # Collect timing information
    local analysis_duration=0
    if [[ -n "${PROGRESS_STATE["analysis_start_time"]}" ]]; then
        analysis_duration=$(($(date +%s) - PROGRESS_STATE["analysis_start_time"]))
    fi
    
    # Collect file processing statistics
    local file_stats='{}'
    if [[ -f "$output_dir/reports/file_inventory.json" ]]; then
        file_stats=$(jq '{
            total_files: length,
            total_size_bytes: ([.[].size_bytes] | add),
            categories: (group_by(.category) | length),
            largest_file: (max_by(.size_bytes) | {filename: .filename, size: .size_human}),
            smallest_file: (min_by(.size_bytes) | {filename: .filename, size: .size_human})
        }' "$output_dir/reports/file_inventory.json")
    fi
    
    # Collect error statistics
    local error_stats='{}'
    if [[ -f "$OUTPUT_BASE_DIR/logs/errors_"*.log ]]; then
        local error_file
        error_file=$(ls -t "$OUTPUT_BASE_DIR/logs/errors_"*.log | head -1)
        
        local error_count
        error_count=$(grep -c "ERROR" "$error_file" 2>/dev/null || echo "0")
        local warning_count
        warning_count=$(grep -c "WARNING" "$error_file" 2>/dev/null || echo "0")
        local recovery_count
        recovery_count=$(grep -c "RECOVERY" "$error_file" 2>/dev/null || echo "0")
        
        error_stats=$(jq -n \
            --argjson errors "$error_count" \
            --argjson warnings "$warning_count" \
            --argjson recoveries "$recovery_count" \
            '{
                errors: $errors,
                warnings: $warnings,
                recoveries: $recoveries,
                success_rate: (1 - ($errors / ($errors + $warnings + 1))) * 100
            }')
    fi
    
    # Create comprehensive statistics
    local statistics
    statistics=$(jq -n \
        --argjson duration "$analysis_duration" \
        --argjson files "$file_stats" \
        --argjson errors "$error_stats" \
        --argjson completed "${PROGRESS_STATE["completed_phases"]}" \
        --argjson total "${PROGRESS_STATE["total_phases"]}" \
        '{
            analysis: {
                duration_seconds: $duration,
                duration_human: ("\($duration / 60 | floor)m \($duration % 60)s"),
                completed_phases: $completed,
                total_phases: $total,
                completion_percentage: (($completed / $total) * 100 | floor)
            },
            files: $files,
            errors: $errors,
            generated_at: "'$(date -Iseconds)'"
        }')
    
    safe_write_output "$stats_file" "$statistics" ""
    
    log_info "Analysis statistics collected: $stats_file"
}

# Cleanup temporary files
cleanup_output_temp() {
    local output_dir="$1"
    
    log_debug "Cleaning up temporary files"
    
    # Remove temporary files
    find "$output_dir/temp" -type f -name "*.tmp" -delete 2>/dev/null || true
    find "$output_dir" -type f -name "*.tmp.*" -delete 2>/dev/null || true
    
    # Compress old log files
    find "$output_dir/logs" -name "*.log" -mtime +7 -exec gzip {} \; 2>/dev/null || true
    
    log_debug "Temporary file cleanup completed"
}

# Export functions for use by main script
if [[ "${BASH_SOURCE[0]}" != "${0}" ]]; then
    # Script is being sourced, export functions
    export -f init_output_management create_output_directories
    export -f update_progress update_file_progress update_analysis_status
    export -f validate_output_schema safe_write_output
    export -f generate_cross_references generate_summary_report
    export -f collect_analysis_statistics cleanup_output_temp
fi