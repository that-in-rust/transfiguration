# P24ClaudePlugin02 — ISG + Local LLM Subagents for Large Rust Codebase Bug Solving

Status: Draft v0.1
Owner: P24 Initiative
Scope: Claude Code plugin evolution with on-device subagents, optimized for Apple Silicon (16 GB+) and large Rust monorepos

—

Executive Summary

Reliability-First Principle (per P24 comment):
- Optimize for accurate 1-go fixes that feel trustworthy and increase user efficacy.
- Prefer CPU-bound static analysis (rust-analyzer overlays, ISG traversals) and small, local, free subagents.
- Keep the reasoning LLM as lean and late as possible; minimize context/tokens; use deterministic transforms whenever feasible.

Shreyas Doshi (product framing): Prioritize first-apply correctness over speed. Design for clarity, safety, and explicit confidence gating. Time is a secondary outcome.

Jeff Dean (systems framing): Make correctness the fast path. Push work to deterministic, cacheable computations (ISG, RA, HNSW). Parallelize retrieval/validation; minimize token movement; measure token-per-fix and cache hit rates.

User Promise: “When I hit a Rust bug, the system produces a single-pass, safe, minimal diff that compiles and (when present) passes tests before applying. Speed is a byproduct; correctness is the KPI.”

—

User Journeys (no diagrams)

Journey A: “Single-Pass Safe Fix” (default path)
- Trigger: User runs /rust:debug-bug on a failing build/test.
- Steps:
  1) Check cached ISG; build/refresh if missing.
  2) Parse rustc/cargo errors → seeds + hints.
  3) Parallel retrieval: (a) ISG 2-hop CALLS/DEPENDS; (b) HNSW vector neighbors.
  4) Validate: anti-pattern detector, pattern recognizer, constraint checker.
  5) Build curated context (10–15K tokens), generate diff, compute confidence.
  6) PreFlight: rust-analyzer overlay + cargo check --quiet.
  7) Present diff with pattern rationale and confidence; user approves; apply + git commit.
- Success criterion: First-Apply Correctness ≥ 0.95; typical latency 60–120s on 16 GB.

Journey B: “Investigate Before Edit”
- Trigger: Security/compliance sensitive repo; user wants diagnosis only.
- Steps: Same as A but skip apply; produce root-cause narrative, candidate patch, confidence, and diagnostic mapping to ISG nodes.
- Output: A red/amber/green report with suggested pattern and constrained blast radius.

Journey C: “Low-Memory Mode” (≤12 GB)
- Trigger: Older Mac or heavy IDE workload.
- Steps: Throttle parallel subagents; use smaller local models (135M/270M class) for A4–A6; cap retrieval set (≤25 items); reason with shorter context (6–8K) using a compact 2B model; identical validation flow.
- Tradeoff: +15–25s latency; similar safety bar.

Journey D: “Air-gapped/Offline”
- Trigger: No network; local-only constraints.
- Steps: Use local GGUF models via llama.cpp (Metal); embeddings/HNSW built locally; rely on rust-analyzer + cargo for correctness; disable cloud orchestrator.
- Output: Fully offline fixes, cached indices, reproducible runs.

Journey E: “CI Gate: Pre-commit/Pre-push”
- Trigger: Hook invoked on diff.
- Steps: Only run PreFlight against staged changes; if fail, explain diagnostics mapped to ISG; offer /rust:debug-bug to fix.
- Budget: ≤20s; zero writes.

Journey F: “Test-First Fix (Proof-Driven)”
- Trigger: Bug lacks failing test; user opts into RED→GREEN flow.
- Steps: Generate minimal failing test from diagnostics and ISG slice; run tests to confirm RED; synthesize fix; ensure GREEN; present diff + new test together.
- Success criterion: New test passes and increases coverage; FACR unaffected.

Journey G: “Zero-LLM Deterministic Patch”
- Trigger: Error maps to deterministic rule-backed transformation (pattern-only path).
- Steps: Apply template patch parameterized by ISG metadata; PreFlight + (optional) targeted tests; present diff.
- Benefits: Zero tokens, lowest variance; ideal for E0277 bound insertions, elided lifetime clarifications, feature cfg consolidation.

—

High-Level Design (HLD)

Core Data Model: Interface Signature Graph (ISG)
- Nodes: function signatures, impl blocks, trait items, types, public APIs, macro-expanded interfaces.
- Edges: CALLS, IMPLEMENTS, USES, DEPENDS, REQUIRES_BOUND, FEATURE_GATED_BY.
- Levels: ISGL1 (raw signature row per symbol), ISGL2 (aggregate per file/module), ISGL3 (aggregate per crate/workspace). Each node stores:
  - Hash(fingerprint), path, span, crate, features, visibility
  - Pattern flags, anti-pattern distances, idiomatic score
  - Embedding vector ref, historical effectiveness stats
- Store: CozoDB (Datalog + HNSW) with columnar payloads for fast filters and range scans.

Local LLM Subagents
- A1 ScopeSeeder: error parsing → seeds + hints (22–50M encoder model).
- A2 ExactRetriever: ISG Datalog 2-hop traversal with filters; cap 30 nodes/hop.
- A3 VectorRetriever: HNSW K=15 nearest, filtered to code-level L4–L5.
- A4 AntiPatternDetector: classifier (MiniLM ~22M) over features + vector similarity threshold (τ ≈ 0.20).
- A5 PatternRecognizer: small decoder-only model (135M) producing pattern_id + example.
- A6 ConstraintEnforcer: structured checker using rust-analyzer types; fill missing trait/lifetime bounds; optional 270M helper for mapping errors to constraints.
- R1 Reasoner (LLM-late, token-capped): default 2–3B local; 7B fallback only when confidence < threshold; ≤3K tokens input; adapts deterministic patch to local style and edge cases; outputs minimal diff + confidence.

Validation Layer
- rust-analyzer overlay: didOpen ephemeral buffers → publishDiagnostics; fail on Error severity.
- cargo check --quiet on real workspace (no temp checkout); cap run ≤ 3s when hot.
- Selective tests (when present): detect nextest; else cargo test -q limited to impacted crates/tests via ISG blast radius; cap runtime; cache test binaries.
- Gate: No I/O writes until PreFlight passes and user approves.

On-Device Runtime
- llama.cpp + Metal for GGUF models; pinned CPU threads, tuned GPU layers for 2–7B.
- Tokio runtime for orchestration; bounded task queues; cooperative yields to keep UI responsive.
- Caching: ISG + HNSW persisted; warm caches on idle.

—

Low-Level Design (LLD)

Key Interfaces (traits)
- ISGRepository: upsert_nodes(), two_hop(seed, filters)->Vec<Node>, annotate(node, meta), stats().
- PatternKB: nearest_patterns(vec_id, k), anti_distance(vec_id)->f32, example(pattern_id).
- ConstraintValidator: required(span)->Bounds, current(span)->Bounds, missing()->BoundsDelta.
- RAOverlayClient: open_buffers(diff)->SessionId, diagnostics(SessionId)->Vec<Diag>, close(SessionId).
- CargoCheckRunner: check_quiet(paths)->CheckResult.
- DiagnosticsMapper: map(Vec<Diag>)->Vec<ISGRef>.
- ContextPacker: pack(A4/A5/A6 outputs)->ContextBundle.
- Reasoner: propose(ContextBundle)->{diff, confidence, blast_radius}.
- SafetyGate: preflight(diff)->{pass, diags}.
- DiffApplier: apply(diff)->CommitId.
- LearningStore: log(bug, solution, accepted, confidence).

Data Structures
- Node: {id, kind, crate, path, span, sig_hash, features, public, scores{idiomatic}, flags{anti[]}, vec_id}.
- Edge: {src, dst, rel: CALLS|DEPENDS|IMPLEMENTS|USES|REQUIRES_BOUND|FEATURE_GATED_BY}.
- Bounds: {traits[], lifetimes[], where_clauses[]}.
- ContextBundle: {errors, anti_hits, patterns, constraints, code_slices, history}.

Scheduling & Concurrency
- Parallel A2/A3/A4/A5/A6; bounded by memory budget.
- Prioritize smallest working set first (exact graph), backfill with vectors.
- LLM-late gating: allocate tokens only after constraints satisfied and top-1 pattern stabilized; default token budget ≤3K.
- Backpressure: drop low-score candidates early; limit context to 10–15K tokens.

Back-of-the-Envelope (16 GB, M2/M3)
- ISG (17k nodes, ~120k edges): ~350–500 MB incl. vectors.
- HNSW K=15, efSearch 64: ~2–6 ms/query hot; cold ~20–40 ms.
- RA overlay diagnostics: 150–800 ms typical per batch.
- 7B Q4_K_M, prompt 12K tokens @ 30–60 tok/s: 200–400 ms prompt processing + 1–4 s generation (short diff). p95 end-to-end ≤ 90 s.

—

Simulations (end-to-end runs)

S0: Deterministic pattern-only (E0277 bound insertion)
- A1: Maps E0277 to missing Send + 'static.
- A5/A6: Produce parameterized where-bound template from ISG; no LLM.
- PreFlight: RA overlay + cargo check pass; optional targeted tests run (<10s).
- Outcome: 1-go apply; tokens used ≈ 0.

S1: Non-Send across await (E0277)
- Repo: 250 crates, 1.2M LOC; workspace build hot.
- A1: Parses E0277; seeds executor.rs, service.rs; hint async_spawn_send.
- A2: 2-hop CALLS from failing fn; pulls 38 nodes; flags Send constraints missing.
- A3: K=15 vector neighbors; 6 examples with Arc<Mutex<T>> pattern.
- A4: Anti-pattern hit: capturing !Send type across await (distance 0.12) → severity High.
- A5: Pattern: spawn_blocking or tokio::spawn + Send + 'static bounds; example patch.
- A6: Missing bounds: T: Send + 'static; add where clauses.
- Context: 12.7K tokens; R1 proposes adding Send bounds & move clones; confidence 0.92.
- PreFlight: RA overlay clean; cargo check passes in 2.1 s.
- Outcome: Diff accepted; time 74 s; learning logs updated.

S2: Lifetime mismatch from iterator adaptor (E0597)
- A1: Seeds collect() scope; hints elided lifetime.
- A2: Pulls map/filter chain; 24 nodes.
- A5: Suggests lift borrow lifetime by cloning small items or re-structuring ownership.
- R1: Rewrites chain to own data pre-closure; confidence 0.81.
- PreFlight: cargo check fails; diagnostics: extra clone increases size in hot path.
- Iterate: A5 alternative: small Cow<'a, T> wrapper; R1 patch v2; confidence 0.88.
- Pass: cargo check 2.6 s. Total 96 s (near p95). Accepted.

S3: Missing trait bound in generic API (E0277 variant)
- A6: Required bounds: S: Serialize + Send; current: Serialize only; missing: Send.
- R1: Minimal where-bound diff; confidence 0.94; passes PreFlight in 1.4 s; total 58 s.

S4: Macro expansion edge-case (proc-macro)
- A2: DEPENDS edges into macro crate; A3: neighbors show prior fixes.
- A4: Anti-pattern: non-deterministic macro path exposure; severity Medium.
- R1: Adds explicit path qualification and feature-gated re-export; confidence 0.77.
- PreFlight: RA overlay warns (Deprecated); cargo ok. Present amber status; user defers apply.

S5: Feature-gated type collision
- A2: FEATURE_GATED_BY edges reveal mutually exclusive cfg(features).
- R1: Introduces cfg_if! consolidation; updates ISG features; confidence 0.89; passes.

S6: Borrow checker hotspot in tight loop (perf-sensitive)
- A5: Pattern: pre-allocate buffer + index writes; remove interior mutability.
- R1: Patch with small perf note; confidence 0.82.
- PreFlight passes; prompts user to run benchmarks; journey B style report.

S7: CI pre-push validation only
- PreFlight on staged diff; maps two diagnostics to ISG nodes; suggests /rust:debug-bug; 8.2 s total.

—

Resource Modes & Token Economy

- Standard Mode (16 GB): A2–A6 parallel; default reasoner 2–3B; ≤3K tokens; context slices capped to function/file.
- Low-Mem (≤12 GB): Reduce parallelism; 135–270M helpers only; 0–1B reasoner or deterministic path; +15–25s typical.
- Offline: All models local (GGUF); strict token caps; prefer deterministic transforms.
- Token KPIs: tokens_per_fix p95 ≤ 3K; token_variance low; avoid LLM on deterministic cases.

Safety & Reliability
- First-Apply Correctness Rate (FACR) ≥ 97% (moving avg); rollback rate ≤ 1%.
- Zero writes before pass + approval.
- Deterministic PreFlight: rust-analyzer overlay + cargo check.
- Rollback: If apply fails, auto-revert; log incident.
- Audit trail: store pattern_id, confidence, diagnostics, and git commit id.

Test-First Executable Specs (selected)
- TDD harness: STUB → RED → GREEN → REFACTOR; ability to auto-generate minimal failing test when feasible.
- Selective tests: prefer targeted tests on impacted crates; fallback to compile-only when tests absent or too slow.
- PreFlight_p95 ≤ 3 s with warm build.
- HNSW_query ≤ 100 ms p95.
- Pattern coverage: ≥150 patterns, ≥100 anti-patterns, ≥200 error mappings; report gaps.
- Concurrency stress: A2–A6 under load without starvation; bound memory spikes ≤ 1.5 GB.
- Error handling: thiserror in libs; anyhow at app boundary; complete diagnostics surfaces.

—

Integration Surface (Claude Code)

Commands
- /rust:debug-bug → full pipeline (A1–A6, R1, PreFlight, present).
- /rust:validate-fix → PreFlight only.
- /rust:build-isg → (re)build ISG + annotate.
- /rust:prove-fix → Generate minimal failing test, then synthesize and validate fix; present test+patch bundle.

Agents
- rust-bug-first-responder, rust-exact-retriever, rust-vector-retriever, rust-anti-pattern-detector, rust-pattern-recognizer, rust-constraint-enforcer, rust-context-builder, rust-deep-reasoner, rust-safety-gate, rust-diff-presenter, rust-learning-orchestrator.

Hooks
- preflight_ra_overlay, cargo_check_wrapper, cache_warmer.

—

Implementation Plan (4–5 sprints)

Sprint 1: ISG + Indexes
- CozoDB schema, HNSW build, rust-analyzer symbol ingest, two_hop queries.
- Unit tests for ISGRepository; performance harness for build time.

Sprint 2: Discovery + Validation
- A1–A3 parallel orchestration; A4–A6 local models wired; diagnostics mapper; test detection and selective test runner.
- Benchmarks: HNSW p95, A2 traversal p95; selective test latency p95.

Sprint 3: Reasoning + Safety
- ContextPacker with token caps; Reasoner default 2–3B (7B fallback); Confidence scoring with high threshold gate.
- PreFlight pipeline (RA overlay + cargo check) + selective tests with timeouts and fallbacks.

Sprint 4: UX + Learning + CI
- Diff presentation bundle; approval workflow; LearningStore; CLI + Claude Code command glue.
- CI gate mode for /rust:validate-fix.

Sprint 5: Polish
- Caching, cache warming, memory tuning, low-mem mode, offline mode.
- Metrics dashboard; p95 adherence; docs.

—

Risks & Mitigations
- Macro/Build.rs complexity → Treat macro crates as first-class; cache expansions; surface amber guidance.
- Feature combinatorics → ISG FEATURE_GATED_BY edges; per-profile caches.
- Memory pressure with 7B → quantization (Q4_K_M), dynamic parallelism throttling.
- RA flakiness under load → isolate overlay sessions; conservative timeouts; retries.
- Pattern brittleness → fallback alternatives; reinforce with LearningStore statistics.

—

Appendix A: Local Model Matrix (indicative)
- A1: 22–50M encoder (Q4) — 50–150 MB.
- A4: MiniLM 22M (Q4) — ~40–80 MB.
- A5: SmolLM2 135M (Q4) — ~300–500 MB.
- A6-helper: Gemma 270M (Q4) — ~600–800 MB.
- R1: Qwen2.5 7B (Q4_K_M) — ~4.5–6.5 GB VRAM-equivalent on Metal; CPU fallback slower.

Appendix B: Pattern Inventory (top 10 by incidence)
1) async_spawn_send (Send + 'static bounds)
2) borrow_to_owned in iterator chains
3) trait bound add-on for generic API
4) cfg feature consolidation to avoid type collisions
5) move closure captures for Send safety
6) Arc<RwLock<T>> to Arc<Mutex<T>> refactor guidance
7) explicit lifetime elision fixes in impls
8) Result error type unification via thiserror
9) dyn Trait object safety fixes
10) from/into impl resolution fixes

Appendix C: Metrics Targets
- first_apply_correctness ≥ 97%
- rollback_rate ≤ 1%
- validation_accuracy (PreFlight→compile) ≥ 98%
- tokens_per_fix p95 ≤ 3K; zero_llm_rate ≥ 30% on common errors
- time_to_safe_fix (informational) p95 ≤ 120 s
- memory_footprint ≤ 12 GB in standard mode

—

Bottom Line

P24ClaudePlugin02 turns ISG + local subagents into a production-grade Rust debugging assistant: one-go reliable fixes with minimal tokens, offline-capable, safety-first, and continuously learning. We win by narrowing the search with ISG, specializing subagents, and validating before any write.
