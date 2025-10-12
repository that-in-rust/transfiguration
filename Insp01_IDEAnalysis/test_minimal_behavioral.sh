#!/bin/bash

set -euo pipefail

# Test minimal behavioral analyzer
analyze_behavioral_patterns() {
    local extracted_path="$1"
    local output_dir="$2"
    
    echo "Starting behavioral pattern analysis"
    
    # Find JavaScript/TypeScript files
    local js_files=()
    while IFS= read -r -d '' file; do
        js_files+=("$file")
    done < <(find "$extracted_path" -type f \( -name "*.js" -o -name "*.ts" \) -print0 2>/dev/null || true)
    
    echo "Found ${#js_files[@]} JavaScript/TypeScript files"
    
    # Create output directory
    mkdir -p "$output_dir/behavior"
    
    # Initialize output files
    echo "[]" > "$output_dir/behavior/event_patterns.json"
    echo "[]" > "$output_dir/behavior/state_patterns.json"
    echo "[]" > "$output_dir/behavior/performance_patterns.json"
    echo "[]" > "$output_dir/behavior/error_patterns.json"
    
    local total_patterns=0
    
    # Process files
    for js_file in "${js_files[@]}"; do
        echo "Processing: $js_file"
        
        # Simple pattern counting (handle grep exit codes properly)
        local event_count=0
        if grep -q "addEventListener\|onClick" "$js_file" 2>/dev/null; then
            event_count=$(grep -c "addEventListener\|onClick" "$js_file" 2>/dev/null)
        fi
        echo "  Event patterns: $event_count"
        
        local state_count=0
        if grep -q "useState\|setState" "$js_file" 2>/dev/null; then
            state_count=$(grep -c "useState\|setState" "$js_file" 2>/dev/null)
        fi
        echo "  State patterns: $state_count"
        
        local perf_count=0
        if grep -q "cache\|memoize" "$js_file" 2>/dev/null; then
            perf_count=$(grep -c "cache\|memoize" "$js_file" 2>/dev/null)
        fi
        echo "  Performance patterns: $perf_count"
        
        local error_count=0
        if grep -q "try\|catch" "$js_file" 2>/dev/null; then
            error_count=$(grep -c "try\|catch" "$js_file" 2>/dev/null)
        fi
        echo "  Error patterns: $error_count"
        
        total_patterns=$((total_patterns + event_count + state_count + perf_count + error_count))
        echo "  Total so far: $total_patterns"
    done
    
    # Generate summary
    echo "{\"total_patterns\": $total_patterns}" > "$output_dir/behavior/summary.json"
    
    echo "Behavioral pattern analysis completed: $total_patterns patterns found"
    return 0
}

# Test the function
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    analyze_behavioral_patterns "${1:-./test_js_samples}" "${2:-./test_behavioral_output}"
fi