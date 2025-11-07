# What Parseltongue Can Learn from ast-grep for ISG

*Adopting structural patterns to turn robust tree-sitter matches into ISG facts*

## The Core Insight

**Short version**: adopt ast-grep's structural-pattern approach to emit ISG facts. Use a small ruleset to turn robust tree-sitter matches into new nodes/edges (Implements, Calls, Uses, Derives, Re-exports, Trait bounds). Keep it incremental, batch the matches, and measure latency.

## 1. ast-grep Lessons for ISG Fact Generation

### Structural Rules as First-Class Fact Sources
- Treat each ast-grep rule as an "edge generator"
- A match yields a typed ISG edge with stable IDs and spans
- Moves correctness from hand-rolled visitors into declarative rules

### Metavariables + Constraints to Cut Noise
- Use captures and "where/contains/has/eq/regex/not" constraints
- Disambiguate trait bounds, method vs free function calls, derive vs manual impl

### Rule Packs as Productized Knowledge
- Maintain a small, curated Rust rule-pack (YAML) for ISG construction
- Version and ship rules; users can toggle rules per workspace

### Spans and "Explain" for Confidence
- Persist match ranges and "explain" string of why a rule matched
- Provides debuggability when edges look wrong

### Incremental Matching Over Dirty Buffers
- Cache compiled rules, only re-run over changed files
- Reuse tree-sitter edits to keep p95 update < 5ms

### Batch IPC and Compact Payloads
- Return matches as compact binary (bincode) with `{rule_id, captures, span}`
- Convert to edges in-process to avoid JSON overhead on hot paths

## 2. Example Rules for Direct ISG Enrichment

Below are illustrative ast-grep-style YAML rules that would emit specific ISG edges:

```yaml
# Implements: Struct -> Trait from impl blocks
- id: rust.impls.trait
  language: Rust
  message: "Implements edge: $Type -> $Trait"
  rule:
    pattern: "impl $Trait for $Type { ... }"
  on-match:
    emit-edge:
      kind: Implements
      from: "$Type"     # node name for Struct/Type
      to: "$Trait"      # node name for Trait
    spans:
      impl_block: "$$"  # whole match

# Derive: Struct -> Trait from #[derive(...)]
- id: rust.derive.trait
  language: Rust
  rule:
    pattern: "#[derive($Trait)] struct $S { ... }"
  on-match:
    emit-edge:
      kind: Implements
      from: "$S"
      to: "$Trait"

# Uses: Function -> Type via params/return
- id: rust.uses.type.in.signature
  language: Rust
  rule:
    any:
      - pattern: "fn $F($p: $T, ..) { ... }"
      - pattern: "fn $F(..) -> $T { ... }"
  on-match:
    emit-edge:
      kind: Uses
      from: "$F"
      to: "$T"

# Calls: Function -> Function (free call)
- id: rust.calls.free
  language: Rust
  rule:
    pattern: "fn $Caller(..) { .. $Callee(..) .. }"
  constraints:
    # Optional: exclude macros, ensure Callee is an identifier path
    has:
      inside: "$Caller"
  on-match:
    emit-edge:
      kind: Calls
      from: "$Caller"
      to: "$Callee"

# Calls: Function -> Method (obj.method(..))
- id: rust.calls.method
  language: Rust
  rule:
    pattern: "fn $Caller(..) { .. $recv.$name(..) .. }"
  on-match:
    # Heuristic: bind $name to a function node "Type::name" if resolvable,
    # else emit Calls edge to a synthetic method node "method_$name"
    emit-edge:
      kind: Calls
      from: "$Caller"
      to: "method::$name"

# Re-export: Module -> Symbol (pub use)
- id: rust.reexport
  language: Rust
  rule:
    pattern: "pub use $path::$Name;"
  on-match:
    emit-edge:
      kind: Uses
      from: "$path"
      to: "$Name"

# Trait bound Uses: Type -> Trait (where/param bounds)
- id: rust.uses.trait.bound
  language: Rust
  any:
    - pattern: "fn $F<$T: $Trait>(..){..}"
    - pattern: "impl<$T: $Trait> $Thing { .. }"
    - pattern: "where $T: $Trait"
  on-match:
    emit-edge:
      kind: Uses
      from: "$T"
      to: "$Trait"
```

### Notes on Implementation
- **Map captures to stable node IDs** with a name→ID resolver:
  - Prefer a 128-bit SigHash built from (lang, crate, module path, canonical signature)
  - Fall back to deterministic synthetic IDs for unresolved method targets
- **Store spans, file, and rule_id** on the edge for auditability

## 3. What to Borrow from Other Tools

| Tool | What to borrow | How to apply in Parseltongue (ISG) | KPI/Guardrail |
|---|---|---|---|
| ast-grep | Structural rule DSL, captures, constraints, explain | Embed a small rule engine to generate edges from matches; ship a Rust rule-pack; persist spans and rule_id on edges | p95 update < 5ms; false-positive rate < 5% on golden repos |
| semgrep | Multi-rule packs, "pattern-not/inside", taint configs | Add negative/inside constraints; optional taint edges (Source→Sink) for blast-radius variants | No >10% memory regression; rule latency variance < 2× |
| tree-sitter queries | Fast incremental queries, capture names | Reimplement hottest rules with TS queries; reuse InputEdit to only rematch dirty subtrees | Re-parse amortized < 2ms per edit |
| ripgrep/ignore | Ignore-aware project scan | Use ignore::WalkBuilder across daemon and batch ingest; align with .gitignore | 5–6× fewer files scanned cold; stable across OS |
| ctags/universal-ctags | Cheap name index | Maintain a name→approximate-def map for quick "find by name" and candidate linking during edge creation | Name lookup p95 < 200µs |
| rust-analyzer (optional) | HIR/type for harder links | Gate behind a flag to improve Calls/impl resolution and trait bounds accuracy | End-to-end improvement >10% accuracy or disable |

## 4. Minimal Integration Plan (1 week)

### Day 1: Rules Engine Skeleton
- Add `isg-rules` crate: load/validate YAML, compile to in-memory rules with captures and constraints
- Wire into daemon ingest/update pipeline after parse

### Day 2: Core Rust Rules (v1)
- Implements, Derive, Uses-in-signature, Calls (free/method), Re-export, Trait-bounds
- Build name→ID resolver; synthesize IDs when unresolved

### Day 3: Incremental Matching
- Use tree-sitter incremental edits (or file-level debounced re-run initially)
- Batch matches per file; convert to edges in-memory; bincode snapshot for tests

### Day 4: JSON-RPC "rules.apply" + Metrics
- Expose stdio JSON-RPC methods: `rules.list`, `rules.apply(file?)`, `rules.stats`
- Counters: matches/sec, edges/emitted, time per rule, rejects by constraint

### Day 5: Golden Tests + Perf Gates
- Add golden repos (small crates) to CI. Assert edge deltas and latency budgets
- Property tests for SigHash stability and spans

### Day 6–7: Polish and Docs
- Explain view on edges; rule pack versioning; examples for extending rules

## 5. KPIs and Validation

### Performance Targets
- **Update latency**: p95 per edited file < 5ms
- **CPU usage**: Hot path dominated by tree-sitter, not YAML parsing
- **Match throughput**: ≥ 50k matches/s on mid-size projects for cached/compiled rules

### Accuracy Improvements
- **Implements edges**: +100% vs heuristic baselines (impl/derive/trait bounds)
- **Calls edges**: +10–20% correctly linked free/method calls on golden sets (without rust-analyzer)

### Stability Requirements
- **SigHash deterministic** across runs
- **Spans round-trip** in snapshots
- **Zero crashes** in CI

### Resource Overhead
- **RSS**: +≤ 30% vs baseline
- **Cold-start**: No more than +5ms rule load time

## 6. Risks and Guardrails

### False Positives/Negatives
- **Mitigate with**: Constraints and "pattern-not/inside"
- **Keep rules**: Small and composable
- **Add**: "Explain" strings and per-rule counters

### Macro and Type Resolution Limits
- **Tag**: Macro-origin nodes
- **Gate**: Hard links behind optional rust-analyzer
- **Otherwise**: Emit provisional edges with explain to avoid blocking

### Performance Regressions
- **Pre-compile**: Rules for reuse
- **Batch**: Matches to avoid JSON on hot paths
- **Add**: Perf gates in CI

### Compatibility and Licensing
- **Use**: OSS-friendly components only
- **Ship**: Your own rule-pack
- **Document**: The contract (rule → edge)

## Bottom Line

Adopt ast-grep's structural rule model to generate ISG edges deterministically and incrementally. Start with a half-dozen high-ROI Rust rules (impl/derive/uses/calls/re-export/trait-bounds), keep payloads compact, expose an explainable API, and enforce tight KPIs.

**Key borrowings**:
- **semgrep's constraint patterns**
- **ripgrep's ignore discipline** 
- **tree-sitter's incremental edits**
- **rust-analyzer gated behind accuracy improvements >10%**

This represents a concrete, high-value optimization path that builds on Parseltongue's existing strengths while learning from the broader ecosystem's best practices.

---

*Technical architecture analysis showing exactly which optimizations are worth doing, complementing the "when to stop optimizing" strategic framework.*