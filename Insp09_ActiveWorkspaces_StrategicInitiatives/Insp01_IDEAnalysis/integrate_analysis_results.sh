#!/bin/bash

# Integration Script for Task 6: Combine analysis results into comprehensive behavioral specification
# This script orchestrates all sub-tasks: 6.1, 6.2, 6.3, and the main aggregation

set -euo pipefail

# Script metadata
readonly SCRIPT_NAME="$(basename "$0")"
readonly SCRIPT_VERSION="1.0.0"
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Default configuration
DEFAULT_OUTPUT_DIR="./kiro_analysis_output"
DEFAULT_CONFIG_FILE="./kiro_analysis_config.json"
DEFAULT_LOG_LEVEL="info"

# Global variables
OUTPUT_DIR=""
CONFIG_FILE=""
LOG_LEVEL=""
VERBOSE=false

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
    Integration script for Task 6: Combine analysis results into comprehensive behavioral specification
    
    This script orchestrates all sub-tasks:
    - 6.1: Implement validation and testing suite for analysis accuracy
    - 6.2: Generate VS Code OSS baseline comparison documentation  
    - 6.3: Create Phase 2 requirements and handoff documentation
    - Main: Result aggregation system to merge all analysis outputs

USAGE:
    ${SCRIPT_NAME} [OPTIONS]

OPTIONS:
    -o, --output PATH       Output directory for analysis results
                           (default: ${DEFAULT_OUTPUT_DIR})
    
    -c, --config FILE       Configuration file path
                           (default: ${DEFAULT_CONFIG_FILE})
    
    -l, --log-level LEVEL   Logging verbosity: debug, info, warn, error
                           (default: ${DEFAULT_LOG_LEVEL})
    
    -v, --verbose           Enable verbose output (equivalent to --log-level debug)
    
    -h, --help             Show this help message
    
    --version              Show version information

EXAMPLES:
    # Basic integration with default settings
    ${SCRIPT_NAME}
    
    # Custom output directory
    ${SCRIPT_NAME} -o /path/to/output
    
    # Verbose integration with debug logging
    ${SCRIPT_NAME} --verbose

TASK 6 SUB-TASKS:
    6.1 Validation Suite: Automated tests for analysis accuracy with confidence levels
    6.2 Baseline Comparison: VS Code OSS differences and migration complexity assessment
    6.3 Phase 2 Handoff: Requirements, constraints, and priority matrix for architecture design
    Main Aggregation: Unified behavioral specification with cross-references and validation

OUTPUT DELIVERABLES:
    - Comprehensive behavioral specification (JSON)
    - Validation and confidence reports
    - VS Code OSS baseline comparison
    - Phase 2 architecture requirements
    - Cross-reference documentation
    - Migration complexity assessment

EOF
}

show_version() {
    echo "${SCRIPT_NAME} version ${SCRIPT_VERSION}"
}

# Argument parsing
parse_arguments() {
    while [[ $# -gt 0 ]]; do
        case $1 in
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
    OUTPUT_DIR="${OUTPUT_DIR:-$DEFAULT_OUTPUT_DIR}"
    CONFIG_FILE="${CONFIG_FILE:-$DEFAULT_CONFIG_FILE}"
    LOG_LEVEL="${LOG_LEVEL:-$DEFAULT_LOG_LEVEL}"
}

# Validation functions
validate_prerequisites() {
    log_info "Validating prerequisites for Task 6 integration"
    
    # Check if output directory exists
    if [[ ! -d "$OUTPUT_DIR" ]]; then
        log_error "Output directory does not exist: $OUTPUT_DIR"
        log_error "Please run the main analysis pipeline first to generate analysis results."
        exit 1
    fi
    
    # Check if config file exists
    if [[ ! -f "$CONFIG_FILE" ]]; then
        log_error "Configuration file not found: $CONFIG_FILE"
        exit 1
    fi
    
    # Check for required analysis results
    local required_dirs=(
        "$OUTPUT_DIR/config"
        "$OUTPUT_DIR/api" 
        "$OUTPUT_DIR/ui"
        "$OUTPUT_DIR/behavior"
        "$OUTPUT_DIR/reports"
    )
    
    local missing_dirs=()
    for dir in "${required_dirs[@]}"; do
        if [[ ! -d "$dir" ]]; then
            missing_dirs+=("$dir")
        fi
    done
    
    if [[ ${#missing_dirs[@]} -gt 0 ]]; then
        log_error "Missing required analysis result directories:"
        for dir in "${missing_dirs[@]}"; do
            log_error "  - $dir"
        done
        log_error "Please run the complete analysis pipeline before integration."
        exit 1
    fi
    
    # Check for required tools
    local missing_tools=()
    command -v jq >/dev/null 2>&1 || missing_tools+=("jq")
    
    if [[ ${#missing_tools[@]} -gt 0 ]]; then
        log_error "Missing required tools: ${missing_tools[*]}"
        exit 1
    fi
    
    log_success "Prerequisites validation passed"
}

# Load analysis modules
load_analysis_modules() {
    log_info "Loading analysis integration modules"
    
    # Load validation framework
    if [[ -f "$SCRIPT_DIR/lib/validation_framework.sh" ]]; then
        source "$SCRIPT_DIR/lib/validation_framework.sh"
        log_debug "Loaded validation_framework.sh"
    else
        log_error "Validation framework module not found: $SCRIPT_DIR/lib/validation_framework.sh"
        exit 1
    fi
    
    # Load baseline comparison
    if [[ -f "$SCRIPT_DIR/lib/baseline_comparison.sh" ]]; then
        source "$SCRIPT_DIR/lib/baseline_comparison.sh"
        log_debug "Loaded baseline_comparison.sh"
    else
        log_error "Baseline comparison module not found: $SCRIPT_DIR/lib/baseline_comparison.sh"
        exit 1
    fi
    
    # Load Phase 2 handoff
    if [[ -f "$SCRIPT_DIR/lib/phase2_handoff.sh" ]]; then
        source "$SCRIPT_DIR/lib/phase2_handoff.sh"
        log_debug "Loaded phase2_handoff.sh"
    else
        log_error "Phase 2 handoff module not found: $SCRIPT_DIR/lib/phase2_handoff.sh"
        exit 1
    fi
    
    # Load result aggregation
    if [[ -f "$SCRIPT_DIR/lib/result_aggregation.sh" ]]; then
        source "$SCRIPT_DIR/lib/result_aggregation.sh"
        log_debug "Loaded result_aggregation.sh"
    else
        log_error "Result aggregation module not found: $SCRIPT_DIR/lib/result_aggregation.sh"
        exit 1
    fi
    
    # Load output management for progress tracking
    if [[ -f "$SCRIPT_DIR/lib/output_management.sh" ]]; then
        source "$SCRIPT_DIR/lib/output_management.sh"
        log_debug "Loaded output_management.sh"
    else
        log_warn "Output management module not found - progress tracking may be limited"
    fi
    
    log_success "All analysis integration modules loaded successfully"
}

# Execute Task 6.1: Validation and Testing Suite
execute_task_6_1() {
    log_info "Executing Task 6.1: Implement validation and testing suite for analysis accuracy"
    
    # Update progress if function is available
    if declare -f update_progress >/dev/null; then
        update_progress "task_6_1" "started" "Running validation and testing suite for analysis accuracy"
    fi
    
    if run_validation_suite "$OUTPUT_DIR" "$CONFIG_FILE"; then
        log_success "Task 6.1 completed: Validation and testing suite executed successfully"
        
        if declare -f update_progress >/dev/null; then
            update_progress "task_6_1" "completed" "Validation suite completed with confidence level reports"
        fi
        
        return 0
    else
        log_error "Task 6.1 failed: Validation and testing suite execution failed"
        
        if declare -f update_progress >/dev/null; then
            update_progress "task_6_1" "failed" "Validation suite execution failed"
        fi
        
        return 1
    fi
}

# Execute Task 6.2: VS Code OSS Baseline Comparison
execute_task_6_2() {
    log_info "Executing Task 6.2: Generate VS Code OSS baseline comparison documentation"
    
    # Update progress if function is available
    if declare -f update_progress >/dev/null; then
        update_progress "task_6_2" "started" "Generating VS Code OSS baseline comparison documentation"
    fi
    
    if run_baseline_comparison "$OUTPUT_DIR" "$CONFIG_FILE"; then
        log_success "Task 6.2 completed: VS Code OSS baseline comparison generated successfully"
        
        if declare -f update_progress >/dev/null; then
            update_progress "task_6_2" "completed" "Baseline comparison and migration complexity assessment completed"
        fi
        
        return 0
    else
        log_error "Task 6.2 failed: VS Code OSS baseline comparison generation failed"
        
        if declare -f update_progress >/dev/null; then
            update_progress "task_6_2" "failed" "Baseline comparison generation failed"
        fi
        
        return 1
    fi
}

# Execute Task 6.3: Phase 2 Requirements and Handoff
execute_task_6_3() {
    log_info "Executing Task 6.3: Create Phase 2 requirements and handoff documentation"
    
    # Update progress if function is available
    if declare -f update_progress >/dev/null; then
        update_progress "task_6_3" "started" "Creating Phase 2 requirements and handoff documentation"
    fi
    
    if run_phase2_handoff "$OUTPUT_DIR" "$CONFIG_FILE"; then
        log_success "Task 6.3 completed: Phase 2 handoff documentation created successfully"
        
        if declare -f update_progress >/dev/null; then
            update_progress "task_6_3" "completed" "Phase 2 requirements, constraints, and priority matrix completed"
        fi
        
        return 0
    else
        log_error "Task 6.3 failed: Phase 2 handoff documentation creation failed"
        
        if declare -f update_progress >/dev/null; then
            update_progress "task_6_3" "failed" "Phase 2 handoff documentation creation failed"
        fi
        
        return 1
    fi
}

# Execute Main Task 6: Result Aggregation
execute_main_task_6() {
    log_info "Executing Main Task 6: Combine analysis results into comprehensive behavioral specification"
    
    # Update progress if function is available
    if declare -f update_progress >/dev/null; then
        update_progress "task_6_main" "started" "Combining analysis results into comprehensive behavioral specification"
    fi
    
    if run_result_aggregation "$OUTPUT_DIR" "$CONFIG_FILE"; then
        log_success "Main Task 6 completed: Comprehensive behavioral specification created successfully"
        
        if declare -f update_progress >/dev/null; then
            update_progress "task_6_main" "completed" "Behavioral specification aggregation and validation completed"
        fi
        
        return 0
    else
        log_error "Main Task 6 failed: Result aggregation failed"
        
        if declare -f update_progress >/dev/null; then
            update_progress "task_6_main" "failed" "Result aggregation failed"
        fi
        
        return 1
    fi
}

# Generate integration summary report
generate_integration_summary() {
    local success_count="$1"
    local total_tasks="$2"
    
    log_info "Generating Task 6 integration summary report"
    
    local summary_file="$OUTPUT_DIR/reports/task_6_integration_summary.md"
    local summary_json="$OUTPUT_DIR/reports/task_6_integration_summary.json"
    
    # Create summary data
    local summary_data=$(jq -n \
        --argjson success "$success_count" \
        --argjson total "$total_tasks" \
        '{
            task_6_integration: {
                version: "1.0.0",
                completion_time: "'$(date -Iseconds)'",
                success_count: $success,
                total_tasks: $total,
                success_rate: (($success * 100) / $total),
                status: (if $success == $total then "complete" else "partial" end)
            }
        }')
    
    echo "$summary_data" > "$summary_json"
    
    # Generate markdown summary
    cat > "$summary_file" << EOF
# Task 6 Integration Summary Report

**Generated**: $(date -Iseconds)  
**Integration Version**: 1.0.0  
**Status**: $(if [[ $success_count -eq $total_tasks ]]; then echo "Complete"; else echo "Partial ($success_count/$total_tasks)"; fi)

## Task Execution Summary

### Task 6.1: Validation and Testing Suite
$(if [[ -d "$OUTPUT_DIR/reports/validation" ]]; then
    echo "✅ **Status**: Complete"
    echo "- Automated validation tests executed"
    echo "- Confidence level reports generated"
    echo "- Regression testing framework created"
else
    echo "❌ **Status**: Failed or Incomplete"
fi)

### Task 6.2: VS Code OSS Baseline Comparison
$(if [[ -d "$OUTPUT_DIR/reports/baseline_comparison" ]]; then
    echo "✅ **Status**: Complete"
    echo "- Baseline comparison documentation generated"
    echo "- Migration complexity assessment completed"
    echo "- Kiro customizations identified"
else
    echo "❌ **Status**: Failed or Incomplete"
fi)

### Task 6.3: Phase 2 Requirements and Handoff
$(if [[ -d "$OUTPUT_DIR/reports/phase2_handoff" ]]; then
    echo "✅ **Status**: Complete"
    echo "- Architecture requirements documented"
    echo "- Technical constraints identified"
    echo "- Priority matrix created"
    echo "- Phase 2 input specification generated"
else
    echo "❌ **Status**: Failed or Incomplete"
fi)

### Main Task 6: Result Aggregation
$(if [[ -d "$OUTPUT_DIR/reports/behavioral_specification" ]]; then
    echo "✅ **Status**: Complete"
    echo "- Comprehensive behavioral specification created"
    echo "- Cross-references generated"
    echo "- Specification completeness validated"
else
    echo "❌ **Status**: Failed or Incomplete"
fi)

## Deliverables Summary

### Primary Deliverables
$(if [[ -f "$OUTPUT_DIR/reports/behavioral_specification/kiro_behavioral_specification.json" ]]; then
    echo "✅ **Behavioral Specification**: Complete JSON specification of all discovered Kiro behaviors"
else
    echo "❌ **Behavioral Specification**: Not available"
fi)

$(if [[ -f "$OUTPUT_DIR/reports/validation/confidence_report.md" ]]; then
    echo "✅ **Validation Reports**: Comprehensive validation and confidence level reports"
else
    echo "❌ **Validation Reports**: Not available"
fi)

$(if [[ -f "$OUTPUT_DIR/reports/baseline_comparison/vs_code_baseline_comparison.md" ]]; then
    echo "✅ **Baseline Comparison**: VS Code OSS differences and migration complexity"
else
    echo "❌ **Baseline Comparison**: Not available"
fi)

$(if [[ -f "$OUTPUT_DIR/reports/phase2_handoff/phase2_handoff_documentation.md" ]]; then
    echo "✅ **Phase 2 Handoff**: Complete requirements and handoff documentation"
else
    echo "❌ **Phase 2 Handoff**: Not available"
fi)

### Supporting Deliverables
$(if [[ -f "$OUTPUT_DIR/reports/behavioral_specification/cross_references.json" ]]; then
    echo "✅ **Cross-References**: Relationships between analysis components"
else
    echo "❌ **Cross-References**: Not available"
fi)

$(if [[ -f "$OUTPUT_DIR/reports/validation/regression/run_regression_tests.sh" ]]; then
    echo "✅ **Regression Framework**: Automated testing for ongoing analysis updates"
else
    echo "❌ **Regression Framework**: Not available"
fi)

## Phase 1 Completion Status

$(if [[ $success_count -eq $total_tasks ]]; then
    echo "🎉 **Phase 1 Complete**: All discovery and documentation tasks completed successfully"
    echo ""
    echo "### Ready for Phase 2"
    echo "- Comprehensive behavioral specification available"
    echo "- Architecture requirements documented"
    echo "- Technical constraints identified"
    echo "- Implementation priority matrix created"
    echo "- Migration complexity assessed"
    echo ""
    echo "### Next Steps"
    echo "1. Begin Phase 2 (Architecture Design) using handoff documentation"
    echo "2. Use behavioral specification as authoritative reference"
    echo "3. Implement behavioral fidelity testing framework"
    echo "4. Start with critical priority components"
else
    echo "⚠️ **Phase 1 Incomplete**: Some tasks failed or are incomplete"
    echo ""
    echo "### Required Actions"
    echo "1. Review failed task logs for specific issues"
    echo "2. Address any missing analysis components"
    echo "3. Re-run integration script after fixes"
    echo "4. Ensure all deliverables are complete before Phase 2"
fi)

## File Locations

### Task 6.1 Outputs
- Validation Reports: \`$OUTPUT_DIR/reports/validation/\`
- Confidence Levels: \`$OUTPUT_DIR/reports/validation/confidence_report.md\`
- Regression Tests: \`$OUTPUT_DIR/reports/validation/regression/\`

### Task 6.2 Outputs  
- Baseline Comparison: \`$OUTPUT_DIR/reports/baseline_comparison/\`
- Migration Assessment: \`$OUTPUT_DIR/reports/baseline_comparison/migration_complexity.json\`

### Task 6.3 Outputs
- Phase 2 Handoff: \`$OUTPUT_DIR/reports/phase2_handoff/\`
- Architecture Requirements: \`$OUTPUT_DIR/reports/phase2_handoff/architecture_requirements.json\`
- Priority Matrix: \`$OUTPUT_DIR/reports/phase2_handoff/priority_matrix.json\`

### Main Task 6 Outputs
- Behavioral Specification: \`$OUTPUT_DIR/reports/behavioral_specification/\`
- Cross-References: \`$OUTPUT_DIR/reports/behavioral_specification/cross_references.json\`
- Final Report: \`$OUTPUT_DIR/reports/behavioral_specification/behavioral_specification_report.md\`

---

*Generated by Task 6 Integration Script v1.0.0*
EOF
    
    log_success "Integration summary report generated: $summary_file"
}

# Main execution function
main() {
    # Parse command line arguments
    parse_arguments "$@"
    
    log_info "Starting Task 6 Integration: Combine analysis results into comprehensive behavioral specification"
    log_info "Integration Script Version: $SCRIPT_VERSION"
    
    # Show configuration in debug mode
    if [[ "$LOG_LEVEL" == "debug" ]]; then
        log_debug "Configuration:"
        log_debug "  Output Dir: $OUTPUT_DIR"
        log_debug "  Config File: $CONFIG_FILE"
        log_debug "  Log Level: $LOG_LEVEL"
    fi
    
    # Validate prerequisites
    validate_prerequisites
    
    # Load analysis modules
    load_analysis_modules
    
    # Initialize progress tracking
    local success_count=0
    local total_tasks=4
    
    # Execute Task 6.1: Validation and Testing Suite
    if execute_task_6_1; then
        ((success_count++))
    fi
    
    # Execute Task 6.2: VS Code OSS Baseline Comparison
    if execute_task_6_2; then
        ((success_count++))
    fi
    
    # Execute Task 6.3: Phase 2 Requirements and Handoff
    if execute_task_6_3; then
        ((success_count++))
    fi
    
    # Execute Main Task 6: Result Aggregation
    if execute_main_task_6; then
        ((success_count++))
    fi
    
    # Generate integration summary
    generate_integration_summary "$success_count" "$total_tasks"
    
    # Final status report
    if [[ $success_count -eq $total_tasks ]]; then
        log_success "Task 6 Integration completed successfully: $success_count/$total_tasks tasks completed"
        log_success "Phase 1 (Discovery & Documentation) is now complete"
        log_info "Phase 2 (Architecture Design) is ready to begin"
        log_info "Review handoff documentation at: $OUTPUT_DIR/reports/phase2_handoff/"
        exit 0
    else
        log_error "Task 6 Integration completed with failures: $success_count/$total_tasks tasks completed"
        log_error "Please review failed tasks and re-run integration after fixes"
        exit 1
    fi
}

# Execute main function with all arguments
main "$@"