# ISG Interface Context (CPU-only): High-ROI Enrichment Plan

Short answer
Yes. Enriching the ISG with “interface context” makes implementors/callers/impact queries materially more useful, without leaving CPU-only territory. You can extract this context deterministically from syntax and attributes using syn and tree-sitter, no GPU, no network, no reverse engineering.

What to add (high ROI, CPU-only)
- Public API surface
  - Visibility, item kind, module path, canonical name (crate::mod::Type).
  - Normalized signatures for functions/methods/traits (params, return).
  - Generics, lifetimes, where-clause bounds.
- Trait-specific context
  - Required vs default methods, associated types/constants, supertraits.
- Impl context
  - Impl target mapping (Type<Args> for Trait<Path<Args>>), fulfilled methods, where-clause.
- Type usage context
  - Types referenced in signatures and fields; re-exports (pub use …), derives (#[derive(X)]).
- Configuration and safety
  - cfg/cfg_attr gating (feature/platform), unsafe/async/const/extern ABI flags.
- Documentation fingerprints
  - First line summary and a small hash of doc comments for change detection (optional).
- API surface digest
  - Stable hash of an item’s public signature to power cheap impact diffs.

CPU-only extraction cookbook
- Parser choice
  - Use tree-sitter for incremental edits, file structure, and cheap matches.
  - Use syn to parse Rust signatures/entities where you need exact tokens (Type, Generics, WhereClause); still CPU-only.
- Canonicalization
  - Normalize paths (crate::… form), whitespace, lifetime renaming (’a → L0, ’b → L1), sort where-bounds for determinism.
- Bounds and generics
  - From syn::Generics and syn::WhereClause, collect trait bounds and emit Uses edges Type → Trait for bounds.
- Derives and re-exports
  - #[derive(Clone, Debug)] → Implements edges Struct → Clone/Debug; keep derive=true flag on edge.
  - pub use x::y as Z; → Reexports edge Module → Symbol; also bind canonical name for Z.
- Associated items
  - Trait: gather required methods, default methods, associated types/consts; emit Contains edges Trait → Method/AssocType and Requires edges Trait → Method(required=true).
  - Impl: map to trait and mark Satisfies edges Impl → TraitMethod for provided methods; DefinesAssocType edges.
- Type usage
  - From function/method/struct-field signatures, emit UsesType edges Function/Struct → Type.
- cfg gating
  - Use cfg-expr to evaluate under a provided feature/target triple; store a bitmask (active/inactive) on nodes/edges.
- Doc and digest
  - Take first doc line (trimmed) and a 64-bit xxhash of full doc; compute api_digest: xxhash of normalized public signature. Store to extras to power fast “what changed” queries.

Minimal schema additions
- Add a compact extras payload to NodeData (or parallel store) with interning + bitflags to keep RSS low:

```rust path=null start=null
#[repr(C)]
pub struct NodeExtras {
    pub vis: u8,                 // public/pub(crate)/private
    pub flags: u32,              // bitflags: unsafe/async/const/extern/no_std/etc.
    pub cfg_active: bool,        // evaluated under current feature set
    pub api_digest: u64,         // stable hash of normalized signature
    pub module_path: Arc<str>,   // crate::a::b
    pub generics: Arc<[Arc<str>]>,
    pub where_bounds: Arc<[Arc<str>]>,
    pub derives: Arc<[Arc<str>]>,
    pub doc_first_line: Option<Arc<str>>,
    pub doc_hash: Option<u64>,
}
```

### Control-flow bit layout (ctrl_bits)

| Bit(s) | Meaning |
|---|---|
| 0–3 | cyclomatic (0–15) |
| 4 | has_loop |
| 5 | early_return |
| 6 | exit_ok |
| 7 | exit_err |
| 8 | exit_none |
| 9 | exit_panic |
| 10–17 | await_cnt (compressed 0–255) |
| 18–25 | unsafe_cnt (compressed 0–255) |
| 26–31 | reserved |

Notes:
- Keep await_cnt and unsafe_cnt also as explicit u8 fields in NodeExtras for readability; ctrl_bits packs a compressed copy for fast filtering.
- exit_kinds are represented as bitflags in storage; expose as a string array in JSON APIs.

Decode helper (Rust, illustrative)

```rust path=null start=null
#[derive(Debug, Default, Clone, Copy)]
pub struct CtrlBits {
    pub cyclomatic: u8,
    pub has_loop: bool,
    pub early_return: bool,
    pub exit_ok: bool,
    pub exit_err: bool,
    pub exit_none: bool,
    pub exit_panic: bool,
    pub await_cnt: u8,
    pub unsafe_cnt: u8,
}

pub fn decode_ctrl_bits(bits: u32) -> CtrlBits {
    CtrlBits {
        cyclomatic: ((bits >> 0) & 0b1111) as u8,
        has_loop: ((bits >> 4) & 1) != 0,
        early_return: ((bits >> 5) & 1) != 0,
        exit_ok: ((bits >> 6) & 1) != 0,
        exit_err: ((bits >> 7) & 1) != 0,
        exit_none: ((bits >> 8) & 1) != 0,
        exit_panic: ((bits >> 9) & 1) != 0,
        await_cnt: ((bits >> 10) & 0xFF) as u8,
        unsafe_cnt: ((bits >> 18) & 0xFF) as u8,
    }
}
```

- Extend EdgeKind (or add a typed “EdgeTag” with compact u8) to avoid enum bloat:

| Tag | Meaning | Example emission |
|---|---|---|
| Contains | container → member | Trait → Method, Struct → Field |
| Requires | trait requires method | Trait → Method(required) |
| DefinesAssocType | impl/trait defines assoc type | Impl/Type → AssocType |
| Reexports | module re-exports symbol | Module → Symbol |
| UsesType | signature/field references type | Function → Type |
| Derives | auto-impl via #[derive] | Struct → Clone (derive=true) |

Note: keep Calls/Implements/Uses as today; add these as additional tags. Use bitflags on edges for attributes (derive, cfg_active).

How to implement (CPU-only), step-by-step
1) Incremental parse
   - Keep tree-sitter Tree updated via InputEdit; on file change, re-run matchers only for affected subtrees.
2) Signature harvesting (syn)
   - For changed items, parse syn::ItemFn/ItemTrait/ItemImpl/ItemStruct, get Type/Generics/WhereClause, normalize to canonical string; compute api_digest.
3) Emit facts
   - From AST and attrs: emits for Derives, Reexports, UsesType, Contains, Requires, DefinesAssocType, Implements.
4) Resolve names deterministically
   - Canonical FQ name = crate_name + module_path + item_name; stable SigHash from normalized signature; fall back to synthetic method IDs for unresolved receivers (reconcile later).
5) cfg evaluation
   - Evaluate cfg-expr per item/edge with provided features/target; mark inactive edges/nodes so queries can filter.
6) Persistence + metrics
   - Store NodeExtras in a side-store keyed by SigHash; attach spans/rule_ids for auditability; log match counters and timings.

Tiny code sketch (syn-only signature context)
```rust path=null start=null
fn enrich_fn(item_fn: &syn::ItemFn, crate_name: &str, module_path: &str) -> NodeExtras {
    use syn::{ReturnType, TypeParamBound};
    let vis = match &item_fn.vis { syn::Visibility::Public(_) => 1, syn::Visibility::Restricted(_) => 2, _ => 0 };

    let mut params: Vec<String> = Vec::new();
    for arg in &item_fn.sig.inputs {
        if let syn::FnArg::Typed(pat_ty) = arg {
            params.push(normalize_type(&pat_ty.ty));
        }
    }
    let ret = match &item_fn.sig.output {
        ReturnType::Default => "()", 
        ReturnType::Type(_, ty) => &normalize_type(ty),
    }.to_string();

    let where_bounds = item_fn.sig.generics.where_clause
        .iter()
        .flat_map(|w| w.predicates.iter().map(|p| normalize_predicate(p)))
        .collect::<Vec<_>>();

    let api_sig = format!(
        "pub fn {}::{}{}({}) -> {} where {}",
        crate_name, module_path, item_fn.sig.ident,
        params.join(", "),
        ret,
        where_bounds.join(" && ")
    );
    NodeExtras {
        vis,
        flags: pack_flags(&item_fn.sig), // async/const/unsafe/extern
        cfg_active: true,
        api_digest: xxhash64(api_sig.as_bytes()),
        module_path: Arc::from(module_path),
        generics: intern_generics(&item_fn.sig.generics),
        where_bounds: intern_vec(where_bounds),
        derives: Arc::from([]),
        doc_first_line: extract_doc_first_line(&item_fn.attrs),
        doc_hash: doc_hash(&item_fn.attrs),
    }
}
```
Notes:
- normalize_type turns syn::Type into a stable, whitespace-free, lifetimes-renamed string.
- Keep everything CPU-bound; no macro expansion required.

KPIs and guardrails
- Latency: p95 update per edited file < 5 ms (rules + syn on changed items only).
- Memory: NodeExtras <= ~96 bytes avg; total RSS +≤ 30% vs baseline.
- Accuracy: +100% Implements (derive/impl/bounds) vs heuristic baseline; +10–20% correct Calls linkage with method heuristics.
- Determinism: api_digest stable across platforms; cfg evaluated consistently from provided features/target triple.
- CI gates: assert p95 update, RSS cap, and golden-edge counts; property tests for canonicalization stability.

Phased add (3 days)
- Day 1: NodeExtras type; syn harvesting for Fn/Struct/Trait; api_digest; UsesType from signatures.
- Day 2: Derives, Reexports, Contains/Requires/AssocType edges; cfg-expr support.
- Day 3: JSON-RPC: node.describe, api.surface, diff.interface; metrics and golden tests.

Bottom line
Enrich the ISG with interface context taken from syntax, attributes, and cfg—no GPUs, no network, no reverse engineering. Use tree-sitter for incremental scope and syn for precise signature extraction; attach compact NodeExtras and a few new edge tags. You’ll unlock better “what-implements,” “who-calls,” “blast radius,” and stable “what changed” answers while keeping p95 update latency under 5 ms.

Evaluation against current Parseltongue (2025-10-13)
- What’s already in code
  - syn-based Rust parsing is present (src/parsers/rust.rs uses syn::parse_file and ItemFn/ItemStruct/ItemTrait/ItemImpl).
  - ISG has EdgeKind with Calls/Implements/Uses and stable SigHash/NodeData plumbing; upsert_edge and tests exist (performance_contract_tests.rs, isg.rs).
- Gaps this plan fills
  - No NodeExtras side-store for interface context (vis, flags, module_path, generics, where-bounds, derives, doc, api_digest).
  - Edge taxonomy is minimal; missing Contains, Requires, DefinesAssocType, Reexports, UsesType, Derives with attributes/bitflags.
  - No cfg/cfg_attr evaluation pipeline; no doc fingerprinting or stable API digests for cheap impact diffs.
  - No incremental tree-sitter path yet (optional for Day 1; add later for p95 < 5ms under heavy edits).
- Recommended next steps (mapped to code)
  - Add NodeExtras and an extras store keyed by SigHash alongside NodeData in isg.rs (or a parallel module); keep it compact with interning.
  - Extend parsers/rust.rs to harvest generics/where-bounds/signatures and compute a canonical signature + api_digest (use xxhash; e.g., the xxhash-rust crate or twox-hash).
  - Emit UsesType edges from signatures immediately; later add Derives and Reexports from attrs/items.
  - Introduce EdgeTag/bitflags or extend EdgeKind carefully to avoid enum bloat; persist cfg_active and derive=true on edges.
  - Add CI perf gates mirroring current performance_contract_tests, targeting p95 < 5ms per edited file for changed items only.

Mermaid: CPU-only extraction flow
```mermaid
flowchart LR
  A[FS change] --> B[Incremental parse]
  B -->|syn Items| C[Signature harvest]
  B -->|attrs/cfg| D[Fact emit]
  C --> E[Canonicalize + api_digest]
  D --> F[Edges: UsesType/Derives/Reexports/Contains/Requires/Defines]
  E --> G[NodeExtras store (by SigHash)]
  F --> H[ISG upsert + edge flags]
  G --> I[Queries: implements/calls/blast radius]
  H --> I
```

Shreyas Doshi-style: minimum risk, maximum output. Ship compact, deterministic signals that unlock impact for an LLM without touching GPUs, networks, or macro expansion. Everything below is CPU-only and incremental.

## Minimal CPU-only signals that help an LLM reason about changes

| Signal | What it helps the LLM do | How to compute (CPU-only, incremental) | Cost |
|---|---|---|---|
| API digest + shape digest | Detect breaking vs non-breaking changes even if names are odd | Normalize signature (types/generics/where), hash; “shape” omits identifier names | O(size(sig)) |
| Public API surface flags | SemVer impact estimation; expose/hidden | Read syn visibility, item kind, extern/const/async/unsafe flags | O(1) |
| Trait contract map | Know required vs default methods, assoc types; breakage if trait changes | From ItemTrait: list required methods/assoc types; from ItemImpl: satisfied methods | O(size(trait/impl)) |
| Derives and auto-impls | Understand “implicit” Implements edges | Parse #[derive(...)] into edges; mark derive=true | O(#attrs) |
| UsesType graph (signatures/fields) | Infer “blast radius” via type coupling | Extract all syn::Type in params/returns/fields; emit UsesType edges | O(#types) |
| Error surface summary | Reason about error paths and handling | Detect Result<T,E>, anyhow/bail!, thiserror, ? operator count, panic!/unwrap! | O(size(body)) simple scan |
| Side-effect heuristic | Purity hints; refactor and test impact | Scan for I/O (fs, net), env, time, logging, global statics | O(size(body)) |
| Concurrency footprint | Deadlock risk, Send/Sync, async edges | Detect Mutex/RwLock/Atomic/Arc/RefCell, tokio::spawn, Send/Sync bounds | O(size(body)) |
| Unsafe/FFI footprint | Safety review focus | Count unsafe blocks, extern "C", transmute/mem ops | O(size(body)) |
| cfg/feature gating mask | Reason “does this change ship to target?” | Evaluate cfg-expr for given features/target; mark active | O(#cfg nodes) |
| Re-export and alias map | Find things despite odd names | Collect pub use x::y as Z; keep alias list | O(#uses) |
| Centrality metrics | Rank “blast radius” and review order | Degree, SCC membership, simple betweenness approx on Calls/Uses | O(E) or O(V+E) |
| Complexity/branching hint | Prioritize risky edits | Cheap cyclomatic estimate: count of if/else/match/loop/&&/|| | O(size(body)) |
| Doc first-line + doc hash | Summarize intent; diff docs | Extract first doc line; 64-bit hash of full doc block | O(#attrs) |

Notes:
- All can be computed from syn/tree-sitter without macro expansion or runtime.
- Keep everything incremental: only recompute for changed nodes, reuse normalized caches, and batch per file.

## A tiny “LLM Delta Packet” you can ship per change

Emit one compact record per changed node; keep under ~1–2 KB each.

```json path=null start=null
{
    "node": {
        "hash": "0x6f91…",
        "kind": "Function",
        "canonical_path": "crate::auth::login",
        "file": "src/auth/login.rs",
        "span": [1234, 1456],
        "cfg_active": true
    },
    "api": {
        "digest": "0x2b8c…",
        "shape_digest": "0x9a71…",
        "vis": "pub",
        "flags": ["async"],
        "generics": ["T"],
        "where_bounds": ["T: Serialize + Send"]
    },
    "effects": {
        "errors": {
            "returns_result": true,
            "error_types": ["AuthError"],
            "uses_anyhow": false,
            "panic_sites": 0,
            "question_mark_uses": 3
        },
        "side_effects": ["io:fs", "log"],
        "concurrency": ["tokio::spawn", "Arc", "Mutex"],
        "unsafe_blocks": 0,
        "ffi_calls": 0
    },
    "coupling": {
        "uses_types": ["SessionToken", "UserId"],
        "callers_sample": ["crate::api::v1::login_handler"],
        "callees_sample": ["crate::auth::validate", "crate::db::users::get"]
    },
    "control": {
        "ctrl_bits": "0x0000_0321",
        "cfg_hash": "0x9f3a…",
        "exit_kinds": ["OK", "Err"],
        "await_cnt": 1,
        "unsafe_cnt": 0
    },
    "trait_contract": {
        "implements": [{"trait": "Service", "via": "manual"}],
        "requires": [],
        "assoc_types_defined": []
    },
    "doc": {
        "first_line": "Logs a user in with password and OTP.",
        "hash": "0x5ad1…",
        "tags": ["auth", "login", "retry"]
    },
    "change": {
        "kind": "modified",
        "api_breakage": "non_breaking", 
        "blast_radius_estimate": 37,
        "centrality_rank": 12
    }
}
```

How the LLM uses this:
- Compare api.digest/shape_digest to prior to spot API vs impl-only changes.
- Use callers_sample/callees_sample + blast_radius_estimate to scope impact.
- Use effects/error/concurrency to plan tests and safety checks.
- Use trait_contract to see whether trait changes break implementors.

## How to compute quickly (tie-in to your code)
- Extend NodeExtras with:
  - vis/flags, module_path, api_digest, shape_digest, generics, where_bounds, derives, cfg_active, doc_first_line/doc_hash.
- Add lightweight analyzers:
  - Error/side-effect/concurrency/unsafe scanners: single pass over function body tokens (syn) with a small allowlist of crate paths (fs, net, env, time, tokio/std sync, log).
  - Centrality: compute on Calls/Uses subgraph; cache degree + SCC id; betweenness approx only in CI if needed.
- Edges:
  - Derives, Reexports, UsesType, Contains/Requires, DefinesAssocType (as tags/bitflags).
- Diff:
  - Keep prior api_digest/shape_digest; classify change: added/removed/modified + breaking/non-breaking (e.g., trait added required method = breaking).

## Endpoints to expose (stdio JSON-RPC)
- graph.delta.summarize({ since_digest? }): returns array of LLM Delta Packets for changed nodes only (includes control.ctrl_bits, control.cfg_hash, control.exit_kinds)
- node.describe({ id/hash }): returns full packet for one node (node/api/effects/coupling/control/trait_contract/doc/change)
- risk.rank({ limit }): returns nodes sorted by blast_radius_estimate × centrality × breaking_risk
- rules.apply/list/stats: manage structural rule execution and metrics

## Start with these 8 signals (fastest, highest ROI)
- api_digest + shape_digest
- vis/flags (async/const/unsafe/extern)
- trait contract (required/default/assoc types)
- derives → Implements edges
- UsesType from signatures/fields
- error surface summary
- concurrency footprint
- cfg_active mask

Target: p95 update < 5 ms; +≤ 30% RSS; each LLM packet ≤ 1.5 KB.

## Why this helps an LLM
- It can reason on graph structure (edges) plus minimal semantics (effects/errors/concurrency/contracts) without reading full files.
- It can classify breakage, scope impact, and propose safe refactors/tests using compact, deterministic facts computed entirely on CPU.
