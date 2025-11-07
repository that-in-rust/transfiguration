# P45 Simple Rust Pattern Files Finder

## Objective

Find all `.md`, `.json`, and `.txt` files on this Mac Mini that contain idiomatic Rust patterns.

## Simple Command

```bash
# Find files with Rust patterns
find /Users -name "*.md" -o -name "*.json" -o -name "*.txt" | \
  xargs grep -l -E "(impl.*for|async.*await|Result<|Option<|Arc<|Rc<|Box<|&mut|'lifetime|unsafe\s*\{|#\[derive|\.unwrap\(\)|\.expect\(|thread::spawn|tokio::|std::sync::)" \
  2>/dev/null | head -50

# More comprehensive search
find /Users -type f \( -name "*.md" -o -name "*.json" -o -name "*.txt" \) | \
  xargs grep -l -E "(async\s+fn|await\s*[.;]|Result<.*,.*>|Option<.*>|Arc<.*>|Rc<.*>|Box<.*>|&mut\s+[a-zA-Z_]+|'[a-z][a-z0-9_]*|unsafe\s*\{|#\[derive\(.*\)|\.unwrap\(\)|\.expect\(|thread::spawn|tokio::|std::sync::|impl.*for.*{)" \
  2>/dev/null

# Focus on most relevant directories
find /Users/Projects -name "*.md" -o -name "*.json" -o -name "*.txt" | \
  xargs grep -l -E "(async.*await|Result<|Option<|Arc<|Rc<|&mut|'lifetime|unsafe)" \
  2>/dev/null

# Also check for Rust examples in documentation
find /Users/Projects -name "*.md" | \
  xargs grep -l -E "```rust|```rs|language.*rust" \
  2>/dev/null
```

## Key Rust Patterns to Search For

### Core Ownership & Borrowing
- `Arc<`, `Rc<`, `Box<`
- `&mut`, `&[a-zA-Z]`
- `'lifetime` annotations
- `move` keyword

### Error Handling
- `Result<type, Error>`
- `Option<type>`
- `?.unwrap()`
- `.expect("message")`

### Concurrency
- `async fn`
- `.await`
- `thread::spawn`
- `tokio::`
- `std::sync::`

### Traits & Generics
- `impl Trait for Type {`
- `Generic<T: Trait>`
- `where T: Trait`

### Memory Safety
- `unsafe { }`
- `*mut pointer`
- `extern "C"`

### Macros & Attributes
- `#derive(Debug, Clone)`
- `println!("message")`
- `debug!("debug message")`

## Quick Execution Commands

```bash
# Quick scan of your current directory
find . -name "*.md" -o -name "*.json" -o -name "*.txt" | \
  xargs grep -l -E "(async|await|Result<|Option<|Arc<|Rc<|Box<|&mut|'lifetime|unsafe)" 2>/dev/null

# Focus on Projects directory (most likely location)
find /Users/Projects -type f \( -name "*.md" -o -name "*.json" -o -name "*.txt" \) | \
  xargs grep -l -E "(impl.*for|async.*await|Result<.*>|Option<.*>|Arc<.*>|Rc<.*>|Box<.*>)" \
  2>/dev/null | sort

# Count files with Rust patterns
find /Users -type f \( -name "*.md" -o -name "*.json" -o -name "*.txt" \) | \
  xargs grep -l -E "(async|await|Result|Option|Arc|Rc|Box|&mut|'lifetime|unsafe)" \
  2>/dev/null | wc -l
```

## What to Look For

### High-Value Files
1. **Rust tutorials and documentation** - likely have many examples
2. **Code examples and playgrounds** - showcase specific patterns
3. **API documentation** - show idiomatic usage
4. **Blog posts and articles** - explain concepts with code

### Low-Value Files
1. **System logs** - might contain keywords but not meaningful patterns
2. **Configuration files** - unlikely to have Rust code
3. **Random text files** - probably no Rust content

## Simple Script

```bash
#!/bin/bash
# Save as find_rust_patterns.sh

echo "=== Finding files with Rust patterns ==="

# Core Rust patterns search
PATTERNS="(async.*await|Result<.*,.*>|Option<.*>|Arc<.*>|Rc<.*>|Box<.*>|&mut\s+[a-zA-Z_]+|'[a-z][a-z0-9_]*|unsafe\s*\{|#\[derive\(.*\)|impl.*for.*{)"

echo "Searching in /Users/Projects..."
find /Users/Projects -type f \( -name "*.md" -o -name "*.json" -o -name "*.txt" \) | \
  xargs grep -l -E "$PATTERNS" 2>/dev/null | \
  while read file; do
    echo "Found: $file"
    # Show a brief preview of patterns found
    grep -E "$PATTERNS" "$file" | head -3 | sed 's/^/  /'
    echo "---"
  done

echo "Total files with Rust patterns:"
find /Users/Projects -type f \( -name "*.md" -o -name "*.json" -o -name "*.txt" \) | \
  xargs grep -l -E "$PATTERNS" 2>/dev/null | wc -l
```

## Run It Now

```bash
# Make the script executable
chmod +x find_rust_patterns.sh

# Run it
./find_rust_patterns.sh

# Or just run the one-liner directly
find /Users/Projects -type f \( -name "*.md" -o -name "*.json" -o -name "*.txt" \) | \
  xargs grep -l -E "(async.*await|Result<.*,.*>|Option<.*>|Arc<.*>|Rc<.*>|Box<.*>|&mut|'lifetime|unsafe\s*\{|#\[derive\(.*\)|impl.*for.*{)" 2>/dev/null
```

This will give you a list of all `.md`, `.json`, and `.txt` files that contain genuine Rust code examples with idiomatic patterns.