

## Short answer
-  CocoIndex can power an incremental/streaming pipeline, but it won’t, by itself, give you syntax-error tolerance for Rust parsing or “all rust-analyzer information.” You’d still implement those parts.
-  For Parseltongue v1.0, use a native Rust pipeline: tree-sitter-rust for tolerant parsing + syn for exact signatures (when valid) + rust-analyzer via LSP for hydration. Persist to CozoDB keyed by the ISG interface ID. This meets all three needs: error tolerance, “code tied to interface,” and RA metadata.

## If you want error tolerance + RA info + interface-bound storage

-  Parser stack
    - P1 Tolerant parse: tree-sitter-rust
        - Extract ISGL1 items even when files are broken. Record parse_error_count and low-confidence flags.
    - P2 Exact parse (optional, when valid): syn
        - Derive canonical signature text for higher fidelity.
    - Macro expansion (selective): rust-analyzer custom LSP method rust-analyzer/expandMacro to capture interfaces created by macros.

-  RA hydration (via LSP; no direct RA crate coupling)
    - Open ephemeral buffers (didOpen) for your current working content.
    - Collect:
        - publishDiagnostics (error tolerance + precise messages)
        - documentSymbol (stable ranges for items)
        - hover on identifiers for typed signatures
        - definition on call sites to build CALLS edges
        - rust-analyzer/expandMacro for macro-generated items
    - Store hydrated fields on the ISG node: ra_signature, resolved_types, macro_expansion_notes, diag_summary.

-  Streaming and incremental updates
    - File watcher (notify crate) + hashing. On changes:
        1) tree-sitter pass → ISGL1 stubs + spans + parse_error_count
        2) syn pass if tree is clean → canonical signature
        3) rust-analyzer overlay → diagnostics + symbols + selective definitions on call_expression tokens
        4) Upsert nodes/edges/blobs/embeddings in Cozo; re-embed only changed code hashes

-  Interface-bound storage (Cozo)
    - ISG ID: filepath-filename-InterfaceName (plus disambiguator if needed)
    - Store code slices exactly on the interface span. If the span is incomplete due to syntax errors, store the best-recovered slice and mark confidence.

| Relation | Columns (additions for error tolerance + RA) |
|---|---|
| isg_nodes | id, file_path, module_path, kind, name, signature_ts, signature_syn, ra_signature, isgl_level, parse_error_count, confidence, span_start, span_end, hash, ra_hydrated_ts |
| isg_edges | src_id, dst_id, kind, weight, provenance in ["heuristic","ra"] |
| code_blobs | id, code, tdd_classification, recovered boolean |
| diagnostics | id, severity, code, message, range, ts |
| embeddings | id, model, dim, vec |

-  CALLS edges with RA
    - From tree-sitter AST, find call_expression nodes and their callee path segment byte ranges.
    - For each callee token position, query LSP definition; if it resolves to a file+range mapping to another ISG node, add CALLS edge with provenance="ra".
    - If unresolved, optionally add a heuristic edge (provenance="heuristic") or skip.

## Minimal implementation sketch

-  Tolerant parse (tree-sitter-rust) to get ISGL1 items even with broken code:
```rust
use tree_sitter::{Parser, Node};
use tree_sitter_rust;

fn extract_isgl1(src: &str) -> Vec<IsgItem> {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_rust::language()).unwrap();
    let tree = parser.parse(src, None).unwrap();
    let root = tree.root_node();

    let mut items = Vec::new();
    for child in root.children(&mut root.walk()) {
        // Match top-level: function_item, trait_item, struct_item, enum_item, impl_item, mod_item
        if ["function_item","trait_item","struct_item","enum_item","impl_item","mod_item"]
            .contains(&child.kind())
        {
            let span = (child.start_byte(), child.end_byte());
            let name = extract_name(child, src); // implement with field traversal
            items.push(IsgItem { name, kind: child.kind().to_string(), span });
        }
    }
    items
}
```

-  RA overlay LSP calls (pseudo JSON-RPC over stdio/unix socket):
```json
// didOpen ephemeral doc


## RA overlay LSP calls you’ll use

| LSP method | Why we call it | What we persist on ISG |
|---|---|---|
| initialize/initialized | Session setup | session metadata |
| textDocument/didOpen, didChange, didClose | Ephemeral buffers for current content (no disk writes) | content version for provenance |
| textDocument/publishDiagnostics (server push) | Precise errors/warnings with ranges and codes | diagnostics rows; A1 seeds |
| textDocument/documentSymbol | Stable top-level item ranges under syntax errors (often available) | span_start/span_end, names, kinds |
| textDocument/hover | Type info and rendered signatures | ra_signature, resolved_types |
| textDocument/definition | Map call sites to targets to build CALLS edges | edges with provenance="ra" |
| rust-analyzer/expandMacro | Capture macro-expanded interfaces | macro_expansion_notes; optional synthetic nodes |
| textDocument/semanticTokens/full (optional) | Token classes for better call-site extraction | improved call-site heuristics |

Notes:
-  Always open the exact in-memory content you parsed (ephemeral didOpen) so diagnostics line up with your spans.
-  For CALLS, send definition requests at callee token positions found by tree-sitter; resolve file+range → ISG id mapping.

## File-watcher streaming pipeline (end-to-end)

-  Watch workspace with `notify` (respect `.gitignore`/`target` exclusions).
-  On change batch (debounced):
    - P1: tree-sitter tolerant parse
        - Extract ISGL1 items; record `parse_error_count`, `confidence="low"` if errors present.
        - Slice code by item spans; compute `hash`.
    - P2: syn exact parse (only if file parses cleanly)
        - Derive `signature_syn`, canonical generics/vis text; bump confidence for nodes from this file.
    - P3: RA hydration via LSP overlay
        - didOpen ephemeral content.
        - Pull documentSymbol, hover for signatures, diagnostics (store as A1 seeds).
        - For each call_expression callee token: definition → target id → `CALLS` edge with `provenance="ra"`.
    - Persist to Cozo incrementally
        - Upsert nodes/edges/code_blobs; only re-embed nodes whose `hash` changed.

## Cozo writes (upsert patterns)

-  isg_nodes: key = `filepath-filename-InterfaceName[-disambiguator]`
    - Columns: file_path, module_path, kind, name, signature_ts, signature_syn, ra_signature, generics, vis, isgl_level, parse_error_count, confidence, span_start, span_end, hash, ra_hydrated_ts.
-  isg_edges: `src_id`, `dst_id`, `kind` in [CALLS, IMPLEMENTS, USES, DEPENDS, FEATURE_GATED_BY], `weight`, `provenance` in [heuristic, ra].
-  code_blobs: `id`, `code`, `tdd_classification`, `recovered` (true if span was incomplete).
-  diagnostics: `id`, `severity`, `code`, `message`, `range`, `ts`.
-  embeddings: `id`, `model`, `dim`, `vec` (HNSW index); rebuild on `hash` change.

Query helpers:
-  Two-hop retrieval (A2) around a seed id over CALLS/DEPENDS, capped fanout.
-  ANN top-k (A3) with your encoder vector, de-dup with A2.

## Embedding regeneration policy

-  Compute vector only when `hash` changes.
-  Skip embedding for `confidence="low"` if you want to avoid garbage vectors; or embed but lower weight during retrieval.
-  Keep encoder tiny (MiniLM 22M Q4 or SmolLM2 135M Q4) for speed; persist vectors in HNSW with tuned efConstruction/efSearch.

## Error classification and confidence gating

-  Confidence scoring suggestion:
    - Start at 1.0
    - If `parse_error_count > 0`: subtract 0.4
    - If no `signature_syn`: subtract 0.2
    - If no `ra_signature` or missing `documentSymbol`: subtract 0.2
    - Min 0.0
-  Use confidence to:
    - Gate which snippets go into R1 prompt.
    - Lower retrieval rank for low-confidence nodes.
    - Decide whether to attempt macro expansion or defer.

## Macro- and generated-code handling

-  Try `rust-analyzer/expandMacro` on macro call sites appearing at ISGL1 level (e.g., `#[derive]`, `macro_rules!` usage).
-  For expanded items that look like interfaces, you may:
    - Store a synthetic ISG node with `kind="macro_interface"` and `macro_expansion_notes`.
    - Or attach the expansion as an annotation on the original node to keep ISGL1 surface minimal.

## Tying code slices to interfaces (even under errors)

-  Always store the best-recovered slice from tree-sitter spans in `code_blobs.code`.
-  If RA provides better symbol ranges, prefer those spans in subsequent updates.
-  Keep `recovered=true` when the slice crosses syntax-error regions; this is a signal to R1 and to your PreFlight to be conservative.

## Where CocoIndex fits (given your goals)

-  If your immediate need is “tolerance to syntax errors” and “all rust-analyzer information,” CocoIndex won’t give that out-of-the-box; you still need the parser + RA overlay described above.
-  CocoIndex can still be valuable later to orchestrate incremental hydration of non-code context (PRDs, PDFs, wikis) and to push to multiple targets. For v1.0 code ingestion, it adds dependencies (Postgres) and complexity without replacing the RA/AST work.

## Decision

-  Use the native Rust path for v1.0:
    - tree-sitter-rust (tolerant) → syn (exact when possible) → RA LSP overlay (hydration) → Cozo upserts keyed by ISG id.
    - This gives you error tolerance, interface-bound storage, and the richness of rust-analyzer, with minimal moving parts.

## Next steps I can draft for you

-  A tiny crate that exposes `ingest_file(path, contents) -> Vec<IsgNodeUpdate>` implementing the pipeline above.
-  Cozo Datalog helpers for A2 retrieval and vector search registration.
-  LSP client wrapper with backpressure and a request budget to keep the UI responsive.
-  Confidence-driven sampler for R1 prompt assembly.

```
====




## Executive Summary

-   Goal: Reliable one-go Rust fixes for large codebases. Correctness is the KPI; speed is a byproduct.
-   Strategy: Deterministic first (rust-analyzer, ISG traversals, Cozo); late-and-lean LLM (Anthropic via ANTHROPIC_KEY) after local validation; small local subagents.
-   Scope: macOS Apple Silicon (M1+) with ≥16 GB RAM. Claude Code plugin drives the workflow.
-   Orchestration: Dobby (local-llama-rust-orchestrator-elf) wraps Ray to schedule subagents and a single StarCoder2 3B worker (optionally), with strict RAM-aware concurrency.

ELI5: We build a careful robot librarian for your Rust repo. It catalogs all the “chapter headings” (interfaces), uses a spellchecker (rust-analyzer) to ensure changes won’t break, asks a smart friend (Claude) only at the end, and never writes to your book until tests pass.

---

## HLD

### System overview

```mermaid
graph LR
    subgraph Editor
        CC[Claude Code]
        EXT[Parseltongue Plugin]
    end

    subgraph Local Services
        Dobby[(Dobby: Ray wrapper)]
        RA[RA Overlay (LSP)]
        Cozo[(CozoDB)]
    end

    subgraph Ingestion
        Streamer[rust-ISG-streamer]
        CodeGraph[CodeGraph (write surface)]
    end

    subgraph Subagents
        A1[A1: Error Seeder]
        A2[A2: ISG Datalog Retriever]
        A3[A3: Vector Retriever]
        R1[R1: Reasoner (Anthropic)]
    end

    CC --> EXT
    EXT <--> Dobby
    EXT <--> RA
    EXT <--> Cozo
    Streamer --> Cozo
    CodeGraph --> Cozo

    Dobby --> A1
    Dobby --> A2
    Dobby --> A3
    Dobby --> R1
```

### Data backbone

-   CozoDB holds:
    - ISG nodes and edges (interface-level graph).
    - CodeGraph (single write surface: Current/Future code slices).
    - Embeddings (HNSW index).
    - Diagnostics (RA overlay).
-   Rule: Only CodeGraph mutates code. All other stores are read-only context.

### Control flow (user journey v0.7)

-   Plugin bootstraps → system check → if supported, start Dobby → ensure StarCoder2 3B model (optional small/CPU subagents) → verify model out.
-   Confirm workspace → rust-ISG-streamer runs (tree-sitter tolerant → syn exact → RA hydration) → write ISG + code slices to Cozo + CodeGraph.
-   When user requests a fix:
    - A1 parses diagnostics → seeds.
    - A2/A3 fetch relevant ISG contexts deterministically.
    - R1 (Anthropic) proposes minimal patch to Future_Code.
    - PreFlight via RA overlay (and tests if present).
    - On pass and approval → flip Future→Current.

ELI5: We check your computer, prepare tools, map your code like a roadmap, find the pothole, patch just that spot, and re-check the road before opening traffic.

---

## LLD

### Component responsibilities

-   Parseltongue Plugin (Claude Code)
    - UX, system gating, Dobby lifecycle, orchestration of flows, UI strings, approvals.

-   Dobby (Ray wrapper)
    - Starts a local Ray runtime.
    - Manages subagent pools (CPU-bound) and optionally a single StarCoder2 3B worker with MPS.
    - RAM-aware concurrency (target concurrency 1–2 for model tasks; up to 5 for lightweight CPU tasks).
    - JSON-RPC to the plugin; dispatches tasks to Ray actors; streams progress.

-   rust-ISG-streamer
    - Tolerant parse with tree-sitter; exact parse with syn when possible.
    - RA LSP hydration (diagnostics, symbols, optional macro expansion).
    - Emits ISGL1 nodes (filepath-filename-InterfaceName) and code slices with spans, confidence, hashes.
    - Incremental: only changed nodes re-embedded and re-upserted.

-   RA Overlay (LSP client)
    - Ephemeral document open/change/close.
    - Collect diagnostics; document symbols; hover signatures; definitions for call edges.
    - No disk writes; all in-memory overlays.

-   CozoDB
    - Relations: isg_nodes, isg_edges, code_blobs, embeddings (HNSW), diagnostics, codegraph.
    - Datalog helpers for 2-hop traversals (A2) and filters.

-   Subagents
    - A1 Seeder: convert RA/cargo errors into seed sets.
    - A2 Retriever: Datalog 2-hop around seeds with caps.
    - A3 Retriever: ANN top-k by embeddings; dedupe with A2.
    - R1 Reasoner (Anthropic): late-stage minimal patch proposal under strict constraints.

-   PreFlight
    - RA diagnostics on Future_Code overlay; optional focused cargo test; only on pass can user approve.

ELI5: Each piece has a simple job—one reads, one checks, one organizes, one fetches clues, one writes the fix, one double-checks.

---

## Interfaces

### 1) Dobby (Ray wrapper) – CLI

-   Command: serve
    - Args: socket_path, ray_temp_dir, max_workers, model_registry_path
    - Behavior: start Ray head (local), spawn actor pools, open JSON-RPC socket.

-   Command: ensure-model
    - Args: model_name=starchoder2-3b, source_url, sha256, target_dir
    - Behavior: download, verify, register.

-   Command: verify
    - Args: model_name, lines=20
    - Behavior: run a deterministic prompt; return exactly N lines for UI.

-   Command: shutdown
    - Args: graceful_timeout_secs
    - Behavior: drain queues; stop Ray; close socket.

ELI5: Start me, get the model, test it, then shut me down.

### 2) Dobby – JSON-RPC methods

| Method | Params | Returns | Notes |
|---|---|---|---|
| ping | {} | {ok: bool} | Health check |
| get_status | {} | {models[], workers[], mem_free_mb, queue_depths} | Resource view |
| ensure_model | {name, url, sha256} | {status: "present"|"downloaded", bytes} | Idempotent |
| open_worker | {kind: "llm"|"cpu", model?: "starcoder2-3b"} | {worker_id} | Enforces concurrency |
| close_worker | {worker_id} | {ok} | Graceful |
| completion | {worker_id, prompt, max_tokens, stop[], temperature} | {text, usage{prompt_tokens, output_tokens}} | Model task |
| embed | {texts[], encoder:"smollm2-135m"} | {vectors: float[][]} | CPU encoder |
| run_task | {agent:"A1"|"A2"|"A3", payload{...}} | {result{...}} | Agent dispatch |
| verify_default | {model:"starcoder2-3b", lines:20} | {lines[]} | Deterministic check |

ELI5: Ask Dobby what’s running, start a worker, ask it to write or embed, and get results.

### 3) rust-ISG-streamer – CLI

-   Command: stream
    - Args: workspace_path, ignore_files[], parser_mode=("tolerant"|"strict"|"auto"), output=("cozo"|"stdout"), fanout_caps, ann_model
    - Emits: records for ISGL1 nodes and edges, code slices, diagnostics summary.

ELI5: “Read this repo, list all important top-level things, and store each with its exact code.”

### 4) RA Overlay – logical methods

| Method | Input | Output | Purpose |
|---|---|---|---|
| open_buffer | {uri, language="rust", text} | {version_id} | Ephemeral doc |
| get_diagnostics | {uri, version_id} | {diagnostics[]} | Errors/warnings |
| get_document_symbols | {uri, version_id} | {symbols[]} | Top-level ranges |
| hover_signature | {uri, position} | {rendered_signature} | Type info |
| go_to_definition | {uri, position} | {target_uri, range} | Build CALLS edges |
| expand_macro (optional) | {uri, range} | {expanded_text} | Macro interfaces |
| close_buffer | {uri} | {ok} | Cleanup |

ELI5: Open the text in memory, ask the analyzer where the problems and definitions are, then close it.

### 5) CodeGraph – operations

| Operation | Inputs | Outputs | Notes |
|---|---|---|---|
| upsert_current | {id, current_code, tdd_classification, current_id=1} | {ok} | Initial fill |
| propose_future | {id, future_code, future_action} | {ok} | From R1 |
| preflight_check | {ids[]} | {compile_ok, test_ok, diagnostics[]} | RA overlay + tests |
| approve | {ids[], approver} | {flipped: count} | Future→Current |
| reject | {ids[], reason} | {cleared: count} | Clear future_* |

ELI5: Store today’s code, propose tomorrow’s code, check it, then commit or discard.

### 6) CozoDB – relations (schema)

| Relation | Columns |
|---|---|
| isg_nodes | id (ISGL1 key), file_path, crate, module_path, kind, name, signature_ts, signature_syn, ra_signature, isgl_level, parse_error_count, confidence, span_start, span_end, hash, ra_hydrated_ts |
| isg_edges | src_id, dst_id, kind (CALLS|IMPLEMENTS|USES|DEPENDS|FEATURE_GATED_BY), weight, provenance ("heuristic"|"ra") |
| code_blobs | id, code, tdd_classification, recovered (bool) |
| embeddings | id, model, dim, vec (HNSW) |
| diagnostics | id, severity, code, message, range, ts |
| codegraph | id, current_code, future_code, future_action, tdd_classification, current_id, future_id |

ELI5: A set of tables that remember “what is what,” “who calls who,” “the exact text,” “how it’s similar,” “what errors exist,” and “what’s proposed.”

### 7) Subagents – interfaces

-   A1 Seeder
    - Signature: seed_errors(inputs: {cargo_stderr?, ra_diagnostics[]}) -> {seeds: [{id, file, range, code, message}]}
    - ELI5: Turn error messages into precise breadcrumbs.

-   A2 ExactRetriever
    - Signature: traverse_isg(inputs: {seed_ids[], kinds[], hops=2, cap_per_hop=30}) -> {nodes[], edges[]}
    - ELI5: Walk the map two steps around the problem and list nearby places.

-   A3 VectorRetriever
    - Signature: ann_retrieve(inputs: {id or text, k=15, model}) -> {neighbors: [{id, dist}]}
    - ELI5: Find similar pieces of code by “meaning.”

-   R1 Reasoner (Anthropic)
    - Signature: propose_patch(inputs: {seeds[], contexts[], constraints, patch_template}) -> {diff, rationale, confidence}
    - ELI5: Ask the smart friend to suggest a tiny, safe edit.

### 8) Plugin UX – commands

-   Parseltongue: Bootstrap
    - System check, start Dobby, ensure model, verify.

-   Parseltongue: Ingest
    - Run rust-ISG-streamer; show progress; index done.

-   Parseltongue: Fix Current Error
    - A1 → A2/A3 → R1 → PreFlight → Approve/Reject diff.

ELI5: Start tools, index code, fix errors with a guided, trust-earning flow.

---

## ELI5 per critical design choice

-   Tolerant parsing first: Even if your file is half-written, we can still detect “the big pieces” and keep going.
-   Analyzer hydration: We ask rust-analyzer to tell us the truth about types and definitions before we guess.
-   Single write surface: Only one place can change code; everything else is read-only. Fewer surprises.
-   Late LLM: We try math and rules before we ask the poet to write; this keeps edits small and safe.
-   Gating: We don’t glue edits into your code until a compile/test door opens cleanly.

---

## Resource and concurrency policy (M1/16 GB)

-   StarCoder2 3B footprint on MPS can exceed several GB plus KV cache; five true-parallel LLM decoders on 16 GB is impractical.
-   Policy:
    - At most one LLM worker active; queue concurrent LLM tasks.
    - Up to five CPU subagents (A1/A2/A3) in parallel (lightweight).
    - Enforce memory guards: if free RAM < safe threshold, refuse to start model.

ELI5: One big chef in the kitchen at a time; several assistants can chop vegetables.

---

## Answering the WIP questions

-   Will Ray run 5 agents in parallel on M1 16 GB without llama.cpp?
    - Yes for lightweight CPU agents (A1/A2/A3). For an LLM like StarCoder2 3B, true parallel inference workers will exceed memory. Use 1 LLM worker (sequential) plus parallel CPU tasks. Ray schedules and isolates tasks; it does not reduce model memory requirements.

-   What will be the format of the command line tool local-llama-rust-orchestrator-elf?
    - Minimal subcommands: serve, ensure-model, verify, shutdown.
    - Environment: PARSELTONGUE_HOME, MODEL_REGISTRY, ANTHROPIC_KEY, SOCKET_PATH.
    - Outputs: JSON logs and a JSON-RPC socket; progress events for UI.

---

## End-to-end flows

### Ingestion flow

-   Input: workspace path.
-   Steps:
    - tree-sitter tolerant pass → ISGL1 stubs (spans, kinds, names), code slices, confidence.
    - syn exact pass (when clean) → canonical signatures.
    - RA overlay hydration → diagnostics, symbols, hover signatures, CALLS edges (definition mapping).
    - Cozo upserts → isg_nodes, isg_edges, code_blobs, diagnostics; embeddings only for changed hashes.

### Fix flow

-   Input: current editor errors or failing tests.
-   Steps:
    - A1 produce seeds from diagnostics.
    - A2 2-hop traversal (CALLS/DEPENDS) around seeds; cap fanout.
    - A3 ANN top-k similar nodes; de-dup; rank by confidence.
    - R1 minimal patch proposal to CodeGraph.Future_Code with strict constraints.
    - PreFlight: RA diagnostics + optional targeted cargo test.
    - Approve: flip Future→Current.

---

## Metrics and guardrails

-   Metrics: token-per-fix, compile-first-pass rate, retrieval cache-hit rate, ingestion latency, memory headroom.
-   Guardrails: max nodes per hop, max tokens in R1 prompt, ban public signature changes unless explicitly requested, auto-rustfmt and re-parse check before PreFlight.

---

## Practical notes on models

-   Default present: CPU-friendly encoder (e.g., Smol-scale) for embeddings.
-   StarCoder2 3B: optional, single active worker on MPS; verify with deterministic 20-line output.
-   Future: allow additional small local GGUF models as CPU tools; keep large models optional to respect memory/bandwidth.

---

## Acceptance criteria (v1.0)

-   System gating works; Dobby starts; model ensure + verify passes.
-   Ingestion produces ISG nodes/edges and code slices under syntax errors; Cozo populated; embeddings indexed.
-   Fix flow yields a single minimal diff that passes PreFlight; user approves to commit.
-   Concurrency guards prevent OOM; UI remains responsive.

---

If you want, I can turn these interfaces into:
-   JSON-RPC schema docs for Dobby,
-   Cozo relation definitions with indices,
-   A task board mapping each subsystem to milestones and test cases.



## CozoDB blueprint for Parseltongue v1.0

-   Role
    - CozoDB is the system of record for all code intelligence: ISG nodes/edges, exact code slices, embeddings, diagnostics, and the CodeGraph write-surface.
    - Single-writer rule: Only CodeGraph mutates code; all other relations are read-only context stores.

-   ELI5
    - Think of Cozo as a clean filing cabinet. We file “what exists,” “who talks to whom,” “the exact text,” and “how similar things are.” We only keep “future edits” in a special tray (CodeGraph) until the supervisor signs off.

## Data model HLD

```mermaid
graph TD
    ISG_NODES[isg_nodes\n(ISGL1)]
    ISG_EDGES[isg_edges\n(CALLS/IMPLEMENTS/USES/...)]
    CODE_BLOBS[code_blobs\n(exact slices)]
    EMB[embeddings\n(HNSW)]
    DIAG[diagnostics]
    CG[codegraph\n(single write surface)]

    ISG_NODES -->|id| CODE_BLOBS
    ISG_NODES -->|id| EMB
    ISG_NODES -->|id| DIAG
    ISG_NODES -->|id| CG
    ISG_NODES -->|src_id/dst_id| ISG_EDGES
```

-   Entity summary
    - isg_nodes: One row per interface at ISGL1 granularity (filepath-filename-InterfaceName).
    - isg_edges: Directed relations like CALLS, IMPLEMENTS, USES, DEPENDS, FEATURE_GATED_BY.
    - code_blobs: Canonical, exact code slice for each interface node (recovered if file is broken).
    - embeddings: Vector representations for retrieval (HNSW index).
    - diagnostics: Latest analyzer diagnostics at node granularity.
    - codegraph: Current and proposed future code, flags, and actions for controlled edits.

-   ELI5
    - Nodes = chapter headings. Edges = references between chapters. Blobs = the exact paragraph of each chapter. Embeddings = how similar chapters feel. Diagnostics = sticky notes with issues. CodeGraph = a safe draft tray.

## Relations (LLD)

| Relation | Columns |
|---|---|
| isg_nodes | id (ISGL1 key), file_path, crate, module_path, kind, name, signature_ts, signature_syn, ra_signature, isgl_level, parse_error_count, confidence, span_start, span_end, hash, ra_hydrated_ts, schema_rev |
| isg_edges | src_id, dst_id, kind (CALLS|IMPLEMENTS|USES|DEPENDS|FEATURE_GATED_BY), weight, provenance ("heuristic"|"ra"), ts |
| code_blobs | id, code, tdd_classification ("TEST_IMPLEMENTATION"|"CODE_IMPLEMENTATION"), recovered (bool), ts |
| embeddings | id, model, dim, vec, ef_search, m_build, ts |
| diagnostics | id, severity, code, message, range, analyzer_rev, ts |
| codegraph | id, current_code, future_code, future_action ("None"|"Create"|"Edit"|"Delete"), tdd_classification, current_id (0/1), future_id (0/1), preflight_state ("none"|"failed"|"passed"), ts |

-   Keys and indices (recommended)
    - Primary keys
        - isg_nodes.id
        - code_blobs.id
        - embeddings.id (composite key: id+model if multi-encoder)
        - codegraph.id
    - Secondary indices
        - isg_nodes: by file_path, by kind, by crate+module_path
        - isg_edges: by src_id, by dst_id, by kind
        - diagnostics: by id, by severity
        - embeddings: HNSW index on vec per model
    - Integrity hints
        - Enforce uniqueness of (file_path, name, kind, isgl_level, disambiguator) via deterministic id composition.

-   ELI5
    - We label every drawer clearly and add shortcuts to find things by file, kind, or relationships. For fast “find similar,” we keep a special map (HNSW).

## Write rules and invariants

-   Single-writer invariant
    - Only codegraph.future_* fields represent proposed code edits; everything else is immutable facts about the current codebase.
-   Flip rule
    - Approval flips future_* into current_* atomically and clears future_* flags.
-   Confidence and provenance
    - confidence on nodes drops if parsing was error-tolerant; edges mark provenance as "ra" (analyzer) or "heuristic".
-   Idempotency
    - Nodes and edges are upserted with stable ids and hashes to avoid duplicates and rework (embeddings recompute only on hash change).
-   ELI5
    - Drafts live in the “future” tray until approved; flipping moves them into the “current” drawer in one swift motion.

## Named “database interfaces” (Datalog API contracts)

Note: These are interface signatures, not code. Treat them as callable queries/transactions with clear inputs/outputs/side-effects.

-   Ingestion and hydration
    - ingest_file_batch
        - Inputs: file_path, crate, module_path, isgl1_nodes[], code_slices[], parse_stats, now_ts
        - Effects: upsert isg_nodes, code_blobs; set confidence based on parse_error_count; refresh hashes; record ra_hydrated_ts when hydration arrives
        - Returns: {upserted_nodes, changed_hashes}
        - ELI5: File changed? Replace its index cards and exact paragraphs.

    - replace_edges_for_file
        - Inputs: file_path, edges[] (src_id, dst_id, kind, provenance), now_ts
        - Effects: delete edges whose src_id belongs to file_path; insert new edges; cap fanout limits if configured
        - Returns: {deleted, inserted}
        - ELI5: Erase stale “who calls whom” lines for that file and redraw them.

    - upsert_diagnostics
        - Inputs: id, diagnostics[], analyzer_rev, now_ts
        - Effects: replace latest diagnostics for node id
        - Returns: {count}
        - ELI5: Stick the newest warning notes on the right cards.

    - upsert_embedding_if_hash_changed
        - Inputs: id, model, dim, vec, node_hash, hnsw_params, now_ts
        - Pre: compare stored hash; skip if unchanged
        - Effects: insert/replace vector; update HNSW
        - Returns: {updated: true|false}
        - ELI5: Only recompute the “similarity map” if the paragraph actually changed.

-   Retrieval (A2/A3)
    - isg_two_hop
        - Inputs: seed_ids[], kinds_filter[], hop_limit=2, cap_per_hop=30
        - Outputs: {nodes[], edges[]} around seeds respecting caps
        - ELI5: Walk two steps in the graph near the problem, but don’t wander too far.

    - ann_top_k
        - Inputs: id or query_text, k, model
        - Outputs: {neighbors: [{id, dist}]}
        - ELI5: Find the most look-alike paragraphs.

    - merge_rank_context
        - Inputs: seed_ids[], graph_nodes[], ann_neighbors[], confidence_floor
        - Outputs: {ranked_ids[], rationale_map}
        - ELI5: Combine the nearby places and the look-alikes, then keep the trustworthy ones.

-   CodeGraph (single write surface)
    - codegraph_upsert_current
        - Inputs: id, current_code, tdd_classification
        - Effects: set current_code; current_id=1
        - Returns: {ok}
        - ELI5: File today’s official paragraph.

    - codegraph_propose_future
        - Inputs: id, future_code, future_action ("Create"|"Edit"|"Delete")
        - Effects: set future_code, future_action; future_id=1; preflight_state="none"
        - Returns: {ok}
        - ELI5: Place a draft edit into the tray.

    - codegraph_mark_preflight
        - Inputs: id, preflight_state ("passed"|"failed")
        - Effects: update preflight_state
        - Returns: {ok}
        - ELI5: Stamp the draft as pass or fail after checks.

    - codegraph_approve_flip
        - Inputs: ids[], approver, now_ts
        - Pre: all ids must have preflight_state="passed"
        - Effects: move future_code→current_code; set future_* empty; set current_id=1; future_id=0
        - Returns: {flipped_count}
        - ELI5: Promote drafts to the official drawer, in one shot.

    - codegraph_reject_clear
        - Inputs: ids[], reason
        - Effects: clear future_*; preflight_state="none"
        - Returns: {cleared_count}
        - ELI5: Toss bad drafts.

-   Search and utilities
    - list_nodes_by_file
        - Inputs: file_path
        - Outputs: ids[]
    - get_node_summary
        - Inputs: id
        - Outputs: {name, kind, signatures, confidence, tdd_classification}
    - recent_diagnostics
        - Inputs: id, limit
        - Outputs: diagnostics[]
    - explain_connectivity
        - Inputs: src_id, dst_id, max_hops=3
        - Outputs: path candidates
        - ELI5: Show a plausible route from one chapter to another.

## Transactions and consistency

-   Batch per file
    - ingest_file_batch + replace_edges_for_file + upsert_embedding_if_hash_changed should run as one transaction to keep ISG and edges aligned.
-   Patch workflow
    - codegraph_propose_future → PreFlight checks (external) → codegraph_mark_preflight → codegraph_approve_flip in a tight transaction.
-   Time-travel (optional)
    - Enable snapshots on isg_nodes, isg_edges, code_blobs, and codegraph to compare historic vs current; useful for debugging fix regressions.
-   ELI5
    - We write changes in small, safe batches, so cards and strings never contradict each other.

## Performance and indexing guidance

-   HNSW embeddings
    - One HNSW index per model. Keep dim small (e.g., 256–768). Choose M and efConstruction for your accuracy/latency budget.
-   Filtering first, ANN next
    - Filter by crate/kind/test vs code before vector search to reduce candidates.
-   Caps
    - Apply fanout caps in A2 and top-k caps in A3 to bound prompt assembly and latency.
-   Hashing
    - Compute node-level hash on the exact code slice to gate embeddings and avoid redundant writes.
-   Partitioning
    - Add crate/module_path fields and query primarily within a crate for large monorepos.

## Confidence and provenance policy

-   confidence
    - Derived from parse_error_count, presence of signature_syn, availability of ra_signature/symbols.
-   Use in retrieval
    - Drop or down-rank nodes below confidence_floor during A2/A3 merge.
-   provenance
    - Mark edges as "ra" when built from definitions; "heuristic" otherwise. Prefer "ra" in ranking.

## Integration points (HLD to LLD mapping)

-   Ingestion pipeline
    - tree-sitter tolerant → syn exact → RA overlay (symbols/defs/diagnostics) → ingest_file_batch + replace_edges_for_file + upsert_diagnostics + upsert_embedding_if_hash_changed.

-   Fix pipeline
    - A1 seeds from diagnostics → A2 isg_two_hop → A3 ann_top_k → merge_rank_context → R1 propose → codegraph_propose_future → PreFlight external → codegraph_mark_preflight → codegraph_approve_flip.

-   ELI5
    - Read, understand, index; then for a fix, gather nearby facts and similar pieces, suggest a tiny change, and only commit after double-checking.

## Operational concerns

-   Storage location
    - Project-local Cozo store under .parseltongue/ by default; support global cache for embeddings.
-   Backups
    - Snapshot relations before mass refactors (especially codegraph).
-   Migrations
    - schema_rev column on isg_nodes; write migration jobs as read-transform-write passes with reversible tags.
-   Observability
    - Track: ingestion latency, two-hop latency, ANN latency, token-per-fix, compile-first-pass rate, HNSW recall checks.

## Acceptance checklist (Cozo-specific)

-   Can ingest a large Rust workspace and answer:
    - Count of ISGL1 nodes by kind and by crate.
    - Two-hop neighborhood sizes capped correctly.
    - ANN search returns reasonable neighbors; re-embedding happens only on hash change.
-   CodeGraph flips are atomic and leave no orphan future_* fields.
-   Diagnostics reflect latest RA overlay outputs for targeted nodes.

## Quick ELI5 recap

-   Cozo is our smart filing cabinet. We:
    - File each interface with its exact paragraph.
    - Draw lines showing who calls whom.
    - Keep a “similarity map” for quick lookups.
    - Stick warnings on the right cards.
    - Store draft edits in a separate tray and only file them after the test stamp.

If you want, I can turn these interface signatures into:
-   Concrete Datalog query specs (names, inputs/outputs, constraints),
-   Index configuration notes for HNSW and common filters,
-   A migration plan with schema_rev transitions and snapshot practices.

=========


## Executive fit (what matters most)

-  Reliability-first and rule-first, LLM-late: prefer rust-analyzer overlays, deterministic transforms, Cozo HNSW, minimal token budgets.
-  Single write surface via CodeGraph; zero I/O to repo until PreFlight passes.
-  Local subagents on 16 GB M1+; orchestrator via ANTHROPIC_KEY (Claude Code plugin), with tiny local models for parallel CPU-bound compute.

## Journey-to-tool mapping

| Stage | Purpose | Recommended tools | Variants and why |
|:---|:---|:---|:---|
| 0. Hardware gate + installer (Dobby) | Verify M1+ 16 GB+, prepare local models, cap concurrency | resource-aware-execution-engine; concurrency-controller; gguf-model-loader-manager; local-orchestrator-daemon | Use llama.cpp backend; detect RAM → set model plan (max 5 subagents); if insisting on Ray, wrap local-orchestrator-daemon via JSON-RPC from Dobby; auto-downshift quantization (Q4_K_M) on pressure |
| 1. Claude Code plugin bootstrap | Plugin registration, bridge calls to local tools, health | claude-code-plugin-framework; claude-code-bridge-server; claude-code-plugin-manager; health-check | Bridge Claude ↔ local endpoints; add health/metrics for trust; expose commands like /rust:debug-bug, /rust:validate-fix |
| 2. Repo context confirmation | Are we in the right repo? Path sanity and readiness | file-hash (for repo fingerprinting); cozo-db-adapter (bootstrap) | Optional: session-store-inspector to keep plugin state lean |
| 3. ISG build (10 min indexing) | Build ISGL1; chunking; persist to Cozo; add summaries | interface-graph-builder ✓; chunk-strategy-engine (ISG-code-chunk-streamer analog); interface-summary-generator; cozo-db-adapter | Use rust-analyzer LSP overlays variant for semantic enrichment; tree-sitter-based chunking; minimal 1-line summaries (rule-first) |
| 4. Embeddings + HNSW | Vector index for hybrid retrieval | embedding-index-builder; cozo-hnsw-vector-search | MiniLM-L6-v2 embeddings 384-d; Cozo HNSW to keep everything CPU + local |
| 5. Workspace/feature awareness | Respect cargo features, multi-crate | workspace-aware-retrieval-system | Feature-conditioned graph traversal; use cargo metadata filters |
| 6. Hybrid retrieval shortlist | Deterministic 2-hop + vector KNN | hybrid-retrieval-engine; hybrid-datalog-vector-query; blast-radius-context-calculator | Graph-first with vector backfill; cap ≤50 nodes; exact two-hop per P40; blast radius for impacted tests |
| 7. Micro-PRD refinement (2 turns) | Clarify tests/behavior/functionality without bloating context | prd-refine-assistant; prd-consistency-checker; context-pack-builder; beaconized-context-packer; reasoning-adapter-bridge (ANTHROPIC_KEY) | Keep context ≤3K tokens; START/END beacons to avoid lost-in-middle; ANTHROPIC_KEY as reasoner for PRD clarity only |
| 8. ISG-level simulations A01/A02 | Plan Create/Edit/Delete rows for tests then non-tests | isg-future-planner; isg-transformation-orchestrator; pattern-knowledge-base | Represent additions as future_ind=1 with empty Future_Code; record Future_Action in CodeGraph |
| 9. Code simulation (Future_Code) | Fill Future_Code safely; iterate if not confident | deterministic-patch-engine; structured-pattern-matcher; pattern-aware-debugging-engine; codegraph-write-surface | Rule-first templates (bounds/lifetimes/cfg); write only to CodeGraph.Future_Code/Action; never touch disk |
| 10. Constraints preflight (zero-write) | LSP overlays on candidate buffers | constraints-overlay-analyzer; preflight-lsp-client | In-memory RA didOpen buffers; compute missing/required bounds; fast fail without writes |
| 11. Compile/test safety gate | cargo check + selective tests; go back on fail | preflight-safety-gate; selective-test-runner; feature-aware-validation-pipeline; diagnostics-scope-mapper | Use impacted tests via ISG closure; sccache/incremental builds; map failures back to ISGL1 for explainability |
| 12. Champion selection + confidence | Calibrate confidence; gate auto-apply | calibrated-confidence-engine; adaptive-confidence-calibrator; validation-cache-optimizer | Confidence gating for apply; cache validations to keep iterations snappy |
| 13. Final apply to repo | Write minimal diff; commit; rollback safe | git-apply-rollback | Atomic apply with commit signing; auto-rollback on failure |
| 14. Clean slate + DB reset | Reset CodeGraph, record audit | codegraph-write-surface (flip Future→Current); cozo-preflight-queue (drain); success-metrics-framework | Commit summary; reset Future_* fields; update metrics |

## Critical variants and configuration notes

-  rust-analyzer overlays first: constraints-overlay-analyzer and preflight-lsp-client before any disk writes; prefer Hybrid validation (LSP fast + Shadow when needed).
-  CozoDB everywhere: interface-graph-builder + cozo-db-adapter for ISG/CodeGraph; cozo-hnsw-vector-search for vectors; hybrid-datalog-vector-query to join exact+semantic.
-  Context budgets: context-pack-builder + beaconized-context-packer keep ≤3K tokens for PRD/reasoning; the heavy current code is excluded by design.
-  Local subagents: local-orchestrator-daemon plus multi-agent-pool-manager/concurrency-controller; cap at 5 subagents based on RAM; keep tiny models for analysis and leave heavy reasoning to ANTHROPIC_KEY.
-  Deterministic transforms first: deterministic-patch-engine with structured-pattern-matcher to generate minimal, template-backed diffs for lifetimes/bounds/cfg.
-  Confidence gating: calibrated-confidence-engine/adaptive-confidence-calibrator decide when to auto-apply vs request user confirmation.

## Gaps and how to cover them

-  Dobby (ray wrapper): Use local-orchestrator-daemon as the core; expose a thin Dobby CLI/daemon that schedules jobs to JSON-RPC llama.cpp workers. If Ray is non-negotiable, Dobby can proxy to Ray while still calling the same tools as child processes.
-  ISG-code-chunk-streamer + ingest-chunks-to-CodeGraph: Combine chunk-strategy-engine + interface-graph-builder + codegraph-write-surface for identical effect.
-  rust-preflight-code-simulator: Covered by constraints-overlay-analyzer + preflight-lsp-client + preflight-safety-gate; add shadow-workspace-validator if you need a full cargo pipeline in isolation.
-  Code-simulation-sorcerer: Realized by isg-future-planner + deterministic-patch-engine + codegraph-write-surface loop plus confidence gating.

## Minimal accuracy-first MVP stack (highest ROI)

-  Data/Index: cozo-db-adapter; interface-graph-builder (RA overlays variant); interface-summary-generator; embedding-index-builder; cozo-hnsw-vector-search.
-  Retrieval: workspace-aware-retrieval-system; hybrid-retrieval-engine; blast-radius-context-calculator.
-  PRD + Context: prd-refine-assistant; prd-consistency-checker; context-pack-builder; beaconized-context-packer.
-  Transforms: structured-pattern-matcher; deterministic-patch-engine; pattern-knowledge-base.
-  Validation: constraints-overlay-analyzer; preflight-lsp-client; preflight-safety-gate; selective-test-runner; diagnostics-scope-mapper.
-  Orchestration & Safety: reasoning-adapter-bridge (ANTHROPIC_KEY); local-orchestrator-daemon; calibrated-confidence-engine; git-apply-rollback.
-  Plugin: claude-code-plugin-framework; claude-code-bridge-server.

## Suggested execution skeleton (end-to-end)

    # 0) Hardware + local orchestrator
    resource-engine --hardware auto --memory-budget 12GB
    local-orchestrator-daemon serve --socket /tmp/llm.sock

    # 1) ISG + embeddings
    interface-graph-builder --repo . --out cozo://isg
    interface-summary-generator --cozo cozo://isg --write summaries
    embedding-index-builder --cozo cozo://isg --source summaries --dim 384

    # 2) PRD loop
    prd-refine-assistant --in micro_prd.md --cozo cozo://isg
    prd-consistency-checker --in prd-normalized.json

    # 3) Plan + simulate in CodeGraph
    isg-future-planner --prd prd-normalized.json --cozo cozo://isg
    deterministic-patch-engine --pattern <id> --diag diags.json | codegraph-write-surface set-future -f -

    # 4) PreFlight + selective tests
    preflight-lsp-client --buffers 15 --features "<active>"
    preflight-safety-gate --candidate <id> --tests impacted.json

    # 5) Apply if confident
    calibrated-confidence --threshold 0.90 --semantic-validation
    git-apply-rollback --candidate <id> --sign

This stack mirrors your v0.7 journey while keeping correctness the fast path (RA overlays, deterministic templates, Cozo queries) and the reasoning LLM lean and late.

