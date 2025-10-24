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
