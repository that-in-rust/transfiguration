========================================
IDIOMATIC RUST PATTERNS
========================================

0Z. CURATION PLAN, TASK LIST, AND PROGRESS
--------------------------------
Plan
- Intake source in 2000-line chunks from `Insp06RustNotes/RustIdioms_trun_rust_idiom_large.txt`
- For each chunk: extract high-signal idioms; for each idiom add: When, Context, Anti-patterns, Micro-example, References
- Update progress log with LOC metrics and chunk range; keep changes additive and non-destructive

Tasks
- [x] Insert scaffolding into this document
- [x] Process chunk 1 (lines 1–500)
- [x] Process chunk 2 (lines 501–1000)
- [x] Process chunk 3 (lines 1001–1500)
- [x] Process chunk 4 (lines 1501–2000)
- [ ] Process chunk 5 (lines 2001–2500)
- [ ] Process chunk 6 (lines 2501–3000)
- [ ] Process chunk 7 (lines 3001–3500)
- [ ] Process chunk 8 (lines 3501–4000)
- [ ] Process chunk 9 (lines 4001–4500)
- [ ] Process chunk 10 (lines 4501–5000)
- [ ] Process chunk 11 (lines 5001–5500)
- [ ] Process chunk 12 (lines 5501–6000)
- [ ] Process chunk 13 (lines 6001–6500)
- [ ] Process chunk 14 (lines 6501–7000)
- [ ] Process chunk 15 (lines 7001–7500)
- [ ] Process chunk 16 (lines 7501–8000)
- [ ] Process chunk 17 (lines 8001–8500)
- [ ] Process chunk 18 (lines 8501–9000)
- [ ] Process chunk 19 (lines 9001–9500)
- [ ] Process chunk 20 (lines 9501–10000)
- [ ] Process chunk 21 (lines 10001–10500)
- [ ] Process chunk 22 (lines 10501–11000)
- [ ] Process chunk 23 (lines 11001–11500)
- [ ] Process chunk 24 (lines 11501–12000)
- [ ] Process chunk 25 (lines 12001–12500)
- [ ] Process chunk 26 (lines 12501–13000)
- [ ] Process chunk 27 (lines 13001–13500)
- [ ] Process chunk 28 (lines 13501–14000)
- [ ] Process chunk 29 (lines 14001–14500)
- [ ] Process chunk 30 (lines 14501–15000)
- [ ] Process chunk 31 (lines 15001–15500)
- [ ] Process chunk 32 (lines 15501–16000)
- [ ] Process chunk 33 (lines 16001–16500)
- [ ] Process chunk 34 (lines 16501–17000)
- [ ] Process chunk 35 (lines 17001–17500)
- [ ] Process chunk 36 (lines 17501–18000)
- [ ] Process chunk 37 (lines 18001–18500)
- [ ] Process chunk 38 (lines 18501–19000)
- [ ] Process chunk 39 (lines 19001–19500)
- [ ] Process chunk 40 (lines 19501–20000)
- [ ] Process chunk 41 (lines 20001–20500)
- [ ] Process chunk 42 (lines 20501–21000)
- [ ] Process chunk 43 (lines 21001–21500)
- [ ] Process chunk 44 (lines 21501–22000)
- [ ] Process chunk 45 (lines 22001–22500)
- [ ] Process chunk 46 (lines 22501–23000)
- [ ] Process chunk 47 (lines 23001–23500)
- [ ] Process chunk 48 (lines 23501–24000)
- [ ] Process chunk 49 (lines 24001–24500)
- [ ] Process chunk 50 (lines 24501–25000)
- [ ] Process chunk 51 (lines 25001–25500)
- [ ] Process chunk 52 (lines 25501–26000)
- [ ] Process chunk 53 (lines 26001–26500)
- [ ] Process chunk 54 (lines 26501–27000)
- [ ] Process chunk 55 (lines 27001–27500)
- [ ] Process chunk 56 (lines 27501–28000)
- [ ] Process chunk 57 (lines 28001–28500)
- [ ] Process chunk 58 (lines 28501–29000)
- [ ] Process chunk 59 (lines 29001–29500)
- [ ] Process chunk 60 (lines 29501–30000)
- [ ] Process chunk 61 (lines 30001–30500)
- [ ] Process chunk 62 (lines 30501–31000)
- [ ] Process chunk 63 (lines 31001–31500)
- [ ] Process chunk 64 (lines 31501–32000)
- [ ] Process chunk 65 (lines 32001–32500)
- [ ] Process chunk 66 (lines 32501–33000)
- [ ] Process chunk 67 (lines 33001–33500)
- [ ] Process chunk 68 (lines 33501–34000)
- [ ] Process chunk 69 (lines 34001–34500)
- [ ] Process chunk 70 (lines 34501–35000)
- [ ] Process chunk 71 (lines 35001–35500)
- [ ] Process chunk 72 (lines 35501–36000)
- [ ] Process chunk 73 (lines 36001–36500)
- [ ] Process chunk 74 (lines 36501–37000)
- [ ] Process chunk 75 (lines 37001–37500)
- [ ] Process chunk 76 (lines 37501–38000)
- [ ] Process chunk 77 (lines 38001–38500)
- [ ] Process chunk 78 (lines 38501–39000)
- [ ] Process chunk 79 (lines 39001–39500)
- [ ] Process chunk 80 (lines 39501–40000)
- [ ] Process chunk 81 (lines 40001–40500)
- [ ] Process chunk 82 (lines 40501–41000)
- [ ] Process chunk 83 (lines 41001–41500)
- [ ] Process chunk 84 (lines 41501–42000)
- [ ] Process chunk 85 (lines 42001–42500)
- [ ] Process chunk 86 (lines 42501–43000)
- [ ] Process chunk 87 (lines 43001–43500)
- [ ] Process chunk 88 (lines 43501–44000)
- [ ] Process chunk 89 (lines 44001–44500)
- [ ] Process chunk 90 (lines 44501–45000)
- [ ] Process chunk 91 (lines 45001–45500)
- [ ] Process chunk 92 (lines 45501–46000)
- [ ] Process chunk 93 (lines 46001–46500)
- [ ] Process chunk 94 (lines 46501–47000)
- [ ] Process chunk 95 (lines 47001–47500)
- [ ] Process chunk 96 (lines 47501–48000)
- [ ] Process chunk 97 (lines 48001–48500)
- [ ] Process chunk 98 (lines 48501–49000)
- [ ] Process chunk 99 (lines 49001–49500)
- [ ] Process chunk 100 (lines 49501–50000)
- [ ] Process chunk 101 (lines 50001–50500)
- [ ] Process chunk 102 (lines 50501–51000)
- [ ] Process chunk 103 (lines 51001–51500)
- [ ] Process chunk 104 (lines 51501–52000)
- [ ] Process chunk 105 (lines 52001–52171)
- [ ] Quality pass and deduplicate across sections

Tasks (2000-line mode)
- [x] Process chunk 1–2000 (completed via 4×500)
- [x] Process chunk 2001–4000
- [x] Process chunk 4001–6000
- [ ] Process chunk 6001–8000
- [ ] Process chunk 8001–10000
- [ ] Process chunk 10001–12000
- [ ] Process chunk 12001–14000
- [ ] Process chunk 14001–16000
- [ ] Process chunk 16001–18000
- [ ] Process chunk 18001–20000
- [ ] Process chunk 20001–22000
- [ ] Process chunk 22001–24000
- [ ] Process chunk 24001–26000
- [ ] Process chunk 26001–28000
- [ ] Process chunk 28001–30000
- [ ] Process chunk 30001–32000
- [ ] Process chunk 32001–34000
- [ ] Process chunk 34001–36000
- [ ] Process chunk 36001–38000
- [ ] Process chunk 38001–40000
- [ ] Process chunk 40001–42000
- [ ] Process chunk 42001–44000
- [ ] Process chunk 44001–46000
- [ ] Process chunk 46001–48000
- [ ] Process chunk 48001–50000
- [ ] Process chunk 50001–52000
- [ ] Process chunk 52001–52171

Measurement methodology
- Source LOC processed = sum of chunk sizes processed so far
- Target LOC total and delta computed after each update versus prior revision baseline
- Delta is recorded below; placeholders will be backfilled automatically

Progress Log
| Chunk | Source lines | Idioms added | Target total LOC | Delta LOC vs prev | Notes | Timestamp |
|---|---|---:|---:|---:|---|---|
| 1 | 1–500 | 12 | 730 | 122 | Initial curated set from foundations, ownership, error handling, async | 2025-10-23 |
| 2 | 501–1000 | 12 | 974 | 244 | QA/tooling + web/service idioms (clippy, fmt, MSRV, doctest, trybuild, proptest, fuzz, coverage, audit, tracing, secrecy, SQLx) | 2025-10-23 |
| 3 | 1001–1500 | 6 | 1055 | 81 | Diagnostics/cancellation/loom/result ergonomics/rustfmt config | 2025-10-23 |
| 4 | 1501–2000 | 7 | 1109 | 54 | Error libs matrix, FFI unwind safety, iter/IntoIterator, closures bounds, DoubleEnded/Fused, FromIterator | 2025-10-23 |
| 5 | 2001–4000 | 12 | 1213 | 104 | Containers/iteration deep dive, closure capture, raw identifiers, trait upcasting, coherence | 2025-10-23 |
| 6 | 4001–6000 | 11 | 1269 | 56 | Cancel safety, panic profiles, capacity planning, BinaryHeap, SmallVec, HashMap/IndexMap, patterns/FFI | 2025-10-23 |
| 7 | 6001–8000 | 10 | 1321 | 52 | Keywords/editions, HRTBs, GATs, object safety, never type, nullable pointer FFI, IndexMap, heap, async poll | 2025-10-23 |

A. Curated Idioms (Deep Dives)
------------------------------

A.1 Expression-Oriented Code
- Use when: a function naturally computes a value; prefer returning the last expression without a trailing semicolon.
- Context: prefer `if/else` and `match` as expressions; use `let` bindings for clarity; reduce mutable state.
- Avoid/Anti-pattern: unnecessary `mut` accumulators when `map/fold` suffice; stray semicolons converting expressions to `()`.

A.2 Accept Slices and Traits in Public APIs
- Use when: callers can pass many input types; prefer `&[T]`, `&str`, `AsRef<T>`, `Borrow<T>` over concrete containers.
- Context: stable, flexible APIs; avoid needless cloning, enable zero-cost conversion.
- Avoid/Anti-pattern: taking `&Vec<T>`/`&String` in signatures; over-generic bounds that hurt inference.

```rust path=null start=null
fn process<S: AsRef<str>>(s: S) {
    let s: &str = s.as_ref();
    // ...
}
```

A.3 Clone-on-Write (Cow<'_, str/[u8]>)
- Use when: sometimes borrow, sometimes need owned/modified data; avoid eager clones on hot paths.
- Context: config, cache keys, deserialized data that is usually borrowed.
- Avoid/Anti-pattern: using `Cow` when data is always owned or always borrowed; hidden `.to_string()` in tight loops.

A.4 From/TryFrom/TryInto for Conversions
- Use when: modeling infallible/fallible conversions instead of ad-hoc constructors.
- Context: ergonomic API surfaces; composable conversions in pipelines.
- Avoid/Anti-pattern: exposing `Result` for infallible cases; implementing for foreign types (orphan rule) instead of a newtype.

A.5 Newtype Pattern (+ repr(transparent) for FFI)
- Use when: enforce domain invariants and give semantic meaning beyond primitives.
- Context: type-safe IDs, units, opaque handles, and FFI boundaries.
- Avoid/Anti-pattern: `type` alias where a distinct type is needed; forgetting `#[repr(transparent)]` for FFI.

```rust path=null start=null
#[repr(transparent)]
struct UserId(String);
impl UserId { fn parse(s: &str) -> Option<Self> { (!s.is_empty()).then(|| Self(s.to_owned())) } }
```

A.6 Error Boundaries: thiserror (libs) vs anyhow (apps)
- Use when: libraries define precise `Error` enums; applications aggregate with context.
- Context: `?` with `.context()` / `.with_context()`; stable public APIs.
- Avoid/Anti-pattern: exposing `anyhow::Error` in public APIs; panicking on recoverable errors.

A.7 Option/Result Combinators over Nested Matching
- Use when: simple transformations/chaining suffice (`map`, `and_then`, `transpose`).
- Context: linear flows; interop between `Option` and `Result`.
- Avoid/Anti-pattern: `.unwrap()`/`.expect()` in production; deep nested matches.

A.8 Interior Mutability vs Synchronization Primitives
- Use when: `Cell/RefCell` for single-thread logical mutability; `Mutex/RwLock` for cross-thread.
- Context: minimize lock scope; short-lived borrows.
- Avoid/Anti-pattern: holding `RefCell` borrows or locks across `.await`; mutex in single-threaded code.

A.9 Trait Objects vs Generics
- Use when: heterogenous collections or dynamic dispatch at API boundaries (`Box<dyn Trait>`).
- Context: use generics for hot paths; trait objects for flexibility and reduced code size.
- Avoid/Anti-pattern: trait objects in tight loops; over-generic public APIs causing monomorphization bloat.

A.10 Pin/Unpin and Self-Referential Types
- Use when: implementing `Future`/state machines that must not move after pinning.
- Context: use `Pin` smartly; maintain invariants; prefer safe APIs that encapsulate pin.
- Avoid/Anti-pattern: `Pin::new_unchecked` without proof; moving pinned fields.

A.11 Async Hygiene: No Blocking and No Locks Across await
- Use when: CPU-bound work inside async contexts (`spawn_blocking`) and short critical sections.
- Context: avoid scheduler starvation; minimize `.await` inside critical regions.
- Avoid/Anti-pattern: blocking I/O/CPU directly on the runtime; holding locks across `.await`.

```rust path=null start=null
let data = Arc::new(Mutex::new(vec![]));
// Anti-pattern:
// let mut guard = data.lock().unwrap();
// async_call().await; // lock held across await
// Correct:
{
    let mut guard = data.lock().unwrap();
    guard.push(1);
}
async_call().await;
```

A.12 Bounded Channels, Backpressure, and Cancellation
- Use when: limit memory growth and propagate shutdown.
- Context: `tokio::sync::mpsc` with capacity; `tokio_util::sync::CancellationToken` for cooperative cancellation.
- Avoid/Anti-pattern: unbounded channels in high-throughput systems; ad-hoc boolean flags for cancellation.

```rust path=null start=null
use tokio::{sync::mpsc, time::{timeout, Duration}};
let (tx, mut rx) = mpsc::channel(1024);
let send = async move { tx.send(item).await.ok(); };
let recv = async move { while let Some(v) = rx.recv().await { /* ... */ } };
let _ = timeout(Duration::from_secs(1), recv).await; // apply backpressure/timeouts
```

A.13 Clippy as a Gate (-D warnings)
- Use when: enforce correctness and idioms across all targets and features in CI.
- Context: run locally and in CI; gate merges on a clean clippy run.
- Avoid/Anti-pattern: allowing warnings in CI; running clippy on a subset of targets only.

```sh path=null start=null
cargo clippy --all-targets --all-features -- -D warnings
```

A.14 rustfmt as Non-negotiable Style
- Use when: enforce consistent formatting across the workspace.
- Context: run `cargo fmt --check` in CI; auto-format locally.
- Avoid/Anti-pattern: hand-tuned formatting; ignoring fmt drift.

```sh path=null start=null
cargo fmt --all -- --check
```

A.15 MSRV Policy (rust-version in Cargo.toml)
- Use when: communicating minimum supported Rust version to users and CI.
- Context: set `package.rust-version`; test on that toolchain; document policy.
- Avoid/Anti-pattern: breaking MSRV silently; relying on nightly-only features in libraries.

```toml path=null start=null
[package]
rust-version = "1.76"
```

A.16 Doctests as Executable Contracts
- Use when: examples express API semantics and invariants.
- Context: small, focused examples; run with `cargo test`.
- Avoid/Anti-pattern: examples that don't compile or assert nothing.

```rust path=null start=null
/// Adds two numbers.
/// ```
/// assert_eq!(my_crate::add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

A.17 Compile-fail UI Tests (trybuild)
- Use when: macros and type errors are part of your public contract.
- Context: maintain `tests/ui/*.rs` cases; assert specific diagnostics.
- Avoid/Anti-pattern: relying on unstable error messages; coupling to nightly-only spans.

```rust path=null start=null
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
```

A.18 Property-based Testing (proptest)
- Use when: invariants must hold across large input spaces.
- Context: seed failures for reproducibility; shrink to minimal counterexamples.
- Avoid/Anti-pattern: only example-based tests for complex parsers/serializers.

```rust path=null start=null
use proptest::prelude::*;
proptest! {
  #[test]
  fn roundtrip(x in any::<u32>()) {
    let enc = encode(x);
    prop_assert_eq!(decode(&enc).unwrap(), x);
  }
}
```

A.19 Fuzz Frontiers (cargo-fuzz/libFuzzer)
- Use when: parsing/untrusted input, protocol boundaries, decoders.
- Context: minimal harness; CI nightly jobs; keep corpus and minimize.
- Avoid/Anti-pattern: running fuzzers without ASan/UBSan; ignoring timeouts.

```rust path=null start=null
#![no_main]
use libfuzzer_sys::fuzz_target;
fuzz_target!(|data: &[u8]| { let _ = my_crate::parse(data); });
```

A.20 Coverage as a Gate (cargo-llvm-cov)
- Use when: prevent coverage regressions on critical modules.
- Context: gate on line/branch coverage thresholds; exclude generated code.
- Avoid/Anti-pattern: chasing 100% blindly; ignoring branch coverage.

```sh path=null start=null
cargo llvm-cov --workspace --branch --fail-under-lines 75
```

A.21 Supply-chain Policies (cargo-audit/deny/vet)
- Use when: enforce vuln and license policies before merge/release.
- Context: nightly job for advisory DB; strict allowlists.
- Avoid/Anti-pattern: pinning yanked/vulnerable crates without justification.

```toml path=null start=null
# deny.toml
[licenses]
allow = ["MIT", "Apache-2.0"]

[advisories]
db-path = "~/.cargo/advisory-db"
```

A.22 Structured Observability (tracing)
- Use when: correlate logs, metrics, and spans in async services.
- Context: `#[instrument]` on boundaries; propagate span fields (trace_id, user_id).
- Avoid/Anti-pattern: println!-style logs in libraries; logging secrets.

```rust path=null start=null
#[tracing::instrument(skip(req))]
pub async fn handle(req: Request) -> Result<Response, Error> {
    tracing::info!(method = %req.method(), path = %req.uri().path());
    // ...
    Ok(resp)
}
```

A.23 Secret Handling (secrecy)
- Use when: hold API keys/passwords safely in memory/logs.
- Context: use `SecretString`/`SecretVec`; avoid Display/Debug.
- Avoid/Anti-pattern: formatting secrets into logs; cloning secrets widely.

```rust path=null start=null
use secrecy::{ExposeSecret, SecretString};
let api_key = SecretString::new(load_key());
// tracing::info!(key = %api_key); // anti-pattern
tracing::info!(has_key = api_key.expose_secret().len() > 0);
```

A.24 SQLx Compile-time Queries and Pooling
- Use when: ensure query validity at build time and safe pooling.
- Context: `query!`/`query_as!` with offline mode; timeouts and max connections.
- Avoid/Anti-pattern: building SQL strings dynamically; holding connections across `.await` unnecessarily.

```rust path=null start=null
#[derive(sqlx::FromRow)]
struct User { id: i64, name: String }
let row = sqlx::query_as!(User, "SELECT id, name FROM users WHERE id = $1", id)
    .fetch_one(&pool).await?;
```

A.25 Cancellation-safe select! and Future design
- Use when: multiplexing operations with `tokio::select!`.
- Context: futures in `select!` should not own partially-consumed state; borrow from a shared buffer or encapsulate state in an owner outside the futures.
- Avoid/Anti-pattern: futures that drop with half-consumed state; mixing blocking I/O in selected branches.

```rust path=null start=null
use tokio::select;
async fn run(mut stream: BufferedStream) {
    loop {
        select! {
            biased;
            maybe = stream.next() => { if maybe.is_none() { break; } },
            _ = shutdown_token.cancelled() => { break; }
        }
    }
}
```

A.26 Panic Strategy: abort in release binaries
- Use when: CLIs/embedded where unwind is undesirable; minimize binary size and fail-fast.
- Context: keep unwind in debug for better backtraces; never expose panic behavior from libraries.
- Avoid/Anti-pattern: setting `panic = "abort"` in libraries; aborting in tests.

```toml path=null start=null
[profile.release]
panic = "abort"
```

A.27 Rich Diagnostics for Applications (color-eyre/miette)
- Use when: human-friendly error reports and panics are required.
- Context: install a global reporter on startup; prefer `eyre::Result` in bin targets only.
- Avoid/Anti-pattern: using these types in public library APIs.

```rust path=null start=null
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // ...
    Ok(())
}
```

A.28 Concurrency Model Testing with loom
- Use when: validating Send/Sync invariants and race-freedom.
- Context: model small critical sections; assert invariants across all interleavings.
- Avoid/Anti-pattern: testing production code paths wholesale; keep models minimal.

```rust path=null start=null
use loom::{sync::Mutex, thread};
loom::model(|| {
    let m = std::sync::Arc::new(Mutex::new(0));
    let m2 = m.clone();
    let t = thread::spawn(move || { *m2.lock().unwrap() += 1; });
    *m.lock().unwrap() += 1;
    t.join().unwrap();
    assert_eq!(*m.lock().unwrap(), 2);
});
```

A.29 Result Ergonomics: context and mapping
- Use when: propagate errors with additional context.
- Context: libraries: `thiserror` + `#[from]` + `map_err`; apps: `anyhow::Context`.
- Avoid/Anti-pattern: `.unwrap()` in non-test code; swallowing sources.

```rust path=null start=null
use anyhow::Context;
fn load(path: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("reading {}", path))
}
```

A.30 rustfmt configuration as code
- Use when: enforce project-wide style to avoid review churn.
- Context: commit `rustfmt.toml` and gate with `cargo fmt --check`.
- Avoid/Anti-pattern: per-contributor styles leading to unstable diffs.

```toml path=null start=null
edition = "2024"
max_width = 100
group_imports = "StdExternalCrate"
```

A.31 Library Error Design: thiserror vs snafu vs eyre/anyhow
- Use when: choosing error modeling for libraries vs applications.
- Context: libraries prefer typed enums (`thiserror`) or context-rich enums (`snafu`) when callsites need structured handling; binaries can use `anyhow`/`eyre` for ergonomic aggregation.
- Avoid/Anti-pattern: returning `anyhow::Error` from library public APIs; losing source error context.

```rust path=null start=null
#[derive(thiserror::Error, Debug)]
pub enum ParseError {
  #[error("invalid header: {0}")]
  InvalidHeader(String),
  #[error(transparent)]
  Io(#[from] std::io::Error),
}
```

A.32 FFI Unwind Safety and extern "C-unwind"
- Use when: calling into or being called from C/C++/other languages where exceptions/longjmp may cross boundaries.
- Context: mark Rust FFI with `extern "C"`; never let Rust panics cross FFI; catch with `std::panic::catch_unwind` and convert to error codes. Consider `extern "C-unwind"` where appropriate.
- Avoid/Anti-pattern: unwinding across FFI without explicit contracts; relying on UB-prone longjmp behavior.

```rust path=null start=null
#[no_mangle]
pub extern "C" fn api_do(ptr: *const u8, len: usize) -> i32 {
  let res = std::panic::catch_unwind(|| unsafe { do_work(std::slice::from_raw_parts(ptr, len)) });
  match res { Ok(Ok(())) => 0, Ok(Err(_e)) => -2, Err(_panic) => -1 }
}
```

A.33 IntoIterator/iter/iter_mut Semantics
- Use when: designing APIs that accept collections generically or implementing iteration for your types.
- Context: prefer bounds like `impl<T: IntoIterator<Item = X>>` for flexibility; use `iter()/iter_mut()` when you need borrow semantics; implement `IntoIterator` for owned iteration of custom collections.
- Avoid/Anti-pattern: surprising ownership moves via `into_iter()` on owned containers when a borrow suffices; ambiguous trait bounds hurting inference.

A.34 Iterator Laziness and Adapter vs Consumer Separation
- Use when: building pipelines; compose with adapters (`map/filter/take`) and drive with consumers (`collect/sum/for_each`).
- Context: prefer iterator combinators over manual loops for clarity and fusion; be mindful of allocation in `collect`.
- Avoid/Anti-pattern: building intermediate `Vec`s unnecessarily; relying on side-effects in adapters.

A.35 Choosing Fn/FnMut/FnOnce Bounds
- Use when: accepting closures; pick the narrowest bound that fits call semantics.
- Context: `Fn` for repeated read-only calls (often Send/Sync friendly); `FnMut` when mutation is required; `FnOnce` when the closure may consume captures.
- Avoid/Anti-pattern: overconstraining to `Fn` when `FnOnce` is needed; accidentally requiring `mut` when not needed.

A.36 DoubleEndedIterator and FusedIterator
- Use when: bi-directional traversal and predictable exhaustion semantics matter.
- Context: implement `DoubleEndedIterator` when reverse iteration is natural; use `.fuse()` or rely on `FusedIterator` for repeated-None guarantees.
- Avoid/Anti-pattern: assuming fused behavior without `.fuse()`; incorrectly implementing `next_back` semantics.

A.37 FromIterator and Collect Patterns
- Use when: constructing collections from iterators; offer `FromIterator` for custom containers.
- Context: `collect::<Vec<_>>()` for materialization; `FromIterator` for explicit types; streaming transforms with minimal allocations.
- Avoid/Anti-pattern: collecting when a lazy pipeline suffices; turbofish misuse obscuring types.

A.38 Vec vs VecDeque vs LinkedList
- Use when: choosing a sequence container.
- Context: default to `Vec` for most cases; `VecDeque` for efficient queue/deque semantics; avoid `LinkedList` unless specific O(1) splice patterns dominate and cache effects are acceptable.
- Avoid/Anti-pattern: using `LinkedList` for general workloads; premature `VecDeque` when simple front-removals can be amortized differently.

A.39 HashMap vs BTreeMap
- Use when: key-value storage; choose `HashMap` for average-case O(1), `BTreeMap` for ordering and predictable iteration.
- Context: determinism requirements, range queries, memory locality; consider worst-case hash DoS and feature `hashbrown`/fxhash only with care in non-adversarial contexts.
- Avoid/Anti-pattern: relying on hash iteration order; using BTreeMap for hot random-access without ordering needs.

A.40 Implement IntoIterator for Owned and Borrowed
- Use when: custom collections should work with `for` over owned and borrowed forms.
- Context: implement `IntoIterator` for `T` and for `&T`/`&mut T` to support all use-sites.
- Avoid/Anti-pattern: only implementing owned iteration, breaking `for &item in &collection` ergonomics.

```rust path=null start=null
#[derive(Debug)]
struct MyCollection(Vec<i32>);
impl IntoIterator for MyCollection {
  type Item = i32;
  type IntoIter = std::vec::IntoIter<i32>;
  fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
impl<'a> IntoIterator for &'a MyCollection {
  type Item = &'a i32;
  type IntoIter = std::slice::Iter<'a, i32>;
  fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}
```

A.41 Return impl Iterator from APIs
- Use when: expose lazy pipelines without committing to concrete adaptor types.
- Context: stable, efficient APIs that can be optimized by the compiler; document when evaluation occurs.
- Avoid/Anti-pattern: returning concrete iterator types in public APIs; boxing iterators on hot paths.

A.42 Fused Iteration and Reversal
- Use when: consumers may call `next()` after exhaustion; add `.fuse()` to guarantee repeated-None semantics.
- Context: use `.rev()` when `DoubleEndedIterator` is available; implement `next_back` correctly for custom iterators.
- Avoid/Anti-pattern: assuming fusion without `.fuse()`; mixing forward/backward iteration state incorrectly.

A.43 Closure Capture and move in Concurrency
- Use when: spawning threads/tasks; prefer `move` closures; ensure captured values are `Send + 'static` where required.
- Context: choose `Fn`/`FnMut`/`FnOnce` bounds precisely; prefer immutable captures for `Fn` when possible.
- Avoid/Anti-pattern: capturing non-`Send` across thread boundaries; overusing `Arc<Mutex<_>>` when channels suffice.

A.44 Arrays, Slices, and IntoIterator Nuances
- Use when: writing generic iteration; prefer `&[T]` bounds and `iter()` for clarity; be aware of array `IntoIterator` semantics on modern Rust.
- Context: avoid surprises from `into_iter()` moving owned arrays; use `iter_mut()` for in-place transforms.
- Avoid/Anti-pattern: ambiguous `IntoIterator` bounds that hinder inference; relying on edition-specific behavior.

A.45 Raw Identifiers to Bridge Editions
- Use when: identifiers conflict with keywords across editions (`r#try`, `r#match`).
- Context: interop with older crates or generated code.
- Avoid/Anti-pattern: renaming public symbols that must stay stable for interop without using raw identifiers.

A.46 Trait Object Upcasting (dyn upcast)
- Use when: upcast `dyn SubTrait` to `dyn SuperTrait` where stable; simplifies API layering without custom `as_super` shims.
- Context: prefer static dispatch in hot loops; use trait objects for dynamic plugin points.
- Avoid/Anti-pattern: constructing invalid vtables in unsafe; leaking raw trait-object pointers.

A.47 Coherence and Orphan Rule Hygiene
- Use when: designing trait extension points; ensure either the trait or the self type is local.
- Context: provide extension traits for foreign types; avoid overlapping impls.
- Avoid/Anti-pattern: blanket impls that would conflict downstream; violating E0117/E0210 orphan rules.

A.48 Specialization Status and min_specialization
- Use when: constrained cases in std or internal crates; avoid in general libraries until fully stabilized.
- Context: prefer explicit trait design over specialization; document semantics if used under feature flags.
- Avoid/Anti-pattern: relying on unsound specialization behavior in public crates.

A.49 Fn/FnMut/FnOnce Bound Selection Patterns
- Use when: picking callable trait bounds; mirror call semantics (read-only, mutable, consuming).
- Context: function pointers implement all three; prefer `Fn` for concurrency-friendly code.
- Avoid/Anti-pattern: over-constraining to `Fn` when mutation is required; capturing by move accidentally when borrowing suffices.

A.50 Iterator Contracts: size_hint and ExactSizeIterator
- Use when: consumers can benefit from pre-allocation or exact length.
- Context: implement `ExactSizeIterator` only when length is truly exact; expose meaningful `size_hint`.
- Avoid/Anti-pattern: relying on `size_hint` correctness in unsafe code; claiming `ExactSizeIterator` without invariants.

A.51 Panic Strategy per Profile
- Use when: balancing diagnostics (unwind) vs size/perf (abort) per binary profile.
- Context: `panic = 'abort'` for release CLIs; `unwind` for tests/debug; be explicit in `Cargo.toml`.
- Avoid/Anti-pattern: panics crossing FFI; assuming platform supports unwinding.

A.52 Cancel-Safe select! Patterns
- Use when: designing async multiplexing; ensure losing branches don’t own partial state.
- Context: encapsulate state in owner types (channels/buffers); use borrowed futures in `select!` arms.
- Avoid/Anti-pattern: storing partial buffers inside futures that may be dropped.

A.53 Collection Capacity Planning
- Use when: heavy pushes known; prefer `with_capacity` and amortized growth.
- Context: understand doubling strategies; avoid frequent reallocations in tight loops.
- Avoid/Anti-pattern: per-push reallocations; ignoring memory locality vs LinkedList.

A.54 BinaryHeap and Priority Queues
- Use when: need top-k, scheduling, or priority work queues.
- Context: `BinaryHeap` from Vec in O(n); beware ascending push pathological cases.
- Avoid/Anti-pattern: using `BTreeMap` to emulate a heap for max operations.

A.55 SSO and SmallVec Trade-offs
- Use when: small fixed upper bound per hot path; reduce heap traffic.
- Context: prefer `SmallVec<[T; N]>` with profiling; watch for spill costs.
- Avoid/Anti-pattern: assuming SSO always wins; ignoring spill hotspots like `spilled()`.

A.56 HashMap Implementation and Load Factors
- Use when: tuning map performance; understand SwissTable/hashbrown behavior.
- Context: expected O(1) with SIMD probing; tune hasher (aHash) only in non-adversarial contexts.
- Avoid/Anti-pattern: custom hashers in hostile inputs; mutating keys in-place.

A.57 IndexMap for Stable Insertion Order
- Use when: deterministic iteration order is required.
- Context: iteration over dense storage; removal semantics affecting order.
- Avoid/Anti-pattern: assuming order preservation after removals without `shift_remove`.

A.58 Pattern Matching: Guards and Ergonomics (2024)
- Use when: refine matches with guards; be explicit under Rust 2024 ergonomics.
- Context: avoid relying on implicit ref/mut; use `cargo fix --edition` for migrations.
- Avoid/Anti-pattern: ambiguous patterns relying on old match ergonomics.

A.59 FFI Layout and repr Choices
- Use when: interop and ABI stability required; choose `repr(C)`/`repr(transparent)` appropriately.
- Context: `Option<non-null>` for nullable pointer optimizations across FFI.
- Avoid/Anti-pattern: assuming Rust default layout across FFI; mixing unwinding across FFI.

A.60 Error Layering Revisited: ? and Context
- Use when: propagate errors ergonomically.
- Context: favor `?` and combinators; attach context at boundaries.
- Avoid/Anti-pattern: deep nested matches; panicking for recoverable errors.

A.61 HRTBs for Callback APIs (for<'a>)
- Use when: generic callbacks borrow data with any lifetime.
- Context: `for<'a> Fn(&'a T) -> U` in trait bounds; avoid leaking concrete lifetimes.
- Avoid/Anti-pattern: requiring 'static unnecessarily; encoding lifetimes as type params when HRTB suffices.

A.62 GATs for Streaming and Lending Patterns
- Use when: associated types depend on lifetimes (e.g., iterator yielding borrowed data).
- Context: use GATs to model lending iterators or parser cursors.
- Avoid/Anti-pattern: forcing owned allocations because associated outputs can’t borrow.

A.63 Dyn Compatibility and Object Safety
- Use when: designing traits for dynamic dispatch.
- Context: avoid methods using `Self` in returns or generics; ensure receivers are object-safe (`&self`, `&mut self`, `Box<Self>`, etc.).
- Avoid/Anti-pattern: non-dyn-compatible APIs in public traits; surprise `impl Trait` in returns.

A.64 Never Type (!) and Diverging APIs
- Use when: functions that never return (process abort, loop forever) or exhaustive matches.
- Context: use `!` for type-level modeling; improves control-flow typing.
- Avoid/Anti-pattern: encoding never-return via panics in library code without documenting.

A.65 Raw Identifiers and Edition Interop
- Use when: cross-edition compatibility requires using keywords as identifiers.
- Context: prefer `r#ident` to avoid breaking API surfaces.
- Avoid/Anti-pattern: renaming public items and causing churn when `r#` solves it.

A.66 Trait Bounds via where for Readability
- Use when: long bounds clutter signatures; prefer `where` for clarity.
- Context: move complex bounds off the main signature.
- Avoid/Anti-pattern: unreadable nested bounds inline.

A.67 Nullable Pointer Optimization in FFI
- Use when: model nullable pointers using `Option<NonNull<T>>` or `Option<extern "C" fn()>`.
- Context: leverage guaranteed representation for `Option<non-null>`.
- Avoid/Anti-pattern: using magic sentinel integers; UB with invalid values.

A.68 Deterministic APIs with IndexMap
- Use when: stable iteration order matters (caches, diffs, tests).
- Context: choose `IndexMap` for dense iteration; document order semantics.
- Avoid/Anti-pattern: relying on HashMap’s nondeterministic order.

A.69 BinaryHeap Top-k and Scheduling
- Use when: maintain rolling top-k or priority scheduling.
- Context: build from Vec in O(n); pop/push O(log n).
- Avoid/Anti-pattern: full sort when only top-k required.

A.70 Async Polling Semantics and Waker
- Use when: building executors or custom futures.
- Context: respect `Poll::Pending` contract; wake with `Waker` when progress possible.
- Avoid/Anti-pattern: busy-loop polling; waking without state change.

0A. WORKSPACE AND DEPENDENCY MANAGEMENT
--------------------------------
0A.1. Workspace-level dependency declaration for version consistency
0A.2. Module-level re-exports via lib.rs/mod.rs for clean public APIs
0A.3. Feature flags for optional dependencies
0A.4. Shared dependency versioning through workspace inheritance
0A.5. Path-based local dependencies for monorepo development
0A.6. Public API organization through prelude modules
0A.7. Conditional compilation with cfg attributes
0A.8. Dependency groups by feature sets
0A.9. Version compatibility through semver
0A.10. Cross-crate type sharing via workspace-common modules
0A.11. Clean Build Pattern
      - Regular execution of 'cargo clean' and './mach clean'
      - Clean before switching branches or major dependency changes
      - Clean when encountering mysterious build or dependency errors
      - Clean when updating workspace-level dependency configurations
      - Verify clean build state before running critical tests

1. OWNERSHIP AND BORROWING PATTERNS
----------------------------------
1.1. Clone-on-Write (Cow) for optional data ownership
1.2. Passing references instead of moving values
1.3. Using Arc for shared ownership in concurrent contexts
1.4. Implementing Clone selectively
1.5. Taking owned values in constructors
1.6. Borrowing in method arguments
1.7. Using Box<dyn Trait> for trait objects
1.8. Smart pointer patterns (Rc, Arc, Box)
1.9. Temporary ownership with mem::replace
1.10. Moving out of collections safely

2. ERROR HANDLING PATTERNS
-------------------------
2.1. Custom error types with thiserror
2.2. Using anyhow for application errors
2.3. Question mark operator chaining
2.4. Context addition with .context() or .with_context()
2.5. Custom Error type with From implementations
2.6. Result wrapping for fallible operations
2.7. Nested error handling with map_err
2.8. Error source chaining
2.9. Using Option for nullable values
2.10. Fallback patterns with unwrap_or_else

3. BUILDER PATTERNS
------------------
3.1. Builder pattern for complex object construction
3.2. Fluent interfaces
3.3. Default trait implementation
3.4. Type-state builders
3.5. Validate-before-build pattern
3.6. Optional field builders
3.7. Consuming builders
3.8. Generic builders
3.9. Builder with phantom types
3.10. Nested builders

4. RESOURCE MANAGEMENT
---------------------
4.1. RAII pattern
4.2. Drop trait implementation
4.3. Guard patterns
4.4. Cleanup in reverse order
4.5. Temporary resource allocation
4.6. Resource pools
4.7. Connection management
4.8. File handle management
4.9. Memory management patterns
4.10. Resource limitation patterns

5. CONCURRENCY PATTERNS
----------------------
5.1. Actor pattern
5.2. Message passing
5.3. Mutex guard pattern
5.4. RwLock patterns
5.5. Channel patterns (mpsc)
5.6. Thread pool implementations
5.7. Async/await patterns
5.8. Future combinators
5.9. Tokio runtime patterns
5.10. Parking_lot synchronization

6. TRAIT PATTERNS
----------------
6.1. Extension traits
6.2. Marker traits
6.3. Associated type patterns
6.4. Trait bounds composition
6.5. Conditional trait implementation
6.6. Sealed traits
6.7. Auto traits
6.8. Trait objects
6.9. Generic traits
6.10. Default trait implementations

7. TYPE SYSTEM PATTERNS
----------------------
7.1. Newtype pattern
7.2. Phantom data
7.3. Type-state programming
7.4. Zero-sized types
7.5. Marker types
7.6. Type-level programming
7.7. Generic type parameters
7.8. Associated types
7.9. Type aliases
7.10. Const generics

8. MEMORY OPTIMIZATION
---------------------
8.1. Small string optimization
8.2. Stack allocation preferences
8.3. Arena allocation
8.4. Memory pooling
8.5. Zero-copy parsing
8.6. Packed structures
8.7. Cache-friendly data layouts
8.8. Memory mapping
8.9. Custom allocators
8.10. Slice optimization

9. API DESIGN PATTERNS
---------------------
9.1. Into/From conversions
9.2. TryFrom/TryInto for fallible conversions
9.3. AsRef/AsMut traits
9.4. IntoIterator implementation
9.5. Display and Debug implementations
9.6. Visitor pattern
9.7. Command pattern
9.8. Factory pattern
9.9. Strategy pattern
9.10. Adapter pattern

10. MACRO PATTERNS
-----------------
10.1. Declarative macros
10.2. Procedural macros
10.3. Derive macros
10.4. Attribute macros
10.5. Function-like macros
10.6. Internal rule patterns
10.7. Recursive macros
10.8. Token manipulation
10.9. Custom syntax extensions
10.10. Hygiene patterns

11. TESTING PATTERNS
-------------------
11.1. Unit test organization
11.2. Integration test patterns
11.3. Property-based testing
11.4. Test fixtures
11.5. Mock objects
11.6. Parameterized tests
11.7. Benchmark patterns
11.8. Test utilities
11.9. Assert macro patterns
11.10. Test harnesses

12. SAFETY PATTERNS
------------------
12.1. Safe wrapper types
12.2. Bounds checking
12.3. Panic guards
12.4. Memory safety patterns
12.5. Thread safety patterns
12.6. Safe abstractions over unsafe code
12.7. Invariant maintenance
12.8. Permission systems
12.9. Capability patterns
12.10. Validation chains

13. PERFORMANCE PATTERNS
-----------------------
13.1. Zero-cost abstractions
13.2. Static dispatch
13.3. Dynamic dispatch optimization
13.4. Lazy initialization
13.5. Caching patterns
13.6. Batch processing
13.7. SIMD optimization
13.8. Memory prefetching
13.9. Lock-free algorithms
13.10. Compile-time computation

14. ASYNC PATTERNS
-----------------
14.1. Stream processing
14.2. Async trait patterns
14.3. Futures composition
14.4. Async resource management
14.5. Backpressure handling
14.6. Timeout patterns
14.7. Rate limiting
14.8. Circuit breaker pattern
14.9. Async initialization
14.10. Error propagation in async

15. COLLECTIONS PATTERNS
-----------------------
15.1. Custom iterators
15.2. Collection transformations
15.3. Efficient searching
15.4. Sorting strategies
15.5. Custom collection types
15.6. Thread-safe collections
15.7. Specialized containers
15.8. Index access patterns
15.9. Collection views
15.10. Cursor patterns

16. MODULE ORGANIZATION
----------------------
16.1. Public API design
16.2. Internal module structure
16.3. Feature flagging
16.4. Conditional compilation
16.5. Platform-specific code
16.6. Library organization
16.7. Dependency management
16.8. Version compatibility
16.9. Documentation organization
16.10. Example code structure

17. SERIALIZATION PATTERNS
-------------------------
17.1. Serde implementations
17.2. Custom serialization
17.3. Versioned serialization
17.4. Binary formats
17.5. Text formats
17.6. Schema evolution
17.7. Validation during deserialization
17.8. Efficient serialization
17.9. Format conversion
17.10. Type-driven serialization

18. NETWORKING PATTERNS
----------------------
18.1. Connection pooling
18.2. Protocol implementations
18.3. Async networking
18.4. Request/response patterns
18.5. Streaming protocols
18.6. Connection management
18.7. Retry mechanisms
18.8. Load balancing
18.9. Service discovery
18.10. Protocol buffers

19. FFI PATTERNS
---------------
19.1. C API wrappers
19.2. Memory management
19.3. Error handling
19.4. Callback patterns
19.5. Type conversion
19.6. String handling
19.7. Array handling
19.8. Function exports
19.9. Platform specifics
19.10. Safety boundaries

20. OPTIMIZATION PATTERNS
------------------------
20.1. Compile-time optimization
20.2. Runtime optimization
20.3. Memory optimization
20.4. CPU cache optimization
20.5. Algorithm selection
20.6. Data structure choice
20.7. Parallel processing
20.8. Resource pooling
20.9. Load distribution
20.10. Bottleneck elimination

21. ASYNC RUNTIME INTERNALS
--------------------------
21.1. Task scheduler implementation
21.2. Waker implementation patterns
21.3. Reactor patterns
21.4. Poll function optimization
21.5. Future state machines
21.6. Task queue management
21.7. Work-stealing schedulers
21.8. Timer wheel implementation
21.9. IO event notification systems
21.10. Task cancellation mechanisms

22. ZERO-COST ABSTRACTION PATTERNS
--------------------------------
22.1. Compile-time dispatch tables
22.2. Static virtual dispatch
22.3. Const generics optimization
22.4. Enum optimization patterns
22.5. Monomorphization strategies
22.6. Inline assembly integration
22.7. SIMD abstraction layers
22.8. Branch prediction hints
22.9. Memory alignment optimization
22.10. Dead code elimination patterns

23. ASYNC MIDDLEWARE PATTERNS
---------------------------
23.1. Tower layer implementation
23.2. Service trait patterns
23.3. Middleware chaining
23.4. Request/response transformation
23.5. Async interceptors
23.6. Filter chains
23.7. Middleware state management
23.8. Cross-cutting concerns
23.9. Conditional middleware
23.10. Middleware composition

24. RUNTIME REFLECTION PATTERNS
-----------------------------
24.1. Type ID manipulation
24.2. Dynamic type registration
24.3. Type metadata handling
24.4. Runtime type checking
24.5. Dynamic dispatch tables
24.6. Type erasure techniques
24.7. Trait object manipulation
24.8. Virtual method tables
24.9. Dynamic loading patterns
24.10. Type reconstruction

25. ADVANCED MACRO PATTERNS
-------------------------
25.1. Token tree manipulation
25.2. Macro hygiene management
25.3. Recursive macro expansion
25.4. Custom syntax parsing
25.5. Macro debugging patterns
25.6. Cross-platform macros
25.7. Conditional compilation
25.8. Code generation patterns
25.9. Macro export patterns
25.10. Macro documentation

26. ASYNC IO PATTERNS
-------------------
26.1. Zero-copy IO operations
26.2. Buffered IO abstractions
26.3. Async file operations
26.4. Network buffer management
26.5. IO completion ports
26.6. Scatter-gather IO
26.7. Direct memory access
26.8. IO uring integration
26.9. Async IO queues
26.10. IO prioritization

27. LOCK-FREE PATTERNS
--------------------
27.1. CAS operations
27.2. Memory ordering
27.3. Atomic reference counting
27.4. Lock-free queues
27.5. Wait-free algorithms
27.6. Memory barriers
27.7. ABA problem solutions
27.8. Lock-free data structures
27.9. Hazard pointers
27.10. Epoch-based reclamation

28. ASYNC STREAM PATTERNS
-----------------------
28.1. Back-pressure implementation
28.2. Stream buffering
28.3. Stream transformation
28.4. Stream composition
28.5. Stream splitting
28.6. Stream multiplexing
28.7. Stream rate limiting
28.8. Stream windowing
28.9. Stream error handling
28.10. Stream cancellation

29. PLATFORM ABSTRACTION
----------------------
29.1. OS API abstraction
29.2. System call wrapping
29.3. Platform-specific features
29.4. Conditional compilation
29.5. Feature detection
29.6. ABI compatibility
29.7. Cross-platform IO
29.8. Platform-specific optimization
29.9. Syscall abstraction
29.10. Platform capability detection

30. ADVANCED TYPE SYSTEM
----------------------
30.1. Higher-kinded types simulation
30.2. GATs implementation
30.3. Type-level computation
30.4. Type state machines
30.5. Dependent type patterns
30.6. Type-level integers
30.7. Type families
30.8. Associated type constructors
30.9. Type-level proofs
30.10. Type inference helpers

31. OPTION AND NULL SAFETY PATTERNS
--------------------------------
31.1. Combinators Over Matching
     - Use .map() when transforming Some values
     - Use .and_then() for chaining Option-returning operations
     - Use .or_else() for fallback computations
     - Use .unwrap_or_else() for lazy default values

31.2. Collection Operations
     - Use .filter_map() instead of filter().map()
     - Use .and_then() for flattening nested Options
     - Use .zip() to combine two Options

31.3. Early Returns and Guards
     - Return None early in functions
     - Use if let Some(x) for single-case matching
     - Chain .ok_or()/.ok_or_else() when converting to Result

31.4. Default Values
     - Use .unwrap_or(default) for simple defaults
     - Use .unwrap_or_else(|| expensive_computation()) for lazy defaults
     - Use .unwrap_or_default() for types implementing Default

31.5. Pattern Matching Best Practices
     - Match on multiple Options using tuple patterns
     - Use @ bindings to reference matched values
     - Prefer if let over match for single patterns

31.6. Option Construction
     - Use Some(val) explicitly for clarity
     - Use None::<Type> when type inference fails
     - Convert from nullable types using .map(|x| Some(x))

31.7. Composition Patterns
     - Chain .as_ref() for borrowing Option contents
     - Use .as_mut() for mutable borrowing
     - Combine with Result using .ok() and .transpose()

31.8. When to Use Each Pattern:
     ┌────────────────────┬──────────────────────────────────────┐
     │ Pattern            │ When to Use                          │
     ├────────────────────┼──────────────────────────────────────┤
     │ .map()             │ Transform Some value without nesting  │
     │ .and_then()        │ Chain operations that return Option   │
     │ .filter()          │ Conditionally keep Some values       │
     │ .or()/.or_else()   │ Provide fallback Options            │
     │ if let Some()      │ Single-case pattern matching        │
     │ match              │ Multiple cases or complex logic      │
     │ .unwrap_or()       │ Simple default values               │
     │ .unwrap_or_else()  │ Expensive default computations      │
     └────────────────────┴──────────────────────────────────────┘

31.9. Anti-patterns to Avoid
     - Avoid .unwrap() in production code
     - Don't use .expect() unless truly impossible
     - Avoid nested match statements on Options
     - Don't use if x.is_some() { x.unwrap() }

31.10. Testing Patterns
     - Use assert_eq!(Some(expected), result)
     - Test None cases explicitly
     - Use Option::as_ref() in assertions

32. ASYNC CHANNEL PATTERNS
------------------------
32.1. Multi-producer channels
32.2. Bounded channel implementation
32.3. Priority channels
32.4. Channel selection
32.5. Channel composition
32.6. Channel broadcasting
32.7. Channel filtering
32.8. Channel transformation
32.9. Channel monitoring
32.10. Channel cleanup

33. UNSAFE CODE PATTERNS
----------------------
33.1. Safe abstraction boundaries
33.2. Pointer manipulation
33.3. Raw memory management
33.4. FFI boundary safety
33.5. Undefined behavior prevention
33.6. Memory mapping safety
33.7. Platform-specific unsafe
33.8. Atomic operation safety
33.9. Exception safety
33.10. Invariant maintenance

34. ASYNC EXECUTOR PATTERNS
-------------------------
34.1. Task spawning
34.2. Executor shutdown
34.3. Task prioritization
34.4. Resource limits
34.5. Executor metrics
34.6. Task grouping
34.7. Executor composition
34.8. Thread pool management
34.9. Work stealing
34.10. Task locality

35. ADVANCED TRAIT PATTERNS
-------------------------
35.1. Trait specialization
35.2. Trait aliases
35.3. Trait composition
35.4. Negative trait bounds
35.5. Conditional trait impl
35.6. Trait object safety
35.7. Associated type defaults
35.8. Trait upcasting
35.9. Trait downcasting
35.10. Trait coherence

36. ASYNC NETWORKING PATTERNS
---------------------------
36.1. Protocol implementation
36.2. Connection management
36.3. TLS integration
36.4. Proxy patterns
36.5. Network timeouts
36.6. Connection pooling
36.7. Protocol negotiation
36.8. Network error handling
36.9. Keep-alive management
36.10. Connection backoff

37. COMPILE-TIME VALIDATION
-------------------------
37.1. Type-level constraints
37.2. Const evaluation
37.3. Static assertions
37.4. Build-time checks
37.5. Compile-time verification
37.6. Type system proofs
37.7. Const generics validation
37.8. Macro-time validation
37.9. Link-time optimization
37.10. Dead code detection

38. ASYNC STATE MANAGEMENT
------------------------
38.1. State machine implementation
38.2. Shared state access
38.3. State synchronization
38.4. State transition validation
38.5. State persistence
38.6. State recovery
38.7. State snapshot
38.8. State migration
38.9. State replication
38.10. State consistency

39. ADVANCED MEMORY PATTERNS
--------------------------
39.1. Custom allocator implementation
39.2. Memory pool management
39.3. Garbage collection
39.4. Reference counting
39.5. Memory fence patterns
39.6. Cache line optimization
39.7. Memory prefetching
39.8. Stack vs heap decisions
39.9. Memory compaction
39.10. Memory defragmentation

40. ASYNC TESTING PATTERNS
------------------------
40.1. Async test harness
40.2. Mock async services
40.3. Async assertions
40.4. Time manipulation
40.5. Race condition testing
40.6. Async property testing
40.7. Network simulation
40.8. Async benchmarking
40.9. Fault injection
40.10. Concurrency testing

41. LIBRARY API DESIGN
--------------------
41.1. Versioning strategies
41.2. Breaking change management
41.3. API stability guarantees
41.4. Feature flagging
41.5. Documentation generation
41.6. Error type design
41.7. Type system ergonomics
41.8. Builder pattern design
41.9. Extension trait design
41.10. Conditional compilation

Each of these patterns represents advanced techniques commonly used in building production-grade async Rust libraries like Tokio and Axum. They focus on performance, safety, and maintainability while providing powerful abstractions for users.
