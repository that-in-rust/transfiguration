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
