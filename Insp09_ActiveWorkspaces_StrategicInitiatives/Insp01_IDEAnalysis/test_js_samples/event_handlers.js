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
