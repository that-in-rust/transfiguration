# Rust Idiomatic Patterns

## Overview
This document catalogs idiomatic Rust patterns found in the official Rust repository, specifically from the core library (`rust/library/core/src`).

Each pattern includes:
- **Pattern Description**: What the pattern does
- **When to Use**: Appropriate contexts and use cases
- **When NOT to Use**: Anti-patterns and inappropriate contexts
- **Example**: Code demonstrating the pattern
- **Source**: File and line reference from the Rust repository

---

## Table of Contents
- [Option Type Patterns](#option-type-patterns)
- [Result Type Patterns](#result-type-patterns)
- [Cell and Interior Mutability Patterns](#cell-and-interior-mutability-patterns)
- [Trait Design Patterns](#trait-design-patterns)
- [Error Handling Patterns](#error-handling-patterns)

---

## Analysis Progress

### Files Analyzed:
- **option.rs** (2,756 lines): ✅ COMPLETED - 41 patterns documented
  - Lines 1-500: ✅ Completed (Patterns 1-7)
  - Lines 501-1000: ✅ Completed (Patterns 8-14)
  - Lines 1001-1500: ✅ Completed (Patterns 15-21)
  - Lines 1501-2000: ✅ Completed (Patterns 22-30)
  - Lines 2001-2500: ✅ Completed (Patterns 31-37)
  - Lines 2501-2756: ✅ Completed (Patterns 38-41)

### Files Planned:
- result.rs (2,202 lines)
- cell.rs (2,698 lines)
- any.rs (908 lines)
- cmp.rs (2,261 lines)
- marker.rs (1,343 lines)
- error.rs (1,084 lines)

---

## Option Type Patterns

### Pattern 1: Using Option for Partial Functions

**Description**: Use `Option<T>` as a return type for functions that may not have a valid result for all inputs.

**When to Use**:
- Functions that perform division (to handle division by zero)
- Lookup operations that may not find a value
- Parsing operations that may fail
- Any operation with a clear "absence of value" semantic
- When you want to force callers to handle the "no value" case

**When NOT to Use**:
- When you need to communicate *why* an operation failed (use `Result<T, E>` instead)
- For validation that has multiple failure modes (use `Result` with specific error types)
- When the absence of a value would be a programming error (use `panic!` or `unwrap()`)

**Example** (from option.rs:22-28):
```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
```

**Source**: `library/core/src/option.rs:22-28`

---

### Pattern 2: Exhaustive Pattern Matching on Option

**Description**: Always use pattern matching to handle both `Some` and `None` cases explicitly.

**When to Use**:
- When you need different behavior for `Some` vs `None`
- In production code where you want to handle all cases
- When the value needs to be processed or transformed
- To make code intent clear and self-documenting

**When NOT to Use**:
- When you're prototyping and want quick unwrapping (use `unwrap()` temporarily)
- When you have a default value to use (use `unwrap_or()` or `unwrap_or_default()`)
- When chaining multiple operations (use combinator methods like `map`, `and_then`)

**Example** (from option.rs:34-39):
```rust
match result {
    Some(x) => println!("Result: {x}"),
    None    => println!("Cannot divide by 0"),
}
```

**Source**: `library/core/src/option.rs:34-39`

---

### Pattern 3: Question Mark Operator for Option Propagation

**Description**: Use the `?` operator to propagate `None` values up the call stack automatically.

**When to Use**:
- In functions that return `Option<T>`
- When you want to short-circuit on `None`
- To reduce boilerplate of explicit pattern matching
- When chaining multiple fallible operations

**When NOT to Use**:
- In functions that don't return `Option` or `Result`
- When you need to handle `None` with custom logic
- When you need to log or track why `None` occurred
- In performance-critical tight loops (may add overhead)

**Example** (from option.rs:101-103):
```rust
fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    Some(stack.pop()? + stack.pop()?)
}
```

**Replaces this verbose pattern**:
```rust
fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    let a = stack.pop();
    let b = stack.pop();
    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,
    }
}
```

**Source**: `library/core/src/option.rs:85-103`

---

### Pattern 4: Null Pointer Optimization (NPO)

**Description**: Leverage Rust's guarantee that `Option<T>` has the same size and alignment as `T` for certain types (references, `Box`, `NonNull`, function pointers, `NonZero*`).

**When to Use**:
- When designing APIs with optional pointers or references
- In FFI boundaries where C expects nullable pointers
- Memory-constrained environments
- When you need `Option<&T>` or `Option<Box<T>>` frequently

**When NOT to Use**:
- Don't rely on this for general types (only guaranteed for specific types)
- Don't use `transmute` unless absolutely necessary and you understand safety implications
- Not applicable for `Option<i32>`, `Option<bool>`, etc.

**Key Insight** (from option.rs:120-155):
```rust
// These types guarantee Option<T> has same size as T:
// - &T, &mut T
// - Box<T>
// - fn pointers
// - NonNull<T>
// - NonZero* types
// - #[repr(transparent)] wrappers around the above

// This means Option<&T> is zero-cost abstraction!
let maybe_ref: Option<&i32> = Some(&42);
// Same size as a single pointer, None represented as null
```

**Source**: `library/core/src/option.rs:118-155`

---

### Pattern 5: Method Chaining with Combinators

**Description**: Chain `Option` methods like `and_then`, `or`, `map`, `filter` to create data processing pipelines.

**When to Use**:
- When transforming or validating data through multiple steps
- To avoid nested pattern matching
- For functional-style data processing
- When early-exit semantics are desired (short-circuit on `None`)

**When NOT to Use**:
- When logic is complex and would be clearer with explicit `match`
- When you need to inspect intermediate values for debugging
- When performance is critical and the chain creates unnecessary closures
- For simple cases where `if let` or `match` is clearer

**Example** (from option.rs:353-369):
```rust
let res = [0u8, 1, 11, 200, 22]
    .into_iter()
    .map(|x| {
        x.checked_sub(1)
            .and_then(|x| x.checked_mul(2))
            .and_then(|x| bt.get(&x))
            .or(Some(&"error!"))
            .copied()
            .unwrap()
    })
    .collect::<Vec<_>>();
```

**Source**: `library/core/src/option.rs:348-370`

---

### Pattern 6: Option as Iterator

**Description**: Treat `Option<T>` as an iterator that produces zero or one element.

**When to Use**:
- When conditionally inserting items into iterator chains
- When returning `impl Iterator` that may or may not yield a value
- To unify iterator types in match expressions
- For conditional iterator extension

**When NOT to Use**:
- When `iter::once()` and `iter::empty()` are clearer
- If the iterator type doesn't need to be unified
- When the conditional logic is complex

**Example** (from option.rs:420-424):
```rust
let yep = Some(42);
let nope = None;
// Option implements IntoIterator
let nums: Vec<i32> = (0..4).chain(yep).chain(4..8).collect();
assert_eq!(nums, [0, 1, 2, 3, 42, 4, 5, 6, 7]);
let nums: Vec<i32> = (0..4).chain(nope).chain(4..8).collect();
assert_eq!(nums, [0, 1, 2, 3, 4, 5, 6, 7]);
```

**Source**: `library/core/src/option.rs:417-425`

---

### Pattern 7: Collecting Iterator<Option<T>> into Option<Collection<T>>

**Description**: Use `collect()` on an iterator of `Option` values to get `Some(collection)` if all values are `Some`, or `None` if any value is `None`.

**When to Use**:
- When all operations must succeed for the result to be valid
- Processing collections where any failure invalidates the whole result
- Validating or parsing multiple items
- Short-circuit behavior is desired

**When NOT to Use**:
- When you want to collect successful values and ignore failures (use `filter_map`)
- When you need to know which specific item failed (use `Result` instead)
- When partial results are acceptable

**Example** (from option.rs:472-478):
```rust
let v = [Some(2), Some(4), None, Some(8)];
let res: Option<Vec<_>> = v.into_iter().collect();
assert_eq!(res, None); // One None makes the whole result None

let v = [Some(2), Some(4), Some(8)];
let res: Option<Vec<_>> = v.into_iter().collect();
assert_eq!(res, Some(vec![2, 4, 8])); // All Some = Some(collection)
```

**Source**: `library/core/src/option.rs:472-478`

---

### Pattern 8: as_ref() and as_mut() for Non-Consuming Operations

**Description**: Use `as_ref()` to convert `&Option<T>` to `Option<&T>` and `as_mut()` for `&mut Option<T>` to `Option<&mut T>` when you need to work with the value without consuming the Option.

**When to Use**:
- When you need to use the value but keep the Option alive
- Before calling `map` or other consuming methods
- When passing Option to functions that would take ownership
- Working with owned types like `String` or `Vec` that you need later

**When NOT to Use**:
- When you're ready to consume the Option
- For Copy types where ownership transfer is cheap
- When you don't need the Option after the operation

**Example** (from option.rs:734-738):
```rust
let text: Option<String> = Some("Hello, world!".to_string());
// as_ref() lets us keep `text` while working with its contents
let text_length: Option<usize> = text.as_ref().map(|s| s.len());
println!("still can print text: {text:?}"); // text still available!
```

Without `as_ref()`, this would fail:
```rust
let text: Option<String> = Some("Hello, world!".to_string());
let text_length: Option<usize> = text.map(|s| s.len()); // text is moved
// println!("{text:?}"); // ERROR: text was consumed
```

**Source**: `library/core/src/option.rs:720-770`

---

### Pattern 9: as_slice() for Unified Slice/Option Iteration

**Description**: Convert `Option<T>` to `&[T]` with length 0 or 1, enabling uniform handling of optional values as slices.

**When to Use**:
- When you need to iterate over zero or one items uniformly
- Interfacing with APIs that expect slices
- When you want to avoid special-casing None
- Creating conditional iterators

**When NOT to Use**:
- When pattern matching is clearer
- For types that don't work well in slices
- When you need to distinguish between Some and None semantically

**Example** (from option.rs:825-837):
```rust
assert_eq!(
    [Some(1234).as_slice(), None.as_slice()],
    [&[1234][..], &[][..]],
);

// Inverse relationship with slice::first()
for i in [Some(1234_u16), None] {
    assert_eq!(i.as_ref(), i.as_slice().first());
}
```

**Source**: `library/core/src/option.rs:815-917`

---

### Pattern 10: expect() with Descriptive Error Messages

**Description**: Use `expect()` instead of `unwrap()` with a message describing *why* you expect the Option to be Some.

**When to Use**:
- When None represents a programmer error or impossible state
- In tests and examples
- When debugging why a panic occurred matters
- During development to make panics informative

**When NOT to Use**:
- In library code where panics should be avoided
- When None is a valid, expected state
- When you can handle None gracefully with combinators

**Best Practice** (from option.rs:942-956):
Frame messages around what "should" be true:
```rust
let item = slice.get(0)
    .expect("slice should not be empty");

let config = env::var("CONFIG")
    .expect("CONFIG environment variable should be set");
```

**Anti-pattern**:
```rust
// Bad: describes what happened, not what should be
let x = option.expect("got None");

// Good: describes the expectation
let x = option.expect("user ID should exist in database");
```

**Source**: `library/core/src/option.rs:923-972`

---

### Pattern 11: Lazy Evaluation with unwrap_or_else()

**Description**: Use `unwrap_or_else()` instead of `unwrap_or()` when the default value is expensive to compute.

**When to Use**:
- When the default value requires function calls
- For expensive computations
- When the default is rarely needed (Option is usually Some)
- When default computation has side effects that should only run when needed

**When NOT to Use**:
- For cheap literal values or Copy types
- When the default is always pre-computed
- If the closure overhead outweighs the default computation

**Example** (from option.rs:1052-1055):
```rust
let k = 10;
assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
assert_eq!(None.unwrap_or_else(|| 2 * k), 20); // Only computes when None
```

**Comparison**:
```rust
// Eager: ALWAYS evaluates expensive_default()
let value = option.unwrap_or(expensive_default());

// Lazy: ONLY evaluates when None
let value = option.unwrap_or_else(|| expensive_default());
```

**Source**: `library/core/src/option.rs:1019-1068`

---

### Pattern 12: is_some_and() for Predicate Checking

**Description**: Use `is_some_and(predicate)` to check if Option is Some AND the value satisfies a condition in one operation.

**When to Use**:
- Combining existence check with value validation
- In conditional statements
- When you need a boolean result
- For readable, declarative code

**When NOT to Use**:
- When you need the value afterward (use `map` or `filter` instead)
- For complex predicates (use `match` for clarity)
- When you need to handle Some and None differently

**Example** (from option.rs:643-654):
```rust
let x: Option<u32> = Some(2);
assert_eq!(x.is_some_and(|x| x > 1), true);

let x: Option<u32> = Some(0);
assert_eq!(x.is_some_and(|x| x > 1), false);

let x: Option<u32> = None;
assert_eq!(x.is_some_and(|x| x > 1), false);

// Use as_ref() to avoid consuming
let x: Option<String> = Some("ownership".to_string());
assert_eq!(x.as_ref().is_some_and(|x| x.len() > 1), true);
println!("still alive {:?}", x); // x not consumed
```

**Source**: `library/core/src/option.rs:638-665`

---

### Pattern 13: Using matches!() Macro for Type Queries

**Description**: Use the `matches!()` macro for concise pattern matching in boolean contexts.

**When to Use**:
- Implementing methods like `is_some()`, `is_none()`
- Checking if a value matches specific patterns
- In const contexts where full match isn't allowed
- For single-expression boolean checks

**When NOT to Use**:
- When you need to extract values from the pattern
- For complex multi-branch logic
- When explicit match is clearer

**Example** (from option.rs:634-636):
```rust
pub const fn is_some(&self) -> bool {
    matches!(*self, Some(_))
}
```

Compare to verbose alternative:
```rust
pub const fn is_some(&self) -> bool {
    match *self {
        Some(_) => true,
        None => false,
    }
}
```

**Source**: `library/core/src/option.rs:634-636`

---

### Pattern 14: Const fn for Compile-Time Evaluation

**Description**: Mark Option methods as `const fn` to enable compile-time evaluation and use in const contexts.

**When to Use**:
- For methods that can be evaluated at compile time
- When building const-friendly APIs
- For core library functions
- When users need const initialization

**When NOT to Use**:
- For methods that require heap allocation
- When implementation needs non-const operations
- For methods with complex runtime behavior

**Example** (from option.rs:633-636, 741-748):
```rust
#[rustc_const_stable(feature = "const_option_basics", since = "1.48.0")]
pub const fn is_some(&self) -> bool {
    matches!(*self, Some(_))
}

#[rustc_const_stable(feature = "const_option_basics", since = "1.48.0")]
pub const fn as_ref(&self) -> Option<&T> {
    match *self {
        Some(ref x) => Some(x),
        None => None,
    }
}

// Usage in const context:
const VALUE: Option<i32> = Some(42);
const IS_PRESENT: bool = VALUE.is_some(); // Evaluated at compile time!
```

**Source**: `library/core/src/option.rs:630-748`

---

### Pattern 15: inspect() for Side Effects in Method Chains

**Description**: Use `inspect()` to perform side effects (like logging or debugging) without consuming or transforming the Option value.

**When to Use**:
- Debugging method chains
- Logging intermediate values
- Performing side effects without breaking the chain
- When you need to observe a value but keep it unchanged

**When NOT to Use**:
- When you need to transform the value (use `map` instead)
- For complex side effects that might fail
- When the side effect is the main purpose (use `if let` or `match`)

**Example** (from option.rs:1176-1186):
```rust
let list = vec![1, 2, 3];

// Prints "got: 2" and continues the chain
let x = list
    .get(1)
    .inspect(|x| println!("got: {x}"))
    .expect("list should be long enough");

// Prints nothing because it's None
list.get(5).inspect(|x| println!("got: {x}"));
```

**Source**: `library/core/src/option.rs:1169-1199`

---

### Pattern 16: map() for Transforming Option Values

**Description**: Use `map()` to transform an `Option<T>` into `Option<U>` by applying a function to the contained value.

**When to Use**:
- Transforming the type or value inside Option
- Chaining transformations
- Converting between types
- When None should remain None

**When NOT to Use**:
- When transformation might fail (use `and_then` instead)
- When you need a default on None (use `map_or` or `map_or_else`)
- For operations with side effects only (use `inspect`)

**Example** (from option.rs:1148-1155):
```rust
let maybe_some_string = Some(String::from("Hello, World!"));
// map takes self by value, consuming maybe_some_string
let maybe_some_len = maybe_some_string.map(|s| s.len());
assert_eq!(maybe_some_len, Some(13));

let x: Option<&str> = None;
assert_eq!(x.map(|s| s.len()), None); // None stays None
```

**Source**: `library/core/src/option.rs:1139-1167`

---

### Pattern 17: and_then() for Chaining Fallible Operations (FlatMap)

**Description**: Use `and_then()` (also known as `flatmap`) to chain operations that return `Option`, avoiding nested Options.

**When to Use**:
- Chaining multiple operations that might fail
- Avoiding `Option<Option<T>>`
- Sequential lookups or computations
- When early-exit on None is desired

**When NOT to Use**:
- When operations don't return Option (use `map` instead)
- For simple transformations
- When you need to handle each None differently

**Example** (from option.rs:1515-1534):
```rust
fn sq_then_to_string(x: u32) -> Option<String> {
    x.checked_mul(x).map(|sq| sq.to_string())
}

assert_eq!(Some(2).and_then(sq_then_to_string), Some("4".to_string()));
assert_eq!(Some(1_000_000).and_then(sq_then_to_string), None); // overflowed!

// Chaining lookups
let arr_2d = [["A0", "A1"], ["B0", "B1"]];
let item_0_1 = arr_2d.get(0).and_then(|row| row.get(1));
assert_eq!(item_0_1, Some(&"A1"));
```

**Source**: `library/core/src/option.rs:1507-1548`

---

### Pattern 18: ok_or() and ok_or_else() for Option to Result Conversion

**Description**: Convert `Option<T>` to `Result<T, E>` by providing an error value for the None case.

**When to Use**:
- When integrating Option-based APIs with Result-based error handling
- Adding context to why None is an error
- Using the `?` operator in functions returning Result
- When you need error types in your API

**When NOT to Use**:
- When None is a valid, non-error state
- In hot paths with expensive error construction (use `ok_or_else` for lazy eval)
- When you can handle None with Option methods

**Example** (from option.rs:1327-1357):
```rust
let x = Some("foo");
assert_eq!(x.ok_or(0), Ok("foo"));

let x: Option<&str> = None;
assert_eq!(x.ok_or(0), Err(0));

// Lazy error construction
assert_eq!(x.ok_or_else(|| {
    // Only executes when None
    expensive_error_creation()
}), Err(...));
```

**Real-world usage**:
```rust
fn read_config(path: &str) -> Result<Config, Error> {
    let file_content = read_file(path)
        .ok_or_else(|| Error::FileNotFound(path.to_string()))?;
    parse_config(&file_content)
}
```

**Source**: `library/core/src/option.rs:1312-1370`

---

### Pattern 19: as_deref() for Smart Pointer Unwrapping

**Description**: Convert `Option<T>` to `Option<&T::Target>` where T implements `Deref`, useful for smart pointers like `String`, `Box`, `Vec`.

**When to Use**:
- Working with `Option<String>` and need `Option<&str>`
- With `Option<Box<T>>` to get `Option<&T>`
- With `Option<Vec<T>>` to get `Option<&[T]>`
- When you want to work with the dereferenced type

**When NOT to Use**:
- With types that don't implement Deref
- When you need the owned value
- For simple reference types

**Example** (from option.rs:1380-1384):
```rust
let x: Option<String> = Some("hey".to_owned());
assert_eq!(x.as_deref(), Some("hey")); // Option<String> -> Option<&str>

let x: Option<String> = None;
assert_eq!(x.as_deref(), None);

// Compare without as_deref:
let x: Option<String> = Some("hey".to_owned());
let y: Option<&str> = x.as_ref().map(|s| s.as_str()); // Verbose
let y: Option<&str> = x.as_deref(); // Concise
```

**Source**: `library/core/src/option.rs:1372-1418`

---

### Pattern 20: filter() for Conditional Validation

**Description**: Keep the Some value if it satisfies a predicate, otherwise return None.

**When to Use**:
- Validating Option values
- Implementing conditional logic on optional values
- In method chains where validation is needed
- When you want Option-like semantics for filtering

**When NOT to Use**:
- When you need both filtered and unfiltered values
- For transformations (use `map` instead)
- When you need to know why filtering failed

**Example** (from option.rs:1564-1571):
```rust
fn is_even(n: &i32) -> bool {
    n % 2 == 0
}

assert_eq!(None.filter(is_even), None);
assert_eq!(Some(3).filter(is_even), None); // Filtered out
assert_eq!(Some(4).filter(is_even), Some(4)); // Passes filter
```

**Real-world usage**:
```rust
let user_age = get_user_age()
    .filter(|age| *age >= 18)
    .ok_or(Error::Unauthorized)?;
```

**Source**: `library/core/src/option.rs:1550-1588`

---

### Pattern 21: Prefer Lazy Evaluation (*_else methods)

**Description**: Use `_else` variants (`map_or_else`, `unwrap_or_else`, `ok_or_else`) when the default/fallback value is expensive to compute.

**When to Use**:
- When default computation is expensive
- When default has side effects
- When default is rarely needed
- For lazy initialization patterns

**When NOT to Use**:
- For cheap literals or Copy types
- When default is pre-computed
- In performance-critical code where closure overhead matters

**Example** (from option.rs:1240-1246):
```rust
let k = 21;

let x = Some("foo");
assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);

let x: Option<&str> = None;
assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);
```

**Comparison**:
```rust
// Eager - ALWAYS creates the error
user.ok_or(Error::new("User not found"))

// Lazy - ONLY creates error when None
user.ok_or_else(|| Error::new("User not found"))
```

**Source**: `library/core/src/option.rs:1234-1279`

---

### Pattern 22: or() and or_else() for Fallback Values

**Description**: Use `or()` to provide a fallback Option if the current Option is None. Use `or_else()` for lazy evaluation.

**When to Use**:
- Providing default/fallback values that are also Optional
- Chaining multiple potential sources
- Implementing fallback logic
- When you want "first available value" semantics

**When NOT to Use**:
- When you need a concrete value (use `unwrap_or` instead)
- For complex fallback logic (use `match`)
- When the fallback is never None (use `unwrap_or`)

**Example** (from option.rs:1601-1615):
```rust
let x = Some(2);
let y = None;
assert_eq!(x.or(y), Some(2)); // x has value, returns x

let x = None;
let y = Some(100);
assert_eq!(x.or(y), Some(100)); // x is None, returns y

// Lazy evaluation
assert_eq!(None.or_else(|| expensive_computation()), Some(...));
```

**Real-world usage**:
```rust
// Try cache, then database, then default
let value = cache.get(key)
    .or_else(|| database.fetch(key))
    .or(Some(default_value));
```

**Source**: `library/core/src/option.rs:1590-1657`

---

### Pattern 23: xor() for Exclusive Or Logic

**Description**: Return Some if exactly one of two Options is Some, otherwise return None.

**When to Use**:
- When you need "one or the other, but not both" semantics
- Validating mutually exclusive optional fields
- Implementing toggle or switch logic
- When both being present is an error condition

**When NOT to Use**:
- When both being Some is valid (use `zip`)
- For simple fallback logic (use `or`)
- When you need to know which one was Some

**Example** (from option.rs:1664-1678):
```rust
let x = Some(2);
let y: Option<u32> = None;
assert_eq!(x.xor(y), Some(2)); // Only x is Some

let x: Option<u32> = None;
let y = Some(2);
assert_eq!(x.xor(y), Some(2)); // Only y is Some

let x = Some(2);
let y = Some(2);
assert_eq!(x.xor(y), None); // Both are Some - returns None!

let x: Option<u32> = None;
let y: Option<u32> = None;
assert_eq!(x.xor(y), None); // Both None
```

**Source**: `library/core/src/option.rs:1659-1692`

---

### Pattern 24: insert() for In-Place Mutation with Reference

**Description**: Insert a value into an Option and immediately get a mutable reference to it, replacing any existing value.

**When to Use**:
- When you need to initialize and immediately modify
- For in-place updates
- When you want a reference to the inserted value
- Building complex structures incrementally

**When NOT to Use**:
- When you want to preserve the old value if Some (use `get_or_insert`)
- For simple assignment without needing a reference
- When you don't need the mutable reference

**Example** (from option.rs:1708-1716):
```rust
let mut opt = None;
let val = opt.insert(1);
assert_eq!(*val, 1);
assert_eq!(opt.unwrap(), 1);

let val = opt.insert(2); // Replaces old value!
assert_eq!(*val, 2);
*val = 3; // Can mutate in place
assert_eq!(opt.unwrap(), 3);
```

**Source**: `library/core/src/option.rs:1698-1729`

---

### Pattern 25: get_or_insert() Family for Lazy Initialization

**Description**: Insert a value only if None, then return a mutable reference. Supports default values, Default trait, and custom functions.

**When to Use**:
- Lazy initialization patterns
- Cache-like behavior
- When you want to ensure a value exists
- Building values on first access

**When NOT to Use**:
- When you want to replace existing values (use `insert`)
- When you don't need a mutable reference
- For immutable access (use `unwrap_or`)

**Example** (from option.rs:1740-1799):
```rust
let mut x = None;
{
    let y: &mut u32 = x.get_or_insert(5);
    assert_eq!(y, &5);
    *y = 7;
}
assert_eq!(x, Some(7));

// With Default trait
let mut x: Option<Vec<String>> = None;
x.get_or_insert_default().push("item".to_string());

// With custom function (lazy)
let mut x = None;
x.get_or_insert_with(|| expensive_computation());
```

**Source**: `library/core/src/option.rs:1731-1816`

---

### Pattern 26: take() for Ownership Transfer

**Description**: Take the value out of an Option, leaving None in its place.

**When to Use**:
- When you need to move the value out while keeping the Option
- Implementing "consume and replace" patterns
- In state machines where you need to take ownership
- When you want to temporarily extract a value

**When NOT to Use**:
- When you want to keep the Option intact (use `as_ref()` or `clone()`)
- For simple unwrapping (use `unwrap()` or `expect()`)
- When you don't need to preserve the Option variable

**Example** (from option.rs:1827-1835):
```rust
let mut x = Some(2);
let y = x.take();
assert_eq!(x, None); // x is now None!
assert_eq!(y, Some(2)); // value moved to y

let mut x: Option<u32> = None;
let y = x.take();
assert_eq!(x, None);
assert_eq!(y, None);
```

**Real-world usage**:
```rust
struct Parser {
    buffer: Option<String>,
}

impl Parser {
    fn consume_buffer(&mut self) -> Option<String> {
        self.buffer.take() // Leaves None, returns the buffer
    }
}
```

**Source**: `library/core/src/option.rs:1822-1843`

---

### Pattern 27: take_if() for Conditional Extraction

**Description**: Take the value out only if it satisfies a predicate, leaving None if taken or keeping the value if not.

**When to Use**:
- Conditional state transitions
- Implementing business logic with ownership transfer
- When predicate can mutate the value before deciding
- Selective consumption patterns

**When NOT to Use**:
- For simple filtering (use `filter()`)
- When you don't need to take ownership
- When the predicate is expensive (it runs even if None)

**Example** (from option.rs:1854-1867):
```rust
let mut x = Some(42);

// Predicate can mutate!
let prev = x.take_if(|v| if *v == 42 {
    *v += 1;
    false // Don't take
} else {
    false
});
assert_eq!(x, Some(43)); // Value was mutated
assert_eq!(prev, None); // But not taken

let prev = x.take_if(|v| *v == 43);
assert_eq!(x, None); // Taken!
assert_eq!(prev, Some(43));
```

**Source**: `library/core/src/option.rs:1845-1877`

---

### Pattern 28: replace() for Swap and Return

**Description**: Replace the value in an Option and return the old value, leaving Some in place.

**When to Use**:
- When you need both old and new values
- Implementing swap operations
- Update-and-return patterns
- When you want to ensure Some remains

**When NOT to Use**:
- When you don't need the old value (use simple assignment)
- When you want to leave None (use `take()`)
- For conditional updates (use `get_or_insert()`)

**Example** (from option.rs:1886-1894):
```rust
let mut x = Some(2);
let old = x.replace(5);
assert_eq!(x, Some(5)); // New value
assert_eq!(old, Some(2)); // Old value returned

let mut x = None;
let old = x.replace(3);
assert_eq!(x, Some(3)); // Now Some!
assert_eq!(old, None); // Was None
```

**Source**: `library/core/src/option.rs:1879-1901`

---

### Pattern 29: zip() for Combining Options

**Description**: Combine two Options into an Option of a tuple. Returns Some only if both are Some.

**When to Use**:
- When you need both values together
- Validation where all inputs must be present
- Combining independent optional results
- Creating pairs from separate sources

**When NOT to Use**:
- When you need "or" semantics (use `or()`)
- For fallback behavior
- When you want to handle each None differently

**Example** (from option.rs:1911-1916):
```rust
let x = Some(1);
let y = Some("hi");
let z = None::<u8>;

assert_eq!(x.zip(y), Some((1, "hi"))); // Both Some
assert_eq!(x.zip(z), None); // One is None
```

**Real-world usage**:
```rust
let user_id = get_user_id();
let session_id = get_session_id();

// Only proceed if both exist
let (uid, sid) = user_id.zip(session_id)
    .ok_or(Error::Unauthorized)?;
```

**Source**: `library/core/src/option.rs:1903-1929`

---

### Pattern 30: zip_with() for Combining and Transforming

**Description**: Combine two Options and transform them with a function in one operation.

**When to Use**:
- Combining and transforming Options atomically
- Building complex types from optional components
- When you need more than just a tuple
- Functional-style option combination

**When NOT to Use**:
- When a simple tuple is sufficient (use `zip()`)
- For operations that might fail (use nested `and_then()`)
- When you need to handle None cases differently

**Example** (from option.rs:1938-1957):
```rust
#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

let x = Some(17.5);
let y = Some(42.7);

assert_eq!(x.zip_with(y, Point::new), Some(Point { x: 17.5, y: 42.7 }));
assert_eq!(x.zip_with(None, Point::new), None);
```

**Source**: `library/core/src/option.rs:1931-1971`

---

### Pattern 31: unzip() for Splitting Tupled Options

**Description**: Convert `Option<(T, U)>` into `(Option<T>, Option<U>)`, the inverse of `zip()`.

**When to Use**:
- When you need to split paired optional values
- Undoing a previous `zip()` operation
- When you want to handle each component independently
- Pattern matching on tuple elements

**When NOT to Use**:
- When you need both values together (keep as tuple)
- For simple destructuring (use pattern matching)
- When both values should remain synchronized

**Example** (from option.rs:2019-2023):
```rust
let x = Some((1, "hi"));
let y = None::<(u8, u32)>;

assert_eq!(x.unzip(), (Some(1), Some("hi")));
assert_eq!(y.unzip(), (None, None));
```

**Real-world usage**:
```rust
// Split coordinates
let point: Option<(f64, f64)> = get_point();
let (x, y) = point.unzip();
// Now handle x and y independently
```

**Source**: `library/core/src/option.rs:2010-2033`

---

### Pattern 32: copied() and cloned() for Reference Options

**Description**: Convert `Option<&T>` or `Option<&mut T>` to `Option<T>` by copying or cloning the referenced value.

**When to Use**:
- After calling `as_ref()` or methods that return references
- When you need an owned value from a reference
- With Copy types (use `copied()`)
- With Clone types (use `cloned()`)
- Working with iterators over references

**When NOT to Use**:
- When the reference itself is what you need
- For non-Copy/Clone types
- When cloning is expensive and unnecessary

**Example** (from option.rs:2042-2073):
```rust
// With Copy types
let x = 12;
let opt_x = Some(&x);
assert_eq!(opt_x, Some(&12));
let copied = opt_x.copied();
assert_eq!(copied, Some(12)); // Owned value

// With Clone types
let s = String::from("hello");
let opt_s = Some(&s);
let cloned = opt_s.cloned();
assert_eq!(cloned, Some(String::from("hello")));
```

**Real-world usage**:
```rust
// Common in iterator chains
let names: Vec<Option<String>> = vec_of_refs
    .iter()
    .map(|item| item.name.as_ref())
    .map(|opt_ref| opt_ref.cloned())
    .collect();
```

**Source**: `library/core/src/option.rs:2035-2137`

---

### Pattern 33: transpose() for Option<Result> ⟷ Result<Option> Conversion

**Description**: Convert between `Option<Result<T, E>>` and `Result<Option<T>, E>`, useful for error handling with optional values.

**When to Use**:
- Integrating Option-based code with Result-based error handling
- When you want to short-circuit on errors but keep None as valid
- Converting between different error handling styles
- Using `?` operator with nested Option/Result

**When NOT to Use**:
- When the semantics of None and Err are different
- For simple unwrapping (use `ok_or`)
- When you don't need the transposed form

**Example** (from option.rs:2152-2154):
```rust
#[derive(Debug, Eq, PartialEq)]
struct SomeErr;

let x: Option<Result<i32, SomeErr>> = Some(Ok(5));
let y: Result<Option<i32>, SomeErr> = Ok(Some(5));
assert_eq!(x.transpose(), y);

// Transformations:
// Some(Ok(x)) => Ok(Some(x))
// Some(Err(e)) => Err(e)  // Error propagates!
// None => Ok(None)         // None is valid
```

**Real-world usage**:
```rust
fn fetch_optional_data(id: Option<u64>) -> Result<Option<Data>, Error> {
    id.map(|id| fetch_data(id)) // Option<Result<Data, Error>>
      .transpose()               // Result<Option<Data>, Error>
}

// Now you can use ? operator:
let data = fetch_optional_data(some_id)?; // Result unwrapped, Option preserved
```

**Source**: `library/core/src/option.rs:2139-2166`

---

### Pattern 34: Implementing Clone for Options Efficiently

**Description**: Use optimized `clone_from()` to avoid unnecessary allocations when cloning into an existing Option.

**When to Use**:
- When repeatedly updating an Option with cloned values
- In performance-critical code
- When the contained type has an efficient `clone_from` implementation
- For reusing allocations

**When NOT to Use**:
- For Copy types (just use copy)
- When clone_from doesn't provide benefits
- In code where clarity is more important than performance

**Example** (from option.rs:2207-2211):
```rust
impl<T: Clone> Clone for Option<T> {
    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Some(to), Some(from)) => to.clone_from(from), // Reuse allocation!
            (to, from) => *to = from.clone(),               // New allocation
        }
    }
}
```

**Benefit**:
```rust
let mut target: Option<String> = Some(String::with_capacity(100));
let source = Some(String::from("short"));

// clone_from reuses the 100-byte allocation
target.clone_from(&source);
```

**Source**: `library/core/src/option.rs:2190-2213`

---

### Pattern 35: Using From Trait for Option Construction

**Description**: Use `From<T> for Option<T>` and related conversions for ergonomic Option construction.

**When to Use**:
- In generic code that accepts `Into<Option<T>>`
- For implicit conversions
- Making APIs more flexible
- Converting between Option and reference types

**When NOT to Use**:
- When explicit `Some()` is clearer
- In simple, non-generic code
- When the conversion might be surprising

**Example** (from option.rs:2287-2293, 2312-2320):
```rust
// T -> Option<T>
let o: Option<u8> = Option::from(67);
assert_eq!(Some(67), o);

// &Option<T> -> Option<&T>
let s: Option<String> = Some(String::from("Hello, Rustaceans!"));
let o: Option<usize> = Option::from(&s).map(|ss: &String| ss.len());
println!("Can still print s: {s:?}"); // s not consumed!

// Function that accepts Into<Option<T>>
fn maybe_process<T>(value: impl Into<Option<T>>) {
    let opt = value.into();
    // Process opt...
}

maybe_process(42);        // T -> Option<T>
maybe_process(Some(42));  // Option<T> -> Option<T>
```

**Source**: `library/core/src/option.rs:2279-2345`

---

### Pattern 36: Option Ordering and Comparison

**Description**: None compares as less than any Some, and Some values compare by their contained values.

**When to Use**:
- Sorting collections of Options
- Using Options as keys in ordered containers
- Implementing ordering for types containing Options
- When None should be "smallest"

**When NOT to Use**:
- When None should not have an ordering
- When custom ordering is needed
- For equality checks only (use PartialEq)

**Example** (from option.rs:2375-2395):
```rust
// Ordering rules:
assert!(None < Some(0));        // None is always less
assert!(Some(0) < Some(1));     // Some values compare normally

// Sorting
let mut opts = vec![Some(3), None, Some(1), Some(2), None];
opts.sort();
assert_eq!(opts, vec![None, None, Some(1), Some(2), Some(3)]);

// In structs
#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct Person {
    age: Option<u32>,  // None means unknown
}

// People with unknown age sort first
```

**Implementation**:
```rust
impl<T: Ord> Ord for Option<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Some(l), Some(r)) => l.cmp(r),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    }
}
```

**Source**: `library/core/src/option.rs:2371-2397`

---

### Pattern 37: Default Implementation (None as Default)

**Description**: Option's Default implementation always returns None, making it useful for initialization.

**When to Use**:
- When initializing collections of Options
- With `#[derive(Default)]` on structs
- In builder patterns
- For "empty" initial state

**When NOT to Use**:
- When a Some default makes more sense (use a const or custom Default)
- When None is not a sensible default

**Example** (from option.rs:2226-2232):
```rust
let opt: Option<u32> = Option::default();
assert!(opt.is_none());

// In structs
#[derive(Default)]
struct Config {
    timeout: Option<Duration>,  // Defaults to None
    retries: Option<u32>,        // Defaults to None
}

let config = Config::default();
assert!(config.timeout.is_none());

// In collections
let opts: Vec<Option<i32>> = vec![Default::default(); 5];
assert_eq!(opts, vec![None, None, None, None, None]);
```

**Source**: `library/core/src/option.rs:2218-2233`

---

### Pattern 38: FromIterator for Short-Circuiting Collection

**Description**: Collect an iterator of `Option<T>` into `Option<Collection<T>>`, short-circuiting on first None.

**When to Use**:
- Processing collections where all operations must succeed
- Validating or transforming all elements
- When you want fail-fast behavior
- With checked arithmetic operations

**When NOT to Use**:
- When you want to collect successful values only (use `filter_map`)
- When you need to know which element failed
- For partial results

**Example** (from option.rs:2590-2634):
```rust
// Success case - all values present
let items = vec![0_u16, 1, 2];
let res: Option<Vec<u16>> = items
    .iter()
    .map(|x| x.checked_add(1))
    .collect();
assert_eq!(res, Some(vec![1, 2, 3]));

// Failure case - first None stops collection
let items = vec![2_u16, 1, 0];
let res: Option<Vec<u16>> = items
    .iter()
    .map(|x| x.checked_sub(1))
    .collect();
assert_eq!(res, None); // Last element would underflow

// Short-circuit behavior - processing stops at first None
let items = vec![3_u16, 2, 1, 10];
let mut shared = 0;
let res: Option<Vec<u16>> = items
    .iter()
    .map(|x| { shared += x; x.checked_sub(2) })
    .collect();
assert_eq!(res, None);
assert_eq!(shared, 6); // Only processed 3, 2, 1 (not 10!)
```

**Source**: `library/core/src/option.rs:2576-2645`

---

### Pattern 39: The Try Trait (? Operator Implementation)

**Description**: Option implements the Try trait, enabling the `?` operator for early return on None.

**When to Use**:
- In functions returning Option
- To propagate None up the call stack
- For cleaner error handling without explicit matching
- Chaining fallible operations

**When NOT to Use**:
- When you need to handle None with custom logic
- In functions not returning Option or Result
- When the short-circuit behavior is surprising

**How It Works** (from option.rs:2649-2665):
```rust
impl<T> Try for Option<T> {
    type Output = T;
    type Residual = Option<Infallible>;

    fn from_output(output: T) -> Option<T> {
        Some(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, T> {
        match self {
            Some(v) => ControlFlow::Continue(v),  // Extract value
            None => ControlFlow::Break(None),      // Early return
        }
    }
}
```

**Usage**:
```rust
fn get_config() -> Option<Config> {
    let path = get_path()?;      // Returns None if get_path() is None
    let contents = read(path)?;   // Returns None if read() is None
    parse(contents)?              // Returns None if parse() is None
}
```

**Source**: `library/core/src/option.rs:2647-2694`

---

### Pattern 40: flatten() for Nested Options

**Description**: Convert `Option<Option<T>>` to `Option<T>`, removing one level of nesting.

**When to Use**:
- After mapping with a function that returns Option
- When you have naturally nested Options
- To avoid `Option<Option<T>>` types
- As an alternative to `and_then` with identity

**When NOT to Use**:
- When you need to distinguish between Some(None) and None
- For more than one level (call `flatten()` multiple times)
- When `and_then` is clearer

**Example** (from option.rs:2704-2719):
```rust
let x: Option<Option<u32>> = Some(Some(6));
assert_eq!(Some(6), x.flatten());

let x: Option<Option<u32>> = Some(None);
assert_eq!(None, x.flatten()); // Some(None) flattens to None

let x: Option<Option<u32>> = None;
assert_eq!(None, x.flatten());

// Flattening only removes one level
let x: Option<Option<Option<u32>>> = Some(Some(Some(6)));
assert_eq!(Some(Some(6)), x.flatten());
assert_eq!(Some(6), x.flatten().flatten());
```

**Relationship to and_then**:
```rust
// These are equivalent:
option.flatten()
option.and_then(|x| x)
```

**Source**: `library/core/src/option.rs:2696-2732`

---

### Pattern 41: Array transpose() for Batch Processing

**Description**: Transpose `[Option<T>; N]` into `Option<[T; N]>`, succeeding only if all elements are Some.

**When to Use**:
- Processing fixed-size arrays of optional values
- When all values must be present to proceed
- Batch validation of array elements
- Converting between representations

**When NOT to Use**:
- With variable-length collections (use `Iterator::collect`)
- When partial arrays are acceptable
- For dynamic-sized data

**Example** (from option.rs:2743-2749):
```rust
let data = [Some(0); 1000];
let data: Option<[u8; 1000]> = data.transpose();
assert_eq!(data, Some([0; 1000])); // All Some -> Some(array)

let data = [Some(0), None];
let data: Option<[u8; 2]> = data.transpose();
assert_eq!(data, None); // One None -> None
```

**Real-world usage**:
```rust
// Parsing fixed-size coordinate data
let coords: [Option<f64>; 3] = [
    parse_coord("1.5"),
    parse_coord("2.5"),
    parse_coord("3.5"),
];

let point: Option<[f64; 3]> = coords.transpose();
match point {
    Some([x, y, z]) => process_3d_point(x, y, z),
    None => handle_invalid_coordinates(),
}
```

**Source**: `library/core/src/option.rs:2734-2756`

---

