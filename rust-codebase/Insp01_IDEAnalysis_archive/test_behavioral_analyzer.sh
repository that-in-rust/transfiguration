#!/bin/bash

# Test script for Behavioral Pattern Analyzer
# Tests the behavioral pattern analysis functionality

set -euo pipefail

# Test configuration
readonly TEST_DIR="./test_behavioral_output"
readonly SAMPLE_JS_DIR="./test_js_samples"
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Colors for output
readonly GREEN='\033[0;32m'
readonly RED='\033[0;31m'
readonly YELLOW='\033[1;33m'
readonly NC='\033[0m'

# Test logging
test_log() {
    echo -e "${GREEN}[TEST]${NC} $*"
}

test_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

test_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

# Setup test environment
setup_test_environment() {
    test_log "Setting up test environment..."
    
    # Clean and create test directories
    rm -rf "$TEST_DIR" "$SAMPLE_JS_DIR"
    mkdir -p "$TEST_DIR" "$SAMPLE_JS_DIR"
    
    # Create sample JavaScript files for testing
    create_sample_js_files
    
    test_log "Test environment setup complete"
}

# Create sample JavaScript files with various patterns
create_sample_js_files() {
    test_log "Creating sample JavaScript files..."
    
    # Sample 1: Event handling patterns
    cat > "$SAMPLE_JS_DIR/event_handlers.js" << 'EOF'
// Event handling patterns
document.addEventListener('DOMContentLoaded', function() {
    console.log('DOM loaded');
});

button.addEventListener('click', handleClick);
input.addEventListener('keydown', handleKeyDown);
form.addEventListener('submit', handleSubmit);

// React-style event handlers
function Component() {
    return (
        <div onClick={handleClick} onKeyDown={handleKeyDown}>
            <input onChange={handleChange} onFocus={handleFocus} />
        </div>
    );
}

// Event delegation
document.addEventListener('click', function(event) {
    if (event.target.matches('.button')) {
        event.preventDefault();
        event.stopPropagation();
        handleButtonClick(event);
    }
});

// Custom events
const customEvent = new CustomEvent('myEvent', { detail: data });
element.dispatchEvent(customEvent);
EOF

    # Sample 2: State management patterns
    cat > "$SAMPLE_JS_DIR/state_management.js" << 'EOF'
// React state patterns
import React, { useState, useReducer, useEffect, useContext } from 'react';

function Component() {
    const [count, setCount] = useState(0);
    const [state, dispatch] = useReducer(reducer, initialState);
    const context = useContext(MyContext);
    
    useEffect(() => {
        // Side effect
    }, [count]);
    
    return <div>{count}</div>;
}

// Class component state
class ClassComponent extends React.Component {
    constructor(props) {
        super(props);
        this.state = { count: 0 };
    }
    
    handleClick = () => {
        this.setState({ count: this.state.count + 1 });
    }
}

// Redux patterns
import { createStore, combineReducers, applyMiddleware } from 'redux';

const store = createStore(rootReducer, applyMiddleware(thunk));
store.dispatch(action);
const state = store.getState();
store.subscribe(listener);

// Observable patterns
import { Observable, Subject, BehaviorSubject } from 'rxjs';

const subject = new Subject();
const observable = new Observable(subscriber => {
    subscriber.next(value);
    subscriber.complete();
});

observable.subscribe(value => console.log(value));
subject.next(data);

// Event emitter patterns
const EventEmitter = require('events');
const emitter = new EventEmitter();
emitter.on('event', handler);
emitter.emit('event', data);
emitter.off('event', handler);
EOF

    # Sample 3: Performance patterns
    cat > "$SAMPLE_JS_DIR/performance_patterns.js" << 'EOF'
// Caching patterns
const cache = new Map();
const memoizedFunction = memoize(expensiveFunction);

// Local storage caching
localStorage.setItem('key', JSON.stringify(data));
const cached = JSON.parse(localStorage.getItem('key'));

// Session storage
sessionStorage.setItem('temp', value);

// IndexedDB
const request = indexedDB.open('database', 1);

// Lazy loading patterns
const LazyComponent = React.lazy(() => import('./Component'));
const loadable = Loadable({
    loader: () => import('./Module'),
});

// Dynamic imports
import('./module.js').then(module => {
    module.default();
});

// Performance measurement
performance.mark('start');
console.time('operation');
// ... operation
console.timeEnd('operation');
performance.mark('end');
performance.measure('operation', 'start', 'end');

// Optimization patterns
const debouncedFunction = debounce(handler, 300);
const throttledFunction = throttle(handler, 100);

requestAnimationFrame(animate);

setTimeout(() => {
    // Delayed execution
}, 1000);

const intervalId = setInterval(update, 1000);
clearInterval(intervalId);

// Resource management
function cleanup() {
    element.removeEventListener('click', handler);
    clearTimeout(timeoutId);
    clearInterval(intervalId);
}

// Component cleanup
useEffect(() => {
    return () => {
        cleanup();
    };
}, []);

componentWillUnmount() {
    this.cleanup();
}
EOF

    # Sample 4: Error handling patterns
    cat > "$SAMPLE_JS_DIR/error_handling.js" << 'EOF'
// Try-catch patterns
try {
    riskyOperation();
} catch (error) {
    console.error('Operation failed:', error);
    handleError(error);
} finally {
    cleanup();
}

// Promise error handling
fetchData()
    .then(data => processData(data))
    .catch(error => {
        console.error('Fetch failed:', error);
        return fallbackData;
    });

Promise.reject(new Error('Something went wrong'))
    .catch(handleError);

// Async/await error handling
async function asyncOperation() {
    try {
        const result = await fetchData();
        return await processData(result);
    } catch (error) {
        console.error('Async operation failed:', error);
        throw error;
    }
}

// Error throwing patterns
function validateInput(input) {
    if (!input) {
        throw new Error('Input is required');
    }
    if (typeof input !== 'string') {
        throw Error('Input must be a string');
    }
}

// Error logging patterns
console.error('Critical error:', error);
console.warn('Warning:', warning);
logger.error('Application error', { error, context });
log.error('Service error', error);

// Error recovery patterns
function retryOperation(operation, maxRetries = 3) {
    return operation().catch(error => {
        if (maxRetries > 0) {
            return retryOperation(operation, maxRetries - 1);
        }
        return fallbackOperation();
    });
}

// React error boundaries
class ErrorBoundary extends React.Component {
    componentDidCatch(error, errorInfo) {
        console.error('Error boundary caught:', error, errorInfo);
        this.setState({ hasError: true });
    }
}

// Default fallback
const result = riskyOperation() || defaultValue;
EOF

    test_log "Sample JavaScript files created"
}

# Test the behavioral pattern analyzer
test_behavioral_analyzer() {
    test_log "Testing behavioral pattern analyzer..."
    
    # Source the behavioral analyzer module
    source "$SCRIPT_DIR/lib/behavioral_pattern_analyzer.sh"
    
    # Set LOG_LEVEL for testing
    export LOG_LEVEL="debug"
    
    # Test the analyzer
    if analyze_behavioral_patterns "$SAMPLE_JS_DIR" "$TEST_DIR"; then
        test_log "Behavioral pattern analysis completed successfully"
    else
        test_error "Behavioral pattern analysis failed"
        return 1
    fi
    
    # Verify output files exist
    verify_output_files
    
    # Validate output content
    validate_output_content
    
    test_log "Behavioral pattern analyzer test completed"
}

# Verify that all expected output files were created
verify_output_files() {
    test_log "Verifying output files..."
    
    local expected_files=(
        "behavior/event_patterns.json"
        "behavior/state_patterns.json"
        "behavior/performance_patterns.json"
        "behavior/error_patterns.json"
        "behavior/behavioral_analysis.json"
        "behavior/state_flow_diagram.md"
        "behavior/performance_patterns.md"
        "behavior/error_handling_patterns.md"
        "behavioral_analysis_summary.json"
        "behavioral_analysis_report.md"
    )
    
    for file in "${expected_files[@]}"; do
        if [[ -f "$TEST_DIR/$file" ]]; then
            test_log "✓ Found: $file"
        else
            test_error "✗ Missing: $file"
            return 1
        fi
    done
    
    test_log "All expected output files found"
}

# Validate the content of output files
validate_output_content() {
    test_log "Validating output content..."
    
    # Check event patterns
    local event_count
    event_count=$(jq 'length' "$TEST_DIR/behavior/event_patterns.json")
    test_log "Event patterns found: $event_count"
    
    if [[ "$event_count" -gt 0 ]]; then
        test_log "✓ Event patterns detected"
    else
        test_warn "No event patterns detected"
    fi
    
    # Check state patterns
    local state_count
    state_count=$(jq 'length' "$TEST_DIR/behavior/state_patterns.json")
    test_log "State patterns found: $state_count"
    
    if [[ "$state_count" -gt 0 ]]; then
        test_log "✓ State patterns detected"
    else
        test_warn "No state patterns detected"
    fi
    
    # Check performance patterns
    local perf_count
    perf_count=$(jq 'length' "$TEST_DIR/behavior/performance_patterns.json")
    test_log "Performance patterns found: $perf_count"
    
    if [[ "$perf_count" -gt 0 ]]; then
        test_log "✓ Performance patterns detected"
    else
        test_warn "No performance patterns detected"
    fi
    
    # Check error patterns
    local error_count
    error_count=$(jq 'length' "$TEST_DIR/behavior/error_patterns.json")
    test_log "Error patterns found: $error_count"
    
    if [[ "$error_count" -gt 0 ]]; then
        test_log "✓ Error patterns detected"
    else
        test_warn "No error patterns detected"
    fi
    
    # Validate behavioral analysis summary
    local total_patterns
    total_patterns=$(jq '.total_behavioral_patterns' "$TEST_DIR/behavior/behavioral_analysis.json")
    test_log "Total behavioral patterns: $total_patterns"
    
    # Check that Mermaid diagrams are present
    if grep -q "\`\`\`mermaid" "$TEST_DIR/behavior/state_flow_diagram.md"; then
        test_log "✓ Mermaid diagrams found in state flow documentation"
    else
        test_warn "No Mermaid diagrams found in state flow documentation"
    fi
    
    test_log "Output content validation completed"
}

# Display test results
display_test_results() {
    test_log "Test Results Summary:"
    
    if [[ -f "$TEST_DIR/behavioral_analysis_summary.json" ]]; then
        echo
        echo "Behavioral Analysis Summary:"
        jq -r '
            "- Task: \(.task)",
            "- Processed Files: \(.summary.processed_files)",
            "- Event Patterns: \(.summary.total_event_patterns)",
            "- State Patterns: \(.summary.total_state_patterns)", 
            "- Performance Patterns: \(.summary.total_performance_patterns)",
            "- Error Patterns: \(.summary.total_error_patterns)",
            "- Total Patterns: \(.summary.total_behavioral_patterns)"
        ' "$TEST_DIR/behavioral_analysis_summary.json"
        echo
    fi
    
    echo "Output files created in: $TEST_DIR"
    echo "Sample files created in: $SAMPLE_JS_DIR"
}

# Cleanup test environment
cleanup_test_environment() {
    test_log "Cleaning up test environment..."
    
    # Optionally remove test directories
    # rm -rf "$TEST_DIR" "$SAMPLE_JS_DIR"
    
    test_log "Test cleanup completed (files preserved for inspection)"
}

# Main test execution
main() {
    test_log "Starting Behavioral Pattern Analyzer Test"
    
    # Check dependencies
    if ! command -v jq >/dev/null 2>&1; then
        test_error "jq is required but not installed"
        exit 1
    fi
    
    # Run tests
    setup_test_environment
    test_behavioral_analyzer
    display_test_results
    cleanup_test_environment
    
    test_log "Behavioral Pattern Analyzer Test completed successfully"
}

# Execute main function
main "$@"