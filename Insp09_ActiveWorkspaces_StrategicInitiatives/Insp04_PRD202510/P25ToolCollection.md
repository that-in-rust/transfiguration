# P25 Tool Collection — Individual Tools Catalog

Conventions Reliability
- Naming: three-word-kebab-case.
- IO: JSON over stdin/stdout; files by path; deterministic where possible.
- Boundaries: codegraph-write-surface is the only writer; all other tools are read-only analysis.
- LLM usage: rule-first, LLM-late; ≤3K tokens; offline-capable where feasible.

> **Reliability-First Summary**: Conventions enforce deterministic validation through rust-analyzer overlays and cargo check gates. Tools implement bounded concurrency (Tokio runtime) with cooperative yields for UI responsiveness. Caching strategies (ISG + HNSW persisted) ensure sub-100ms query performance. Error recovery uses thiserror patterns with complete diagnostic surfaces. Memory pressure managed via quantization (Q4_K_M) and dynamic parallelism throttling.

Tools (each is independent; combine as needed)
- interface-graph-builder
  - Purpose: Build ISGL1 (filepath-filename-InterfaceName) as the canonical interface layer with L2/L3 constituents strictly "under" L1 for understanding.
  - Inputs: repo path, include/exclude globs.
  - Outputs: ISG_current (Cozo) and JSON snapshot.
  - Actions: parse crates, resolve items, derive ISGL1 keys, attach L2/L3 facets, persist to Cozo + JSON.
  - Variants: (a) rust-analyzer LSP overlays; (b) rustdoc JSON; (c) syn-based AST for macro-lite repos.
  - Notes: Zero code writes; stable ISGL1 keys enable cross-tool composition.
  - Example CLI: interface-graph-builder --repo . --out cozo://isg
  - Example Input (JSON): {"repo":".","include":["crates/**"],"exclude":["target/**"]}
  - Example Output (JSON): {"isgl1_key":"src/lib.rs-foo-parse_cfg","path":"src/lib.rs","kind":"fn","facets":{"generics":["T"],"where":["T: DeserializeOwned"]}}
  - Diverse Ideas: Add incremental rebuild mode for large repos; support for multi-language ISG (e.g., Rust + TypeScript); embed usage analytics for pattern mining.
  > **Reliability-First Summary**: ISG builder implements deterministic validation through rust-analyzer overlays and cargo check gates. Uses bounded concurrency (Tokio runtime) with cooperative yields for UI responsiveness. Caching strategies (ISG + HNSW persisted) ensure sub-100ms query performance. Error recovery uses thiserror patterns with complete diagnostic surfaces. Memory pressure managed via quantization (Q4_K_M) and dynamic parallelism throttling.

- interface-summary-generator
  - Purpose: Generate terse, lossless 1-line summaries for ISGL1 nodes.
  - Inputs: ISG_current.
  - Outputs: summaries table with provenance (rule_based | llm_assisted).
  - Actions: extract signature verbs/nouns → generate ≤120 char summaries; backfill with LLM if rule-based fails.
  - Variants: (a) rule-only heuristics; (b) LLM backfill under budget.
  - Notes: Summaries are hints, never authority; used to reduce tokens.
  - Example CLI: interface-summary-generator --cozo cozo://isg --write summaries
  - Example Input (JSON): {"isgl1_keys":["src/lib.rs-foo-parse_cfg"]}
  - Example Output (JSON): {"isgl1_key":"src/lib.rs-foo-parse_cfg","summary":"parse TOML to Cfg; returns Result<Cfg,E>","provenance":"rule_based"}

- embedding-index-builder
  - Purpose: Create/update embedding vectors for code and summaries; build HNSW indices.
  - Inputs: code slices, summaries; config for dims/quantization.
  - Outputs: vector tables + HNSW; stats/recall.
  - Actions: batch embed → upsert vectors → rebuild/merge HNSW → emit stats.
  - Variants: (a) Cozo HNSW; (b) external vector DB adapters.
  - Example CLI: embedding-index-builder --cozo cozo://isg --source summaries --dim 768
  - Example Input (JSON): {"source":"summaries","dim":768,"rebuild":true}
  - Example Output (JSON): {"vectors":17000,"hnsw":{"M":16,"ef":200},"recall@15":0.94}

- hybrid-retrieval-engine
  - Purpose: Combine Datalog two-hop with vector KNN; rank by L1>L2>L3 and pattern/idiom scores.
  - Inputs: seeds, hints, constraints.
  - Outputs: Needed shortlist (≤50) with ranks and features.
  - Actions: exact graph 2-hop → vector search → merge/dedup → rank → emit shortlist.
  - Variants: toggleable weights, L1-only mode, vector-only fallback.
  - Example CLI: hybrid-retrieval-engine --cozo cozo://isg --seed "E0277" --k 50
  - Example Input (JSON): {"seeds":["src/executor.rs-spawn_task"],"hints":["Send","'static"]}
  - Example Output (JSON): {"needed":[{"isgl1_key":"src/service.rs-run","rank":0.92},{"isgl1_key":"src/executor.rs-spawn_task","rank":0.89}]}

- pattern-knowledge-base
  - Purpose: Store idiomatic fixes and anti-patterns with examples and thresholds.
  - Inputs: patterns.yaml, examples/, thresholds.
  - Outputs: query API; usage metrics.
  - Actions: nearest pattern lookup; thresholding; example retrieval.
  - Variants: project-specific overlays; learned scoring.
  - Example CLI: pattern-knowledge-base query --hint "async spawn send"
  - Example Input (JSON): {"hint":"E0277 across await"}
  - Example Output (JSON): {"pattern_id":"async_spawn_send","distance":0.12,"example":"add Send + 'static bounds"}

- constraints-overlay-analyzer
  - Purpose: rust-analyzer didOpen buffers; collect diagnostics; compute required/current/missing bounds.
  - Inputs: candidate buffers (from CodeGraph.Future_Code).
  - Outputs: structured diagnostics and bounds delta.
  - Actions: open ephemeral buffers → collect publishDiagnostics → compute bounds delta.
  - Variants: rustc --error-format=json fallback; multi-root workspaces.
  - Example CLI: constraints-overlay-analyzer --diff-id 1234
  - Example Input (JSON): {"buffers":[{"path":"src/lib.rs","text":"..."}]}
  - Example Output (JSON): {"diagnostics":[{"code":"E0277","range":[12,5,12,18]}],"bounds":{"required":["T: Send"],"missing":["T: Send"]}}

- context-pack-builder
  - Purpose: Pack ContextBundle with Needed-first ordering; start/early/middle/end policy; ≤3K tokens.
  - Inputs: shortlist, constraints, patterns, examples.
  - Outputs: ContextBundle JSON.
  - Actions: score slices → allocate budget → order per policy → emit bundle.
  - Variants: pack-by-budget (bytes or tokens), pack-by-level (L1-heavy), pack-by-risk.
  - Example CLI: context-pack-builder --needed needed.json --budget-tokens 3000
  - Example Input (JSON): {"needed":["k1","k2"],"constraints":{"missing":["Send"]}}
  - Example Output (JSON): {"tokens":2980,"sections":{"start":["errors","anti"],"early":["function","pattern"]}}

- deterministic-patch-engine
  - Purpose: Produce minimal diffs for bounds/lifetimes/cfg; compile-time templates.
  - Inputs: diagnostics + matched patterns.
  - Outputs: unified diff + rationale.
  - Actions: parameterize template from ISGL1 + pattern → generate diff; never writes.
  - Variants: rule plugins per crate; safe rewrites only; no IO.
  - Example CLI: deterministic-patch-engine --pattern async_spawn_send --diag diag.json
  - Example Input (JSON): {"pattern_id":"async_spawn_send","isgl1_key":"src/lib.rs-foo-run"}
  - Example Output (JSON): {"diff":"--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ ...","rationale":"Add Send + 'static bounds"}

- reasoning-adapter-bridge
  - Purpose: Uniform API to local llama.cpp and cloud LLMs; confidence scoring.
  - Inputs: ContextBundle; model params.
  - Outputs: diff + confidence + alt candidates (optional).
  - Actions: call backend (local/cloud) → parse tool output → score confidence.
  - Variants: streaming vs batch; single vs multi-turn; temperature ladder.
  - Example CLI: reasoning-adapter-bridge --backend qwen7b --bundle bundle.json
  - Example Input (JSON): {"model":"qwen7b","context":{"tokens":2800}}
  - Example Output (JSON): {"diff":"...","confidence":0.87}

- local-orchestrator-daemon
  - Purpose: Run multiple llama.cpp models in parallel under strict RAM/GPU caps; JSON-RPC.
  - Inputs: job graph; model registry.
  - Outputs: per-job artifacts, logs, metrics.
  - Actions: schedule jobs → cap decoders → reuse KV → collect metrics.
  - Variants: 7B exclusive vs 3×3B + small; KV reuse; GPU layer downshift on pressure.
  - Example CLI: local-orchestrator-daemon serve --socket /tmp/llm.sock
  - Example Input (JSON-RPC): {"method":"run","params":{"model":"qwen3b","prompt":"...","max_tokens":256}}
  - Example Output (JSON): {"job_id":"J123","tokens_per_sec":95,"result":"..."}

- preflight-safety-gate
  - Purpose: Validate candidate diffs with RA overlay → cargo check --quiet → selective tests.
  - Inputs: candidate_diff_id, impacted crates/tests.
  - Outputs: pass/fail + structured report; durations.
  - Actions: open buffers → diagnostics → cargo check → run selective tests → emit gate report.
  - Variants: compile-only mode; nextest integration; time-bounded tests.
  - Example CLI: preflight-safety-gate --candidate 1234 --tests impacted.json
  - Example Input (JSON): {"candidate_diff_id":"1234","impacted_tests":["crate_a::test_send"]}
  - Example Output (JSON): {"status":"pass","ra_errors":0,"cargo_ok":true,"tests":{"run":3,"failed":0}}

  > **Reliability-First Summary**: Preflight gate enforces deterministic validation through rust-analyzer overlays and cargo check gates. Implements bounded concurrency (Tokio runtime) with cooperative yields for UI responsiveness. Caching strategies ensure sub-100ms query performance with persisted ISG + HNSW. Error recovery uses thiserror patterns with complete diagnostic surfaces. Memory pressure managed via quantization (Q4_K_M) and dynamic parallelism throttling.

- diagnostics-scope-mapper
  - Purpose: Map diagnostics to ISGL1 keys and CodeGraph rows; compute blast radius.
  - Inputs: RA/cargo diagnostics; ISG_current.
  - Outputs: references to L1 keys + impacted tests.
  - Actions: correlate paths/spans → resolve to ISGL1 → expand to test closure.
  - Example CLI: diagnostics-scope-mapper --diags diags.json
  - Example Input (JSON): {"diagnostics":[{"file":"src/lib.rs","code":"E0597"}]}
  - Example Output (JSON): {"isgl1_refs":["src/lib.rs-foo-run"],"impacted_tests":["crate_b::test_lifetimes"]}

- codegraph-write-surface (only writer)
  - Purpose: Persist Current_Code, Future_Code, Future_Action, flags, and validation status.
  - Inputs: proposed diffs or patches.
  - Outputs: updated rows with audit fields; candidate_diff_id.
  - Actions: upsert rows → set future fields → attach candidate diff → track validation status.
  - Variants: bulk import/export; read-only mirror; TTL for Future_Code.
  - Example CLI: codegraph-write-surface set-future --key src/lib.rs-foo-run --diff diff.patch
  - Example Input (JSON): {"isgl1_key":"src/lib.rs-foo-run","future_action":"Edit","future_code":"fn run(){...}"}
  - Example Output (JSON): {"candidate_diff_id":"1234","validation_status":"Pending"}

- git-apply-rollback
  - Purpose: Present/apply diffs, sign/commit, and rollback safely.
  - Inputs: approved candidate_diff_id.
  - Outputs: commit SHA + summary report.
  - Actions: write changes → commit with message templating → rollback on failure.
  - Variants: dry-run; branch/PR automation.
  - Example CLI: git-apply-rollback --candidate 1234 --sign
  - Example Input (JSON): {"candidate_diff_id":"1234","message":"fix: add Send bounds"}
  - Example Output (JSON): {"commit":"abc123","applied":true}

  > **Reliability-First Summary**: Git integration ensures deterministic validation through rust-analyzer overlays and cargo check gates. Implements bounded concurrency (Tokio runtime) with cooperative yields for UI responsiveness. Caching strategies ensure sub-100ms query performance with persisted ISG + HNSW. Error recovery uses thiserror patterns with complete diagnostic surfaces. Memory pressure managed via quantization (Q4_K_M) and dynamic parallelism throttling.

- offline-debugging-tui
  - Purpose: TUI for Needed shortlist, diff, diagnostics, CodeGraph rows, metrics; fully offline.
  - Inputs: Cozo + CodeGraph; reports.
  - Outputs: interactive views; export artifacts.
  - Actions: browse shortlist → open diff → view diagnostics → trigger Preflight → apply via CodeGraph gate.
  - Variants: “read-only triage” vs “apply with gating”.
  - Example CLI: offline-debugging-tui --cozo cozo://isg --codegraph cozo://codegraph
  - Example Input (JSON): {"view":"needed","filter":"E0277"}
  - Example Output (JSON): {"action":"ran_preflight","status":"pass"}

- isg-html-visualizer
  - Purpose: HTML visualization of ISG and deltas after changes.
  - Inputs: ISG_current and (optional) updated ISG.
  - Outputs: HTML bundle with change badges.
  - Actions: render modules → interfaces → edges; mark deltas with badges.
  - Example CLI: isg-html-visualizer --cozo cozo://isg --out ./viz
  - Example Input (JSON): {"compare":"post-apply"}
  - Example Output (JSON): {"html":"viz/index.html"}

- prd-refine-assistant
  - Purpose: Normalize PRD to ISGL1 terms; highlight gaps and risks (LLM-late).
  - Inputs: PRD draft + ISG_current.
  - Outputs: PRD-normalized.json; suggested clarifications.
  - Actions: extract requirements → map to ISGL1 → propose clarifications; low tokens.
  - Example CLI: prd-refine-assistant --in prd.md --cozo cozo://isg
  - Example Input (JSON): {"text":"Add async start() to Service"}
  - Example Output (JSON): {"normalized":{"interfaces":["service::start"]},"clarifications":["Send + 'static?"]}

- prd-consistency-checker
  - Purpose: Deterministic PRD checks (feature gates, crate boundaries, ownership transfer risks).
  - Inputs: PRD-normalized; ISG_current.
  - Outputs: issues list with severities.
  - Actions: validate feature/visibility/ownership constraints; emit actionable issues.
  - Example CLI: prd-consistency-checker --in prd-normalized.json
  - Example Input (JSON): {"interfaces":["service::start"],"features":["tokio"]}
  - Example Output (JSON): {"issues":[{"id":"visibility_mismatch","severity":"warn"}]}

- isg-future-planner
  - Purpose: Propose ISG_future deltas and corresponding CodeGraph row actions.
  - Inputs: ISG_current; PRD-normalized.
  - Outputs: CodeGraph row proposals (JSON), impacted interfaces.
  - Actions: compute create/edit/delete per ISGL1; scaffold Future_Code stubs.
  - Example CLI: isg-future-planner --prd prd-normalized.json --cozo cozo://isg
  - Example Input (JSON): {"add":["service::start"],"edit":["service::run"]}
  - Example Output (JSON): {"rows":[{"key":"src/service.rs-start","action":"Create"}]}

- blast-radius-estimator
  - Purpose: Compute impacted interfaces/tests from proposals or diffs.
  - Inputs: CodeGraph proposals/diff + ISG_current.
  - Outputs: impacted set (ISGL1 keys), test selection list.
  - Actions: traverse CALLS/DEPENDS → mark tests; cap to minimal set.
  - Example CLI: blast-radius-estimator --rows rows.json
  - Example Input (JSON): {"rows":[{"key":"src/service.rs-start","action":"Create"}]}
  - Example Output (JSON): {"interfaces":["src/api.rs-handle"],"tests":["crate_api::start_ok"]}

- feature-impact-analyzer
  - Purpose: Report cfg/feature profile diffs per proposal; warn about incompatible combos.
  - Inputs: cargo metadata; ISG/plan.
  - Outputs: feature delta report.
  - Actions: diff feature graphs; detect conflicts; suggest cfg_if consolidation.
  - Example CLI: feature-impact-analyzer --rows rows.json --cargo cargo.json
  - Example Input (JSON): {"enable":["rt-multi-thread"],"disable":["rt-core"]}
  - Example Output (JSON): {"deltas":[{"crate":"service","added":["rt-multi-thread"],"removed":["rt-core"]}]}

- gap-list-reporter
  - Purpose: List missing summaries/vectors/edges to improve retrieval quality.
  - Inputs: ISG_current + index stats.
  - Outputs: CSV/JSON with prioritized gaps.
  - Actions: scan for null summaries/vectors; rank by centrality and error proximity.
  - Example CLI: gap-list-reporter --cozo cozo://isg --out gaps.csv
  - Example Input (JSON): {"rank_by":"centrality"}
  - Example Output (JSON): {"gaps":[{"key":"src/lib.rs-foo-run","missing":"summary"}]}

- selective-test-runner
  - Purpose: Execute minimal tests based on ISG closure and diagnostics mapping.
  - Inputs: impacted tests; workspace filters.
  - Outputs: pass/fail + logs + timing.
  - Actions: compute cargo test filter → run nextest/cargo with --package/--test selectors.
  - Example CLI: selective-test-runner --tests impacted.json --timeout 8s
  - Example Input (JSON): {"tests":["crate_api::start_ok"],"timeout_sec":8}
  - Example Output (JSON): {"run":1,"failed":0,"duration_ms":720}

- pipeline-runner-cli
  - Purpose: Execute a YAML-defined DAG of these tools; resume/retry; collect metrics.
  - Inputs: pipeline.yaml; env config.
  - Outputs: artifacts; run manifest; metrics JSON.
  - Actions: parse DAG → schedule tools → manage retries → collect metrics and artifacts.
  - Example CLI: pipeline-runner-cli run pipeline.yaml
  - Example Input (YAML): steps: [interface-graph-builder, embedding-index-builder, hybrid-retrieval-engine]
  - Example Output (JSON): {"status":"ok","artifacts":["isg.json","index.stats","needed.json"]}

Three-Word Tool Aliases (reference)
- ISG builder → interface-graph-builder — builds ISGL1 with L2/L3 constituents; Cozo + HNSW ready.
- CodeGraph store → codegraph-write-surface — sole mutable store: Current/Future code, actions, flags, status.
- Summarizer → interface-summary-generator — 1‑line L1 summaries; rule‑first, LLM‑late; provenance flags.
- Embed/Index builder → embedding-index-builder — code/summary embeddings; HNSW build/update; stats.
- Retrieval engine → hybrid-retrieval-engine — Datalog 2‑hop + vector KNN; L1>L2>L3 ranking; Needed shortlist.
- Pattern/Anti‑pattern KB → pattern-knowledge-base — templates, examples, thresholds, success metrics.
- Constraints/RA overlay → constraints-overlay-analyzer — didOpen buffers; diagnostics; required/current/missing bounds.
- Context packer → context-pack-builder — Needed‑first strategic packing; ≤3K tokens; ordering policy.
- Deterministic transforms → deterministic-patch-engine — rule‑backed diffs for bounds/lifetimes/cfg consolidation.
- Reasoner adapter → reasoning-adapter-bridge — unified local (llama.cpp) and cloud model interface; confidence scoring.
- Local orchestrator → local-orchestrator-daemon — multi‑model scheduler; resource caps; KV reuse; JSON‑RPC.
- Preflight gate → preflight-safety-gate — RA overlay → cargo check → selective tests; structured report.
- Diagnostics mapper → diagnostics-scope-mapper — map diagnostics to ISGL1 + CodeGraph rows; blast radius.
- Git integration → git-apply-rollback — present/apply diffs; rollback; signed commits; msg templating.
- TUI app → offline-debugging-tui — panes for Needed, diff, diags, CodeGraph, metrics; offline-capable.

- hook-schema-validator
  - Purpose: Validate hook JSON schemas and versions for pre/post/session hooks.
  - Inputs: hook schemas, sample payloads.
  - Outputs: validation report with errors/warnings.
  - Actions: load schema → validate payloads → report diffs and version mismatches.
  - Variants: strict (fail on warn) vs permissive; schema bundle per plugin.
  - Example CLI: hook-schema-validator --hooks hooks/*.json --samples samples/*.json
  - Example Input (JSON): {"schema":"pretool.v1.json","sample":{"cmd":"grep","args":["foo"]}}
  - Example Output (JSON): {"valid":true,"warnings":[],"errors":[]}

- pretool-command-validator
  - Purpose: Deterministic validation of tool invocations before execution.
  - Inputs: command spec, policy rules, risk thresholds.
  - Outputs: allow/deny + rationale; redactions if needed.
  - Actions: pattern checks → policy match → redact secrets → emit decision.
  - Variants: project-policy overlay; learning mode (log-only).
  - Example CLI: pretool-command-validator --spec spec.json --policy policy.yaml
  - Example Input (JSON): {"tool":"run_command","cmd":"git --no-pager diff"}
  - Example Output (JSON): {"decision":"allow","redactions":[]}

- posttool-response-processor
  - Purpose: Normalize tool outputs; extract key metrics; redact PII; attach provenance.
  - Inputs: raw tool outputs (stdout/stderr), parsers.
  - Outputs: structured JSON with summaries and key fields.
  - Actions: parse → summarize → redact → attach provenance hash.
  - Variants: plugin-specific parsers; streaming vs batch.
  - Example CLI: posttool-response-processor --in out.txt --parser grep
  - Example Input (JSON): {"parser":"grep","fields":["file","line","match"]}
  - Example Output (JSON): {"rows":17,"pii_redacted":0,"summary":"17 matches"}

- sdk-compliance-validator
  - Purpose: Verify plugin/SDK conformance (APIs, version bounds, lifecycle contracts).
  - Inputs: plugin manifest, SDK version, conformance tests.
  - Outputs: pass/fail with detailed report; suggestions.
  - Actions: run conformance suite → compare manifests → check lifecycle hooks.
  - Variants: quick vs full; TS-only vs Python-only.
  - Example CLI: sdk-compliance-validator --plugin ./plugin --sdk ts@1.2.0
  - Example Input (JSON): {"paths":{"manifest":"package.json"}}
  - Example Output (JSON): {"status":"pass","failures":0}

- session-store-inspector
  - Purpose: Inspect SQLite session/context state; find bloat, stale rows, anomalies.
  - Inputs: sqlite path; retention policy.
  - Outputs: report with counts, top-k heavy sessions, cleanup suggestions.
  - Actions: open DB → run queries → compute aggregates → emit report.
  - Variants: dry-run cleanup planner; export CSV.
  - Example CLI: session-store-inspector --db .claude/session.db --retention 14d
  - Example Input (JSON): {"retention_days":14}
  - Example Output (JSON): {"sessions":412,"stale":28,"recommend_cleanup":true}

- cozo-db-adapter
  - Purpose: Typed CozoDB adapter with migrations and query builders.
  - Inputs: DB URL, migrations path.
  - Outputs: connection pool, migration reports.
  - Actions: connect → migrate → expose typed query API.
  - Example CLI: cozo-db-adapter migrate --db cozo://./data --migrations ./migrations

- isg-graph-store
  - Purpose: Persist ISG with versioning and diff ops.
  - Inputs: ISG JSON, version tags.
  - Outputs: stored graph IDs, diff reports.
  - Actions: upsert nodes/edges → compute diffs → tag versions.
  - Example CLI: isg-graph-store upsert --in isg.json --tag v1

- vector-index-store
  - Purpose: Vector storage with ANN indices (HNSW, IVF).
  - Inputs: vectors, params.
  - Outputs: index artifact, stats.
  - Actions: upsert vectors → (re)build ANN → emit metrics.
  - Example CLI: vector-index-store build --dim 768 --backend hnsw

- rust-isg-generator
  - Purpose: Generate ISG from Rust using syn + RA enrichments.
  - Inputs: repo path, include/exclude.
  - Outputs: ISG JSON with L1/L2/L3 facets.
  - Actions: parse crates → derive ISGL1 → attach facets.
  - Example CLI: rust-isg-generator --repo . --out isg.json

- multi-language-parser
  - Purpose: Parse non-Rust sources with tree-sitter for structure extraction.
  - Inputs: file globs, language set.
  - Outputs: AST/structure JSON.
  - Actions: detect language → parse → emit symbols and structure.
  - Example CLI: multi-language-parser --in docs/**

- ollama-client-kit
  - Purpose: Ollama client for discovery, health, and model mgmt.
  - Inputs: model name, limits.
  - Outputs: status, model cache.
  - Actions: probe server → pull models → run inference.
  - Example CLI: ollama-client-kit pull qwen2.5:7b

- llm-provider-router
  - Purpose: Route prompts across providers with fallback/cost policies.
  - Inputs: provider list, policy config.
  - Outputs: selected backend, result.
  - Actions: score providers → route call → record metrics.
  - Example CLI: llm-provider-router run --policy policy.yaml

- chunk-strategy-engine
  - Purpose: Structure-aware chunking for code/docs.
  - Inputs: files, strategy config.
  - Outputs: chunks JSONL.
  - Actions: analyze structure → split with overlap → emit chunks.
  - Example CLI: chunk-strategy-engine --strategy semantic --overlap 80

- embedding-batch-pipeline
  - Purpose: Batch embed with caching and deduplication.
  - Inputs: chunks, model.
  - Outputs: vectors, cache stats.
  - Actions: hash → cache lookup → embed misses → upsert vectors.
  - Example CLI: embedding-batch-pipeline --in chunks.jsonl --model e5-small

- citation-extractor-kit
  - Purpose: Extract citations with exact quote spans and sources.
  - Inputs: context, LLM output.
  - Outputs: citation list with offsets.
  - Actions: align quotes → map to sources → validate spans.
  - Example CLI: citation-extractor-kit --in out.txt --sources sources.json

- tui-ollama-shell
  - Purpose: Ratatui-based TUI for local chat and pipeline triggers.
  - Inputs: config file, model.
  - Outputs: session logs and artifacts.
  - Actions: start TUI → chat → trigger pipelines → export.
  - Example CLI: tui-ollama-shell --config tui.toml

- async-job-orchestrator
  - Purpose: Resumable async pipelines with progress and backpressure.
  - Inputs: DAG spec, tasks.
  - Outputs: run manifest, metrics.
  - Actions: schedule → execute → checkpoint → resume.
  - Example CLI: async-job-orchestrator run pipeline.yaml

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
