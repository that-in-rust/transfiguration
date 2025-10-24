#!/bin/bash

# Error Handling and Recovery Framework
# Part of Kiro Behavioral Analysis Pipeline
# Compatible with bash 3.2 (macOS default)

# Error codes and categories (compatible with bash 3.2)
get_error_code() {
    local error_name="$1"
    
    case "$error_name" in
        "FILE_NOT_FOUND") echo "101" ;;
        "FILE_NOT_READABLE") echo "102" ;;
        "FILE_TOO_LARGE") echo "103" ;;
        "INVALID_JSON") echo "201" ;;
        "INVALID_ENCODING") echo "202" ;;
        "CORRUPTED_FILE") echo "203" ;;
        "ANALYSIS_FAILED") echo "501" ;;
        "TIMEOUT_EXCEEDED") echo "502" ;;
        "DISK_FULL") echo "601" ;;
        *) echo "999" ;;
    esac
}

# Error context tracking (using simple variables)
ERROR_CONTEXT_OPERATION=""
ERROR_CONTEXT_FILE=""
ERROR_CONTEXT_PHASE=""
ERROR_LOG_FILE=""
ERROR_COUNT=0
WARNING_COUNT=0

# Initialize error handling system
init_error_handling() {
    local output_dir="$1"
    
    ERROR_LOG_FILE="$output_dir/logs/errors_$(date +%Y%m%d_%H%M%S).log"
    mkdir -p "$(dirname "$ERROR_LOG_FILE")"
    
    # Create error log header
    cat > "$ERROR_LOG_FILE" << EOF
# Kiro Analysis Error Log
# Started: $(date -Iseconds)

EOF
    
    log_debug "Error handling system initialized: $ERROR_LOG_FILE"
}

# Context management
set_error_context() {
    local operation="$1"
    local file="$2"
    local phase="$3"
    
    ERROR_CONTEXT_OPERATION="$operation"
    ERROR_CONTEXT_FILE="$file"
    ERROR_CONTEXT_PHASE="$phase"
}

clear_error_context() {
    ERROR_CONTEXT_OPERATION=""
    ERROR_CONTEXT_FILE=""
    ERROR_CONTEXT_PHASE=""
}

# Export functions for use by main script
if [[ "${BASH_SOURCE[0]}" != "${0}" ]]; then
    export -f init_error_handling set_error_context clear_error_context
fi