# A005: Reasoning with ISG (CPU-only) — Making LLDs LLM-Friendly

Purpose
Answer: Can an LLM reliably reason through a low-level design (LLD) using Parseltongue’s ISG? Yes—if we combine structural edges (from syntax) with minimal “interface context” signals and expose compact deltas over stdio. This stays CPU-only, deterministic, and incremental.

References
- See A003_StructuralPatternISG.md for structural rule lessons and YAML examples
- See A004_InterfaceContextISG.md for NodeExtras, signals, delta packet, and endpoints

Essence (Answer)
- Make RustRover analysis JVM-first and legally safe, adding targeted native helpers only where they deliver measurable wins; evolve Parseltongue into a Rust-only, Zed-style incremental indexer and lightweight query server with a few high-ROI improvements.
- Prioritize five pragmatic changes now (incremental parsing, rope+interner, ignore-aware scanning, debounced workers, stdio JSON-RPC), measure with hard KPIs, and defer heavy reverse engineering or broad binary ingestion unless proven necessary.
- For LLM reasoning on LLDs, enrich the ISG with minimal, CPU-only interface context and structural rules so the model can reason via graph structure and compact deltas—no macro expansion, no GPU, no network.

Key Messages (Layer 1)
- Pivot RustRover deconstruction to JVM/plugin analysis; validate performance via runtime instrumentation; respect licensing.
- Use native helpers as bounded accelerators (demangling, DWARF, object scanning) with strict FFI contracts and black-box validation.
- Specialize Parseltongue for Rust; adopt Zed-style incremental parsing, scheduling, and editor-facing stdio service.
- Implement five “do-it-now” improvements backed by simulations; add batch APIs and metrics if time permits.
- Prefer realistic, phase-specific KPIs over blanket claims; lock reproducibility.
- Zed is better for keystroke-latency on large files; RustRover excels at persistent indices—adopt selectively.
- Enrich ISG with structural pattern rules (ast-grep-style) and interface context (A003/A004) for LLM-friendly LLDs.

Mermaid: End-to-end flow (CPU-only)
```mermaid path=null start=null
flowchart LR
  E[Edit] --> P[Incremental parse (tree-sitter)]
  P --> H[Signature harvest (syn)]
  H --> C[Canonicalize → api_digest + shape_digest]
  P --> R[Rule matches (ast-grep-style)]
  R --> F[Emit edges (Calls/Implements/Uses + tags)]
  C --> X[NodeExtras store]
  F --> G[ISG upsert]
  X --> D[Delta packet builder]
  G --> D
  D --> L[LLM / tools via stdio JSON-RPC]
```

Supporting Arguments (Layer 2)
- JVM-first RustRover analysis
  - Most value is in Kotlin/Java bytecode, plugin.xml, and IntelliJ’s extension points.
  - Static-only unpacking misses dynamic activation; add controlled runs with JFR/async-profiler.
  - Replace speculative labels with invariant tests (extension points, services, plugin IDs, classpaths).
- Native helpers (Hybrid Plan 2.0)
  - Accelerate compute-heavy edges (demangling, DWARF parsing, object metadata) via batched FFI.
  - Define ABI/versioning, buffer ownership, threading limits; compact binary outputs; graceful JVM fallbacks.
- Parseltongue Rust-only specialization
  - Simpler model → higher fidelity on traits/impls/macros/cfg; cargo_metadata improves workspace accuracy.
  - Stronger queries (implementors, callers, where-used, trait bounds) with stable IDs, optional enrichment behind a flag.
- Zed-inspired architecture
  - Tree-sitter incremental edits + rope storage lower update latency and memory.
  - Debounced, bounded worker queues reduce jitter; stdio JSON-RPC unlocks editor integrations.
  - .gitignore-aware scanning cuts noise; batch endpoints + bincode reduce IPC overhead.
- Measurement, reproducibility, legality
  - Use JFR/async-profiler for JVM; perf/Instruments for native; pin versions.
  - Respect JetBrains EULA; rely on OSS sources and runtime introspection.

Evidence and Examples (Layer 3)
- Selective extraction: focus on lib/*.jar, plugins/**/META-INF/plugin.xml; avoid full inflates.
- JVM/plugin analysis: parse plugin.xml, manifests, services; jdeps for dependency surface; invariant tests for EPs/IDs.
- Runtime instrumentation: JFR for startup/indexing; async-profiler for CPU/alloc; IDE diagnostics.
- Native helper contracts: demangle_batch, dwarf_summary, object_summary; direct buffers, versioned symbols.
- Zed-style components: tree-sitter queries for functions/traits/impls/use; ropey buffers; ignore::WalkBuilder; stdio JSON-RPC server.

Priority Actions and Tradeoffs (Decisions)
| Action | Why (benefit) | Effort | Risk | Verdict |
|---|---|---|---|---|
| Incremental parsing (tree-sitter + InputEdit) | ~80% cut in re-parse; smoother typing on large files | 3–4 d | Low | Do it |
| Rope buffers + interner | ~30% lower RSS; faster splice math | 2 d | Low | Do it |
| .gitignore + cargo_metadata scanning | 6× fewer files indexed; faster cold ingest | 1 d | Negligible | Do it |
| Debounced worker queue | Lower jitter; p95 updates < 5 ms | 1 d | Low | Do it |
| Stdio JSON-RPC (LSP-lite) | Editor integration; batch queries | 2 d | Medium | Do it |
| Batch endpoints + bincode | 2–4× IPC throughput | 1 d | Low | If time |
| Metrics (counters/timers) | Catch regressions; CI gates | 1 d | Low | If time |

Timeline Snapshots (Layer 4)
- 5–7 day Parseltongue sprint
  - Days 1–2: ignore-aware scan + cargo workspace loader; rope + interner baseline.
  - Days 3–4: tree-sitter incremental parsing wired; debounced worker queue.
  - Day 5: Stdio JSON-RPC with what-implements, who-calls, blast-radius (+ batch variants).
  - Day 6: bincode transport; metrics; CI perf gates.
  - Day 7: Perf polish and docs.
- 10-day Hybrid Plan integration (native helpers)
  - Days 1–4: FFI verbs + JNI bridge; demangle_batch and object_summary; per-OS artifacts.
  - Days 5–8: Benchmarks vs JVM-only; add DWARF summarize; JFR markers; observability.
  - Days 9–10: Packaging security (hash/sig), fallbacks, Stability Week prep.

Comparative Perspective (Layer 4)
- Zed: superior for keystroke-latency on very large files via incremental parsing/scheduling.
- RustRover: better persistent stub index but heavy; overkill for a lean service.
- Adopt Zed’s low-risk techniques; borrow RustRover’s measurement rigor.

Risks and Mitigations (Layer 5)
- Legal/EULA: avoid decompilation/redistribution; prefer OSS sources and runtime introspection.
- JNI/FFI overhead: batch aggressively; direct buffers; per-call counters; fallbacks.
- Cross-platform: signed artifacts, minimum glibc, MSVC static CRT; smoke tests per OS/arch.
- Data size/memory: stream DWARF; compact binary summaries; enforce pool/thread limits.

Metrics for Success (Layer 5)
- Update latency: p95 per edited file < 5 ms (goal), < 12 ms (floor).
- Query latency: p95 < 1 ms for core queries; batch throughput targets documented.
- Ingest throughput: cold ingest reductions from ignore-aware scan; reproducible across OS images.
- Native helper ROI: demangling ×50–150; DWARF 30–150 MB/s; end-to-end feature deltas 10–30% (JFR-verified).
- Stability: deterministic IDs; cross-platform load success; zero crash regressions in CI.

What Parseltongue can learn from ast-grep (and friends) for ISG (A003 summary)
- Structural rules as first-class fact sources; each match emits typed ISG edges with stable IDs and spans.
- Metavariables + constraints to cut noise; rule packs as productized knowledge; explain strings for confidence.
- Incremental matching on dirty buffers; batch IPC with compact payloads.

Minimal CPU-only signals for LLM reasoning (A004 summary)
- Signals: api_digest + shape_digest; vis/flags; trait contract map; derives→Implements; UsesType; error surface; side-effects/concurrency/unsafe/FFI footprint; cfg_active; re-export/alias; centrality/complexity hints.
- Delta packet: compact (~≤1–2 KB) per changed node with node/api/effects/coupling/trait_contract/doc/change.
- Endpoints: graph.delta.summarize, node.describe, risk.rank, node.find, rules.apply/list/stats.

Why this helps an LLM
- Structure dominates labels: edges and type shapes remain meaningful even if names are odd.
- Identity is name-agnostic: digests, canonical paths, spans; names become presentation-only.
- Compact deltas let the LLM reason about breakage, blast radius, and test plans without reading entire files.

High-Quality Notes: Behavioural hints (CPU-only)

### Goal  
Give an LLM enough behavioural hints (not just type-level signatures) so it can answer questions like  
“will `foo()` always commit the DB?” or “is there an early-return that skips validation?”—while staying 100 % CPU-only and incremental.

Below is a “cheapest-first” menu of extra facts you can bolt onto the current Interface-Signature-Graph. Every line is already extracted by tools such as rust-analyzer’s CFG builder or the `rustc` MIR dump — no SAT solvers, no GPUs, no foreign servers.

----

#### 1 – Harvest control-flow fingerprints (function-level)

| Extra bit | Why an LLM cares | How to get it (≤ 2 ms/function) |
|---|---|---|
| `cyclomatic` u8 | branchiness / test surface | count `if/else`, `match`, `loop`, `&&/||` with a syn visitor |
| `has_loop` bool | possible non-termination | detect `loop` / `while` tokens |
| `early_return` bool | skipped clean-up risk | scan for `return` outside last block |
| `exit_kinds` bitflags (`OK`, `Err`, `None`, `Panic`) | error-path reasoning | walk MIR/CFG terminators |
| `await_cnt`, `unsafe_cnt` u8 | async & safety audit | simple token scan (tree-sitter) |
| `may_panic` bool | reliability | detect `panic!`, `.unwrap`, `.expect`, `todo!` |
| `loop_bound_hint` (0 = none, >0 best guess) | complexity | heuristic on literal loop limits |

Add these as a 32-bit bit-field inside `NodeExtras`; no new edges needed.

----

#### 2 – Tiny intra-function CFG thumbprint

If you can afford ~1 KiB extra per function:

1. Use rust-analyzer’s `control_flow` crate (or port its ~1k LOC) – it builds a Basic-Block graph directly from the unexpanded HIR.
2. Hash the shape of that CFG (sorted adjacency list, ignoring spans) into `cfg_hash: u64`.  
   • Two versions of a function that only reorder statements get the same `cfg_hash` → zero-noise “did-logic-change?” signal.  
   • LLM can diff hashes to decide whether to read the body.

Optional: expose a lazy-loaded `cfg_json` (Vec<{block_id, succs}>) on demand.

Cost: ~2–3 µs per unchanged function after incremental hashing.

----

#### 3 – Data-flow stubs (super-cheap)

| EdgeKind | From → To | Emit when |
|-------------------------|-----------|-----------|
| `WritesField` | fn/method → struct.field | any `member =` assign in body |
| `ReadsField` | fn/method → struct.field | dot access on RHS |
| `MutatesSelf` (edge flag) | method → self | body contains `self.` with `&mut` |

Extraction with a regex over tokens is good enough; false-positives are fine—LLM only needs “possible”.

----

#### 4 – Doc-driven intent

Already parse first doc-line; add:

```rust path=null start=null
// purpose tags extracted from docs
// e.g., ["serialize", "cache", "retry"]
purpose_tags: &[Arc<str>]
```

Simple unigram matching on verbs/nouns (<200 keywords) gives surprising mileage.

----

#### 5 – Source-of-truth inspirations

| Project / crate | Why copy it | What to cherry-pick |
|---|---|---|
| rust-analyzer (`ra_ap_*`) | Incremental CFG builder & loop/panic detection already written | `control_flow`, `hir_def::find_loop_bodies` |
| Clippy lints | Many single-file data-flow scans that run in microseconds | `nearly_cyclomatic_complexity`, `panic_unwrap_macro` |
| `cargo +nightly rustc -- -Zunpretty=mir-cfg` | MIR CFG for free without full LLVM | Parse text, compute `exit_kinds`, `may_panic` |
| cargo-geiger | Proof that scanning for `unsafe` & `extern` costs ~1–2 ms/crate | reuse its visitor |

All of them compile to pure CPU libs; link statically and call per file.

----

### Minimal schema diff

```rust path=null start=null
pub struct NodeExtras {
    // existing …
    pub api_digest: u64,

    // NEW --------------------------------
    pub ctrl_bits: u32,   // layout: [0-3 cyclo][4 has_loop][5 early_ret]…
    pub cfg_hash: u64,    // 0 = not calculated
    pub await_cnt: u8,
    pub unsafe_cnt: u8,
    pub doc_tags: Arc<[Arc<str>]>,
}
```

Edge additions (`WritesField`, `ReadsField`) are optional; flag them with a new 3-bit “Data” segment in `EdgeKind`.

----

### Pay-off

• 150–300 bytes extra per function → LLM can:

1. Decide which function bodies to read (cfg hash diff).  
2. Infer error propagation (“returns `Result` but `early_return & exit.Err = true`”).  
3. Warn on missing clean-up (“has_loop && may_panic”).  
4. Summarise modules: “90 % of funcs are pure (no writes / no side-effects)”.

All with zero GPU and <10 ms incremental overhead for a 10 k-function crate.

Bottom Line
- Yes—ISG enables reliable, CPU-only LLD reasoning when augmented with structural rules and minimal interface context.
- Ship the five high-ROI infra changes in a week, enforce KPIs, and expand only where measurable wins exist.
