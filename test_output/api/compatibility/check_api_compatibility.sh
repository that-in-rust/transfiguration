#!/bin/bash

# API Compatibility Checker
# Validates extension API usage against VS Code baseline

check_extension_compatibility() {
    local extension_manifest="$1"
    local baseline_file="$2"
    local output_file="$3"
    
    if [[ ! -f "$extension_manifest" ]]; then
        echo "Error: Extension manifest not found: $extension_manifest"
        return 1
    fi
    
    if [[ ! -f "$baseline_file" ]]; then
        echo "Error: Baseline file not found: $baseline_file"
        return 1
    fi
    
    # Extract extension info
    local ext_name
    ext_name=$(jq -r '.name // "unknown"' "$extension_manifest")
    
    # Check contribution points compatibility
    local contrib_points
    contrib_points=$(jq -r '.contributes | keys[]?' "$extension_manifest" 2>/dev/null || echo "")
    
    local compatibility_issues=()
    local warnings=()
    
    # Validate each contribution point
    while IFS= read -r contrib_point; do
        if [[ -n "$contrib_point" ]]; then
            local is_supported
            is_supported=$(jq -e ".contribution_points.\"$contrib_point\"" "$baseline_file" >/dev/null 2>&1 && echo "true" || echo "false")
            
            if [[ "$is_supported" == "false" ]]; then
                compatibility_issues+=("Unsupported contribution point: $contrib_point")
            fi
        fi
    done <<< "$contrib_points"
    
    # Generate compatibility report
    jq -n \
        --arg extension "$ext_name" \
        --argjson issues "$(printf '%s\n' "${compatibility_issues[@]}" | jq -R . | jq -s .)" \
        --argjson warnings "$(printf '%s\n' "${warnings[@]}" | jq -R . | jq -s .)" \
        '{
            extension: $extension,
            compatibility_status: (if ($issues | length) == 0 then "compatible" else "incompatible" end),
            issues: $issues,
            warnings: $warnings,
            check_timestamp: now | strftime("%Y-%m-%dT%H:%M:%SZ")
        }' > "$output_file"
    
    echo "Compatibility check completed for $ext_name"
}

# Export function
export -f check_extension_compatibility
