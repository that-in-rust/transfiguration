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
