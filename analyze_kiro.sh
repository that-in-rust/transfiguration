#!/bin/bash

# Kiro Behavioral Analysis Pipeline
# Phase 1: Discovery & Documentation
# 
# This script systematically extracts and documents all discoverable behaviors
# from the extracted Kiro application files to create a comprehensive 
# behavioral specification for Rust/WASM transfiguration.

set -euo pipefail

# Script metadata
readonly SCRIPT_NAME="$(basename "$0")"
readonly SCRIPT_VERSION="1.0.0"
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Default configuration
DEFAULT_EXTRACTED_KIRO_PATH="/Users/neetipatni/Desktop/extracted_kiro"
DEFAULT_OUTPUT_DIR="./kiro_analysis_output"
DEFAULT_CONFIG_FILE="./kiro_analysis_config.json"
DEFAULT_LOG_LEVEL="info"

# Global variables
EXTRACTED_KIRO_PATH=""
OUTPUT_DIR=""
CONFIG_FILE=""
LOG_LEVEL=""
VERBOSE=false
DRY_RUN=false

# Color codes for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly PURPLE='\033[0;35m'
readonly CYAN='\033[0;36m'
readonly NC='\033[0m' # No Color

# Logging functions
log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

log_warn() {
    [[ "$LOG_LEVEL" =~ ^(debug|info|warn)$ ]] && echo -e "${YELLOW}[WARN]${NC} $*" >&2
}

log_info() {
    [[ "$LOG_LEVEL" =~ ^(debug|info)$ ]] && echo -e "${BLUE}[INFO]${NC} $*" >&2
}

log_debug() {
    [[ "$LOG_LEVEL" == "debug" ]] && echo -e "${PURPLE}[DEBUG]${NC} $*" >&2
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*" >&2
}

# Help documentation
show_help() {
    cat << EOF
${SCRIPT_NAME} v${SCRIPT_VERSION}

DESCRIPTION:
    Kiro Behavioral Analysis Pipeline - Phase 1: Discovery & Documentation
    
    Systematically extracts and documents all discoverable behaviors from 
    extracted Kiro application files to create a comprehensive behavioral 
    specification for future Rust/WASM transfiguration phases.

USAGE:
    ${SCRIPT_NAME} [OPTIONS]

OPTIONS:
    -i, --input PATH        Path to extracted Kiro directory
                           (default: ${DEFAULT_EXTRACTED_KIRO_PATH})
    
    -o, --output PATH       Output directory for analysis results
                           (default: ${DEFAULT_OUTPUT_DIR})
    
    -c, --config FILE       Configuration file path
                           (default: ${DEFAULT_CONFIG_FILE})
    
    -l, --log-level LEVEL   Logging verbosity: debug, info, warn, error
                           (default: ${DEFAULT_LOG_LEVEL})
    
    -v, --verbose           Enable verbose output (equivalent to --log-level debug)
    
    -n, --dry-run          Show what would be done without executing
    
    -h, --help             Show this help message
    
    --version              Show version information

EXAMPLES:
    # Basic analysis with default settings
    ${SCRIPT_NAME}
    
    # Custom input and output paths
    ${SCRIPT_NAME} -i /path/to/extracted_kiro -o /path/to/output
    
    # Verbose analysis with debug logging
    ${SCRIPT_NAME} --verbose
    
    # Dry run to see what would be analyzed
    ${SCRIPT_NAME} --dry-run

ANALYSIS PHASES:
    1. File Discovery & Validation
    2. Configuration Analysis (package.json, product.json)
    3. API Surface Mapping (TypeScript definitions, extensions)
    4. UI Structure Analysis (HTML, CSS, themes)
    5. Behavioral Pattern Inference (event handlers, state management)
    6. Integration & Documentation Generation

OUTPUT STRUCTURE:
    \${OUTPUT_DIR}/
    ├── config/           # Configuration analysis results
    ├── api/              # API surface mapping results  
    ├── ui/               # UI structure analysis results
    ├── behavior/         # Behavioral pattern inference results
    ├── reports/          # Summary reports and documentation
    └── logs/             # Analysis execution logs

REQUIREMENTS ADDRESSED:
    - REQ-6.1: Static analysis methodology implementation
    - REQ-6.2: Configuration structure analysis
    - REQ-6.4: Validation and testing suite for analysis accuracy
    - REQ-6.5: Integration and validation implementation

EOF
}

show_version() {
    echo "${SCRIPT_NAME} version ${SCRIPT_VERSION}"
}

# Argument parsing
parse_arguments() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -i|--input)
                EXTRACTED_KIRO_PATH="$2"
                shift 2
                ;;
            -o|--output)
                OUTPUT_DIR="$2"
                shift 2
                ;;
            -c|--config)
                CONFIG_FILE="$2"
                shift 2
                ;;
            -l|--log-level)
                LOG_LEVEL="$2"
                if [[ ! "$LOG_LEVEL" =~ ^(debug|info|warn|error)$ ]]; then
                    log_error "Invalid log level: $LOG_LEVEL. Must be one of: debug, info, warn, error"
                    exit 1
                fi
                shift 2
                ;;
            -v|--verbose)
                VERBOSE=true
                LOG_LEVEL="debug"
                shift
                ;;
            -n|--dry-run)
                DRY_RUN=true
                shift
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            --version)
                show_version
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                echo "Use --help for usage information."
                exit 1
                ;;
        esac
    done
    
    # Set defaults for unspecified options
    EXTRACTED_KIRO_PATH="${EXTRACTED_KIRO_PATH:-$DEFAULT_EXTRACTED_KIRO_PATH}"
    OUTPUT_DIR="${OUTPUT_DIR:-$DEFAULT_OUTPUT_DIR}"
    CONFIG_FILE="${CONFIG_FILE:-$DEFAULT_CONFIG_FILE}"
    LOG_LEVEL="${LOG_LEVEL:-$DEFAULT_LOG_LEVEL}"
}

# Validation functions
validate_dependencies() {
    local missing_deps=()
    
    # Check for required tools
    command -v jq >/dev/null 2>&1 || missing_deps+=("jq")
    command -v find >/dev/null 2>&1 || missing_deps+=("find")
    command -v grep >/dev/null 2>&1 || missing_deps+=("grep")
    command -v sed >/dev/null 2>&1 || missing_deps+=("sed")
    command -v awk >/dev/null 2>&1 || missing_deps+=("awk")
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        log_error "Missing required dependencies: ${missing_deps[*]}"
        log_error "Please install missing tools and try again."
        exit 1
    fi
    
    log_debug "All required dependencies are available"
}

validate_input_path() {
    if [[ ! -d "$EXTRACTED_KIRO_PATH" ]]; then
        log_error "Extracted Kiro directory does not exist: $EXTRACTED_KIRO_PATH"
        log_error "Please extract Kiro application files and specify the correct path."
        exit 1
    fi
    
    if [[ ! -r "$EXTRACTED_KIRO_PATH" ]]; then
        log_error "Cannot read extracted Kiro directory: $EXTRACTED_KIRO_PATH"
        log_error "Please check directory permissions."
        exit 1
    fi
    
    log_debug "Input path validation passed: $EXTRACTED_KIRO_PATH"
}

# Configuration management
create_default_config() {
    local config_path="$1"
    
    log_info "Creating default configuration file: $config_path"
    
    cat > "$config_path" << 'EOF'
{
  "analysis": {
    "version": "1.0.0",
    "description": "Kiro Behavioral Analysis Configuration",
    "phases": {
      "file_discovery": {
        "enabled": true,
        "max_file_size_mb": 100,
        "excluded_extensions": [".log", ".tmp", ".cache"],
        "excluded_directories": ["node_modules", ".git", "target", "build"]
      },
      "configuration_analysis": {
        "enabled": true,
        "target_files": ["package.json", "product.json", "*.config.js", "*.config.json"],
        "extract_schemas": true,
        "validate_json": true
      },
      "api_surface_mapping": {
        "enabled": true,
        "typescript_files": ["*.d.ts", "*.ts"],
        "manifest_files": ["package.json", "extension.json"],
        "extract_interfaces": true,
        "map_contribution_points": true
      },
      "ui_structure_analysis": {
        "enabled": true,
        "html_files": ["*.html", "*.htm"],
        "css_files": ["*.css", "*.scss", "*.less"],
        "theme_files": ["*theme*.json", "*color*.json"],
        "extract_components": true,
        "analyze_styling": true
      },
      "behavioral_inference": {
        "enabled": true,
        "javascript_files": ["*.js", "*.ts", "*.jsx", "*.tsx"],
        "event_patterns": ["addEventListener", "onClick", "onKeyDown", "onFocus"],
        "state_patterns": ["setState", "state\\.", "useState", "useReducer"],
        "performance_patterns": ["performance", "optimization", "cache", "memoize"]
      }
    },
    "output": {
      "formats": ["json", "markdown", "html"],
      "include_source_references": true,
      "generate_summary_report": true,
      "create_cross_references": true
    },
    "validation": {
      "enabled": true,
      "cross_validate_results": true,
      "generate_confidence_levels": true,
      "compare_vs_code_baseline": true
    }
  },
  "paths": {
    "vs_code_baseline": "",
    "temp_directory": "/tmp/kiro_analysis",
    "cache_directory": "./.kiro_analysis_cache"
  },
  "performance": {
    "parallel_processing": true,
    "max_concurrent_jobs": 4,
    "memory_limit_mb": 2048,
    "timeout_seconds": 3600
  }
}
EOF
    
    log_success "Default configuration created: $config_path"
}

load_configuration() {
    if [[ ! -f "$CONFIG_FILE" ]]; then
        log_warn "Configuration file not found: $CONFIG_FILE"
        create_default_config "$CONFIG_FILE"
    fi
    
    # Validate JSON syntax
    if ! jq empty "$CONFIG_FILE" 2>/dev/null; then
        log_error "Invalid JSON in configuration file: $CONFIG_FILE"
        exit 1
    fi
    
    log_debug "Configuration loaded successfully: $CONFIG_FILE"
}

# Directory structure creation
create_output_structure() {
    local base_dir="$1"
    
    log_info "Creating output directory structure: $base_dir"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        log_info "[DRY RUN] Would create directory structure at: $base_dir"
        return 0
    fi
    
    # Create main directories
    mkdir -p "$base_dir"/{config,api,ui,behavior,reports,logs,temp}
    
    # Create subdirectories for organized output
    mkdir -p "$base_dir/config"/{commands,settings,keybindings,menus,themes}
    mkdir -p "$base_dir/api"/{interfaces,contribution_points,activation_events,compatibility}
    mkdir -p "$base_dir/ui"/{components,layouts,styling,animations,assets}
    mkdir -p "$base_dir/behavior"/{events,state,performance,errors}
    mkdir -p "$base_dir/reports"/{summary,cross_references,validation}
    mkdir -p "$base_dir/logs"/{analysis,errors,debug}
    
    # Create initial status files
    echo "Analysis started at $(date -Iseconds)" > "$base_dir/logs/analysis_start.log"
    echo "{\"status\": \"initialized\", \"timestamp\": \"$(date -Iseconds)\"}" > "$base_dir/status.json"
    
    log_success "Output directory structure created successfully"
}

# Main execution function
main() {
    log_info "Starting Kiro Behavioral Analysis Pipeline v${SCRIPT_VERSION}"
    
    # Parse command line arguments
    parse_arguments "$@"
    
    # Show configuration in debug mode
    if [[ "$LOG_LEVEL" == "debug" ]]; then
        log_debug "Configuration:"
        log_debug "  Input Path: $EXTRACTED_KIRO_PATH"
        log_debug "  Output Dir: $OUTPUT_DIR"
        log_debug "  Config File: $CONFIG_FILE"
        log_debug "  Log Level: $LOG_LEVEL"
        log_debug "  Dry Run: $DRY_RUN"
    fi
    
    # Validate environment and dependencies
    validate_dependencies
    validate_input_path
    
    # Load configuration
    load_configuration
    
    # Create output directory structure
    create_output_structure "$OUTPUT_DIR"
    
    # Initialize logging for this session
    local log_file="$OUTPUT_DIR/logs/analysis_$(date +%Y%m%d_%H%M%S).log"
    exec 1> >(tee -a "$log_file")
    exec 2> >(tee -a "$log_file" >&2)
    
    log_success "Analysis pipeline initialization completed"
    log_info "Ready to begin file discovery and analysis phases"
    log_info "Log file: $log_file"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        log_info "[DRY RUN] Analysis pipeline would proceed with actual file processing"
        exit 0
    fi
    
    # Load analysis modules
    source "$SCRIPT_DIR/lib/file_discovery.sh"
    source "$SCRIPT_DIR/lib/error_handling.sh"
    source "$SCRIPT_DIR/lib/output_management.sh"
    
    # Initialize subsystems
    init_error_handling "$OUTPUT_DIR"
    init_output_management "$OUTPUT_DIR"
    
    # Run file discovery and validation
    log_info "Starting Phase 1: File Discovery and Validation"
    update_progress "file_discovery" "started" "Discovering and validating files in extracted Kiro directory"
    
    if run_file_discovery "$EXTRACTED_KIRO_PATH" "$OUTPUT_DIR" "$CONFIG_FILE"; then
        update_progress "file_discovery" "completed" "File discovery and validation completed successfully"
        log_success "Phase 1 completed: File Discovery and Validation"
    else
        update_progress "file_discovery" "failed" "File discovery and validation failed"
        log_error "Phase 1 failed: File Discovery and Validation"
        exit 1
    fi
    
    # Generate initial reports
    generate_cross_references "$OUTPUT_DIR"
    generate_summary_report "$OUTPUT_DIR"
    collect_analysis_statistics "$OUTPUT_DIR"
    
    # Update final status
    update_analysis_status "phase1_completed" "Core analysis infrastructure and file discovery completed"
    
    log_success "Analysis pipeline Phase 1 completed successfully"
    log_info "Ready for Phase 2: Configuration Analysis (to be implemented in next task)"
}

# Execute main function with all arguments
main "$@"