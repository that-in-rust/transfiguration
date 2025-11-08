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
- **option.rs** (2,756 lines): In progress - 21 patterns documented
  - Lines 1-500: ✅ Completed (Patterns 1-7)
  - Lines 501-1000: ✅ Completed (Patterns 8-14)
  - Lines 1001-1500: ✅ Completed (Patterns 15-21)
  - Lines 1501-2000: Pending
  - Lines 2001-2500: Pending
  - Lines 2501-2756: Pending

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

