#!/bin/bash

# UI Structure Analyzer
# Part of Kiro Behavioral Analysis Pipeline
#
# This module implements comprehensive UI structure analysis including:
# - HTML template and component structure analysis
# - CSS styling systems and theme definitions
# - Theme and customization systems
# - Media assets and visual resources cataloging

# Source required modules
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/error_handling.sh"
source "$SCRIPT_DIR/output_management.sh"

# CSS Analysis Functions

# Parse CSS files to extract styling rules and selectors
analyze_css_styling_systems() {
    local input_path="$1"
    local output_dir="$2"
    
    log_info "Analyzing CSS styling systems and theme definitions"
    set_error_context "css_analysis" "$input_path" "ui_structure"
    
    local css_files_list="$output_dir/temp/css_files.txt"
    local css_analysis_file="$output_dir/ui/styling/css_analysis.json"
    local css_rules_file="$output_dir/ui/styling/css_rules.json"
    local css_variables_file="$output_dir/ui/styling/css_variables.json"
    local css_animations_file="$output_dir/ui/styling/css_animations.json"
    local css_media_queries_file="$output_dir/ui/styling/css_media_queries.json"
    
    mkdir -p "$output_dir/ui/styling" "$output_dir/temp"
    
    # Find all CSS files
    log_debug "Discovering CSS files in: $input_path"
    find "$input_path" -type f \( -name "*.css" -o -name "*.scss" -o -name "*.sass" -o -name "*.less" \) \
        -not -path "*/node_modules/*" \
        -not -path "*/.git/*" \
        -not -path "*/target/*" \
        -not -path "*/build/*" > "$css_files_list"
    
    local css_file_count
    css_file_count=$(wc -l < "$css_files_list")
    log_info "Found $css_file_count CSS files to analyze"
    
    if [[ $css_file_count -eq 0 ]]; then
        log_warning "No CSS files found for analysis"
        return 0
    fi
    
    # Initialize analysis results
    local css_analysis='{"css_files": [], "summary": {"total_files": 0, "total_rules": 0, "total_selectors": 0}}'
    local all_css_rules='[]'
    local all_css_variables='[]'
    local all_css_animations='[]'
    local all_media_queries='[]'
    
    local processed_files=0
    
    # Process each CSS file
    while IFS= read -r css_file; do
        if [[ ! -f "$css_file" ]]; then
            continue
        fi
        
        ((processed_files++))
        log_debug "Processing CSS file ($processed_files/$css_file_count): $(basename "$css_file")"
        
        # Extract CSS rules and selectors
        local file_analysis
        file_analysis=$(analyze_single_css_file "$css_file")
        
        # Add to overall analysis
        css_analysis=$(echo "$css_analysis" | jq --argjson file_data "$file_analysis" '.css_files += [$file_data]')
        
        # Extract CSS variables (custom properties)
        local css_vars
        css_vars=$(extract_css_variables "$css_file")
        if [[ "$css_vars" != "[]" ]]; then
            all_css_variables=$(echo "$all_css_variables" | jq ". + $css_vars")
        fi
        
        # Extract animations and transitions
        local css_anims
        css_anims=$(extract_css_animations "$css_file")
        if [[ "$css_anims" != "[]" ]]; then
            all_css_animations=$(echo "$all_css_animations" | jq ". + $css_anims")
        fi
        
        # Extract media queries
        local media_queries
        media_queries=$(extract_css_media_queries "$css_file")
        if [[ "$media_queries" != "[]" ]]; then
            all_media_queries=$(echo "$all_media_queries" | jq ". + $media_queries")
        fi
        
        # Progress reporting
        if ((processed_files % 10 == 0)); then
            log_debug "Processed $processed_files/$css_file_count CSS files..."
        fi
        
    done < "$css_files_list"
    
    # Update summary statistics
    local total_rules
    total_rules=$(echo "$css_analysis" | jq '[.css_files[].rules_count] | add // 0')
    local total_selectors
    total_selectors=$(echo "$css_analysis" | jq '[.css_files[].selectors_count] | add // 0')
    
    css_analysis=$(echo "$css_analysis" | jq --argjson total_files "$processed_files" \
        --argjson total_rules "$total_rules" \
        --argjson total_selectors "$total_selectors" \
        '.summary = {
            total_files: $total_files,
            total_rules: $total_rules,
            total_selectors: $total_selectors,
            analysis_date: now | strftime("%Y-%m-%dT%H:%M:%SZ")
        }')
    
    # Save analysis results
    safe_write_output "$css_analysis_file" "$css_analysis"
    safe_write_output "$css_variables_file" "$all_css_variables"
    safe_write_output "$css_animations_file" "$all_css_animations"
    safe_write_output "$css_media_queries_file" "$all_media_queries"
    
    # Generate CSS rules summary
    generate_css_rules_summary "$css_analysis" "$css_rules_file"
    
    log_success "CSS styling systems analysis completed: $processed_files files processed"
    clear_error_context
    
    return 0
}

# Analyze a single CSS file
analyze_single_css_file() {
    local css_file="$1"
    
    local file_size
    file_size=$(stat -f%z "$css_file" 2>/dev/null || stat -c%s "$css_file" 2>/dev/null)
    
    # Extract basic CSS information
    local rules_count
    rules_count=$(grep -c '{' "$css_file" 2>/dev/null || echo "0")
    
    local selectors_count
    selectors_count=$(grep -o '[^{}]*{' "$css_file" 2>/dev/null | wc -l | tr -d ' ')
    
    # Extract imports and dependencies
    local imports
    imports=$(grep -E '^@import' "$css_file" 2>/dev/null | sed 's/@import[[:space:]]*["\x27]//g' | sed 's/["\x27].*//g' | jq -R . | jq -s .)
    
    # Create file analysis object
    jq -n \
        --arg path "$css_file" \
        --arg filename "$(basename "$css_file")" \
        --argjson size "$file_size" \
        --argjson rules_count "$rules_count" \
        --argjson selectors_count "$selectors_count" \
        --argjson imports "$imports" \
        '{
            path: $path,
            filename: $filename,
            size_bytes: $size,
            rules_count: $rules_count,
            selectors_count: $selectors_count,
            imports: $imports,
            analyzed_at: now | strftime("%Y-%m-%dT%H:%M:%SZ")
        }'
}

# Extract CSS custom properties (variables)
extract_css_variables() {
    local css_file="$1"
    
    # Extract CSS custom properties (--variable-name: value;)
    local css_vars
    css_vars=$(grep -oE '--[a-zA-Z0-9_-]+[[:space:]]*:[^;]+;' "$css_file" 2>/dev/null | \
        sed 's/[[:space:]]*:[[:space:]]*/:/g' | \
        sed 's/;$//' | \
        while IFS=':' read -r var_name var_value; do
            jq -n \
                --arg name "$var_name" \
                --arg value "$var_value" \
                --arg file "$(basename "$css_file")" \
                --arg file_path "$css_file" \
                '{
                    name: $name,
                    value: $value,
                    source_file: $file,
                    source_path: $file_path
                }'
        done | jq -s .)
    
    echo "${css_vars:-[]}"
}

# Extract CSS animations and transitions
extract_css_animations() {
    local css_file="$1"
    
    local animations='[]'
    
    # Extract @keyframes definitions
    local keyframes
    keyframes=$(grep -oE '@keyframes[[:space:]]+[a-zA-Z0-9_-]+' "$css_file" 2>/dev/null | \
        sed 's/@keyframes[[:space:]]*//g' | \
        while read -r animation_name; do
            jq -n \
                --arg name "$animation_name" \
                --arg type "keyframes" \
                --arg file "$(basename "$css_file")" \
                --arg file_path "$css_file" \
                '{
                    name: $name,
                    type: $type,
                    source_file: $file,
                    source_path: $file_path
                }'
        done | jq -s .)
    
    # Extract animation properties
    local animation_props
    animation_props=$(grep -oE 'animation[^:]*:[^;]+;' "$css_file" 2>/dev/null | \
        while IFS=':' read -r prop_name prop_value; do
            jq -n \
                --arg property "$prop_name" \
                --arg value "$prop_value" \
                --arg type "animation-property" \
                --arg file "$(basename "$css_file")" \
                --arg file_path "$css_file" \
                '{
                    property: $property,
                    value: $value,
                    type: $type,
                    source_file: $file,
                    source_path: $file_path
                }'
        done | jq -s .)
    
    # Extract transition properties
    local transition_props
    transition_props=$(grep -oE 'transition[^:]*:[^;]+;' "$css_file" 2>/dev/null | \
        while IFS=':' read -r prop_name prop_value; do
            jq -n \
                --arg property "$prop_name" \
                --arg value "$prop_value" \
                --arg type "transition-property" \
                --arg file "$(basename "$css_file")" \
                --arg file_path "$css_file" \
                '{
                    property: $property,
                    value: $value,
                    type: $type,
                    source_file: $file,
                    source_path: $file_path
                }'
        done | jq -s .)
    
    # Combine all animation-related data
    animations=$(echo "$keyframes" | jq ". + $animation_props + $transition_props")
    
    echo "${animations:-[]}"
}

# Extract CSS media queries
extract_css_media_queries() {
    local css_file="$1"
    
    # Extract @media rules
    local media_queries
    media_queries=$(grep -oE '@media[^{]+' "$css_file" 2>/dev/null | \
        sed 's/@media[[:space:]]*//g' | \
        while read -r media_query; do
            jq -n \
                --arg query "$media_query" \
                --arg file "$(basename "$css_file")" \
                --arg file_path "$css_file" \
                '{
                    query: $query,
                    source_file: $file,
                    source_path: $file_path
                }'
        done | jq -s .)
    
    echo "${media_queries:-[]}"
}

# Generate CSS rules summary
generate_css_rules_summary() {
    local css_analysis="$1"
    local output_file="$2"
    
    # Create comprehensive CSS rules summary
    local css_rules_summary
    css_rules_summary=$(echo "$css_analysis" | jq '{
        summary: .summary,
        files_by_size: (.css_files | sort_by(-.size_bytes) | .[0:10] | map({filename, size_bytes, rules_count})),
        files_by_rules: (.css_files | sort_by(-.rules_count) | .[0:10] | map({filename, rules_count, selectors_count})),
        total_imports: ([.css_files[].imports[]] | length),
        import_analysis: (
            [.css_files[].imports[]] | 
            group_by(.) | 
            map({import: .[0], count: length}) | 
            sort_by(-.count)
        )
    }')
    
    safe_write_output "$output_file" "$css_rules_summary"
}

# HTML Analysis Functions

# Parse HTML files to extract component hierarchies and DOM structures
analyze_html_templates() {
    local input_path="$1"
    local output_dir="$2"
    
    log_info "Analyzing HTML templates and component structures"
    set_error_context "html_analysis" "$input_path" "ui_structure"
    
    local html_files_list="$output_dir/temp/html_files.txt"
    local html_analysis_file="$output_dir/ui/components/html_analysis.json"
    local html_components_file="$output_dir/ui/components/html_components.json"
    local html_templates_file="$output_dir/ui/components/html_templates.json"
    
    mkdir -p "$output_dir/ui/components" "$output_dir/temp"
    
    # Find all HTML files
    log_debug "Discovering HTML files in: $input_path"
    find "$input_path" -type f \( -name "*.html" -o -name "*.htm" \) \
        -not -path "*/node_modules/*" \
        -not -path "*/.git/*" \
        -not -path "*/target/*" \
        -not -path "*/build/*" > "$html_files_list"
    
    local html_file_count
    html_file_count=$(wc -l < "$html_files_list")
    log_info "Found $html_file_count HTML files to analyze"
    
    if [[ $html_file_count -eq 0 ]]; then
        log_warning "No HTML files found for analysis"
        return 0
    fi
    
    # Initialize analysis results
    local html_analysis='{"html_files": [], "summary": {"total_files": 0, "total_elements": 0, "total_components": 0}}'
    local all_components='[]'
    local all_templates='[]'
    
    local processed_files=0
    
    # Process each HTML file
    while IFS= read -r html_file; do
        if [[ ! -f "$html_file" ]]; then
            continue
        fi
        
        ((processed_files++))
        log_debug "Processing HTML file ($processed_files/$html_file_count): $(basename "$html_file")"
        
        # Extract HTML structure and components
        local file_analysis
        file_analysis=$(analyze_single_html_file "$html_file")
        
        # Add to overall analysis
        html_analysis=$(echo "$html_analysis" | jq --argjson file_data "$file_analysis" '.html_files += [$file_data]')
        
        # Extract template variables and dynamic content
        local template_vars
        template_vars=$(extract_html_template_variables "$html_file")
        if [[ "$template_vars" != "[]" ]]; then
            all_templates=$(echo "$all_templates" | jq ". + $template_vars")
        fi
        
        # Extract component structures
        local components
        components=$(extract_html_components "$html_file")
        if [[ "$components" != "[]" ]]; then
            all_components=$(echo "$all_components" | jq ". + $components")
        fi
        
        # Progress reporting
        if ((processed_files % 5 == 0)); then
            log_debug "Processed $processed_files/$html_file_count HTML files..."
        fi
        
    done < "$html_files_list"
    
    # Update summary statistics
    local total_elements
    total_elements=$(echo "$html_analysis" | jq '[.html_files[].elements_count] | add // 0')
    local total_components
    total_components=$(echo "$all_components" | jq 'length')
    
    html_analysis=$(echo "$html_analysis" | jq --argjson total_files "$processed_files" \
        --argjson total_elements "$total_elements" \
        --argjson total_components "$total_components" \
        '.summary = {
            total_files: $total_files,
            total_elements: $total_elements,
            total_components: $total_components,
            analysis_date: now | strftime("%Y-%m-%dT%H:%M:%SZ")
        }')
    
    # Save analysis results
    safe_write_output "$html_analysis_file" "$html_analysis"
    safe_write_output "$html_components_file" "$all_components"
    safe_write_output "$html_templates_file" "$all_templates"
    
    log_success "HTML templates analysis completed: $processed_files files processed"
    clear_error_context
    
    return 0
}

# Analyze a single HTML file
analyze_single_html_file() {
    local html_file="$1"
    
    local file_size
    file_size=$(stat -f%z "$html_file" 2>/dev/null || stat -c%s "$html_file" 2>/dev/null)
    
    # Count HTML elements
    local elements_count
    elements_count=$(grep -o '<[^/][^>]*>' "$html_file" 2>/dev/null | wc -l | tr -d ' ')
    
    # Extract DOCTYPE and HTML version info
    local doctype
    doctype=$(grep -i '<!DOCTYPE' "$html_file" 2>/dev/null | head -1 | sed 's/^[[:space:]]*//' || echo "")
    
    # Extract meta tags
    local meta_tags
    meta_tags=$(grep -oE '<meta[^>]*>' "$html_file" 2>/dev/null | jq -R . | jq -s .)
    
    # Extract script and link references
    local scripts
    scripts=$(grep -oE '<script[^>]*src[^>]*>' "$html_file" 2>/dev/null | jq -R . | jq -s .)
    
    local stylesheets
    stylesheets=$(grep -oE '<link[^>]*rel[[:space:]]*=[[:space:]]*["\x27]stylesheet["\x27][^>]*>' "$html_file" 2>/dev/null | jq -R . | jq -s .)
    
    # Create file analysis object
    jq -n \
        --arg path "$html_file" \
        --arg filename "$(basename "$html_file")" \
        --argjson size "$file_size" \
        --argjson elements_count "$elements_count" \
        --arg doctype "$doctype" \
        --argjson meta_tags "$meta_tags" \
        --argjson scripts "$scripts" \
        --argjson stylesheets "$stylesheets" \
        '{
            path: $path,
            filename: $filename,
            size_bytes: $size,
            elements_count: $elements_count,
            doctype: $doctype,
            meta_tags: $meta_tags,
            scripts: $scripts,
            stylesheets: $stylesheets,
            analyzed_at: now | strftime("%Y-%m-%dT%H:%M:%SZ")
        }'
}

# Extract HTML template variables and dynamic content patterns
extract_html_template_variables() {
    local html_file="$1"
    
    local template_vars='[]'
    
    # Extract various template variable patterns
    # Angular-style: {{variable}}
    local angular_vars
    angular_vars=$(grep -oE '\{\{[^}]+\}\}' "$html_file" 2>/dev/null | \
        sed 's/[{}]//g' | \
        while read -r var_name; do
            jq -n \
                --arg name "$var_name" \
                --arg type "angular" \
                --arg pattern "{{$var_name}}" \
                --arg file "$(basename "$html_file")" \
                --arg file_path "$html_file" \
                '{
                    name: $name,
                    type: $type,
                    pattern: $pattern,
                    source_file: $file,
                    source_path: $file_path
                }'
        done | jq -s .)
    
    # Vue-style: v-bind, v-model, etc.
    local vue_directives
    vue_directives=$(grep -oE 'v-[a-zA-Z0-9-]+[[:space:]]*=[[:space:]]*["\x27][^"\x27]*["\x27]' "$html_file" 2>/dev/null | \
        while read -r directive; do
            local directive_name
            directive_name=$(echo "$directive" | sed 's/=.*//')
            local directive_value
            directive_value=$(echo "$directive" | sed 's/.*=[[:space:]]*["\x27]//' | sed 's/["\x27]$//')
            
            jq -n \
                --arg name "$directive_name" \
                --arg value "$directive_value" \
                --arg type "vue-directive" \
                --arg pattern "$directive" \
                --arg file "$(basename "$html_file")" \
                --arg file_path "$html_file" \
                '{
                    name: $name,
                    value: $value,
                    type: $type,
                    pattern: $pattern,
                    source_file: $file,
                    source_path: $file_path
                }'
        done | jq -s .)
    
    # Data attributes: data-*
    local data_attributes
    data_attributes=$(grep -oE 'data-[a-zA-Z0-9-]+[[:space:]]*=[[:space:]]*["\x27][^"\x27]*["\x27]' "$html_file" 2>/dev/null | \
        while read -r data_attr; do
            local attr_name
            attr_name=$(echo "$data_attr" | sed 's/=.*//')
            local attr_value
            attr_value=$(echo "$data_attr" | sed 's/.*=[[:space:]]*["\x27]//' | sed 's/["\x27]$//')
            
            jq -n \
                --arg name "$attr_name" \
                --arg value "$attr_value" \
                --arg type "data-attribute" \
                --arg pattern "$data_attr" \
                --arg file "$(basename "$html_file")" \
                --arg file_path "$html_file" \
                '{
                    name: $name,
                    value: $value,
                    type: $type,
                    pattern: $pattern,
                    source_file: $file,
                    source_path: $file_path
                }'
        done | jq -s .)
    
    # Combine all template variables
    template_vars=$(echo "$angular_vars" | jq ". + $vue_directives + $data_attributes")
    
    echo "${template_vars:-[]}"
}

# Extract HTML component structures
extract_html_components() {
    local html_file="$1"
    
    local components='[]'
    
    # Extract custom elements (components with hyphens)
    local custom_elements
    custom_elements=$(grep -oE '<[a-zA-Z][a-zA-Z0-9]*-[a-zA-Z0-9-]*[^>]*>' "$html_file" 2>/dev/null | \
        sed 's/[[:space:]].*//' | sed 's/>//' | sed 's/<//' | \
        sort | uniq | \
        while read -r element_name; do
            jq -n \
                --arg name "$element_name" \
                --arg type "custom-element" \
                --arg file "$(basename "$html_file")" \
                --arg file_path "$html_file" \
                '{
                    name: $name,
                    type: $type,
                    source_file: $file,
                    source_path: $file_path
                }'
        done | jq -s .)
    
    # Extract elements with component-like classes
    local component_classes
    component_classes=$(grep -oE 'class[[:space:]]*=[[:space:]]*["\x27][^"\x27]*["\x27]' "$html_file" 2>/dev/null | \
        sed 's/.*=[[:space:]]*["\x27]//' | sed 's/["\x27]$//' | \
        tr ' ' '\n' | grep -E '^[A-Z][a-zA-Z0-9]*$' | \
        sort | uniq | \
        while read -r class_name; do
            jq -n \
                --arg name "$class_name" \
                --arg type "component-class" \
                --arg file "$(basename "$html_file")" \
                --arg file_path "$html_file" \
                '{
                    name: $name,
                    type: $type,
                    source_file: $file,
                    source_path: $file_path
                }'
        done | jq -s .)
    
    # Combine all components
    components=$(echo "$custom_elements" | jq ". + $component_classes")
    
    echo "${components:-[]}"
}

# Theme and Customization Analysis Functions

# Parse theme definition files and color scheme configurations
analyze_theme_systems() {
    local input_path="$1"
    local output_dir="$2"
    
    log_info "Analyzing theme and customization systems"
    set_error_context "theme_analysis" "$input_path" "ui_structure"
    
    local theme_files_list="$output_dir/temp/theme_files.txt"
    local theme_analysis_file="$output_dir/ui/themes/theme_analysis.json"
    local color_schemes_file="$output_dir/ui/themes/color_schemes.json"
    local theme_configs_file="$output_dir/ui/themes/theme_configs.json"
    local icon_themes_file="$output_dir/ui/themes/icon_themes.json"
    
    mkdir -p "$output_dir/ui/themes" "$output_dir/temp"
    
    # Find theme-related files
    log_debug "Discovering theme files in: $input_path"
    find "$input_path" -type f \( \
        -name "*theme*.json" -o \
        -name "*color*.json" -o \
        -name "*icon*.json" -o \
        -name "product.json" -o \
        -path "*/themes/*" -o \
        -path "*/color-themes/*" -o \
        -path "*/icon-themes/*" \
    \) \
        -not -path "*/node_modules/*" \
        -not -path "*/.git/*" \
        -not -path "*/target/*" \
        -not -path "*/build/*" > "$theme_files_list"
    
    local theme_file_count
    theme_file_count=$(wc -l < "$theme_files_list")
    log_info "Found $theme_file_count theme-related files to analyze"
    
    if [[ $theme_file_count -eq 0 ]]; then
        log_warning "No theme files found for analysis"
        return 0
    fi
    
    # Initialize analysis results
    local theme_analysis='{"theme_files": [], "summary": {"total_files": 0, "color_themes": 0, "icon_themes": 0}}'
    local all_color_schemes='[]'
    local all_theme_configs='[]'
    local all_icon_themes='[]'
    
    local processed_files=0
    
    # Process each theme file
    while IFS= read -r theme_file; do
        if [[ ! -f "$theme_file" ]]; then
            continue
        fi
        
        ((processed_files++))
        log_debug "Processing theme file ($processed_files/$theme_file_count): $(basename "$theme_file")"
        
        # Analyze theme file
        local file_analysis
        file_analysis=$(analyze_single_theme_file "$theme_file")
        
        # Add to overall analysis
        theme_analysis=$(echo "$theme_analysis" | jq --argjson file_data "$file_analysis" '.theme_files += [$file_data]')
        
        # Extract color schemes
        local color_scheme
        color_scheme=$(extract_color_scheme "$theme_file")
        if [[ "$color_scheme" != "null" && "$color_scheme" != "{}" ]]; then
            all_color_schemes=$(echo "$all_color_schemes" | jq ". + [$color_scheme]")
        fi
        
        # Extract theme configurations
        local theme_config
        theme_config=$(extract_theme_configuration "$theme_file")
        if [[ "$theme_config" != "null" && "$theme_config" != "{}" ]]; then
            all_theme_configs=$(echo "$all_theme_configs" | jq ". + [$theme_config]")
        fi
        
        # Extract icon theme definitions
        local icon_theme
        icon_theme=$(extract_icon_theme_definitions "$theme_file")
        if [[ "$icon_theme" != "null" && "$icon_theme" != "{}" ]]; then
            all_icon_themes=$(echo "$all_icon_themes" | jq ". + [$icon_theme]")
        fi
        
    done < "$theme_files_list"
    
    # Update summary statistics
    local color_themes_count
    color_themes_count=$(echo "$all_color_schemes" | jq 'length')
    local icon_themes_count
    icon_themes_count=$(echo "$all_icon_themes" | jq 'length')
    
    theme_analysis=$(echo "$theme_analysis" | jq --argjson total_files "$processed_files" \
        --argjson color_themes "$color_themes_count" \
        --argjson icon_themes "$icon_themes_count" \
        '.summary = {
            total_files: $total_files,
            color_themes: $color_themes,
            icon_themes: $icon_themes,
            analysis_date: now | strftime("%Y-%m-%dT%H:%M:%SZ")
        }')
    
    # Save analysis results
    safe_write_output "$theme_analysis_file" "$theme_analysis"
    safe_write_output "$color_schemes_file" "$all_color_schemes"
    safe_write_output "$theme_configs_file" "$all_theme_configs"
    safe_write_output "$icon_themes_file" "$all_icon_themes"
    
    log_success "Theme systems analysis completed: $processed_files files processed"
    clear_error_context
    
    return 0
}

# Analyze a single theme file
analyze_single_theme_file() {
    local theme_file="$1"
    
    local file_size
    file_size=$(stat -f%z "$theme_file" 2>/dev/null || stat -c%s "$theme_file" 2>/dev/null)
    
    # Determine theme file type
    local theme_type="unknown"
    if [[ "$theme_file" == *"color"* ]]; then
        theme_type="color-theme"
    elif [[ "$theme_file" == *"icon"* ]]; then
        theme_type="icon-theme"
    elif [[ "$theme_file" == *"product.json" ]]; then
        theme_type="product-config"
    elif [[ "$theme_file" == *"theme"* ]]; then
        theme_type="general-theme"
    fi
    
    # Extract basic theme information if it's a valid JSON file
    local theme_content="{}"
    if [[ "$theme_file" == *.json ]] && jq empty "$theme_file" 2>/dev/null; then
        # Extract key theme properties
        local theme_name
        theme_name=$(jq -r '.name // .displayName // .label // empty' "$theme_file" 2>/dev/null || echo "")
        
        local theme_id
        theme_id=$(jq -r '.id // .identifier // empty' "$theme_file" 2>/dev/null || echo "")
        
        local theme_description
        theme_description=$(jq -r '.description // empty' "$theme_file" 2>/dev/null || echo "")
        
        theme_content=$(jq -n \
            --arg name "$theme_name" \
            --arg id "$theme_id" \
            --arg description "$theme_description" \
            '{
                name: (if $name != "" then $name else null end),
                id: (if $id != "" then $id else null end),
                description: (if $description != "" then $description else null end)
            }')
    fi
    
    # Create file analysis object
    jq -n \
        --arg path "$theme_file" \
        --arg filename "$(basename "$theme_file")" \
        --argjson size "$file_size" \
        --arg theme_type "$theme_type" \
        --argjson theme_content "$theme_content" \
        '{
            path: $path,
            filename: $filename,
            size_bytes: $size,
            theme_type: $theme_type,
            theme_content: $theme_content,
            analyzed_at: now | strftime("%Y-%m-%dT%H:%M:%SZ")
        }'
}

# Extract color scheme from theme file
extract_color_scheme() {
    local theme_file="$1"
    
    if [[ "$theme_file" != *.json ]] || ! jq empty "$theme_file" 2>/dev/null; then
        echo "null"
        return 0
    fi
    
    # Extract color definitions
    local colors
    colors=$(jq -r '
        if .colors then
            {
                type: "vscode-color-theme",
                colors: .colors,
                tokenColors: (.tokenColors // []),
                name: (.name // .displayName // "Unknown"),
                source_file: "'$(basename "$theme_file")'",
                source_path: "'$theme_file'"
            }
        elif .workbench then
            {
                type: "workbench-colors",
                colors: .workbench,
                name: (.name // "Unknown"),
                source_file: "'$(basename "$theme_file")'",
                source_path: "'$theme_file'"
            }
        else
            null
        end
    ' "$theme_file" 2>/dev/null)
    
    echo "${colors:-null}"
}

# Extract theme configuration
extract_theme_configuration() {
    local theme_file="$1"
    
    if [[ "$theme_file" != *.json ]] || ! jq empty "$theme_file" 2>/dev/null; then
        echo "null"
        return 0
    fi
    
    # Extract theme configuration settings
    local theme_config
    theme_config=$(jq -r '
        if .contributes.themes then
            {
                type: "theme-contribution",
                themes: .contributes.themes,
                source_file: "'$(basename "$theme_file")'",
                source_path: "'$theme_file'"
            }
        elif .workbench.colorTheme then
            {
                type: "workbench-theme-config",
                colorTheme: .workbench.colorTheme,
                iconTheme: (.workbench.iconTheme // null),
                source_file: "'$(basename "$theme_file")'",
                source_path: "'$theme_file'"
            }
        elif .defaultColorTheme then
            {
                type: "default-theme-config",
                defaultColorTheme: .defaultColorTheme,
                defaultIconTheme: (.defaultIconTheme // null),
                source_file: "'$(basename "$theme_file")'",
                source_path: "'$theme_file'"
            }
        else
            null
        end
    ' "$theme_file" 2>/dev/null)
    
    echo "${theme_config:-null}"
}

# Extract icon theme definitions
extract_icon_theme_definitions() {
    local theme_file="$1"
    
    if [[ "$theme_file" != *.json ]] || ! jq empty "$theme_file" 2>/dev/null; then
        echo "null"
        return 0
    fi
    
    # Extract icon theme definitions
    local icon_theme
    icon_theme=$(jq -r '
        if .iconDefinitions then
            {
                type: "icon-theme",
                iconDefinitions: .iconDefinitions,
                fileExtensions: (.fileExtensions // {}),
                fileNames: (.fileNames // {}),
                folderNames: (.folderNames // {}),
                name: (.displayName // .name // "Unknown"),
                source_file: "'$(basename "$theme_file")'",
                source_path: "'$theme_file'"
            }
        elif .contributes.iconThemes then
            {
                type: "icon-theme-contribution",
                iconThemes: .contributes.iconThemes,
                source_file: "'$(basename "$theme_file")'",
                source_path: "'$theme_file'"
            }
        else
            null
        end
    ' "$theme_file" 2>/dev/null)
    
    echo "${icon_theme:-null}"
}

# Media Assets Analysis Functions

# Catalog media assets and visual resources
analyze_media_assets() {
    local input_path="$1"
    local output_dir="$2"
    
    log_info "Cataloging media assets and visual resources"
    set_error_context "media_analysis" "$input_path" "ui_structure"
    
    local media_files_list="$output_dir/temp/media_files.txt"
    local media_analysis_file="$output_dir/ui/assets/media_analysis.json"
    local images_catalog_file="$output_dir/ui/assets/images_catalog.json"
    local fonts_catalog_file="$output_dir/ui/assets/fonts_catalog.json"
    local icons_catalog_file="$output_dir/ui/assets/icons_catalog.json"
    local svg_analysis_file="$output_dir/ui/assets/svg_analysis.json"
    
    mkdir -p "$output_dir/ui/assets" "$output_dir/temp"
    
    # Find all media asset files
    log_debug "Discovering media assets in: $input_path"
    find "$input_path" -type f \( \
        -name "*.png" -o -name "*.jpg" -o -name "*.jpeg" -o -name "*.gif" -o \
        -name "*.svg" -o -name "*.ico" -o -name "*.webp" -o -name "*.bmp" -o \
        -name "*.ttf" -o -name "*.otf" -o -name "*.woff" -o -name "*.woff2" -o -name "*.eot" -o \
        -name "*.mp4" -o -name "*.webm" -o -name "*.ogg" -o -name "*.mp3" -o -name "*.wav" \
    \) \
        -not -path "*/node_modules/*" \
        -not -path "*/.git/*" \
        -not -path "*/target/*" \
        -not -path "*/build/*" > "$media_files_list"
    
    local media_file_count
    media_file_count=$(wc -l < "$media_files_list")
    log_info "Found $media_file_count media asset files to catalog"
    
    if [[ $media_file_count -eq 0 ]]; then
        log_warning "No media assets found for analysis"
        return 0
    fi
    
    # Initialize analysis results
    local media_analysis='{"media_files": [], "summary": {"total_files": 0, "total_size": 0, "by_type": {}}}'
    local images_catalog='[]'
    local fonts_catalog='[]'
    local icons_catalog='[]'
    local svg_analysis='[]'
    
    local processed_files=0
    local total_size=0
    
    # Process each media file
    while IFS= read -r media_file; do
        if [[ ! -f "$media_file" ]]; then
            continue
        fi
        
        ((processed_files++))
        log_debug "Processing media file ($processed_files/$media_file_count): $(basename "$media_file")"
        
        # Analyze media file
        local file_analysis
        file_analysis=$(analyze_single_media_file "$media_file")
        
        # Add to overall analysis
        media_analysis=$(echo "$media_analysis" | jq --argjson file_data "$file_analysis" '.media_files += [$file_data]')
        
        # Update total size
        local file_size
        file_size=$(echo "$file_analysis" | jq -r '.size_bytes')
        total_size=$((total_size + file_size))
        
        # Categorize by file type
        local file_type
        file_type=$(echo "$file_analysis" | jq -r '.media_type')
        
        case "$file_type" in
            "image")
                images_catalog=$(echo "$images_catalog" | jq ". + [$file_analysis]")
                ;;
            "font")
                fonts_catalog=$(echo "$fonts_catalog" | jq ". + [$file_analysis]")
                ;;
            "icon")
                icons_catalog=$(echo "$icons_catalog" | jq ". + [$file_analysis]")
                ;;
        esac
        
        # Special handling for SVG files
        if [[ "$media_file" == *.svg ]]; then
            local svg_info
            svg_info=$(analyze_svg_file "$media_file")
            if [[ "$svg_info" != "null" ]]; then
                svg_analysis=$(echo "$svg_analysis" | jq ". + [$svg_info]")
            fi
        fi
        
        # Progress reporting
        if ((processed_files % 20 == 0)); then
            log_debug "Processed $processed_files/$media_file_count media files..."
        fi
        
    done < "$media_files_list"
    
    # Update summary statistics
    local by_type_summary
    by_type_summary=$(echo "$media_analysis" | jq '[.media_files] | flatten | group_by(.media_type) | map({type: .[0].media_type, count: length, total_size: ([.[].size_bytes] | add)})')
    
    media_analysis=$(echo "$media_analysis" | jq --argjson total_files "$processed_files" \
        --argjson total_size "$total_size" \
        --argjson by_type "$by_type_summary" \
        '.summary = {
            total_files: $total_files,
            total_size: $total_size,
            total_size_human: (if $total_size > 1073741824 then "\($total_size / 1073741824 | floor)GB" elif $total_size > 1048576 then "\($total_size / 1048576 | floor)MB" elif $total_size > 1024 then "\($total_size / 1024 | floor)KB" else "\($total_size)B" end),
            by_type: $by_type,
            analysis_date: now | strftime("%Y-%m-%dT%H:%M:%SZ")
        }')
    
    # Save analysis results
    safe_write_output "$media_analysis_file" "$media_analysis"
    safe_write_output "$images_catalog_file" "$images_catalog"
    safe_write_output "$fonts_catalog_file" "$fonts_catalog"
    safe_write_output "$icons_catalog_file" "$icons_catalog"
    safe_write_output "$svg_analysis_file" "$svg_analysis"
    
    log_success "Media assets analysis completed: $processed_files files processed"
    clear_error_context
    
    return 0
}

# Analyze a single media file
analyze_single_media_file() {
    local media_file="$1"
    
    local file_size
    file_size=$(stat -f%z "$media_file" 2>/dev/null || stat -c%s "$media_file" 2>/dev/null)
    
    local filename
    filename=$(basename "$media_file")
    local extension="${filename##*.}"
    
    # Determine media type
    local media_type="unknown"
    case "$extension" in
        png|jpg|jpeg|gif|webp|bmp)
            media_type="image"
            ;;
        svg)
            if [[ "$filename" == *"icon"* ]] || [[ "$media_file" == *"/icons/"* ]]; then
                media_type="icon"
            else
                media_type="image"
            fi
            ;;
        ico)
            media_type="icon"
            ;;
        ttf|otf|woff|woff2|eot)
            media_type="font"
            ;;
        mp4|webm|ogg)
            media_type="video"
            ;;
        mp3|wav)
            media_type="audio"
            ;;
    esac
    
    # Extract additional metadata using file command
    local file_info
    file_info=$(file "$media_file" 2>/dev/null || echo "")
    
    # Create file analysis object
    jq -n \
        --arg path "$media_file" \
        --arg filename "$filename" \
        --arg extension "$extension" \
        --argjson size "$file_size" \
        --arg media_type "$media_type" \
        --arg file_info "$file_info" \
        '{
            path: $path,
            filename: $filename,
            extension: $extension,
            size_bytes: $size,
            size_human: (if $size > 1048576 then "\($size / 1048576 | floor)MB" elif $size > 1024 then "\($size / 1024 | floor)KB" else "\($size)B" end),
            media_type: $media_type,
            file_info: $file_info,
            analyzed_at: now | strftime("%Y-%m-%dT%H:%M:%SZ")
        }'
}

# Analyze SVG file for styling patterns
analyze_svg_file() {
    local svg_file="$1"
    
    # Extract SVG-specific information
    local svg_content
    if ! svg_content=$(head -50 "$svg_file" 2>/dev/null); then
        echo "null"
        return 0
    fi
    
    # Extract viewBox and dimensions
    local viewbox
    viewbox=$(echo "$svg_content" | grep -oE 'viewBox[[:space:]]*=[[:space:]]*["\x27][^"\x27]*["\x27]' | sed 's/.*=[[:space:]]*["\x27]//' | sed 's/["\x27]$//' | head -1)
    
    local width
    width=$(echo "$svg_content" | grep -oE 'width[[:space:]]*=[[:space:]]*["\x27][^"\x27]*["\x27]' | sed 's/.*=[[:space:]]*["\x27]//' | sed 's/["\x27]$//' | head -1)
    
    local height
    height=$(echo "$svg_content" | grep -oE 'height[[:space:]]*=[[:space:]]*["\x27][^"\x27]*["\x27]' | sed 's/.*=[[:space:]]*["\x27]//' | sed 's/["\x27]$//' | head -1)
    
    # Count SVG elements
    local path_count
    path_count=$(grep -o '<path' "$svg_file" 2>/dev/null | wc -l | tr -d ' ')
    
    local circle_count
    circle_count=$(grep -o '<circle' "$svg_file" 2>/dev/null | wc -l | tr -d ' ')
    
    local rect_count
    rect_count=$(grep -o '<rect' "$svg_file" 2>/dev/null | wc -l | tr -d ' ')
    
    # Extract colors used
    local colors
    colors=$(grep -oE 'fill[[:space:]]*=[[:space:]]*["\x27][^"\x27]*["\x27]' "$svg_file" 2>/dev/null | \
        sed 's/.*=[[:space:]]*["\x27]//' | sed 's/["\x27]$//' | \
        sort | uniq | jq -R . | jq -s .)
    
    # Create SVG analysis object
    jq -n \
        --arg path "$svg_file" \
        --arg filename "$(basename "$svg_file")" \
        --arg viewbox "$viewbox" \
        --arg width "$width" \
        --arg height "$height" \
        --argjson path_count "$path_count" \
        --argjson circle_count "$circle_count" \
        --argjson rect_count "$rect_count" \
        --argjson colors "$colors" \
        '{
            path: $path,
            filename: $filename,
            viewbox: (if $viewbox != "" then $viewbox else null end),
            width: (if $width != "" then $width else null end),
            height: (if $height != "" then $height else null end),
            elements: {
                paths: $path_count,
                circles: $circle_count,
                rectangles: $rect_count
            },
            colors: $colors,
            analyzed_at: now | strftime("%Y-%m-%dT%H:%M:%SZ")
        }'
}

# Main UI structure analysis function
run_ui_structure_analysis() {
    local input_path="$1"
    local output_dir="$2"
    local config_file="$3"
    
    log_info "Running UI structure analysis"
    update_progress "ui_structure_analysis" "started" "Analyzing UI components, styling, and assets"
    
    # Run CSS analysis (Task 4.1)
    analyze_css_styling_systems "$input_path" "$output_dir"
    
    # Run HTML analysis (Task 4)
    analyze_html_templates "$input_path" "$output_dir"
    
    # Run theme analysis (Task 4.2)
    analyze_theme_systems "$input_path" "$output_dir"
    
    # Run media assets analysis (Task 4.3)
    analyze_media_assets "$input_path" "$output_dir"
    
    update_progress "ui_structure_analysis" "completed" "UI structure analysis completed successfully"
    log_success "UI structure analysis completed successfully"
    
    return 0
}

# Export functions for use by main script
if [[ "${BASH_SOURCE[0]}" != "${0}" ]]; then
    export -f analyze_css_styling_systems analyze_html_templates analyze_theme_systems analyze_media_assets
    export -f analyze_single_css_file extract_css_variables extract_css_animations extract_css_media_queries
    export -f analyze_single_html_file extract_html_template_variables extract_html_components
    export -f analyze_single_theme_file extract_color_scheme extract_theme_configuration extract_icon_theme_definitions
    export -f analyze_single_media_file analyze_svg_file
    export -f run_ui_structure_analysis
fi