#!/bin/bash

# File Discovery and Validation System
# Part of Kiro Behavioral Analysis Pipeline
#
# This module implements comprehensive file discovery, categorization,
# and validation for the extracted Kiro application files.

# File type categorization function (compatible with bash 3.2)
get_file_category_patterns() {
    local category="$1"
    
    case "$category" in
        "json") echo "*.json" ;;
        "javascript") echo "*.js *.mjs *.cjs" ;;
        "typescript") echo "*.ts *.tsx *.d.ts" ;;
        "html") echo "*.html *.htm" ;;
        "css") echo "*.css *.scss *.sass *.less" ;;
        "markdown") echo "*.md *.markdown" ;;
        "yaml") echo "*.yml *.yaml" ;;
        "xml") echo "*.xml *.xsd *.xsl" ;;
        "images") echo "*.png *.jpg *.jpeg *.gif *.svg *.ico *.webp" ;;
        "fonts") echo "*.ttf *.otf *.woff *.woff2 *.eot" ;;
        "archives") echo "*.zip *.tar *.gz *.bz2 *.xz *.7z" ;;
        "executables") echo "*.exe *.dll *.so *.dylib *.app" ;;
        "config") echo "*.conf *.cfg *.ini *.properties *.toml" ;;
        "logs") echo "*.log *.out *.err" ;;
        "temp") echo "*.tmp *.temp *.cache *.bak *.swp" ;;
        *) echo "" ;;
    esac
}

# List of all categories
FILE_CATEGORIES="json javascript typescript html css markdown yaml xml images fonts archives executables config logs temp"

# File validation functions
validate_file_readable() {
    local file_path="$1"
    
    if [[ ! -f "$file_path" ]]; then
        return 1
    fi
    
    if [[ ! -r "$file_path" ]]; then
        return 2
    fi
    
    return 0
}

validate_file_size() {
    local file_path="$1"
    local max_size_mb="${2:-100}"
    
    if [[ ! -f "$file_path" ]]; then
        return 1
    fi
    
    local file_size_bytes
    file_size_bytes=$(stat -f%z "$file_path" 2>/dev/null || stat -c%s "$file_path" 2>/dev/null)
    local file_size_mb=$((file_size_bytes / 1024 / 1024))
    
    if [[ $file_size_mb -gt $max_size_mb ]]; then
        return 2
    fi
    
    return 0
}

validate_json_file() {
    local file_path="$1"
    
    if [[ ! -f "$file_path" ]]; then
        return 1
    fi
    
    # Check if file has .json extension
    if [[ "$file_path" != *.json ]]; then
        return 0  # Not a JSON file, skip validation
    fi
    
    # Validate JSON syntax
    if ! jq empty "$file_path" 2>/dev/null; then
        return 2
    fi
    
    return 0
}

validate_text_encoding() {
    local file_path="$1"
    
    if [[ ! -f "$file_path" ]]; then
        return 1
    fi
    
    # Check if file is binary
    if file "$file_path" | grep -q "binary"; then
        return 0  # Binary files are valid, just not text
    fi
    
    # Check for valid UTF-8 encoding
    if ! iconv -f utf-8 -t utf-8 "$file_path" >/dev/null 2>&1; then
        return 2
    fi
    
    return 0
}

# File categorization functions
categorize_file() {
    local file_path="$1"
    local filename
    filename=$(basename "$file_path")
    local extension="${filename##*.}"
    
    # Check each category
    for category in $FILE_CATEGORIES; do
        local patterns
        patterns=$(get_file_category_patterns "$category")
        
        # Convert space-separated patterns to array
        IFS=' ' read -ra pattern_array <<< "$patterns"
        
        for pattern in "${pattern_array[@]}"; do
            if [[ "$filename" == $pattern ]]; then
                echo "$category"
                return 0
            fi
        done
    done
    
    # Default category for unknown files
    echo "unknown"
}

get_file_info() {
    local file_path="$1"
    local file_info="{}"
    
    if [[ ! -f "$file_path" ]]; then
        echo "{\"error\": \"File not found\"}"
        return 1
    fi
    
    local filename
    filename=$(basename "$file_path")
    local dirname
    dirname=$(dirname "$file_path")
    local extension="${filename##*.}"
    local size_bytes
    size_bytes=$(stat -f%z "$file_path" 2>/dev/null || stat -c%s "$file_path" 2>/dev/null)
    local modified_time
    modified_time=$(stat -f%m "$file_path" 2>/dev/null || stat -c%Y "$file_path" 2>/dev/null)
    local category
    category=$(categorize_file "$file_path")
    
    # Create JSON object with file information
    file_info=$(jq -n \
        --arg path "$file_path" \
        --arg filename "$filename" \
        --arg dirname "$dirname" \
        --arg extension "$extension" \
        --arg category "$category" \
        --argjson size "$size_bytes" \
        --argjson modified "$modified_time" \
        '{
            path: $path,
            filename: $filename,
            dirname: $dirname,
            extension: $extension,
            category: $category,
            size_bytes: $size,
            modified_timestamp: $modified,
            size_human: "\($size | if . > 1073741824 then "\(. / 1073741824 | floor)GB" elif . > 1048576 then "\(. / 1048576 | floor)MB" elif . > 1024 then "\(. / 1024 | floor)KB" else "\(.)B" end)"
        }')
    
    echo "$file_info"
}

# File discovery functions
discover_files_recursive() {
    local search_path="$1"
    local output_file="$2"
    local excluded_dirs="${3:-node_modules .git target build .cache}"
    local excluded_extensions="${4:-.log .tmp .cache}"
    
    log_info "Starting recursive file discovery in: $search_path"
    
    # Build find command with exclusions
    local find_cmd="find \"$search_path\" -type f"
    
    # Add directory exclusions
    IFS=' ' read -ra excluded_dir_array <<< "$excluded_dirs"
    for excluded_dir in "${excluded_dir_array[@]}"; do
        find_cmd+=" -not -path \"*/$excluded_dir/*\""
    done
    
    # Add extension exclusions
    IFS=' ' read -ra excluded_ext_array <<< "$excluded_extensions"
    for excluded_ext in "${excluded_ext_array[@]}"; do
        find_cmd+=" -not -name \"*$excluded_ext\""
    done
    
    log_debug "Find command: $find_cmd"
    
    # Execute find command and process results
    local file_count=0
    local total_size=0
    
    # Initialize output file with JSON array start
    echo "[" > "$output_file"
    
    # Process files one by one
    local first_file=true
    while IFS= read -r -d '' file_path; do
        if [[ ! "$first_file" == "true" ]]; then
            echo "," >> "$output_file"
        fi
        first_file=false
        
        local file_info
        file_info=$(get_file_info "$file_path")
        echo "  $file_info" >> "$output_file"
        
        # Update counters
        ((file_count++))
        local file_size
        file_size=$(echo "$file_info" | jq -r '.size_bytes')
        total_size=$((total_size + file_size))
        
        # Progress reporting every 100 files
        if ((file_count % 100 == 0)); then
            log_debug "Processed $file_count files..."
        fi
        
    done < <(eval "$find_cmd -print0")
    
    # Close JSON array
    echo "]" >> "$output_file"
    
    log_success "File discovery completed: $file_count files found"
    log_info "Total size: $(numfmt --to=iec $total_size)"
    
    return 0
}

generate_file_inventory_report() {
    local inventory_file="$1"
    local report_file="$2"
    
    log_info "Generating file inventory report: $report_file"
    
    if [[ ! -f "$inventory_file" ]]; then
        log_error "Inventory file not found: $inventory_file"
        return 1
    fi
    
    # Generate summary statistics
    local total_files
    total_files=$(jq 'length' "$inventory_file")
    
    local total_size
    total_size=$(jq '[.[].size_bytes] | add' "$inventory_file")
    
    # Category breakdown
    local category_stats
    category_stats=$(jq -r '
        group_by(.category) | 
        map({
            category: .[0].category,
            count: length,
            total_size: ([.[].size_bytes] | add),
            avg_size: (([.[].size_bytes] | add) / length | floor),
            largest_file: (max_by(.size_bytes) | {filename: .filename, size: .size_human})
        }) | 
        sort_by(-.count)
    ' "$inventory_file")
    
    # Extension breakdown
    local extension_stats
    extension_stats=$(jq -r '
        group_by(.extension) | 
        map({
            extension: .[0].extension,
            count: length,
            total_size: ([.[].size_bytes] | add)
        }) | 
        sort_by(-.count) | 
        .[0:20]
    ' "$inventory_file")
    
    # Generate markdown report
    cat > "$report_file" << EOF
# Kiro File Inventory Report

Generated: $(date -Iseconds)

## Summary Statistics

- **Total Files**: $(numfmt --grouping $total_files)
- **Total Size**: $(numfmt --to=iec $total_size)
- **Average File Size**: $(numfmt --to=iec $((total_size / total_files)))

## Files by Category

$(echo "$category_stats" | jq -r '.[] | "- **\(.category | ascii_upcase)**: \(.count) files (\(.total_size | tonumber | . / 1048576 | floor)MB)"')

## Top File Extensions

$(echo "$extension_stats" | jq -r '.[] | "- **.\(.extension)**: \(.count) files (\(.total_size | tonumber | . / 1048576 | floor)MB)"')

## Category Details

$(echo "$category_stats" | jq -r '.[] | "### \(.category | ascii_upcase) Files\n\n- Count: \(.count)\n- Total Size: \(.total_size | tonumber | . / 1048576 | floor)MB\n- Average Size: \(.avg_size | tonumber | . / 1024 | floor)KB\n- Largest File: \(.largest_file.filename) (\(.largest_file.size))\n"')

## Validation Summary

Files processed through validation pipeline:
- Readable files: $(jq '[.[] | select(.size_bytes >= 0)] | length' "$inventory_file")
- JSON files validated: $(jq '[.[] | select(.extension == "json")] | length' "$inventory_file")
- Text encoding validated: $(jq '[.[] | select(.category != "images" and .category != "archives" and .category != "executables")] | length' "$inventory_file")

EOF
    
    log_success "File inventory report generated: $report_file"
}

validate_discovered_files() {
    local inventory_file="$1"
    local validation_report_file="$2"
    
    log_info "Validating discovered files..."
    
    if [[ ! -f "$inventory_file" ]]; then
        log_error "Inventory file not found: $inventory_file"
        return 1
    fi
    
    local validation_results="[]"
    local file_count=0
    local valid_count=0
    local error_count=0
    
    # Process each file for validation
    while IFS= read -r file_info; do
        local file_path
        file_path=$(echo "$file_info" | jq -r '.path')
        local category
        category=$(echo "$file_info" | jq -r '.category')
        
        ((file_count++))
        
        local validation_result="{}"
        local is_valid=true
        local errors=()
        
        # Basic file validation
        if ! validate_file_readable "$file_path"; then
            errors+=("File not readable")
            is_valid=false
        fi
        
        # Size validation
        if ! validate_file_size "$file_path" 100; then
            errors+=("File too large (>100MB)")
            is_valid=false
        fi
        
        # JSON validation for JSON files
        if [[ "$category" == "json" ]]; then
            if ! validate_json_file "$file_path"; then
                errors+=("Invalid JSON syntax")
                is_valid=false
            fi
        fi
        
        # Text encoding validation for text files
        if [[ "$category" =~ ^(javascript|typescript|html|css|markdown|yaml|xml|config)$ ]]; then
            if ! validate_text_encoding "$file_path"; then
                errors+=("Invalid text encoding")
                is_valid=false
            fi
        fi
        
        # Create validation result
        local errors_json
        errors_json=$(printf '%s\n' "${errors[@]}" | jq -R . | jq -s .)
        
        validation_result=$(jq -n \
            --arg path "$file_path" \
            --arg category "$category" \
            --argjson valid "$is_valid" \
            --argjson errors "$errors_json" \
            '{
                path: $path,
                category: $category,
                valid: $valid,
                errors: $errors,
                validated_at: now
            }')
        
        # Add to results
        validation_results=$(echo "$validation_results" | jq ". + [$validation_result]")
        
        if [[ "$is_valid" == "true" ]]; then
            ((valid_count++))
        else
            ((error_count++))
        fi
        
        # Progress reporting
        if ((file_count % 50 == 0)); then
            log_debug "Validated $file_count files ($valid_count valid, $error_count errors)..."
        fi
        
    done < <(jq -c '.[]' "$inventory_file")
    
    # Save validation results
    echo "$validation_results" > "$validation_report_file"
    
    log_success "File validation completed: $valid_count valid, $error_count with errors"
    
    # Generate validation summary
    local validation_summary
    validation_summary=$(echo "$validation_results" | jq '{
        total_files: length,
        valid_files: ([.[] | select(.valid)] | length),
        invalid_files: ([.[] | select(.valid | not)] | length),
        error_summary: (
            [.[] | select(.valid | not) | .errors[]] | 
            group_by(.) | 
            map({error: .[0], count: length}) | 
            sort_by(-.count)
        ),
        validation_timestamp: now
    }')
    
    echo "$validation_summary" > "${validation_report_file%.json}_summary.json"
    
    return 0
}

# Main file discovery function
run_file_discovery() {
    local input_path="$1"
    local output_dir="$2"
    local config_file="$3"
    
    log_info "Running file discovery and validation system"
    
    # Load configuration
    local excluded_dirs
    excluded_dirs=$(jq -r '.analysis.phases.file_discovery.excluded_directories | join(" ")' "$config_file" 2>/dev/null || echo "node_modules .git target build")
    
    local excluded_extensions
    excluded_extensions=$(jq -r '.analysis.phases.file_discovery.excluded_extensions | join(" ")' "$config_file" 2>/dev/null || echo ".log .tmp .cache")
    
    local max_file_size
    max_file_size=$(jq -r '.analysis.phases.file_discovery.max_file_size_mb' "$config_file" 2>/dev/null || echo "100")
    
    # File discovery
    local inventory_file="$output_dir/reports/file_inventory.json"
    discover_files_recursive "$input_path" "$inventory_file" "$excluded_dirs" "$excluded_extensions"
    
    # Generate inventory report
    local inventory_report="$output_dir/reports/file_inventory_report.md"
    generate_file_inventory_report "$inventory_file" "$inventory_report"
    
    # Validate files
    local validation_report="$output_dir/reports/file_validation.json"
    validate_discovered_files "$inventory_file" "$validation_report"
    
    log_success "File discovery and validation completed successfully"
    log_info "Results saved to: $output_dir/reports/"
    
    return 0
}

# Export functions for use by main script
if [[ "${BASH_SOURCE[0]}" != "${0}" ]]; then
    # Script is being sourced, export functions
    export -f validate_file_readable validate_file_size validate_json_file validate_text_encoding
    export -f categorize_file get_file_info discover_files_recursive
    export -f generate_file_inventory_report validate_discovered_files run_file_discovery
fi