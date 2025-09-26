#!/bin/bash

# Error Handling and Recovery Framework
# Part of Kiro Behavioral Analysis Pipeline
#
# This module implements comprehensive error handling, recovery mechanisms,
# and graceful degradation for the analysis pipeline.

# Error codes and categories
declare -A ERROR_CODES=(
    # File system errors (1xx)
    ["FILE_NOT_FOUND"]=101
    ["FILE_NOT_READABLE"]=102
    ["FILE_TOO_LARGE"]=103
    ["DIRECTORY_NOT_ACCESSIBLE"]=104
    ["PERMISSION_DENIED"]=105
    
    # Data format errors (2xx)
    ["INVALID_JSON"]=201
    ["INVALID_ENCODING"]=202
    ["CORRUPTED_FILE"]=203
    ["UNSUPPORTED_FORMAT"]=204
    
    # Configuration errors (3xx)
    ["INVALID_CONFIG"]=301
    ["MISSING_CONFIG"]=302
    ["CONFIG_PARSE_ERROR"]=303
    
    # Dependency errors (4xx)
    ["MISSING_DEPENDENCY"]=401
    ["TOOL_NOT_FOUND"]=402
    ["VERSION_MISMATCH"]=403
    
    # Analysis errors (5xx)
    ["ANALYSIS_FAILED"]=501
    ["TIMEOUT_EXCEEDED"]=502
    ["MEMORY_EXCEEDED"]=503
    ["PROCESSING_ERROR"]=504
    
    # System errors (6xx)
    ["DISK_FULL"]=601
    ["NETWORK_ERROR"]=602
    ["SYSTEM_OVERLOAD"]=603
)

# Error context tracking
declare -A ERROR_CONTEXT
ERROR_CONTEXT["current_operation"]=""
ERROR_CONTEXT["current_file"]=""
ERROR_CONTEXT["current_phase"]=""
ERROR_CONTEXT["start_time"]=""

# Recovery strategies
declare -A RECOVERY_STRATEGIES=(
    ["FILE_NOT_FOUND"]="skip_with_warning"
    ["FILE_NOT_READABLE"]="retry_with_permissions"
    ["FILE_TOO_LARGE"]="skip_with_warning"
    ["INVALID_JSON"]="skip_with_fallback"
    ["INVALID_ENCODING"]="convert_encoding"
    ["CORRUPTED_FILE"]="skip_with_warning"
    ["MISSING_DEPENDENCY"]="fail_fast"
    ["ANALYSIS_FAILED"]="retry_with_reduced_scope"
    ["TIMEOUT_EXCEEDED"]="retry_with_extended_timeout"
    ["MEMORY_EXCEEDED"]="retry_with_chunking"
)

# Error tracking
ERROR_LOG_FILE=""
ERROR_COUNT=0
WARNING_COUNT=0
RECOVERY_COUNT=0

# Initialize error handling system
init_error_handling() {
    local output_dir="$1"
    
    ERROR_LOG_FILE="$output_dir/logs/errors_$(date +%Y%m%d_%H%M%S).log"
    mkdir -p "$(dirname "$ERROR_LOG_FILE")"
    
    # Create error log header
    cat > "$ERROR_LOG_FILE" << EOF
# Kiro Analysis Error Log
# Started: $(date -Iseconds)
# PID: $$
# Command: $0 $*

EOF
    
    # Set up error trapping
    set -eE  # Exit on error, inherit ERR trap
    trap 'handle_unexpected_error $? $LINENO $BASH_COMMAND' ERR
    trap 'cleanup_on_exit' EXIT
    
    log_debug "Error handling system initialized: $ERROR_LOG_FILE"
}

# Error logging functions
log_error_to_file() {
    local error_code="$1"
    local error_message="$2"
    local context="$3"
    local timestamp
    timestamp=$(date -Iseconds)
    
    cat >> "$ERROR_LOG_FILE" << EOF
[$timestamp] ERROR $error_code: $error_message
Context: $context
Operation: ${ERROR_CONTEXT["current_operation"]}
File: ${ERROR_CONTEXT["current_file"]}
Phase: ${ERROR_CONTEXT["current_phase"]}
---
EOF
    
    ((ERROR_COUNT++))
}

log_warning_to_file() {
    local warning_message="$1"
    local context="$2"
    local timestamp
    timestamp=$(date -Iseconds)
    
    cat >> "$ERROR_LOG_FILE" << EOF
[$timestamp] WARNING: $warning_message
Context: $context
Operation: ${ERROR_CONTEXT["current_operation"]}
File: ${ERROR_CONTEXT["current_file"]}
Phase: ${ERROR_CONTEXT["current_phase"]}
---
EOF
    
    ((WARNING_COUNT++))
}

log_recovery_to_file() {
    local recovery_action="$1"
    local original_error="$2"
    local success="$3"
    local timestamp
    timestamp=$(date -Iseconds)
    
    cat >> "$ERROR_LOG_FILE" << EOF
[$timestamp] RECOVERY: $recovery_action
Original Error: $original_error
Success: $success
Operation: ${ERROR_CONTEXT["current_operation"]}
File: ${ERROR_CONTEXT["current_file"]}
Phase: ${ERROR_CONTEXT["current_phase"]}
---
EOF
    
    ((RECOVERY_COUNT++))
}

# Context management
set_error_context() {
    local operation="$1"
    local file="$2"
    local phase="$3"
    
    ERROR_CONTEXT["current_operation"]="$operation"
    ERROR_CONTEXT["current_file"]="$file"
    ERROR_CONTEXT["current_phase"]="$phase"
    ERROR_CONTEXT["start_time"]=$(date +%s)
}

clear_error_context() {
    ERROR_CONTEXT["current_operation"]=""
    ERROR_CONTEXT["current_file"]=""
    ERROR_CONTEXT["current_phase"]=""
    ERROR_CONTEXT["start_time"]=""
}

# Safe file processing wrapper
safe_process_file() {
    local file_path="$1"
    local processing_function="$2"
    local max_retries="${3:-3}"
    local timeout_seconds="${4:-300}"
    
    set_error_context "process_file" "$file_path" "${ERROR_CONTEXT["current_phase"]}"
    
    local retry_count=0
    local success=false
    
    while [[ $retry_count -lt $max_retries ]] && [[ "$success" == "false" ]]; do
        if [[ $retry_count -gt 0 ]]; then
            log_warn "Retrying file processing (attempt $((retry_count + 1))/$max_retries): $file_path"
            sleep $((retry_count * 2))  # Exponential backoff
        fi
        
        # Pre-processing validation
        if ! validate_file_for_processing "$file_path"; then
            local error_code=$?
            handle_file_error "$error_code" "$file_path"
            break
        fi
        
        # Process with timeout
        if timeout "$timeout_seconds" "$processing_function" "$file_path"; then
            success=true
            log_debug "Successfully processed: $file_path"
        else
            local exit_code=$?
            ((retry_count++))
            
            if [[ $exit_code -eq 124 ]]; then
                # Timeout
                log_warning_to_file "Processing timeout for file: $file_path" "timeout=${timeout_seconds}s"
                if [[ $retry_count -lt $max_retries ]]; then
                    timeout_seconds=$((timeout_seconds * 2))  # Double timeout for retry
                fi
            else
                # Other error
                log_error_to_file "PROCESSING_ERROR" "Processing failed for file: $file_path" "exit_code=$exit_code"
            fi
        fi
    done
    
    clear_error_context
    
    if [[ "$success" == "true" ]]; then
        return 0
    else
        return 1
    fi
}

# File validation for processing
validate_file_for_processing() {
    local file_path="$1"
    
    # Check if file exists
    if [[ ! -f "$file_path" ]]; then
        return "${ERROR_CODES["FILE_NOT_FOUND"]}"
    fi
    
    # Check if file is readable
    if [[ ! -r "$file_path" ]]; then
        return "${ERROR_CODES["FILE_NOT_READABLE"]}"
    fi
    
    # Check file size (100MB limit)
    local file_size
    file_size=$(stat -f%z "$file_path" 2>/dev/null || stat -c%s "$file_path" 2>/dev/null)
    if [[ $file_size -gt 104857600 ]]; then  # 100MB in bytes
        return "${ERROR_CODES["FILE_TOO_LARGE"]}"
    fi
    
    # Check available disk space (need at least 1GB free)
    local available_space
    available_space=$(df "$(dirname "$file_path")" | awk 'NR==2 {print $4}')
    if [[ $available_space -lt 1048576 ]]; then  # 1GB in KB
        return "${ERROR_CODES["DISK_FULL"]}"
    fi
    
    return 0
}

# Error handling functions
handle_file_error() {
    local error_code="$1"
    local file_path="$2"
    
    case $error_code in
        "${ERROR_CODES["FILE_NOT_FOUND"]}")
            log_warning_to_file "File not found, skipping: $file_path" "error_code=$error_code"
            return 0  # Continue processing
            ;;
        "${ERROR_CODES["FILE_NOT_READABLE"]}")
            if attempt_permission_fix "$file_path"; then
                log_recovery_to_file "permission_fix" "FILE_NOT_READABLE" "true"
                return 0
            else
                log_error_to_file "FILE_NOT_READABLE" "Cannot read file: $file_path" "permission_fix_failed"
                return 1
            fi
            ;;
        "${ERROR_CODES["FILE_TOO_LARGE"]}")
            log_warning_to_file "File too large, skipping: $file_path" "size=$(stat -f%z "$file_path" 2>/dev/null || stat -c%s "$file_path" 2>/dev/null)"
            return 0  # Continue processing
            ;;
        "${ERROR_CODES["DISK_FULL"]}")
            log_error_to_file "DISK_FULL" "Insufficient disk space" "path=$file_path"
            return 1  # Stop processing
            ;;
        *)
            log_error_to_file "UNKNOWN_ERROR" "Unknown error processing file: $file_path" "error_code=$error_code"
            return 1
            ;;
    esac
}

# Recovery mechanisms
attempt_permission_fix() {
    local file_path="$1"
    
    log_debug "Attempting to fix permissions for: $file_path"
    
    # Try to make file readable
    if chmod +r "$file_path" 2>/dev/null; then
        return 0
    fi
    
    # If that fails, try with sudo (if available and appropriate)
    if command -v sudo >/dev/null 2>&1 && [[ -n "${SUDO_USER:-}" ]]; then
        if sudo chmod +r "$file_path" 2>/dev/null; then
            return 0
        fi
    fi
    
    return 1
}

attempt_encoding_conversion() {
    local file_path="$1"
    local output_path="$2"
    
    log_debug "Attempting encoding conversion for: $file_path"
    
    # Try common encoding conversions
    local encodings=("iso-8859-1" "windows-1252" "utf-16" "utf-32")
    
    for encoding in "${encodings[@]}"; do
        if iconv -f "$encoding" -t utf-8 "$file_path" > "$output_path" 2>/dev/null; then
            log_recovery_to_file "encoding_conversion" "INVALID_ENCODING" "true"
            return 0
        fi
    done
    
    log_recovery_to_file "encoding_conversion" "INVALID_ENCODING" "false"
    return 1
}

# Graceful degradation
handle_analysis_failure() {
    local analysis_type="$1"
    local error_message="$2"
    local fallback_action="$3"
    
    log_error_to_file "ANALYSIS_FAILED" "$analysis_type failed: $error_message" "fallback=$fallback_action"
    
    case "$fallback_action" in
        "skip_analysis")
            log_warn "Skipping $analysis_type analysis due to error"
            return 0
            ;;
        "use_partial_results")
            log_warn "Using partial results for $analysis_type analysis"
            return 0
            ;;
        "retry_with_reduced_scope")
            log_info "Will retry $analysis_type with reduced scope"
            return 2  # Signal for retry
            ;;
        *)
            log_error "No fallback available for $analysis_type"
            return 1
            ;;
    esac
}

# Batch processing with error handling
safe_batch_process() {
    local file_list="$1"
    local processing_function="$2"
    local max_parallel="${3:-4}"
    local continue_on_error="${4:-true}"
    
    log_info "Starting batch processing with error handling"
    
    local total_files
    total_files=$(wc -l < "$file_list")
    local processed_count=0
    local success_count=0
    local error_count=0
    
    # Process files in parallel batches
    while IFS= read -r file_path; do
        # Wait for available slot if at max parallel
        while [[ $(jobs -r | wc -l) -ge $max_parallel ]]; do
            sleep 0.1
        done
        
        # Process file in background
        {
            if safe_process_file "$file_path" "$processing_function"; then
                echo "SUCCESS: $file_path"
            else
                echo "ERROR: $file_path"
                if [[ "$continue_on_error" != "true" ]]; then
                    exit 1
                fi
            fi
        } &
        
        ((processed_count++))
        
        # Progress reporting
        if ((processed_count % 10 == 0)); then
            log_debug "Batch progress: $processed_count/$total_files files queued"
        fi
        
    done < "$file_list"
    
    # Wait for all background jobs to complete
    wait
    
    # Count results
    success_count=$(jobs -p | wc -l)  # This is approximate
    error_count=$((processed_count - success_count))
    
    log_success "Batch processing completed: $success_count successful, $error_count errors"
    
    return 0
}

# Unexpected error handler
handle_unexpected_error() {
    local exit_code="$1"
    local line_number="$2"
    local command="$3"
    
    log_error_to_file "UNEXPECTED_ERROR" "Unexpected error in script" "exit_code=$exit_code line=$line_number command='$command'"
    
    # Generate stack trace if possible
    local stack_trace=""
    local frame=0
    while caller $frame; do
        ((frame++))
    done 2>/dev/null | while read line func file; do
        stack_trace+="  at $func ($file:$line)\n"
    done
    
    if [[ -n "$stack_trace" ]]; then
        echo -e "Stack trace:\n$stack_trace" >> "$ERROR_LOG_FILE"
    fi
    
    log_error "Unexpected error occurred. Check error log: $ERROR_LOG_FILE"
}

# Cleanup function
cleanup_on_exit() {
    local exit_code=$?
    
    if [[ -n "$ERROR_LOG_FILE" ]]; then
        cat >> "$ERROR_LOG_FILE" << EOF

# Analysis completed: $(date -Iseconds)
# Exit code: $exit_code
# Error count: $ERROR_COUNT
# Warning count: $WARNING_COUNT
# Recovery count: $RECOVERY_COUNT

EOF
    fi
    
    # Kill any remaining background jobs
    jobs -p | xargs -r kill 2>/dev/null || true
    
    if [[ $exit_code -ne 0 ]]; then
        log_error "Analysis pipeline exited with errors (code: $exit_code)"
        log_error "Error summary: $ERROR_COUNT errors, $WARNING_COUNT warnings, $RECOVERY_COUNT recoveries"
    else
        log_success "Analysis pipeline completed successfully"
        log_info "Summary: $ERROR_COUNT errors, $WARNING_COUNT warnings, $RECOVERY_COUNT recoveries"
    fi
}

# Error reporting
generate_error_report() {
    local output_file="$1"
    
    if [[ ! -f "$ERROR_LOG_FILE" ]]; then
        log_warn "No error log file found"
        return 1
    fi
    
    log_info "Generating error report: $output_file"
    
    # Parse error log and generate structured report
    cat > "$output_file" << EOF
# Error Analysis Report

Generated: $(date -Iseconds)

## Summary

- **Total Errors**: $ERROR_COUNT
- **Total Warnings**: $WARNING_COUNT  
- **Recovery Actions**: $RECOVERY_COUNT
- **Error Log**: $ERROR_LOG_FILE

## Error Breakdown

$(grep "ERROR" "$ERROR_LOG_FILE" | cut -d: -f2 | sort | uniq -c | sort -nr | head -10 | while read count error; do
    echo "- **$error**: $count occurrences"
done)

## Warning Breakdown

$(grep "WARNING" "$ERROR_LOG_FILE" | cut -d: -f2 | sort | uniq -c | sort -nr | head -10 | while read count warning; do
    echo "- **$warning**: $count occurrences"
done)

## Recovery Actions

$(grep "RECOVERY" "$ERROR_LOG_FILE" | cut -d: -f2 | sort | uniq -c | sort -nr | while read count recovery; do
    echo "- **$recovery**: $count attempts"
done)

## Recommendations

EOF

    # Add recommendations based on error patterns
    if [[ $ERROR_COUNT -gt 0 ]]; then
        echo "### Error Mitigation" >> "$output_file"
        echo "" >> "$output_file"
        
        if grep -q "FILE_NOT_FOUND" "$ERROR_LOG_FILE"; then
            echo "- Review file paths and ensure all expected files are present" >> "$output_file"
        fi
        
        if grep -q "PERMISSION_DENIED" "$ERROR_LOG_FILE"; then
            echo "- Check file permissions and consider running with appropriate privileges" >> "$output_file"
        fi
        
        if grep -q "INVALID_JSON" "$ERROR_LOG_FILE"; then
            echo "- Validate JSON files before processing or implement better JSON error handling" >> "$output_file"
        fi
        
        if grep -q "TIMEOUT_EXCEEDED" "$ERROR_LOG_FILE"; then
            echo "- Consider increasing timeout values or optimizing processing algorithms" >> "$output_file"
        fi
    fi
    
    log_success "Error report generated: $output_file"
}

# Export functions for use by main script
if [[ "${BASH_SOURCE[0]}" != "${0}" ]]; then
    # Script is being sourced, export functions
    export -f init_error_handling set_error_context clear_error_context
    export -f safe_process_file safe_batch_process handle_analysis_failure
    export -f generate_error_report cleanup_on_exit
fi