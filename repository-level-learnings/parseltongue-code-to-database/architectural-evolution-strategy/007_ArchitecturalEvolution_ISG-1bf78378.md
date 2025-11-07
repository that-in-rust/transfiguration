# A007: Project Parseltongue — An Architectural Evolution for AI‑Powered Code Reasoning

Executive Summary
This report outlines a strategic and technical blueprint for the evolution of the Parseltongue code analysis tool. The central objective is to re‑architect the platform to serve as a high‑performance foundation for a Large Language Model (LLM) capable of sophisticated, semantic reasoning about code structure, logic, and the impact of changes.

The core thesis: by creating a high‑level, stable semantic representation of a codebase—termed the Interface Signature Graph (ISG)—and managing its computation via a rigorous, on‑demand incremental framework, Parseltongue can provide an LLM with structured, token‑efficient, and semantically rich context required for true code intelligence. This enables deep impact analysis, automated architectural validation, and agentic refactoring—beyond autocomplete.

The proposed architecture is a multi‑layered, incremental pipeline inspired by best‑in‑class tools: Rope (Zed) for efficient text, Tree‑sitter for fast incremental parsing, a High‑level IR (HIR) akin to rust‑analyzer, and Salsa for orchestrating incremental computation. The ISG sits at the apex, abstracting away implementation details into stable contracts and relationships.

Section 1: Foundations of Modern, High‑Performance Code Analysis
1.1 The Performance Bedrock: From Text Buffer to Syntax Tree
- Rope data structure (Zed SumTree variant) for low‑latency edits and rich indexing.
- Tree‑sitter for incremental GLR parsing and CST generation tolerant to errors.
- Virtuous loop: Rope localizes edits → Tree‑sitter reparses minimal regions.

1.2 The Semantic Stratum: From Syntax to Meaning
- HIR as a desugared, stable semantic layer (rust‑analyzer model) for name/type/trait resolution.
- HIR provides a stability boundary against CST churn; ideal for early cutoffs.

1.3 The Query Primitive: Declarative Structural Matching at Scale
- ast‑grep patterns as code‑isomorphic declarative queries for structural matching.
- Deterministic grounding for LLMs: LLM produces rules; engine executes; facts feed back to LLM.

Section 2: A New Architectural Blueprint for Parseltongue
2.1 The Parseltongue Incremental Pipeline (4 stages)
- Stage 1: Rope (Text) — high‑performance rope; immutable, snapshot‑friendly.
- Stage 2: CST (Syntax) — Tree‑sitter incremental CST on every edit.
- Stage 3: HIR (Semantics) — lowering + semantic analysis; resilient to superficial syntax change.
- Stage 4: ISG (Interfaces) — nodes = interfaces; edges = structural relations.

2.2 The Interface Signature Graph (ISG)
- Nodes: Module, Class, Struct, Interface, Enum, Function, Method, TypeAlias, Global, Constant.
- Metadata: FQ name, visibility, params/returns, modifiers (async/const/unsafe), generics.
- Edges: calls, implements, inherits_from, returns, has_parameter, accesses, reexports, derives, uses.
- ISG construction: queries over HIR bodies; signatures build nodes; bodies emit edges.
- Stability goal: unchanged bodies → unchanged ISG when signatures are stable → powerful early cutoff.

2.3 Salsa: Orchestrating On‑Demand Incremental Computation
- Queries: file_text → cst(path) → hir_for_item(def_id) → isg_node(def_id) → isg_edges(def_id).
- Early cutoff: unchanged signature preserves isg_node; recompute edges only for changed bodies.

Section 3: Empowering the LLM — From Graph to Reasoning
3.1 Token‑Efficient ISG Serialization
- Serialize ISG nodes/edges concisely; prefer ISG diff for changes instead of full graph.
- Example (illustrative):
```text path=null start=null
- FN: com.example.UserService::getUser(userId: int) -> User
+ FN: com.example.UserService::getUser(userId: int, useCache: bool) -> User
+ EDGE: com.example.UserService::getUser CALLS com.example.Cache::get
```

3.2 Declarative ISG Query Language
- Deterministic filters over nodes/edges; LLM requests facts via queries; facts feed prompts.
- Example queries (illustrative):
```text path=null start=null
find nodes (FN [visibility=public, async=true]) having (CALLS -> FN [name=~".*db.*"]) 
find nodes (CLASS) having (IMPLEMENTS -> INTERFACE)
```

3.3 Semantics of Change via ISG Diffs
- Automated code review: signature change → enumerate callers; suggest tests/migration.
- Security analysis: new path to open_socket from public API → SSRF risk prompt with facts.
- Fact‑based prompting: deterministic facts bound the LLM; reduce hallucinations; increase auditability.

Section 4: Strategic Roadmap and Competitive Positioning
4.1 Phased Implementation Roadmap
- Phase 1 (Months 1–4): Rope + Tree‑sitter; ship CST‑derived features (highlighting, folding, selection).
- Phase 2 (Months 5–12): HIR + Salsa; ship Go‑to‑Definition, References, Hovers parity.
- Phase 3 (Months 13–18): ISG + ISG diff + Query language; ship AI code review on PR diffs.
- Phase 4 (Months 19+): Agentic futures; ISG‑grounded refactoring; interactive “AI Architect”.

4.2 Competitive Moat via ISG
- Beyond LSP: “super‑LSP” features (architectural validation, precise semantic diffs) not available via LSP.
- Virtuous cycle: unique ISG data → superior AI → more users → better models → stronger moat.
- Enterprise platform: productize ISG + queries for architectural rules as code.

Appendix: Practical CPU‑Only Enrichments (bridging to A004/A005)
- Interface context (A004): NodeExtras fields (vis/flags/module_path/generics/where_bounds/derives/doc_first_line/doc_hash/api_digest/shape_digest/cfg_active) and edge tags (Contains/Requires/DefinesAssocType/Reexports/UsesType/Derives).
- Behavioural hints (A005): ctrl_bits bitfield (cyclomatic, has_loop, early_return, exit_kinds, await_cnt, unsafe_cnt), cfg_hash; optional WritesField/ReadsField/MutatesSelf tags; doc purpose_tags.
- LLM Delta Packet: compact JSON with node/api/effects/coupling/control/trait_contract/doc/change; schema provided.

References (selected)
- Zed Rope & SumTree, Tree‑sitter parsing, rust‑analyzer HIR, Salsa incremental computation.
- ast‑grep declarative structural matching and performance practices.

Works Cited (verbatim sources listed in A006 and analysis references)
- Zed — Rope & SumTree; Tree‑sitter blog posts; Zed product docs
- rust‑analyzer docs on HIR and Durable Incrementality
- Salsa overview and documentation
- ast‑grep docs and performance posts

Notes
- This document synthesizes and organizes prior strategic and technical material across A002–A005 into a single architectural blueprint aimed at enabling robust, CPU‑only LLM reasoning over code changes.
