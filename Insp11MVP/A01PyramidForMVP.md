# User Journey 01 Minimal User Journey in Mermaid

```mermaid
graph TD
    A[User Reads About Parseltongue our Claude-Code Plugin in Github Repo or somewhere else] -->|User Downloads Plugin| B(User Installs Plugin)
    B --> |Parseltongue is triggered| B1(Parseltongue Analyzes System)
    B1 --> |Wait time UI| C[UI: Parseltongue in analyzing system compatibility, please wait in the headmaster's office]
    C --> |System is M1+ & 16GB roadmap+| D[UI: Parseltongue will NOT work for you, press <WIP> to exit]
    C --> |local-llama-rust-orchestrator-elf is triggered| E[UI: Parseltongue will work in your system,type Y to trigger download of models which might take max 3 minutes at 5 MBPS]
    E --> | User presses Y or y or `*y*` or `*Y*`| F[UI: Parseltongue is downloading models, please have this butter beer while we have them ready for you]
    F --- F1[Architecture & Background processes initiated]
    subgraph horizontal["🪄 🪄"]
        direction LR
        F1 --- Install["Install Models<br>ELI5: Fetch recipes with sealed packaging"]
        Install --- Verify["Verify Models<br>ELI5: Cook & plate 20 lines to verify edible"]
        Verify --- Chat["Enable Chat<br>ELI5: Now place any order confidently"]
        Install -.->|Parallel Option| Chat
    end
    Chat --- F3[UI: Parseltongue is ready to work]

```







Scope for Parseltongue v1.0

Use this as a filter for Rust Tools or Libraries you are ideating as part of building the Parseltongue plugin or skill or something for Claude Code

# A01 Initial Scope

- ANTHROPIC_KEY will be the orchestrator and reasoning LLM
- Executive Summary
    - User Segment: Developers on large Rust codebases ONLY
    - Reliability-First Principle:
        - Optimize for accurate 1-go fixes that feel trustworthy and increase user efficacy.
        - Prefer CPU-bound static analysis (rust-analyzer overlays, ISG traversals) and small, local, free subagents.
        - Keep the reasoning LLM as lean and late as possible; minimize context/tokens; use deterministic transforms whenever feasible.
    - Shreyas Doshi (product framing): Prioritize first-apply correctness over speed. Design for clarity, safety, and explicit confidence gating. Time is a secondary outcome.
    - Jeff Dean (systems framing): Make correctness the fast path. Push work to deterministic, cacheable computations (ISG, RA, HNSW). Parallelize retrieval/validation; minimize token movement; measure token-per-fix and cache hit rates.
    - User Promise: “When I hit a Rust bug, the system produces a single-pass, safe, minimal diff that compiles and (when present) passes tests before applying. Speed is a byproduct; correctness is the KPI.”

- Constraints of tech stack
    - llama.cpp because Ollama does not allow parallelism
    - CozoDB because it is a graph database
    - We will be based on Claude Code as a plugin or skill or something because we want to focus on the core differentiation which is ISG & similar Aggregated Code Context which can ensure reliable bug solving with high accuracy
        - Hence ideally whatever we want to be part of Parseltongue could be a Cargo Library or Tool, it ensures we can experminent with combination permutations
        - We need to FOCUS HARD on MVP 1.0 Scope and note down everything else as Backlog
    - Core Data Model: Interface Signature Graph (ISG)
        - Nodes: function signatures, impl blocks, trait items, types, public APIs, macro-expanded interfaces.
        - Edges: CALLS, IMPLEMENTS, USES, DEPENDS, REQUIRES_BOUND, FEATURE_GATED_BY.
        - Levels: ISGL1 (interface node keyed as filepath-filename-InterfaceName, 1 level below file/module), ISGL2/ISGL3 are constituents under ISGL1 used for understanding only.
        - Store: CozoDB (Datalog + HNSW) with columnar payloads for fast filters and range scans.
    - CodeGraph (single write surface)
        - indexed by ISGL1 key (filepath-filename-InterfaceName)
        - columns (minimal, opinionated):
            - Current_Code (canonical pre-edit slice),
            - Future_Code (candidate patch slice, ephemeral until approval),
            - Future_Action (None|Create|Edit|Delete),
            - TDD_Classification (TEST_IMPLEMENTATION, CODE_IMPLEMENTATION)
            - current_id (0/1: 0 meaning NOT in current code, 1 meaning in current code),
            - future_id (0/1: 0 meaning NOT in future code, 1 meaning in future code)
        - Rule: All code-iteration writes happen only in CodeGraph. All other CozoDB tables (ISG nodes/edges, embeddings, pattern KB indices) are read-only context stores and never mutate code.
        - Flow: PreFlight compiles Future_Code via RA overlay; on approval, flip Future→Current, clear future_* flags
    - Many types of ISGs can be created
        - ISGL1 (interface node keyed as filepath-filename-InterfaceName, 1 level below file/module)
        - ISGL2 is 1 extra distance below file / module for e.g. if a function is inside main function in main.rs that will be ISGL3
        - ISGL3 is 1 extra distance below ISGL2 for e.g. if a function is inside main function and then further inside another function in main.rs that will be ISGL3
        - ISGs are a proxy for what we called Aggregated Code Context
            - It is not just about interface signatures, it can be any pyramidal way to aggregate the codebase
            - for e.g. we can using sub-agents generically summarize all ISGL1 related code blobs to 1 liner summaries, and that will be significantly HQ code context and much lesser than the actual codebase
                - a variation of this can be running small agents with context of the PRD
            - for e.g. we can try to get HIR or other Rust analyzer meta-data for all ISGL1 related code blobs and that will be significantly HQ code context and much lesser than the actual codebase
- Local LLM Subagents Function Types (can be more)
    - A1 Seeder: error parsing → seeds + hints (no R1).
    - A2 ExactRetriever: ISG Datalog 2-hop traversal with filters; cap 30 nodes/hop.
    - A3 VectorRetriever
    - A4 AntiPatternDetector
    - A5 PatternRecognizer
    - A6 ConstraintEnforcer
    - R1 Reasoner
    - Can be more
- More ideas
    - rust-analyzer overlay: didOpen ephemeral buffers → publishDiagnostics; fail on Error severity.
    - Gate: No I/O writes until PreFlight passes and user approves.
    - llama.cpp + Metal for GGUF models; pinned CPU threads, tuned GPU layers for 2–7B.
    - Tokio runtime for orchestration; bounded task queues; cooperative yields to keep UI responsive.

- Appendix A: Local Model Matrix (indicative)
    - 22–50M encoder (Q4) — 50–150 MB.
    - MiniLM 22M (Q4) — ~40–80 MB.
    - SmolLM2 135M (Q4) — ~300–500 MB.
    - Gemma 270M (Q4) — ~600–800 MB.
    - Qwen2.5 7B (Q4_K_M) — ~4.5–6.5 GB VRAM-equivalent on Metal



# A02 Pyramid of tasks

## Decisions to reduce options

What decisions can we take which will significantly simply dev without reducing the effectiveness a lot

- Parseltongue will be a Claude Code Plugin
    - How do Plugins get integrated


## Minimal User Journey with reasonable differentiation

Search with <WIP>

- User reads about your plugin in github repo
- User downloads the plugin
- User <WIP> so that parseltongue can be triggered
- Parseltongue should analyze the system
    - Outcome 1: If the system is NOT M1+ And 16 GB+, this tool will NOT work for you
    - Outcome 2: If the system is M1+ And 16 GB+, we will trigger our local-llama-rust-orchestrator-elf named Dobby
- local-llama-rust-orchestrator-elf will be a command line tool with default installation config of naming & downloading following models
    - Large Model List: Qwen2.5 7B (Q4_K_M)
    - Medium Model List: Gemma 270M (Q4) — ~600–800 MB
    - Small Model List: SmolLM2 135M (Q4) — ~300–500 MB.
- local-llama-rust-orchestrator-elf will prove that all 3 models are working by automatically triggering a default prompt of 20 lines of output and showing the 3 of them to the user in chat




### Components identified

#### **local-orchestrator-daemon** ✓ **MVP ESSENTIAL**
- **Purpose**: Run multiple llama.cpp models in parallel under strict RAM/GPU caps; JSON-RPC
- **MVP Relevance**: Core subagent orchestration for A1-A6 local models with parallelism (not Ollama)
- **Inputs**: job graph; model registry
- **Outputs**: per-job artifacts, logs, metrics
- **Actions**: schedule jobs → cap decoders → reuse KV → collect metrics
- **Variants**: 7B exclusive vs 3×3B + small; KV reuse; GPU layer downshift on pressure
- **MVP Notes**: Required for llama.cpp parallelism, resource management, subagent coordination
- **Example CLI**: local-orchestrator-daemon serve --socket /tmp/llm.sock
- **MVP Implementation**: llama.cpp integration, model pool management, JSON-RPC interface, resource capping







# A99 Serendipity ideas

- Given the popularity of Claude Code, you could literally be building an army of OSS Rust based plugins which solves problems with Claude Code workflow

# A98 Architecture Ideations

## High-Level Design (HLD)

-  Build Parseltongue as a Claude Code plugin that “gives your agent a computer” (tools to run commands, edit files, verify work) and follows an agent loop of gather context → take action → verify → repeat, aligning with reliability-first gating and preflight checks. [^1]
-  Distribute local capabilities via an MCP server bundled inside the plugin so tools auto-appear when the plugin is enabled; define servers in .mcp.json or inline in plugin.json and auto-start them with the plugin lifecycle. [^2]
-  Use the plugin-bundled MCP server to front a local orchestrator process (e.g., Dobby) over stdio or HTTP transport; expose tools like “system check,” “model install/test,” and “ISG ops” to Claude Code through MCP. [^2]
-  Provide a CLI/SDK wrapping mode for headless or non-IDE usage, following community patterns that wrap the Claude Code CLI/SDK and add plugin-like middleware without blocking core operation. [^3]
-  Validate third-party extensibility by referencing existing community integrations (e.g., Neovim), reinforcing that external plugins can integrate smoothly with Claude Code environments. [^4]

## Low-Level Design (LLD)

-  Plugin manifests:
    - Define plugin MCP servers in .mcp.json or plugin.json (e.g., command: ${CLAUDE_PLUGIN_ROOT}/servers/dobby, args, env). On enable, Claude Code starts the server automatically and tools become available in /mcp. Support stdio/HTTP transports per your server implementation. [^2]
-  Tool surface:
    - Expose scoped MCP tools (e.g., check_system, install_models, verify_models, build_isg, query_isg) that Claude can call; each tool performs deterministic steps and returns structured JSON, aligning with the agent loop’s verify phase. [^1] [^2]
-  Orchestrator process (Dobby):
    - Start as a child process of the plugin MCP server or as the server itself; communicate via JSON-RPC/stdio or HTTP. The MCP server mediates requests from Claude to Dobby, maintaining resource caps and returning logs/metrics to the IDE. (Note: No official llama.cpp parallel orchestration guidance present in supplied results.) [^2]
-  Agent loop integration:
    - Within Claude Code, drive actions through the loop: gather project context → call MCP tools (e.g., system check, ISG retrieval) → verify outputs (compilation/tests) → iterate only if verification fails, consistent with Agent SDK best practices. [^1]
-  CLI/SDK mode:
    - Offer an optional CLI wrapper for Parseltongue that proxies Claude Code SDK calls, adds middleware (caching/metrics), and keeps zero-latency passthrough semantics similar to community approaches; useful for CI or non-IDE environments. [^3]
-  Known gaps and next steps:
    - The provided results do not include official details on multi-model parallelism for llama.cpp or a formal Claude Code “plugin store.” Proceed using MCP-bundled servers for distribution, and design your orchestrator’s concurrency model independently. [^2] [^1]

[^1]: https://www.anthropic.com/engineering/building-agents-with-the-claude-agent-sdk
[^2]: https://docs.claude.com/en/docs/claude-code/mcp
[^3]: https://github.com/instantlyeasy/claudeware
[^4]: https://github.com/greggh/claude-code.nvim

