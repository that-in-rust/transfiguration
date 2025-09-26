#!/bin/bash

# Extension Manifest Validator
# Validates extension package.json against VS Code schema

validate_extension_manifest() {
    local manifest_file="$1"
    local schema_file="$2"
    local output_file="$3"
    
    if [[ ! -f "$manifest_file" ]]; then
        echo "Error: Manifest file not found: $manifest_file"
        return 1
    fi
    
    local validation_errors=()
    local warnings=()
    
    # Basic JSON validation
    if ! jq empty "$manifest_file" 2>/dev/null; then
        validation_errors+=("Invalid JSON syntax")
    else
        # Required fields validation
        local required_fields=("name" "version" "engines")
        for field in "${required_fields[@]}"; do
            if ! jq -e ".$field" "$manifest_file" >/dev/null 2>&1; then
                validation_errors+=("Missing required field: $field")
            fi
        done
        
        # Version format validation
        local version
        version=$(jq -r '.version // ""' "$manifest_file")
        if [[ -n "$version" && ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-.*)?$ ]]; then
            validation_errors+=("Invalid version format: $version")
        fi
        
        # Engine requirement validation
        local vscode_engine
        vscode_engine=$(jq -r '.engines.vscode // ""' "$manifest_file")
        if [[ -z "$vscode_engine" ]]; then
            validation_errors+=("Missing vscode engine requirement")
        fi
    fi
    
    # Generate validation report
    local ext_name
    ext_name=$(jq -r '.name // "unknown"' "$manifest_file" 2>/dev/null || echo "unknown")
    
    jq -n \
        --arg extension "$ext_name" \
        --argjson errors "$(printf '%s\n' "${validation_errors[@]}" | jq -R . | jq -s .)" \
        --argjson warnings "$(printf '%s\n' "${warnings[@]}" | jq -R . | jq -s .)" \
        '{
            extension: $extension,
            validation_status: (if ($errors | length) == 0 then "valid" else "invalid" end),
            errors: $errors,
            warnings: $warnings,
            validation_timestamp: now | strftime("%Y-%m-%dT%H:%M:%SZ")
        }' > "$output_file"
    
    echo "Manifest validation completed for $ext_name"
}

# Export function
export -f validate_extension_manifest
