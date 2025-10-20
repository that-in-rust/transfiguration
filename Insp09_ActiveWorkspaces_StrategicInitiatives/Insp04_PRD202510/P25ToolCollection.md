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
  - Diverse Ideas:
    * **Incremental Intelligence**: Delta-based ISG rebuilding with semantic fingerprinting; enables real-time analysis on monorepos with <100ms update latency through change propagation graphs.
    * **Cross-Language Harmony**: Multi-language ISG that understands Rust↔TypeScript↔Python FFI boundaries; automatically maps type signatures and memory layouts across language boundaries with WASM bridging.
    * **Pattern Evolution Engine**: Self-learning pattern discovery that identifies recurring idioms from successful PRs and auto-generates new templates; continuously improves suggestion quality through reinforcement learning from code review outcomes.
    * **Semantic Compression**: Lossless graph compression using algebraic topology techniques; stores full ISG semantics in 10x smaller space through persistent homology and quotient graph constructions.
    * **Temporal ISG**: Version-aware interface graphs that track API evolution, deprecation cycles, and migration patterns across git history with semantic diffing.
    * **Collaborative Intelligence**: Federated learning across organizations to discover cross-project patterns while maintaining code privacy through differential privacy and secure aggregation.
    * **Performance Oracle**: Hardware-tuned query optimization that adapts to user's M1/M2 specs and RAM constraints; automatically selects optimal algorithms and caching strategies.
    * **Causal Reasoning Layer**: Causal inference engine that identifies root causes of bugs beyond correlation; uses counterfactual analysis to suggest minimal interventions.
    * **Zero-Knowledge Proofs**: ZK-proof generation for code correctness properties; enables verifiable computation without revealing source code in enterprise environments.
    * **PRD-to-Code Delta Engine**: Converts product requirements directly into ISG diffs showing exact interface changes and ripple analysis; enables 3-day "simple features" to stay 3-day features instead of 2-week refactors.
    * **Live Codebase Memory**: Persistent incremental knowledge base that survives restarts and explains "why this code exists" by linking commits, PR discussions, and design decisions to ISG nodes.
    * **Context Surgeon Intelligence**: Surgical LLM context assembly using cfg_hash and api_digest to achieve 80-95% token reduction by sending only semantic deltas, not full text diffs.
    * **Refactor Confidence Engine**: Pre-refactor analysis showing exact ripple effects, missing test coverage gaps, and borrow-checker hotspots with calibrated confidence scores.
    * **Cross-Language Bridge Extension**: Unified ISG across Rust↔C/C++↔Python with ABI compatibility checking, enabling automatic binding updates and cross-language refactoring.
    * **Trait Resolution Visualizer**: Interactive trait resolution tree visualization that identifies ambiguous impls, missing bounds, and conflicting trait implementations with one-click fix suggestions.
    * **Universal Context Atlas**: Universal ISG that creates stable semantic addresses for any structured document (code, papers, contracts, medical records) with content-addressed meaning storage and semantic diffing.
    * **System Detective Integration**: Auto-detection of system capabilities (RAM, GPU, CPU, services) with zero-touch configuration that profiles hardware and recommends optimal LLM setups automatically.
    * **Graph Atlas Universal Storage**: Embedded CozoDB as universal KV+graph storage that makes trivial to store anything as (identifier, blob, edges) with zero-config setup.
    * **Intent Parser Engine**: LLM-powered intent parser that converts natural language requirements into structured, validated plans with automatic feasibility checking.
    * **Delta Reasoner System**: Calculates minimal changes from current to future state with constraint solving, impact analysis, and confidence scoring for safe transformations.
    * **Validation Loop Framework**: AI reviewing AI through multi-stage validation where AI critiques its own output, generates tests, and only shows validated results to users.
    * **Universal TUI Chat Infrastructure**: Zero-config TUI chat interface with rich rendering, LLM routing, streaming, and persistent history that works across domains.
    * **LLM Router Intelligence**: Abstracts model complexity with auto-detection, cost-aware routing, and quality-based selection that learns from usage patterns across domains.
    * **Trustworthy AI Validation System**: Multi-stage validation where AI reviews AI through rubber duck critique, automatic test generation, and iterative refinement until 95%+ confidence.
    * **Zero-Config Philosophy Engine**: Every component provides intelligent defaults with infinite customization, eliminating 30-minute setup tutorials and enabling immediate adoption.
    * **Portfolio Strategy Platform**: 7 meta-patterns × 10 domains = compound advantage where each pattern improves others through shared infrastructure and cross-domain learning.
    * **Social-Media Optimized Workflow**: Comic-book snake layout visualization that preserves complete technical nuance while being viewport-friendly for Twitter/X, LinkedIn, and Instagram sharing.
    * **Component Orchestration System**: JSON-driven build manifest with 7 modular crates (system-detective → universal-isg → graph-atlas → intent-parser → delta-reasoner → validation-loop → universal-tui-chat) that compose into intelligent tools.
    * **Multi-Variant Architecture**: Each component supports multiple implementation variants (rust-analyzer-only, tree-sitter-enhanced, incremental) enabling rapid experimentation through build.py configuration.
    * **User Journey Exception Handling**: Comprehensive loop-back flows for validation failures, infeasible solutions, test failures, and user rejection with automatic retry and graceful degradation.
    * **Quality Gate Integration**: Automated quality assurance pipeline with clippy linting, cargo fmt formatting, test coverage, documentation generation, and security auditing through build.py quality-check.
    * **Orchestrator Pattern Architecture**: Fluent API pattern for component composition where each capability can be chained together to create specialized tools for different domains.
    * **Binary vs Library Flexibility**: Dual deployment model supporting both standalone binary execution and library integration for different usage patterns and ecosystem integration.
    * **Security-First Hook System**: Proactive security validation hooks with pattern matching, session-aware warning management, and configurable security policies per environment.
    * **Containerized Development Environment**: Isolated, reproducible development environments with Docker/Podman support, environment specifications, and security profiles.
    * **Intelligent Duplicate Detection**: Semantic duplicate detection for workflow artifacts with similarity scoring, auto-resolution, and merge recommendations.
    * **Plugin Marketplace with Metadata**: Structured plugin registry with rich metadata, dependency management, quality metrics, and compatibility verification.
    * **Comprehensive Issue Tracking Integration**: Automated bug report generation, workflow execution tracking, and GitHub integration with label mapping.
    * **Agent-Based Workflow Engine**: Composable agents rather than monolithic flow with parallel execution capabilities and easy workflow variant creation.
    * **Plugin System with Hot Reloading**: Dynamic plugin loading/unloading with third-party workflow extensions and marketplace potential.
    * **Event-Driven State Management**: Decoupled state management with event bus for monitoring, logging, and observable workflow progression.
    * **MCP-Compatible Tool Interface**: Standardized tool interfaces for external integrations and future-proof AI agent ecosystem compatibility.
    * **Configuration-Driven Workflow Variants**: JSON/YAML configuration for different workflow behaviors with runtime customization and user-friendly management.
    * **Persistent Session Management**: Save/restore workflow state across sessions with audit trail and sharing capabilities between tools.
    * **Multi-Modal Interface Support**: Terminal, IDE, and web interfaces with consistent UX across platforms and easy interface addition.
    * **Advanced Permission System**: Fine-grained permissions for workflow operations with security, audit trails, and configurable policies.
    * **Command System with Hook Integration**: Extensible command system with pre/post execution hooks for decoupled cross-cutting concerns and plugin-friendly architecture.
    * **Configuration-Driven Command Routing**: JSON/YAML configuration for customizing workflow command behavior with zero-code customization and environment-specific configurations.
    * **Advanced Vim-like Editing Modes**: Multiple editing modes (Vim, Emacs, Standard, Accessible) with customizable keybindings and accessibility support.
    * **Advanced Analytics and Monitoring**: OpenTelemetry-based workflow analytics with performance optimization insights, usage pattern analysis, and privacy-conscious data collection.
    * **Claude-Inspired Agent Architecture**: Agent-based workflow engine with composable agents, plugin hot reloading, and event-driven state management for secure, extensible workflows.
    * **Multi-Modal Interface System**: Terminal, IDE, and web interfaces with support for images, web search, file uploads, and consistent UX across all platforms.
    * **Security-First Hook Integration**: Proactive vulnerability detection with pre/post execution hooks, command injection prevention, XSS scanning, and audit trail generation.
    * **Specialized Agent Ecosystems**: Three focused agent systems for Rust bug solving, codebase research, and academic research with deep domain expertise and pattern recognition.
    * **Rust Ecosystem Mastery**: Deep understanding of Cargo.toml, workspace structure, async patterns, and Rust idioms for bug analysis and idiomatic fix generation.
    * **Large Codebase Navigation**: Efficient traversal of massive projects with pattern extraction, design decision explanation, and learning resource generation.
    * **Academic Research Processing**: PDF, markdown, and text analysis with contribution extraction, citation tracking, and implementation gap identification.
    * **Multi-Journey Value Proposition**: Time reduction (80% bug solving, 90% research speed), quality improvement, and ecosystem contribution across developer workflows.
    * **Complete 3-Journey Architecture**: Unified platform integrating Rust bug solving, codebase research, and academic research with shared Syn AST parsing, Cozo database, and cross-journey learning.
    * **PRD-First Validation Architecture**: Mandatory ISG/graph validation loops before execution with multi-turn refinement, user approval gates, and failure recovery for high-confidence outcomes.
    * **Performance-Optimized Infrastructure**: Context slicing, incremental updates, query caching, and parallel processing achieving ≤5s ingestion, <100ms queries, and linear scaling.
    * **Safety-First Reliability**: Reset semantics, validation loops, error recovery, audit trails, and compile-time checks ensuring deterministic, recoverable operations.
    * **Innovation Pipeline Integration**: Research-to-production workflow connecting academic papers → pattern discovery → codebase enhancement → bug resolution with continuous learning feedback loops.
    * **Cross-Journey Learning Architecture**: Bidirectional knowledge flow where bug insights inform research patterns, academic validation guides bug analysis, and pattern knowledge connects to practice implementation.
    * **Comprehensive Performance Engineering**: Advanced optimization strategies including context slicing, incremental updates, query caching, and parallel processing achieving sub-100ms queries with 90% cache hit rates.
    * **Multi-Layered Safety Architecture**: Input validation, error boundaries, recovery strategies, safety checks, and comprehensive audit trails ensuring deterministic, recoverable operations with graph invariants.
    * **Enterprise-Grade Security**: Local-first defaults, sandboxed execution, process isolation, data encryption, security monitoring, and incident response with complete compliance checking.
    * **Implementation Timeline Management**: Phase-by-phase development with Gantt chart tracking, success metrics dashboards, and real-time KPI visualization for predictable delivery.
    * **Unified Output Integration**: Multiple export formats, cross-referenced results, GitHub integration, web dashboards, and REST API connectivity for enterprise workflow integration.
    * **Data-Driven Optimization Framework**: Real-time metrics collection, interactive dashboards, trend analysis, and automated reporting for continuous improvement and performance tuning.
    * **Machine Learning Pattern Effectiveness**: Usage data analysis for pattern effectiveness optimization, success rate improvement, and automated learning from production deployments.
    * **Complete 3-Journey Mind Map Architecture**: Unified platform vision with Rust ecosystem bug solving, academic bridge document processing, technical foundation, and developer experience layers.
    * **Comprehensive Success Metrics**: Time efficiency (80% bug resolution, 10x academic processing), quality improvements (90%+ success rates), learning impact (1000+ developers), and ecosystem growth metrics.
    * **Core Cozo Data Model Architecture**: Structured relations for sig_metadata, isg_edges, prds, future_actions, and future_code with reproducible ISG snapshots and JSON metadata.
    * **Anthropic-Compatible Configuration System**: Flexible LLM provider support (local Ollama, remote Anthropic, custom proxy) with embedded Cozo database path configuration.
    * **Critical PRD-First Architecture Update**: Mandatory ISG/graph validation loops before execution with user approval gates, preventing incorrect solutions and wasted compute through proper context validation.
    * **Implementation Requirements Architecture**: ISG Query Engine, Multi-turn Refinement, Approval Gates, Failure Recovery, and Audit Trail for high-confidence decision making.
    * **Technical North Star Vision**: Production-ready Rust tool with research-grade code analysis and innovation bridge connecting research to practice through citation networks.
    * **Architecture Validation**: Rust-exclusive tech stack, PRD iteration loops integrated, journey-specific primary key strategies, and no critical architectural inconsistencies identified.
    * **Architecture Validation Framework**: Complete P15 validation process ensuring Rust-exclusive tech stack compliance, PRD iteration loop integration, journey-specific primary key strategies, and zero critical architectural inconsistencies through systematic validation protocols.
    * **Production-Ready Implementation Path**: Technical implementation requirements derived from P14 architecture including ISG Query Engine, Multi-turn Refinement, Approval Gates, Failure Recovery, and Audit Trail components for high-confidence decision making.
    * **Three-Journey Technical Architecture**: Complete integration of Rust bug solving, codebase research, and academic research workflows with shared Syn AST parsing, Cozo database, and cross-journey learning feedback loops for maximum developer impact.
    * **Mandatory PRD-First Validation**: Critical architecture update requiring ISG/graph validation loops before any execution, preventing incorrect solutions and wasted compute through proper context validation with user approval gates.
    * **Technical North Star Alignment**: Production-ready Rust tool with research-grade code analysis and innovation bridge connecting research to practice through citation networks and implementation gap identification.
    * **Mode Profiles Configuration**: Fast|balanced|accuracy-first modes with configurable SLOs per tool and pipeline, selecting latency vs coverage vs rigor without changing contracts.
    * **Confidence Gate Framework**: Calibrated probability scoring using coverage, agreement, consistency, and static_precheck features with mode-specific thresholds (0.70 fast, 0.80 balanced, 0.90 accuracy-first).
    * **Multi-Level ISG Architecture**: ISGL1-ISGL5 simultaneous extraction with single-pass generation, hierarchical relationships, and query flexibility across crate/module/type/function/statement granularities.
    * **Journey-Specific Data Models**: Separate CozoDB relations per journey with namespace isolation, cross-journey joins when needed, and clean separation of concerns.
    * **Hybrid rust-analyzer Integration**: Two-stage extraction combining syn syntax parsing with rust-analyzer semantic enrichment for type information, trait analysis, lifetime info, and call hierarchy.
    * **Production-Ready Technology Stack**: Validated Rust-exclusive components with accepted C++ dependencies (RocksDB, LaTeX tools) and community SDK integration (anthropic-sdk-rust).
    * **Incremental Intelligence System**: Delta-based ISG rebuilding with semantic fingerprinting, change propagation graphs, and real-time analysis on monorepos with <100ms update latency.
    * **Zero-Configuration Philosophy**: Every component provides intelligent defaults with infinite customization, eliminating setup tutorials and enabling immediate adoption through auto-detection.
    * **Journey-Specific Data Models**: Separate CozoDB relations per journey with namespace isolation (j1_isg_nodes, j2_pattern_nodes, j3_doc_chunks), cross-journey joins when needed, and clean separation of concerns.
    * **Production-Validated Technology Stack**: All components validated including pure Rust lopdf for PDF processing, anthropic-sdk-rust community SDK integration, and accepted C++ dependencies (RocksDB, LaTeX tools) with clear justification.
    * **Single-Pass Multi-Level Extraction**: Generate ISGL1-ISGL5 simultaneously in one codebase traversal with single I/O pass, consistent metadata, automatic parent-child links, query flexibility, and no re-parsing overhead.
    * **Cross-Journey Key Translation**: Bidirectional links between code-centric keys (filepath::module::interface), pattern-centric keys (pattern_type::crate::location), and document-centric keys (doc_hash::chunk_idx::summary) with unified graph support.
    * **PRD Iteration Loop Integration**: Mandatory ISG validation before execution with multi-turn refinement, user approval gates, failure recovery, and context validation across all three journeys for high-confidence decision making.
    * **Community SDK Integration Strategy**: Use anthropic-sdk-rust (~50 LOC integration) vs custom implementation (~1,500-2,500 LOC), providing feature-complete streaming, tools, vision, and type safety with active community maintenance.
    * **Cross-Journey Unified Graph Architecture**: Unified graph with multiple key namespaces and bidirectional links between code-centric keys (filepath::module::interface), pattern-centric keys (pattern_type::crate::location), and document-centric keys (doc_hash::chunk_idx::summary).
    * **Architecture Risk Resolution Framework**: Comprehensive risk assessment with resolved issues including PRD iteration loops, namespace collision solutions, and validated technology stack decisions with clear acceptance criteria.
    * **Performance Validated CozoDB Architecture**: RocksDB-backed persistent storage with ACID transactions, 100K+ QPS performance, and use case optimization for ISG persistence, PRD history, pattern libraries, and citation networks.
    * **Implementation-Ready Development Roadmap**: Clear path forward with multi-level ISG extraction (ISGL1-L5), journey-specific CozoDB relations, and anthropic-sdk-rust integration with all design decisions finalized.
    * **Production-Ready Component Validation**: All critical components validated including pure Rust lopdf for PDF processing, community SDKs for LLM integration, and accepted C++ dependencies with performance justification.
    * **Complete Architecture Validation Summary**: Comprehensive P15 validation framework with resolved all critical issues, validated technology stack (RocksDB, CozoDB, anthropic-sdk-rust), and systematic validation protocols ensuring zero critical architectural inconsistencies.
    * **Validated Development Roadmap**: Clear implementation path with all design decisions finalized, multi-level ISG extraction (ISGL1-L5), journey-specific CozoDB relations, and systematic validation protocols with no remaining critical architectural decisions.
    * **Specialized Sub-Agent Architecture**: 7-8 parallel sub-agents with context length preservation strategy, using tiny models (MiniLM 22M, SmolLM2 135M, Gemma 270M) for data processing and Qwen 14B for deep reasoning, achieving 10x context compression while maintaining information density.
    * **Hybrid CPU-Optimized Search**: CozoDB integration with HNSW vector indexing and Datalog graph queries, enabling exact/graph/relational searches at 250K+ QPS with semantic similarity search, perfect for ISG workflows without GPU requirements.
    * **Multi-Journey Agent Orchestration**: Specialized agent teams for bug solving, pattern research, and code generation with parallel processing (5-10 search agents, 2-3 validation agents, 1-2 refinement agents, 1 reasoning agent) fitting within 16GB RAM constraints.
    * **Context Budget Optimization**: Filter → Enrich → Reason → Validate pipeline where sub-agents process 35-40K tokens in parallel but only pass 5-10K enriched summaries to reasoning LLM, preserving 50% context budget for deep thinking and edge case analysis.
    * **128K Context Model Strategy**: Hybrid approach using Qwen2.5-Coder-14B with 128K context for academic research (Journey 3), while maintaining sub-agent architecture for bug fixing (Journey 1) and pattern research (Journey 2), optimizing for speed vs accuracy trade-offs.
    * **Memory-Optimized Agent Distribution**: 16GB Mac Mini support with 5-10 search agents (2-5GB), 2-3 validation agents (1-2GB), 1-2 refinement agents (1-2GB), 1 reasoning agent (8-10GB), CozoDB (50-500MB), fitting within 13-16GB total usage with room to spare.
    * **Performance Benchmarking**: 5MB codebase processing in 1 minute vs 10+ minutes sequential, with specific speedups: ISG build 3-4x faster, bug analysis 5-7x faster, pattern search 5-10x faster, code generation 4-6x faster through parallel agent orchestration.
    * **Complete Rust Workspace Implementation**: Production-ready coordinator crate with tokio async parallelism, CozoDB embedded storage, and multi-agent orchestration using llama.cpp servers on separate ports, enabling 10-30+ parallel agents within 16GB RAM constraints.
    * **Journey-Specific Workflow Integration**: Detailed agent mapping for bug solving (Journey 1), pattern research (Journey 2), and code generation (Journey 3) with specialized search, validation, refinement, and reasoning agents optimized for each use case.
    * **Hybrid CozoDB-Agent Architecture**: Mixed search strategy combining Datalog exact queries, HNSW vector similarity, and graph algorithms with agent post-processing, achieving <1ms exact queries, <10ms vector searches, and 250K+ QPS performance on CPU-only systems.
    * **Universal Agent Roster System**: 9-agent architecture (A1-A8 + R1) with standardized contracts working across all journeys, featuring scope seeding, exact/vector retrieval, classification, pattern tagging, constraint validation, summarization, change forecasting, and heavy reasoning with 5-8x context compression.
    * **Journey-Specific Configuration Framework**: Same agent roles with journey-tuned parameters (J1: latency-optimized 20K context, J2: hybrid 128K + sub-agents, J3: accuracy-optimized 128K context), achieving optimal speed vs accuracy trade-offs with performance targets of 60s for bugs, 45-75s for patterns, 90-120s for academic research.
    * **Confidence Gating with Mathematical Formula**: C = 0.35×coverage + 0.25×agreement + 0.25×consistency + 0.15×static_precheck, triggering cargo validation only when C ≥ 0.70 and no high-severity risks, ensuring safe autonomous operations with calibrated probability scoring.
    * **Production-Ready Rust Coordinator Skeleton**: Complete async tokio orchestration with semaphore-controlled concurrency, CozoDB integration, HTTP clients for tiny LLM models, and systematic 7-phase pipeline from scope seeding through cargo validation with proper error handling.
    * **Structured Data Contracts**: JSON-based RetrievalItem, ContextPack, Evidence, Risk, and IsgOperation schemas enabling standardized agent communication with token estimation, evidence tracking, and future ISG operation planning with minimal context overhead.
    * **Complete Workspace Architecture**: Modular parseltongue workspace with coordinator, agents, reasoner, and CozoDB storage components, featuring HTTP wrapper servers for each model and production-ready implementation checklist with context mode switching and RAM detection.
    * **Shreyas/Jeff Dean Strategic Synthesis**: Product impact + systems thinking with journey-specific optimization, visual Mermaid diagrams, and performance targets (60s for bugs, 45-75s for patterns, 90-120s for academic), combining high-level strategy with concrete implementation details.
    * **"Lost in Middle" Mitigation Strategy**: Context vs accuracy trade-off awareness with hybrid strategies combining 128K breadth with sub-agent precision, journey-specific context sizing, and blast radius definition as Exact(seeds, radius=N) ∪ Vector(seeds, k=K) for optimal retrieval precision.
    * **Implementation Roadmap with Checklist**: Ready-to-build workspace structure with comprehensive implementation checklist covering CozoDB schema creation, agent trait definitions, tokio orchestration, HTTP clients, context packaging, confidence gating, cargo pipeline, and advanced features like context mode switching and RAM detection for optimal deployment.
    * **128K Context Intelligence**: Journey-specific context strategy using 128K models for academic research (J3) while maintaining sub-agents for bug fixing (J1), with hybrid approach for pattern research (J2) optimizing for speed vs accuracy trade-offs.
    * **"Lost in Middle" Mitigation Engine**: Context positioning optimization that places critical information at start/end of 128K context windows, achieving 80-95% attention accuracy by avoiding middle 60% attention drop-off through intelligent content reordering.
    * **Memory-Aware Model Selection**: Dynamic model routing based on available RAM and context requirements, automatically selecting between 20K sub-agent mode (8-10GB) and 128K context mode (14-18GB) with graceful fallback for resource-constrained environments.
    * **Cost-Optimized API Strategy**: 125x cost reduction for API deployments through sub-agent filtering (10K tokens vs 1.25M tokens), making large-scale analysis economically viable while maintaining high accuracy through targeted context curation.
    * **Performance Benchmarking Framework**: Comprehensive decision matrix comparing sub-agents vs 128K context across speed (45-60s vs 75-120s), accuracy (88% vs 62% for bugs), memory usage (8-10GB vs 14-18GB), and API cost ($0.10 vs $12.50 per run) with journey-specific recommendations.
    * **Context Window Optimization**: Intelligent context sizing based on task characteristics with <80K tokens for pure 128K mode, >80K tokens for hybrid mode, and automatic "lost in middle" warnings when context exceeds optimal attention thresholds.
    * **Journey-Specific Model Architecture**: Optimized model selection per journey - sub-agents for Journey 1 (bug fixing), hybrid 128K + agents for Journey 2 (pattern research), pure 128K for Journey 3 (academic research) - maximizing both speed and accuracy.
    * **Real-Time Performance Monitoring**: Live metrics collection for inference time, memory usage, and accuracy across different context strategies, enabling dynamic optimization and automatic model switching based on performance feedback.
    * **Future-Proof 1M+ Context Strategy**: Scalable architecture ready for next-generation models (Claude 3.5, Gemini 1.5, GPT-5) with sub-agent remaining relevant for latency-sensitive, cost-sensitive, and precision-critical tasks despite larger context windows.
    * **Production Usage Case Studies**: Real-world validation from tokio bug fixing (45s, 88% success vs 75s, 62% success), serde pattern research (75s, 91% accuracy with hybrid), and academic literature review (120s comprehensive synthesis) proving journey-specific optimization.
    * **Dynamic Memory Management**: Runtime RAM detection and automatic model switching with fallback strategies, preventing OOM conditions on 16GB systems while maximizing performance across different hardware configurations.
    * **Cost-Effectiveness Optimization**: 125x API cost reduction through intelligent filtering, making enterprise-scale deployment economically viable while maintaining high accuracy through surgical context selection.
    * **Implementation Readiness**: Simple model configuration changes with comprehensive error handling, requiring only context length parameter adjustments while maintaining robust memory management and graceful degradation.
    * **Benchmark-Driven Development**: Data-backed decision framework with comprehensive metrics across speed, accuracy, memory, and cost dimensions, enabling evidence-based architecture decisions.
    * **Journey-Specific Performance Targets**: Calibrated expectations per journey (60s bug fixing, 45-75s pattern research, 90-120s academic) with measured accuracy benchmarks (88%, 91%, 95% respectively).
    * **Context Mode Flexibility**: Runtime switching between sub-agent, 128K, and hybrid modes based on task characteristics, resource availability, and user preferences without architectural changes.
    * **Research-Backed Architecture**: Implementation based on academic studies (Liu et al., 2023) about attention patterns in long contexts, ensuring scientifically-sound design decisions.
    * **Scalable Decision Framework**: Extensible decision trees and configuration matrices that accommodate new models, larger contexts, and emerging use cases without requiring architectural overhauls.
    * **Pattern Database Architecture**: Comprehensive Rust knowledge base with 150 patterns, 100 anti-patterns, and 200 error mappings, transforming bug fixing from generic to specialized with 90-95% accuracy through pattern-guided fixes.
    * **RataTUI ISG Indexing Pipeline**: Visual 4-phase indexing (AST parsing, edge extraction, vector embeddings, CozoDB indexing) with progress bars and real-time statistics, enabling 3-4 minute one-time setup for complete codebase understanding.
    * **Journey-Specific Bug Fixing Workflow**: Complete 5-phase journey from user bug report through pattern discovery, validation, reasoning, cargo validation, and learning - achieving 60-90s resolution time with 30-90x speedup vs manual debugging.
    * **Pattern-Guided Agent Orchestration**: Enhanced sub-agent architecture where A1 maps errors to patterns, A4 detects anti-patterns (95% confidence), A5 suggests idiomatic patterns with example code, and A6 validates constraints, creating a Rust-specialized expert system.
    * **Error-to-Pattern Intelligence**: Instant mapping of compiler errors to fix patterns ("cannot be sent" → async_spawn_send) with historical effectiveness tracking and confidence boosting from 70% to 85-95% through pattern recognition.
    * **Anti-Pattern Detection System**: Proactive identification of known bad practices with 95% confidence, enabling prevention-focused fixes rather than reactive solutions through pattern database awareness.
    * **Context Reordering Strategy**: Strategic placement of critical information at START/END of 128K context windows (80% attention zones) to overcome "lost in middle" problem and achieve 85-90% accuracy with spacious reasoning buffer.
    * **Incremental Learning Loop**: Pattern effectiveness tracking where every fix updates success metrics, enabling continuous improvement with 94% historical success rates for top patterns and growing intelligence per codebase.
    * **Comprehensive Phase Architecture**: Clear separation between discovery (parallel sub-agents), validation (pattern-aware analysis), reasoning (128K context with examples), validation (cargo pipeline), and learning (pattern updates) for predictable 60-90s bug resolution.
    * **Production-Ready Implementation Roadmap**: 8-week phased development plan from core infrastructure through learning systems, with detailed weekly milestones and validation criteria for enterprise deployment.
    * **Product Vision Integration**: Shreyas Doshi JTBD framework (time to validated fix) combined with Jeff Dean systems thinking (phase-separated parallelism) creating journey-aware orchestration optimized for developer workflows.
    * **Rust Ecosystem Specialization**: Deep integration with cargo, rust-analyzer, syn AST parsing, and idiomatic Rust patterns, creating a domain expert rather than generic code assistant through specialized pattern knowledge.
    * **Performance Benchmarking**: Comprehensive metrics showing 90-95% accuracy (vs 85-90% baseline), 85-95% confidence (vs 75-85% baseline), 85% first-try success (vs 70% baseline), and 95% idiomatic fixes (vs 60% baseline) through pattern integration.
    * **RataTUI ISG Visualization Pipeline**: Real-time visual indexing interface with 4-phase progress bars (AST parsing, edge extraction, vector embeddings, CozoDB indexing), showing file counts, node statistics, and ETAs for transparent 3-4 minute one-time setup.
    * **Detailed ISG Metadata Architecture**: Comprehensive node structure including L1-L5 hierarchies, semantic embeddings, constraint metadata, test classification, historical bug records, change frequency tracking, and documentation indexing for complete codebase understanding.
    * **Multi-Phase Indexing Pipeline**: Automated 4-stage process (syn AST parsing → edge extraction → MiniLM vector embeddings → CozoDB HNSW indexing) with incremental updates (5-15s) and performance scaling across codebase sizes from small (30-60s) to XL (3-5min).
    * **CozoDB Schema Design**: Complete Datalog schema with ISG nodes, edges, embeddings, constraints, bug history, and HNSW indices enabling sub-10ms vector queries, ACID transactions, and persistent storage with compression.
    * **Pattern-Aware Agent Roles**: Enhanced sub-agent architecture where A1 performs error-to-pattern mapping, A2/A3 handle pattern-aware retrieval, A4-A6 provide anti-pattern detection and pattern validation, creating specialized Rust expertise.
    * **Context Optimization Strategy**: Intelligent packing with START (80% attention), EARLY (70%), MIDDLE (40%), END (70%) zones, achieving 85-90% accuracy through strategic information placement and 113K reasoning buffer.
    * **Comprehensive Performance Timeline**: Detailed 4-minute first-run breakdown and 60-90s subsequent runs with specific timing per phase, enabling predictable performance and user experience management.
    * **Learning and Evolution System**: Historical bug storage, pattern effectiveness tracking, similar bug lookup via HNSW, and growing intelligence per codebase with 94% success rates for top patterns.
    * **Edge Case Handling**: Systematic solutions for out-of-radius bugs, model timeouts, multiple bugs, and flaky tests with fallback strategies and graceful degradation.
    * **JSON Data Contracts**: Structured schemas for all agent I/O including seeds, candidates, validation results, and final fixes with evidence tracking and confidence scoring.
    * **Simplified Agent Roster**: Optimized 4-agent architecture (A1 STLM 50M, A2/A3 CozoDB, A4-A6 Gemma 270M) plus R1 Qwen 7B (128K) achieving 12-14GB RAM usage and fitting 16GB Mac Mini constraints.
    * **Confidence Gating Framework**: Mathematical formula C = 0.35×coverage + 0.25×agreement + 0.25×consistency + 0.15×static_check with 0.75 threshold and iterative refinement loops.
    * **Comprehensive Validation Pipeline**: Sequential cargo validation (fmt → check → build → test) with error routing, temporary workspaces, and deterministic builds for reliable fix verification.
    * **Pure CozoDB Architecture**: Complete graph-relational-vector system with mother relations (interface_code as central source of truth), Datalog recursive queries for blast radius analysis, HNSW vector indexing, and built-in time travel for rollback capability.
    * **Pattern-Guided ISG Transformation**: Integration of P00's ISG transformation philosophy with P20's pattern intelligence, where 150+ patterns guide micro-PRD generation and ISG_future modeling before code generation, enabling predictive validation.
    * **Production-Ready Schema Design**: Complete CozoDB schema with 12 core relations (interface_code, pattern_library, transformation_plan, micro_prds, etc.) supporting versioning, feasibility checks, pattern effectiveness tracking, and historical learning.
    * **Research-Backed Pattern Categories**: Systematic classification based on 2024 Rust bug studies - AsyncSync (40% of bugs), Ownership (30%), ErrorHandling (15%), GenericBounds (10%), Concurrency (5%) - enabling targeted pattern matching.
    * **Advanced Datalog Query Examples**: Production-ready queries for pattern-guided error mapping, blast radius analysis, micro-PRD generation, ISG_future transformation, feasibility validation, pattern effectiveness learning, and time travel rollback operations.
    * **Time Travel and Rollback Capability**: Native CozoDB versioning system with parent-child relationships, snapshot management, rollback availability, and complete audit trails for safe experimental transformations.
    * **Micro-PRD Generation Framework**: Automated specification creation (200-500 tokens) from bug + pattern analysis, with traceable requirements, success criteria, pattern recommendations, and interface mapping for predictable development.
    * **Feasibility Validation System**: Pre-transformation checking of caller constraints, cycle detection, test reachability, and compatibility analysis to prevent breaking changes before code generation.
    * **Pattern Effectiveness Learning**: Continuous improvement system tracking times_suggested, times_accepted, success_rate, avg_confidence per repository with adaptation notes and historical accuracy metrics (94% success rate).
    * **Complete Module Architecture**: 5-module system (ISG Core, Pattern Intelligence, Sub-Agent Coordination, PRD Iteration Loop, Learning & Versioning) providing comprehensive bug fixing capabilities with research-validated patterns.
    * **Performance Validation Metrics**: Research-backed targets showing 90-95% accuracy (vs 85-90% baseline), 0.85-0.95 confidence (vs 0.75-0.85), 95% idiomatic fixes (vs 60%), with 60-90s resolution time after 3-4 minute setup.
    * **Graph-Relational-Vector Integration**: Unified system combining graph queries (dependency traversal), relational operations (structured data), and vector search (pattern similarity) in single CozoDB instance for optimal performance.
    * **Rust-Analyzer Overlay Validation**: Zero-risk code validation layer using in-memory RA overlays before any file I/O, enabling deterministic type/trait diagnostics with sub-5s validation and 95%+ error coverage.
    * **PreFlight Architecture Pattern**: Complete validation workflow between ISG_future and Apply phases, with candidate queue management, results tracking, policy configuration, and CozoDB orchestration for bulletproof code generation.
    * **Shadow-Free Workspace Design**: Lightweight validation approach eliminating temporary workspaces by using RA overlays for type checking and real workspace cargo check for compilation validation, achieving 1.5-5s total validation time.
    * **LSP Protocol Integration**: Complete JSON-RPC communication with rust-analyzer for candidate validation, including didOpen, publishDiagnostics, and didClose operations with configurable timeouts and severity gates.
    * **Diagnostics-to-ISG Mapping**: Perfect explainability through file_spans relation mapping diagnostics back to specific ISG nodes, enabling root-cause analysis and R1 feedback loops for refinement.
    * **CozoDB PreFlight Schema**: Three new relations (preflight_queue, preflight_results, preflight_policy) extending P21's schema with candidate tracking, validation outcomes, and workspace configuration.
    * **Guardrails and Enforcement**: CozoDB-side guards blocking transformation_plan execution without PreFlight passing, ensuring zero-risk code application with complete audit trails.
    * **Performance Optimization**: Shadow-free flow achieving 1.5-5s validation (vs 15-30s with shadow), 90-95% validation coverage, and ~100-200MB memory usage per RA instance.
    * **RA Process Wrapper Architecture**: Language-agnostic JSON-RPC over stdio communication with instance pooling per workspace configuration and candidate serialization for validation processing.
    * **Implementation Readiness**: Complete checklist including schema extensions, RA wrapper implementation, CozoDB guards, lightweight cargo check integration, and diagnostic mapping for production deployment.
    * **Zero-I/O Risk Validation**: Complete safety guarantee where no repository changes occur until PreFlight passes, with type safety validation in-memory and compilation safety on real workspace before any disk writes.
  > **Reliability-First Summary**: ISG builder implements deterministic validation through rust-analyzer overlays and cargo check gates. Uses bounded concurrency (Tokio runtime) with cooperative yields for UI responsiveness. Caching strategies (ISG + HNSW persisted) ensure sub-100ms query performance. Error recovery uses thiserror patterns with complete diagnostic surfaces. Memory pressure managed via quantization (Q4_K_M) and dynamic parallelism throttling. Mode profiles select latency vs coverage vs rigor without changing contracts.

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
  - Diverse Ideas:
    * **Adaptive Embedding Sizing**: Dynamic dimension selection based on code complexity; simple functions get 256-dim, complex architectures get 1536-dim vectors for optimal memory/performance trade-offs.
    * **Multi-Modal Fusion**: Joint embedding of code, documentation, and test cases into unified vector space; enables cross-modal retrieval between implementation and specification.
    * **Curriculum Learning**: Progressive embedding training where simple patterns learned first, then fine-tuned on complex architectural patterns; improves robustness across codebases.
    * **Hierarchical Navigable Small World with Pruning**: HNSW variant that automatically removes redundant edges and clusters similar vectors; reduces query latency by 40% while maintaining recall.
    * **Hardware-Aware Quantization**: M1/M2 Metal-optimized quantization schemes (Q3_K_M for Mac, Q4_K_M for cross-platform) with automatic fallback to CPU based on available VRAM.
    * **Embedding Drift Detection**: Continuous monitoring of embedding quality over time; automatically flags when codebase changes require index rebuild due to semantic shift.
    * **Zero-Shot Transfer Learning**: Pre-trained embeddings from large code corpora that can be fine-tuned on specific codebases with as few as 100 examples.
    * **Temporal Embedding Alignment**: Aligns embeddings across different time periods to enable consistent retrieval even as code evolves and conventions change.

- hybrid-retrieval-engine
  - Purpose: Combine Datalog two-hop with vector KNN; rank by L1>L2>L3 and pattern/idiom scores.
  - Inputs: seeds, hints, constraints.
  - Outputs: Needed shortlist (≤50) with ranks and features.
  - Actions: exact graph 2-hop → vector search → merge/dedup → rank → emit shortlist.
  - Variants: toggleable weights, L1-only mode, vector-only fallback.
  - Example CLI: hybrid-retrieval-engine --cozo cozo://isg --seed "E0277" --k 50
  - Example Input (JSON): {"seeds":["src/executor.rs-spawn_task"],"hints":["Send","'static"]}
  - Example Output (JSON): {"needed":[{"isgl1_key":"src/service.rs-run","rank":0.92},{"isgl1_key":"src/executor.rs-spawn_task","rank":0.89}]}
  - Diverse Ideas:
    * **Multi-Hop Reasoning Chain**: Graph traversal beyond 2-hops using beam search with dynamic pruning; enables deep reasoning across complex dependency chains while maintaining sub-second performance.
    * **Contextual Reranking**: Learning-to-rank model that optimizes for specific developer contexts (bug fixing vs feature development); improves precision by 35% through personalized scoring.
    * **Neural-Symbolic Fusion**: Combines symbolic graph reasoning with neural similarity scores; uses differentiable reasoning to learn optimal traversal strategies from developer interaction data.
    * **Temporal Decay Weighting**: Time-aware scoring that prefers recently modified code and recent patterns; automatically adapts to changing codebase conventions and deprecation cycles.
    * **Query Intent Classification**: Automatic detection of user intent (debug, refactor, learn) and adaptive retrieval strategy selection; matches the right algorithmic approach to the user's mental model.
    * **Distributed Query Processing**: Parallel query execution across multiple cores and even multiple machines for large codebases; scales to millions of interfaces with linear performance improvements.
    * **Explainable Retrieval**: Generates human-readable explanations for why each result was selected (graph path, similarity score, pattern match); builds trust and teaches developers about their own codebase.
    * **Adaptive Result Window**: Dynamic adjustment of result count based on query complexity and confidence; shows 5 results for simple lookups, 50 for complex explorations.

- pattern-knowledge-base
  - Purpose: Store idiomatic fixes and anti-patterns with examples and thresholds.
  - Inputs: patterns.yaml, examples/, thresholds.
  - Outputs: query API; usage metrics.
  - Actions: nearest pattern lookup; thresholding; example retrieval.
  - Variants: project-specific overlays; learned scoring.
  - Example CLI: pattern-knowledge-base query --hint "async spawn send"
  - Example Input (JSON): {"hint":"E0277 across await"}
  - Example Output (JSON): {"pattern_id":"async_spawn_send","distance":0.12,"example":"add Send + 'static bounds"}
  - Diverse Ideas:
    * **Pattern Evolution Engine**: Self-learning pattern discovery that identifies recurring idioms from successful PRs and auto-generates new templates; builds training data from every successful refactor.
    * **Paper-to-PRD Pattern Bridge**: Extracts architectural patterns from research papers and converts them into actionable code patterns with implementation templates and risk assessments.
    * **Study Protocol Pattern Library**: Converts research methodologies into reusable patterns for experimental code scaffolding, parameter tuning, and result validation.
    * **Macro Pattern Recognition**: Identifies common derive macro patterns and procedural macro idioms; generates specialized templates for macro-heavy codebases.
    * **Cargo Ecosystem Navigator Patterns**: Stores dependency update patterns, migration strategies, and common breakage scenarios across the Cargo ecosystem.
    * **Cross-Language Pattern Mapping**: Maps equivalent patterns between Rust, C++, and Python (e.g., RAII, async patterns, error handling) for polyglot teams.
    * **Performance Pattern Analyzer**: Identifies performance-critical patterns (allocation patterns, async boundaries, lock contention) with micro-benchmarking data.
    * **Trait Implementation Patterns**: Catalogs common trait implementation patterns (Builder, Iterator, Future) with automatic suggestion of missing trait bounds.
    * **Claude Code Plugin Pattern Integration**: Specialized pattern templates for Claude Code commands, hooks, agents, and workflows with plugin.json compatibility validation and MCP integration patterns.
    * **Multi-Agent Coordination Patterns**: Agent orchestration patterns for parallel discovery, validation, reasoning, and safety phases with standardized contracts and communication protocols.
    * **Journey-Specific Pattern Curation**: Pattern subsets optimized for bug solving (Journey 1), pattern research (Journey 2), and academic research (Journey 3) with journey-specific success metrics.
    * **Test-First Architecture Patterns**: Executable specification patterns for contracts, preconditions, postconditions, and error conditions with DI scaffolding and mock generation.

- constraints-overlay-analyzer
  - Purpose: rust-analyzer didOpen buffers; collect diagnostics; compute required/current/missing bounds.
  - Inputs: candidate buffers (from CodeGraph.Future_Code).
  - Outputs: structured diagnostics and bounds delta.
  - Actions: open ephemeral buffers → collect publishDiagnostics → compute bounds delta.
  - Variants: rustc --error-format=json fallback; multi-root workspaces.
  - Example CLI: constraints-overlay-analyzer --diff-id 1234
  - Example Input (JSON): {"buffers":[{"path":"src/lib.rs","text":"..."}]}
  - Example Output (JSON): {"diagnostics":[{"code":"E0277","range":[12,5,12,18]}],"bounds":{"required":["T: Send"],"missing":["T: Send"]}}
  - Diverse Ideas:
    * **Refactor Confidence Pre-Analysis**: Before making changes, analyze proposed modifications to predict exact ripple effects, missing test coverage gaps, and borrow-checker hotspots with calibrated confidence scores.
    * **Trait Resolution Debugging**: Interactive trait resolution tree visualization that identifies ambiguous impls, missing bounds, and conflicting trait implementations with step-by-step explanation.
    * **ABI Compatibility Checking**: Cross-language FFI boundary analysis for Rust↔C/C++ bindings with layout validation, calling convention verification, and automatic binding updates.
    * **Macro Expansion Tracking**: Multi-stage macro expansion debugging with ISG node tracking, hygiene violation detection, and interactive step-through visualization.
    * **Dependency Blast Radius Analysis**: Cargo ecosystem navigator that analyzes dependency update impact across entire workspace with CVE detection and migration planning.
    * **Borrow Checker Hotspot Detection**: Advanced lifetime and borrowing analysis that identifies potential borrow checker issues before compilation with visual lifetime diagrams.
    * **Unsafe Block Analysis**: Comprehensive unsafe code analysis with memory safety verification, FFI boundary checking, and unsafe block minimization suggestions.
    * **Async Boundary Validation**: Async/await boundary analysis that identifies blocking operations in async contexts, Send/Sync boundary violations, and async runtime compatibility issues.

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
  - Diverse Ideas:
    * **Proof-Carrying Code**: Generates formal correctness proofs alongside patches; uses Hoare logic and separation logic to verify that patches maintain invariants and don't introduce new bugs.
    * **Multi-Patch Synthesis**: Simultaneously generates multiple alternative patches for the same issue; ranks them by code quality metrics (cyclomatic complexity, performance impact, readability).
    * **Semantic Patch Diffing**: Understands code semantics beyond syntax; can safely refactor function signatures, trait implementations, and async boundaries while preserving behavior.
    * **Patch Impact Simulation**: Pre-executes patches in sandboxed environment to measure performance impact, test coverage changes, and dependency ripple effects before applying.
    * **Adversarial Testing**: Automatically generates edge cases and property-based tests to validate patch correctness; uses fuzzing to find subtle bugs in generated patches.
    * **Contextual Template Selection**: Chooses patch templates based on surrounding code style, architectural patterns, and team conventions; maintains consistency with existing codebase.
    * **Incremental Refactoring**: Supports long chains of small, safe patches that together achieve complex refactoring goals while keeping each step compilable and testable.
    * **Patch Composition Algebra**: Mathematical framework for combining multiple patches safely; ensures that overlapping or interacting patches don't introduce conflicts.

- reasoning-adapter-bridge
  - Purpose: Uniform API to local llama.cpp and cloud LLMs; confidence scoring.
  - Inputs: ContextBundle; model params.
  - Outputs: diff + confidence + alt candidates (optional).
  - Actions: call backend (local/cloud) → parse tool output → score confidence.
  - Variants: streaming vs batch; single vs multi-turn; temperature ladder.
  - Example CLI: reasoning-adapter-bridge --backend qwen7b --bundle bundle.json
  - Example Input (JSON): {"model":"qwen7b","context":{"tokens":2800}}
  - Example Output (JSON): {"diff":"...","confidence":0.87}
  - Diverse Ideas:
    * **Ensemble Reasoning**: Combines outputs from multiple models (local small + cloud large) using voting mechanisms and confidence-weighted averaging; achieves higher reliability than any single model.
    * **Chain-of-Thought Verification**: Generates multiple reasoning paths and selects the most consistent one; detects logical fallacies and self-contradictions in model reasoning.
    * **Adaptive Model Selection**: Automatically chooses optimal model based on query complexity, latency requirements, and cost constraints; uses small models for simple tasks, large models for complex reasoning.
    * **Confidence Calibration**: Bayesian confidence estimation that correlates with actual accuracy; calibrated probabilities enable reliable decision-making under uncertainty.
    * **Few-Shot Context Optimization**: Dynamically selects optimal examples from few-shot library based on semantic similarity to current query; improves model performance with minimal context.
    * **Multi-Modal Reasoning**: Integrates code, documentation, and visual diagrams (architecture diagrams, flowcharts) for comprehensive understanding; supports input from multiple formats simultaneously.
    * **Privacy-Preserving Split Computing**: Keeps sensitive code context on local device while using cloud for generic reasoning; splits computation to maintain security while leveraging cloud capabilities.
    * **Real-Time Feedback Loop**: Learns from user corrections to improve future suggestions; builds user-specific model of preferences and common patterns over time.

- local-orchestrator-daemon
  - Purpose: Run multiple llama.cpp models in parallel under strict RAM/GPU caps; JSON-RPC.
  - Inputs: job graph; model registry.
  - Outputs: per-job artifacts, logs, metrics.
  - Actions: schedule jobs → cap decoders → reuse KV → collect metrics.
  - Variants: 7B exclusive vs 3×3B + small; KV reuse; GPU layer downshift on pressure.
  - Example CLI: local-orchestrator-daemon serve --socket /tmp/llm.sock
  - Example Input (JSON-RPC): {"method":"run","params":{"model":"qwen3b","prompt":"...","max_tokens":256}}
  - Example Output (JSON): {"job_id":"J123","tokens_per_sec":95,"result":"..."}
  - Diverse Ideas:
    * **Temporal ISG Memory System**: Persistent incremental knowledge base that survives restarts and explains "why this code exists" by linking commits, PR discussions, and design decisions to ISG nodes with semantic search capabilities.
    * **Live Codebase Knowledge Graph**: Real-time integration of git events, PR discussions, and design decisions into a temporal CozoDB graph that enables natural language queries about code history and rationale.
    * **Study Protocol Generator**: Converts research papers into executable study protocols with experiment scaffolding, replication checklists, and compute budget estimation for ML practitioners.
    * **Macro Expansion Orchestrator**: Manages multi-stage macro expansion pipelines with ISG node tracking, hygiene violation detection, and interactive step-through visualization for derive macro debugging.
    * **Cross-Language Bridge Orchestrator**: Coordinates unified ISG across Rust↔C/C++↔Python with ABI compatibility checking, automatic binding updates, and cross-language refactoring support.
    * **Dependency Update Coordinator**: Orchestrates Cargo ecosystem navigation with deep dependency graph analysis, security scanning, and update impact prediction across entire workspaces.
    * **Trait Resolution Orchestrator**: Manages trait solver visualization with ambiguity detection, impl suggestion engine, and interactive trait resolution tree debugging.
    * **Protocol Card Engine**: Orchestrates paper-to-protocol conversion with structured extraction, validation, and scaffold generation for research replication workflows.
    * **Claude Code Plugin Orchestrator**: Manages Claude Code plugin lifecycle with hot reloading, command registration, agent orchestration, and hook execution with proper error boundaries and resource isolation.
    * **Multi-Agent Workflow Coordinator**: Orchestrates complex multi-agent workflows (A1-A6 + R1) with parallel execution, result aggregation, confidence scoring, and human approval gates for enterprise deployment.
    * **Reliability-First Optimization Engine**: Prioritizes first-apply correctness over speed with deterministic transforms, CPU-bound static analysis, and minimal LLM usage while achieving 95%+ success rates through confidence gating.
    * **Apple Silicon Performance Tuner**: Hardware-aware optimization for M1/M2/M3 with Metal-accelerated quantization (Q4_K_M), dynamic parallelism throttling, and memory pressure management for 16GB+ systems.
    * **Journey-Specific Workflow Adapter**: Supports 7 distinct user journeys (Single-Pass Fix, Investigate-First, Low-Memory, Air-gapped, CI Gate, Test-First, Zero-LLM) with adaptive resource allocation and success criteria per journey.
    * **Token Economy Optimizer**: Achieves p95 ≤3K tokens per fix through intelligent context curation, deterministic-first approach, and LLM-late gating while maintaining 97%+ first-apply correctness rates.
    * **Local Subagent Architecture**: Efficient local model orchestration using tiny models (22M-270M) for data processing and 2-7B models for reasoning, achieving 10x context compression while maintaining information density.
    * **Safety-First Validation Pipeline**: Multi-stage validation (rust-analyzer overlay → cargo check → selective tests) with zero writes before pass+approval, complete audit trails, and automatic rollback capabilities.

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
  - Diverse Ideas:
    * **5-Orchestration Strategy Engine**: Supports 5 distinct Claude+Local orchestration patterns (Claude-as-Reasoner, Claude-as-Orchestrator, Local-first with Escalation, Spec Planner + Implementor, Critic/Selector) with automatic strategy selection based on network availability, memory constraints, and complexity requirements.
    * **Apple Silicon Performance Optimizer**: Metal-aware orchestration with Q4_K_M quantization, dynamic parallelism throttling, and memory pressure management achieving 30-110 tok/s throughput for 2-7B models with automatic KV cache optimization.
    * **Token Economy Manager**: Enforces ≤3K token budget through intelligent context curation, deterministic-first processing, and LLM-late gating while maintaining 97%+ first-apply correctness rates across all orchestration strategies.
    * **Enterprise Adoption Scenario Engine**: Supports 5 deployment strategies (Constellation, Two-Pillars + Orchestrator, Triptych Planes, Fix-Engine Centric, Monorepo Workspace) with compliance-friendly separation, CI-ready pipelines, and modular adoption patterns.
    * **HP-Themed Architecture Integration**: MarauderMap (ISG builder), Pensieve (CodeGraph store), Remembrall (Summarizer), Portkey (Embed/Index), FlooNetwork (Retrieval), RestrictedSection (Patterns), Revelio (Constraints), BeadedBag (Context), Spellbook (Transforms), Prefect (Reasoner), PhoenixOrder (Orchestrator), Patronus (PreFlight), Howler (Diagnostics), OwlPost (Git), DailyProphet (TUI) with unified JSON contracts.
    * **Multi-Strategy Resource Manager**: Dynamic resource allocation across 5 orchestration patterns with memory-aware model selection (2-7B local vs cloud), automatic fallback strategies, and performance monitoring with p95 ≤120s targets.
    * **Open-Source Tool Decomposition**: Modular primitive architecture supporting 5 adoption scenarios from Graph+Gate core (zero LLM) through Understand-first analysis kit, Local-first fixes, Parallel candidates, to CI toolkit with headless hooks and JSON reporters.
    * **Customer Journey Mapping System**: 7 distinct user journeys (Single-Pass Safe Fix, Investigate Before Edit, Low-Memory Mode, Air-gapped/Offline, CI Gate, Test-First Fix, Zero-LLM Deterministic Patch) with adaptive resource allocation and success criteria per journey.
    * **Reliability-First Principle Engine**: Optimizes for accurate 1-go fixes with trustworthy user experience, prioritizing CPU-bound static analysis and small local subagents while keeping reasoning LLM lean and late.
    * **Shreyas Doshi Product Framework**: Prioritizes first-apply correctness over speed with clarity, safety, and explicit confidence gating where time is secondary outcome and user efficacy is primary KPI.
    * **Jeff Dean Systems Architecture**: Makes correctness the fast path through deterministic cacheable computations (ISG, RA, HNSW), parallelized retrieval/validation, minimized token movement, and measured token-per-fix efficiency.
    * **User Promise Assurance System**: Guarantees single-pass safe minimal diffs that compile and pass tests before applying, with speed as byproduct and correctness as the primary success metric.

Appendix A: Local Model Matrix (indicative)
- A1: 22–50M encoder (Q4) — 50–150 MB.
- A4: MiniLM 22M (Q4) — ~40–80 MB.
- A5: SmolLM2 135M (Q4) — ~300–500 MB.
- A6-helper: Gemma 270M (Q4) — ~600–800 MB.
- R1: Qwen2.5 7B (Q4_K_M) — ~4.5–6.5 GB VRAM-equivalent on Metal; CPU fallback slower.

Appendix C: Claude Code Plugin Architecture Integration
- 5-Phase Journey Integration: Discovery (A1-A3), Validation (A4-A6), Reasoning (R1), Safety (PreFlight), Presentation (Diff + Apply) with 60-90s target time
- Multi-Agent System: 11 specialized agents (A1-A6 + R1 + safety-gate + diff-presenter + learning-orchestrator) with standardized contracts
- Performance Metrics: 95%+ first-apply correctness, 85-95% confidence scoring, p95 ≤120s resolution time, 3K token economy
- Hardware Optimization: Apple Silicon Metal acceleration, Q4_K_M quantization, 16GB+ memory optimization with dynamic throttling
- Safety Validation: Zero writes before PreFlight pass, rust-analyzer overlay validation, cargo check pipeline, automatic rollback capabilities
- Journey-Specific Workflows: Single-Pass Fix, Investigate-First, Low-Memory, Air-gapped, CI Gate, Test-First, Zero-LLM modes with adaptive resource allocation
- Plugin Lifecycle Management: Hot reloading, command registration (/rust:debug-bug, /rust:validate-fix, /rust:build-isg), agent orchestration, hook execution
- Learning Loop Integration: Pattern effectiveness tracking, historical bug storage, confidence calibration, continuous improvement per codebase

Appendix D: Orchestration Strategies & Performance Analysis
- 5 Claude+Local Orchestration Patterns: Claude-as-Reasoner (60-90s), Claude-as-Orchestrator (70-100s), Local-first with Escalation (55-85s), Spec Planner + Implementor (80-120s), Critic/Selector (70-110s)
- Apple Silicon Performance Matrix: Qwen2.5 7B (30-55 tok/s decode, 600-1200 tok/s prefill), Qwen 3B (60-110 tok/s decode, 1200-2200 tok/s prefill), 2-3B implementors (80-140 tok/s), 270M (220-450 tok/s), 135M (400-800 tok/s), 22-50M encoders (800-1500 tok/s)
- Memory Optimization: 16GB+ support with dynamic throttling, Q4_K_M quantization, Metal layer downshift on pressure, KV cache optimization preventing thrash
- Token Throughput Engineering: Parallel scaling with 65-80% effective aggregate throughput, deterministic-first processing achieving 30% zero-LLM rate on common errors, escalation logic preventing token thrash
- Enterprise Adoption Framework: Constellation (maximum modularity), Two-Pillars + Orchestrator (compliance-friendly), Triptych Planes (performance budgets), Fix-Engine Centric (fastest value), Monorepo Workspace (team coherence)
- Performance Targets: FACR ≥97%, tokens_per_fix p95 ≤3K, validation_accuracy ≥98%, time_to_safe_fix p95 ≤120s, memory_footprint ≤12GB standard mode

Appendix E: Customer Journey Mapping & User Experience Design
- 7 Comprehensive User Journeys: Single-Pass Safe Fix (default), Investigate Before Edit (security/compliance), Low-Memory Mode (≤12GB), Air-gapped/Offline, CI Gate (pre-commit/pre-push), Test-First Fix (RED→GREEN flow), Zero-LLM Deterministic Patch
- Journey-Specific Success Criteria: First-Apply Correctness ≥95% (default), +15-25s latency tradeoff (low-memory), ≤20s budget (CI gate), zero tokens (deterministic), coverage increase (test-first)
- Adaptive Resource Allocation: Dynamic model selection based on journey type, memory constraints, and network availability with automatic fallback strategies
- Trust & Safety Framework: Zero writes before PreFlight pass + user approval, complete audit trails, automatic rollback capabilities, red/amber/green risk reporting
- User Efficacy Optimization: Clear confidence scoring, pattern rationale explanation, diagnostic mapping to ISG nodes, constrained blast radius analysis
- Performance-First Experience: Cached ISG with 3-4 minute one-time setup, sub-100ms query performance, warm caches on idle, predictable 60-120s resolution times
- Production-Grade Assistant Vision: ISG + local subagents creating production-grade Rust debugging assistant with one-go reliable fixes, minimal tokens, offline capability, safety-first validation, and continuous learning
- Strategic Win Condition: Narrowing search with ISG, specializing subagents, and validating before any write creates competitive advantage through accuracy, efficiency, and trustworthiness

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

---

## Essential User Onboarding & Hardware Detection (P34 Deep Insights)

### **Critical User Requirements Gap Identified**

**Missing Essential Components** (P34 Analysis):
- **No "Start Here" journey** for first-time users
- **No hardware detection** for M1+ 16GB RAM+ requirements
- **No model recommendations** based on user hardware
- **No clear onboarding path** for different user types

### **Hardware Detection & Model Recommendation Matrix**

| Hardware | RAM | Recommended Models | Context | Tokens/sec | Est. RAM Usage |
|----------|-----|-------------------|---------|------------|----------------|
| **M1/M2** | 16GB | 2B (Q4_K_M) | 4K | 45-60 | ~6GB |
| **M1/M2** | 32GB | 7B (Q4_K_M) | 8K | 25-35 | ~10GB |
| **M3** | 16GB | 3B (Q4_K_M) | 8K | 35-45 | ~7GB |
| **M3** | 32GB | 7B (Q4_K_M) | 16K | 20-30 | ~12GB |
| **Intel** | 16GB | 1.5B (Q4_K_M) | 4K | 8-12 | ~5GB |
| **Intel** | 32GB | 3B (Q4_K_M) | 8K | 4-8 | ~8GB |

### **User Journey Onboarding Framework**

**First-Time User Experience**:
1. **Hardware Detection**: Automatically detect M1+ 16GB RAM+ requirement
2. **Model Setup**: Recommend optimal models based on hardware capabilities
3. **Tutorial Journey**: Guided walkthrough of simple bug fixing scenario
4. **Confidence Building**: Start with high-confidence, low-risk fixes
5. **Gradual Complexity**: Progress to more complex scenarios as trust builds

**User Type Segmentation**:
- **Beginner**: Start with 20K mode, high-confidence patterns, heavy validation
- **Intermediate**: Hybrid mode with contextual model selection
- **Expert**: 128K mode with full autonomy and customization options
- **Enterprise**: Compliance mode with audit trails and approval workflows

### **Essential Onboarding Checklist**

**Pre-Flight Setup**:
- [ ] Hardware capability assessment
- [ ] Model download and installation
- [ ] ISG index build for target project
- [ ] Baseline performance benchmarking
- [ ] User preference configuration

**Success Milestones**:
- [ ] First successful bug fix (within 5 minutes)
- [ ] First multi-crate issue resolution
- [ ] First custom pattern creation
- [ ] First autonomous bug fixing session
- [ ] First team collaboration scenario

---

## Parseltongue Architectural Philosophy (P34 Integration)

### **Journey-Aware Orchestration Principles**

**Core Philosophy**: "Think like Shreyas Doshi about product requirements before thinking like Jeff Dean about implementation"

**Product-First Thinking**:
- **User Promise**: Single-pass, safe, minimal diffs that compile and pass tests
- **Success Criteria**: First-apply correctness ≥ 95%
- **Trust Building**: Transparent reasoning with confidence scoring
- **Progressive Disclosure**: Start simple, add complexity as user gains confidence

**Implementation Excellence**:
- **Reliability-First**: Accuracy over speed, correctness over efficiency
- **Deterministic Safety**: Zero writes before validation, auto-rollback on failure
- **Adaptive Performance**: Hardware-aware resource allocation
- **Continuous Learning**: Pattern evolution based on user feedback

### **Multi-Modal Learning Integration**

**Learning from All Interactions**:
- **Success Patterns**: Reinforce effective fix strategies
- **Failure Modes**: Learn from rollbacks and user corrections
- **User Preferences**: Adapt to individual coding styles and conventions
- **Project Context**: Understand domain-specific patterns and requirements

**Cross-Project Knowledge Transfer**:
- **Pattern Abstraction**: Extract reusable patterns from specific fixes
- **Domain Specialization**: Build expertise for specific Rust domains
- **Team Learning**: Share patterns across teams with privacy preservation
- **Community Wisdom**: Aggregate anonymized success patterns across users

---

## Production-Ready LLM Prompt Engineering (P34 Raycast AI Integration)

### **System Prompt Architecture for Parseltongue-R1**

**Core Agent Contract**:
- **Role**: Parseltongue-R1, a Rust bug-fixing agent operating over an Interface Signature Graph (ISG) in CozoDB
- **Objective**: Produce minimal, validated diffs that fix failing tests/builds without degrading style or semantics
- **Tools**: cozo.query(), cargo.run(), fs.read(), patch.apply() with deterministic interfaces
- **Hard Constraints**: ≤2 files, ≤30 changed lines, ISG blast radius ≤2 hops, structured surgical edits only

**Input/Output Schema Specification**:
```json
{
  "bug_report": {
    "symptom": "string",
    "stderr_snippet": "string",
    "failing_test_names": ["string"],
    "seed_uids": ["string"]
  },
  "limits": {
    "time_budget_sec": 60,
    "max_files_changed": 2,
    "max_lines_changed": 30
  }
}
```

**Required Output Contracts**:
- **micro_prd**: Concise problem statement with interfaces to modify and success criteria
- **context_pack**: Compressed facts with blast radius, type constraints, and risk flags
- **transformation_plan**: Graph-first modifications with feasibility checks
- **validation_plan**: Selective tests and validation guards
- **confidence_report**: Calibrated probability with coverage/agreement/consistency metrics
- **patch**: Unified diff only when gate = proceed

### **Deterministic Workflow Engineering**

**6-Step Validation Pipeline**:
1. **Retrieve Symbols**: Exact 2-hop neighborhood via CALLS/DEPENDS edges
2. **Compress to ContextPack**: Type/borrow constraints with ≤3 candidate edit sites
3. **Plan Edit**: 1-2 transformations preferring signature/bound changes
4. **Confidence Estimate**: Calibrated p(pass) with mathematical formula
5. **Generate Patch**: Unified diff within budget touching only approved UIDs
6. **Emit Outputs**: JSON + unified diff with ≤5 bullet rationale

**Confidence Gating Formula**:
```
p = σ(w₀ + w₁·coverage + w₂·agreement + w₃·consistency + w₄·static_precheck)
```
- If p < 0.70: ask targeted questions or widen retrieval once
- Run validation in order: fmt_check → check → test_selective → test_full
- Abort on new warnings promoted to deny

### **Query Snippets for Deterministic Operations**

**Blast Radius Calculation**:
```datalog
blast(uid, hop) := start[uid], hop = 0
blast(uid, hop) := blast(prev, h), h < 2,
  *interface_relationships{from_uid: prev, to_uid: uid, edge_type: 'CALLS'|'DEPENDS'},
  hop = h + 1
```

**Pattern Matching via Vector Similarity**:
```datalog
?[pattern_id, name, distance] :=
  ~pattern_library:pattern_vec_idx{
    pattern_id, name | query: vec_embed($stderr), k: 5, bind_distance: distance
  }
```

**Code and Metadata Retrieval**:
```datalog
?[uid, code, signature] :=
  blast(uid, _),
  *interface_code{uid, current_code: code},
  *interface_metadata{uid, signature}
```

### **Production-Ready Example Transformation**

**Input**: Async spawn Send bound error
**Output**:
```json
{
  "micro_prd": {
    "title": "Add Send bound to spawn generic",
    "problem_statement": "Future cannot be sent between threads safely",
    "pattern_recommendation": "async_spawn_send",
    "interfaces_to_modify": ["src/runtime.rs-runtime-spawn"]
  },
  "transformation_plan": [{
    "uid": "src/runtime.rs-runtime-spawn",
    "action": "MODIFY",
    "signature_old": "pub fn spawn<F: Future>(f: F) -> JoinHandle<F::Output>",
    "signature_new": "pub fn spawn<F: Future + Send + 'static>(f: F) -> JoinHandle<F::Output>",
    "constraints_added": ["Send", "'static"]
  }],
  "confidence_report": {
    "coverage": 0.82, "agreement": 0.78, "consistency": 0.85,
    "static_precheck": 0.76, "p_pass": 0.88, "gate": "proceed"
  },
  "patch": "--- a/src/runtime.rs\n+++ b/src/runtime.rs\n@@\n-pub fn spawn<F: Future>(f: F) -> JoinHandle<F::Output> {\n+pub fn spawn<F: Future + Send + 'static>(f: F) -> JoinHandle<F::Output> {\n"
}
```

### **Prompt Hygiene and Validation Rules**

**Remove**: Marketing claims, emojis, repeated blocks, unverifiable metrics, vague goals
**Add**: Agent role definition, tool interfaces, input/output schemas, deterministic workflows
**Constrain**: Patch minimality, edit surfaces, time/token budgets, retry/escalation rules
**Validate**: Confidence thresholds, test impact selection, clippy/rustdoc guards

**Critical Success Factors**:
- Single source of truth for schemas
- Deterministic order of operations
- Hard stop criteria and validation gates
- No speculative code outside blast radius
- Calibrated confidence with mathematical thresholds

---

## Advanced Validation Architecture (P34 Integration)

### **Multi-Stage Validation Pipeline**

**Pre-Flight Validation Stack**:
1. **Static Analysis**: rust-analyzer overlay with didOpen buffers
2. **Compilation Check**: cargo check --quiet with error mapping
3. **Selective Testing**: Targeted test execution based on ISG blast radius
4. **Style Validation**: cargo fmt + clippy with deny warnings
5. **Documentation**: rustdoc validation when enabled

**Confidence Calibration Framework**:
- **Coverage**: Completeness of ISG blast radius analysis
- **Agreement**: Consistency between pattern matching and graph traversal
- **Consistency**: Internal coherence of transformation plan
- **Static Precheck**: Deterministic validation via rust-analyzer

**Validation Gate Decision Matrix**:
| Confidence | Action | Validation Required |
|------------|--------|-------------------|
| p ≥ 0.85 | Proceed | Full validation pipeline |
| 0.70 ≤ p < 0.85 | Proceed | Enhanced validation + fallback tests |
| p < 0.70 | Hold | Expand retrieval or ask clarifying questions |

### **Error Recovery and Rollback Strategy**

**Automatic Rollback Triggers**:
- New warnings promoted to deny
- Test failures in previously passing suite
- Compilation errors after patch application
- Performance regression beyond thresholds

**Recovery Procedures**:
1. **Git Revert**: Automatic rollback to previous commit
2. **ISG Restoration**: Revert interface_code to previous state
3. **Pattern Learning**: Log failure for pattern effectiveness tracking
4. **User Notification**: Clear explanation of rollback reason

**Audit Trail Requirements**:
- Complete transformation history with timestamps
- Confidence scores and validation results
- Pattern effectiveness metrics per repository
- User feedback incorporation for learning

---

## Health-Optimized Development Framework (P34 Chat Integration)

### **Innovative Multi-Perspective Health Insights for Developers**

**Evolutionary Biology Integration**:
- **South Asian "Thrifty Genotype" Recognition**: Understanding genetic predispositions for fat storage and famine adaptation
- **Ancestral Activity Mimicry**: HIIT protocols that enhance insulin sensitivity 20-30% beyond calorie cuts
- **Protein Prioritization Framework**: 1.6 g/kg protein intake optimized for cognitive performance and sustained development sessions

**Health Metrics Integration with Development Performance**:
- **BMI-WHtR-ABSI Composite Scoring**: Multi-dimensional health assessment superior to BMI alone
- **DEXA-Validated Body Composition**: ±2% accuracy for true fat mass monitoring
- **Metabolic Health Screening**: Pre-diabetes/CVD risk assessment for high-performance developers
- **Circadian Performance Optimization**: Aligning development sessions with metabolic peaks

**Developer-Specific Health Architecture**:
```json
{
  "health_profile": {
    "anthropometric_data": {
      "bmi": 27.18,
      "whtr": 0.58,
      "absi_z_score": 0.5663,
      "risk_classification": "high"
    },
    "development_performance_correlation": {
      "cognitive_endurance": "enhanced with protein timing",
      "focus_quality": "correlated with insulin sensitivity",
      "problem_solving_capacity": "improved with HIIT protocols"
    },
    "optimization_targets": {
      "target_weight": "58-62 kg",
      "waist_target": "<82.5 cm",
      "protein_target": "1.6 g/kg bodyweight",
      "hiit_frequency": "3x/week for 20min sessions"
    }
  }
}
```

**Health-Aware Development Scheduling**:
- **Metabolic Peak Development**: Schedule complex problem-solving during optimal insulin sensitivity windows
- **Recovery-Integrated Coding**: Break cycles aligned with metabolic recovery patterns
- **Nutrition-Timed Sessions**: Protein-intake coordinated with intensive development periods
- **Stress-Cognition Balance**: Cortisol management for sustained analytical performance

**Performance Monitoring Dashboard**:
- **Real-time Health Metrics**: Continuous monitoring of metabolic markers during development
- **Cognitive Performance Tracking**: Correlation between health parameters and coding efficiency
- **Adaptive Workload Management**: Automatic adjustment of task complexity based on health indicators
- **Preventive Health Alerts**: Early warning system for metabolic stress affecting development quality

**Community Health Learning Loop**:
- **Anonymous Health Analytics**: Aggregate health patterns across development teams
- **Performance Optimization Insights**: Data-driven recommendations for health-development integration
- **Peer Health Motivation**: Community challenges for maintaining optimal developer health
- **Research Contribution**: Anonymous data contribution to developer health science

**Implementation Integration**:
- **Parseltongue Health Coach**: Integrated health guidance within the development environment
- **Automated Health Reminders**: Breaks, nutrition, and exercise prompts synchronized with development cycles
- **Health-Performance Analytics**: Dashboard showing correlation between health metrics and development productivity
- **Medical Integration**: Physician consultation coordination for high-risk health profiles

---

## Shreyas Doshi Minimalist Product Architecture (P34 Deep Analysis)

### **The One-Line Job Story**

**"When I hit a Rust bug, get me to a validated, minimal patch in under 90 seconds."**

### **Highest-ROI, Lowest-Risk Functional Changes**

| Priority | Change | ROI Impact | Effort | Risk | Implementation Time |
|----------|--------|------------|---------|------|-------------------|
| **1** | **Caching Basics** (sccache + persistent RA + memoized Cozo) | **5-10x validation speed** | Low | Low | 1-2 days |
| **2** | **Deterministic 2-Hop Retrieval** (CALLS/DEPENDS with node cap) | **4-10x context compression** | Low | Low | 2-3 days |
| **3** | **ContextPack v0** (seeds, candidates, type/bound hints) | **3-8x reasoning quality** | Low | Low | 1-2 days |
| **4** | **Pattern DB v0** (3 high-yield fixes) | **70% bug coverage** | Low | Low | 2-3 days |
| **5** | **Signature-Only Edits** with hard limits | **95% first-try success** | Low | Low | 1 day |
| **6** | **Short-Circuit Validation** (fmt → check → failing tests) | **Fast feedback loop** | Low | Low | 1 day |
| **7** | **Rule-Based Confidence Gate** (trigger + feasibility) | **Zero bad compiles** | Low | Low | 1 day |
| **8** | **Stable Structural UIDs** (module_path::item_kind::name) | **Prevents reference drift** | Low | Low | 1 day |

### **If You Can Only Build Two Things**

**1. Caching Infrastructure** (Highest Impact)
- **Why**: PreFlight validation becomes the bottleneck without caching
- **Components**: sccache, persistent rust-analyzer, memoized Cozo queries
- **Impact**: Sub-5s validation per candidate vs 30-60s cold starts
- **Implementation**: Native tool integration, no custom logic needed

**2. Deterministic Retrieval + ContextPack** (Second Highest)
- **Why**: Reduces reasoning load and eliminates wasted candidates
- **Components**: 2-hop CALLS/DEPENDS traversal, compressed context packing
- **Impact**: 4-10x less irrelevant context for reasoning model
- **Implementation**: Datalog queries + JSON artifact standardization

### **Minimal Viable Pattern Set v0**

**Three High-Yield Rust Fixes** (70% coverage of common bugs):

1. **async_spawn_send**
   - **Trigger**: "cannot be sent between threads", "future cannot be shared"
   - **Transform**: Add `Send + 'static` bounds to async function parameters
   - **Template**: `fn spawn<F: Future + Send + 'static>(f: F) -> JoinHandle<F::Output>`

2. **lifetime_extend**
   - **Trigger**: "does not live long enough", "borrowed value does not live long enough"
   - **Transform**: Add lifetime parameters or use references
   - **Template**: `fn process<'a>(data: &'a str) -> &'a str`

3. **error_propagate**
   - **Trigger**: "unwrap() used on Result", "expect() used on Result"
   - **Transform**: Replace with `?` operator or explicit error handling
   - **Template**: `fn process() -> Result<T, Error> { ...? }`

### **Overcomplication Traps to Avoid**

**High-Complexity, Low-ROI Features**:
- Multi-agent orchestration systems
- Complex calibration models with logistic regression
- Full pattern libraries (100+ patterns)
- Cross-crate refactoring capabilities
- Advanced anti-pattern detection
- Time-travel UI and versioning
- IMPLEMENTS/HAS_MEMBER edge types
- Vector search as primary (vs hint)

**Why These Overcomplicate**:
- Add architectural complexity without proportional user value
- Increase maintenance burden and failure modes
- Distract from core job story of fast, reliable fixes
- Create calibration and tuning nightmares

### **Success Criteria for Minimal Architecture**

**Performance Targets**:
- **p50 Time to Validated Fix**: ≤90s (after initial indexing)
- **First-Try Success Rate**: ≥70% (3-pattern set)
- **Median Patch Size**: 1 file, ≤10 lines
- **Zero Regressions**: On unrelated test smoke subset

**Technical Metrics**:
- **ISG Build Time**: ≤3 minutes (one-time)
- **Query Performance**: <100ms (cached)
- **Validation Time**: 1-5s per candidate
- **Memory Usage**: ≤8GB (minimal footprint)

### **One-Week Execution Plan**

**Day 1-2: ISG v0 + Caching**
- Build interface_metadata + interface_relationships (CALLS/DEPENDS)
- Implement sccache + persistent rust-analyzer integration
- Add memoized Cozo query layer

**Day 2-3: Retrieval + Context**
- Implement 2-hop blast radius with node caps
- Create ContextPack v0 schema and compression logic
- Add stable UID generation

**Day 3-4: Pattern DB v0**
- Encode 3 patterns with signature transforms
- Add keyword trigger + vector hint logic
- Implement feasibility checks

**Day 4-5: Validation Pipeline**
- Short-circuit validation: fmt → check → failing tests
- Rule-based confidence gate
- Patch generation with rollback capability

**Day 6-7: Testing + Polish**
- Evaluate on 10-20 real bugs across 3 patterns
- Fix top 3 failure modes
- Performance tuning and documentation

### **The Shreyas Doshi Product Philosophy**

**Core Principles**:
- **Job Story First**: Every technical decision serves the "bug to patch in 90s" mission
- **Time to Value**: Prioritize features that reduce time-to-first-success
- **Simplicity Over Completeness**: Better to do 3 things perfectly than 100 things poorly
- **Measurable Impact**: Every feature has clear success metrics and ROI targets

**Decision Framework**:
1. **Does this reduce time to validated fix?**
2. **Does this increase first-try success rate?**
3. **Does this add minimal complexity for maximum value?**
4. **Can we implement this in <3 days with <2 developers?**

If any answer is "no", defer or simplify the feature.

### **Integration with P22 PreFlight Architecture**

**How Minimal Architecture Enhances PreFlight**:
- **Tighter Scoping**: Fewer, higher-quality candidates for validation
- **Faster Iteration**: Sub-5s validation enables rapid candidate testing
- **Reliable Patterns**: Signature-only fixes have predictable PreFlight behavior
- **Clear Rollback**: Structured edits make PreFlight failure recovery trivial

**PreFlight Amplification Effects**:
- **Caching**: Turns 30-60s validation into 1-5s feedback loop
- **Deterministic Retrieval**: Reduces candidate quantity by 4-10x
- **Pattern Constraints**: Ensures 95% PreFlight pass rate on first try

---

## Feasibility-Validated Architecture (P34 Mathematical Analysis)

### **Performance Mathematics Validation**

**LLM Throughput Calculations**:
- **Assumptions**: Qwen2.5-Coder-7B at 30-40 tokens/s on 16GB machine
- **Input**: 10-15K tokens (compressed context)
- **Output**: 300-800 tokens (generated diff)
- **Processing Time**:
  ```
  T_LLM = output_tokens / tokens_per_sec + KV_processing_overhead
  T_LLM ≈ 600/35 + 10-20s = 27-37 seconds
  ```

**Cargo Pipeline Benchmarks**:
- **fmt**: ~1s (trivial)
- **check**: 5-12s (warm incremental builds)
- **test build**: 6-10s (target compilation)
- **test execution**: 6-15s (impacted tests only)
- **Total Validation**: 18-38s

**Overall Feasibility**: 60-90s target achievable with:
- Workspace caching and incremental builds
- Impacted test selection (vs full suite)
- Parallel validation phases

### **Beaconized Context Packing Architecture**

**START/END Salience Engineering**:
```
==== BUG_SUMMARY ====
Error: E0277 cannot be sent between threads
Symptom: panic in spawn under load
Target: src/runtime.rs::spawn

==== REQUIRED_CONSTRAINTS ====
Required: where F: Future + Send + 'static
Current: where F: Future

==== PRIMARY_CODE ====
[function span + immediate deps]

==== RECOMMENDED_PATTERN ====
id: async_spawn_send
success_rate: 0.94

==== REQUIRED_CONSTRAINTS (RECAP) ====
Required: where F: Future + Send + 'static
```

**Dual-Anchor Strategy**:
- **Critical Duplication**: Error codes, one-sentence summary, required bounds, target signature
- **Section Sentinels**: Explicit beacons for START/END attention zones
- **Constraint Reinforcement**: Required constraints repeated at both START and END

### **Workspace-Aware Retrieval System**

**Feature-Conditioned Graph Traversal**:
```datalog
# 2-hop forward with feature filtering
?[to_uid, edge_type] :=
  *isg_edges { from_uid: $seed, to_uid, edge_type },
  *isg_edges { from_uid: to_uid, to_uid: next_uid, edge_type: next_edge },
  # Feature constraint enforcement
  *isg_constraints {
    uid: to_uid,
    constraint_type: "feature",
    constraint_data: $active_features
  }

# Impacted test selection
?[test_uid] :=
  *isg_edges { from_uid: test_uid, to_uid: dep, edge_type: "CALLS" },
  test_interfaces { uid: test_uid, is_test: true },
  changed(dep)
```

**Workspace Graph Integration**:
- **L0 Workspace Nodes**: Crates, packages, features, test targets
- **Edge Types**: WITHIN_CRATE, BETWEEN_CRATES, ENABLED_BY_FEATURE
- **Feature Filtering**: Avoid false-context from disabled features
- **Test Impact Analysis**: Reverse closure from changed UIDs to tests

### **Calibrated Confidence Engineering**

**Mathematical Confidence Model**:
```
p = σ(w₀ + w₁·coverage + w₂·agreement + w₃·consistency + w₄·static_precheck)
```

**Calibration Strategy**:
- **Platt Scaling or Isotonic Regression**: Map component scores to calibrated probability
- **Auto-Apply Threshold**: p ≥ 0.8 for automatic application
- **User Confirmation**: p < 0.8 requires human review
- **Semantic Consistency Check**: Verify pattern pre-conditions satisfied by diff

**Semantic Consistency Validation**:
- **Pattern Pre-conditions**: Formal type variable positions and required bounds
- **Diff Verification**: Ensure changes actually add required constraints
- **AST Predicate Matching**: Anti-pattern detection with structural validation

### **Rust-Native Pattern Representation**

**Structured Pattern Schema**:
```json
{
  "id": "async_spawn_send",
  "applies_to": {
    "fn_name": "spawn",
    "generic": "F",
    "constraints_missing": ["Send", "'static"],
    "where_clause_site": "fn"
  },
  "transform": {
    "action": "add_bounds",
    "on": "generic:F",
    "bounds": ["Send", "'static"]
  },
  "examples": [
    "pub fn spawn<F>(f: F) where F: Future + Send + 'static { ... }"
  ],
  "preconditions": [
    "F: Future",
    "spawn called across threads"
  ],
  "metrics": { "success_rate": 0.94 }
}
```

**Pattern Matching Engine**:
- **Formal Pre-conditions**: Type variable positions and bound requirements
- **Anti-Pattern Matchers**: AST predicates for bad practice detection
- **Example Code**: Placeholders with rewriter specifications
- **Success Metrics**: Historical effectiveness tracking per pattern

### **Multi-Crate Architecture Support**

**Procedural Macro Handling**:
- **Macro Expansion Tracking**: ISG nodes for generated code with hygiene preservation
- **Macro Crate Integration**: Cross-crate edge generation for macro-expanded interfaces
- **Generated Code Validation**: Special handling for derive and procedural macros

**Cross-Crate Dependency Analysis**:
- **Inter-Crate Edge Types**: BETWEEN_CRATES relationships with feature conditions
- **Workspace Boundary Awareness**: Respect workspace member boundaries
- **Feature Flag Integration**: Conditional edge activation based on active features

**Generated Code ISG Integration**:
- **Macro Expansion Nodes**: Virtual ISG nodes representing expanded code
- **Hygiene-Aware Mapping**: Preserve macro hygiene information in ISG relationships
- **Build Script Integration**: Include build.rs generated code in ISG analysis

### **Optimized Validation Pipeline**

**Build Acceleration Strategies**:
- **Target Directory Pre-warming**: Per-crate and feature-set optimization
- **Incremental Compilation**: cargo check --profile=dev with sccache integration
- **Test Optimization**: cargo test --no-run for impact analysis, then selective execution

**Error Routing and Canonicalization**:
- **Error Code Extraction**: E-codes and trait bound diffs for prompt reinforcement
- **Canonical Error Format**: Consistent error representation to avoid prompt drift
- **Panic Detection**: Backtrace parsing with file:line → UID mapping

**Smart Test Selection**:
- **Impacted Test Graph**: Reverse traversal from changed nodes to dependent tests
- **Test Prioritization**: Fast-failing tests first, comprehensive tests second
- **Test Parallelization**: Parallel test execution with result aggregation

### **Performance Optimization Matrix**

| Component | Baseline | Optimized | Improvement | Implementation Effort |
|-----------|----------|-----------|-------------|----------------------|
| **Context Packing** | Linear | Beaconized | 35% attention accuracy | Low |
| **Retrieval** | Naive | Feature-aware | 50% precision improvement | Medium |
| **Validation** | Full suite | Selective | 60% time reduction | Low |
| **Confidence** | Linear | Calibrated | 25% better decisions | Medium |
| **Pattern Matching** | Keyword | Structured | 40% accuracy gain | High |

### **Implementation Priority Queue**

**Phase 1: Foundation (Days 1-3)**
1. **Beaconized Context Packing**: Section sentinels and dual-anchor duplication
2. **Feature-Aware Retrieval**: Workspace graph integration with feature filtering
3. **Selective Test Selection**: Impact-based test routing

**Phase 2: Intelligence (Days 4-6)**
1. **Calibrated Confidence**: Mathematical model with Platt scaling
2. **Structured Pattern DB**: Rust-native pattern representation
3. **Semantic Consistency**: Pre-condition verification system

**Phase 3: Optimization (Days 7-10)**
1. **Build Pipeline Optimization**: sccache integration and incremental builds
2. **Multi-Crate Support**: Cross-crate analysis and macro handling
3. **Performance Tuning**: End-to-end optimization and bottleneck elimination

### **Success Validation Framework**

**Quantitative Metrics**:
- **Time to Fix**: p50 ≤ 60s, p95 ≤ 90s
- **First-Try Success**: ≥85% with calibrated confidence
- **Pattern Coverage**: ≥70% for common Rust errors
- **Validation Accuracy**: ≥95% PreFlight pass rate

**Qualitative Outcomes**:
- **User Trust**: Transparent reasoning with confidence scoring
- **Developer Experience**: Minimal disruption to existing workflows
- **Learning Capability**: Pattern effectiveness improvement over time
- **Reliability**: Zero regressions on unrelated codebases

---

## Accuracy-First Architecture (P34 Optimization Strategy)

### **Primary Focus: First-Try Correctness Over Speed**

**Core Philosophy Shift**:
- **Speed**: Secondary (3-10 minutes per fix acceptable)
- **Accuracy**: Primary target (95-98% first-try success)
- **Approach**: Pattern DB → Micro-PRD → ISG_future → Deep Validation → Apply Once → Learn

**Why Accuracy First**: Optimize for "one-and-done" fixes with minimal regressions, even if it costs more time per run. Developer trust and reliability are more valuable than raw speed.

### **Reprioritized KPI Framework**

| Priority | Metric | Target | Rationale |
|----------|--------|--------|-----------|
| **Primary** | First-try success rate | ≥95% (compile + tests + validators) | Developer trust and reliability |
| **Secondary** | Regression rate in CI | ≤1% post-merge | Production stability |
| **Tertiary** | Idiomaticity score | ≥0.95 | Code quality and maintainability |
| **Tertiary** | Diff minimality | ≥0.85 | Surgical, reviewable changes |
| **Monitoring** | Latency | Monitored but not gated | Performance tracking without sacrificing accuracy |

### **Enhanced Discovery Pipeline (Broader, Safer Coverage)**

**Retrieval Optimization for Accuracy**:
- **A2 Exact Retrieval**: Radius 3 hops (was 2), cap 50 nodes/hop with salience scoring
- **A3 Vector Retrieval**: K=30 (was 15), bias by pattern hints and active features
- **Workspace Awareness**: Complete cargo metadata integration with feature-aware edge filtering
- **Symmetric Analysis**: Include callers and callees for target functions
- **Type-Level Context**: Pull adjacent trait-bound and lifetime nodes

**Feature-Conditioned Analysis**:
```rust
// Active feature vector management
struct FeatureContext {
    active_features: HashSet<String>,
    ci_feature_union: Option<HashSet<String>>,
    mode: FeatureMode // Active | ActiveAndCI
}

// Edge filtering by feature conditions
fn filter_edges_by_features(edges: Vec<ISGEdge>, context: &FeatureContext) -> Vec<ISGEdge> {
    edges.into_iter()
        .filter(|edge| is_edge_enabled(edge, &context.active_features))
        .collect()
}
```

### **Multi-Crate Architecture Support**

**Procedural Macro Integration**:
- **Macro Expansion Tracking**: ISG nodes for generated code with hygiene preservation
- **Span Mapping**: Map generated code back to original source via rust-analyzer spans
- **Build Script Integration**: Include build.rs generated code in ISG analysis
- **Generated Code Warnings**: Alert when patches target generated code

**Cross-Crate Dependency Management**:
```datalog
// Cross-crate edge generation
?[cross_crate_edge] :=
  *isg_edges { from_uid: source_uid, to_uid: target_uid, edge_type: "BETWEEN_CRATES" },
  *cargo_metadata { source_crate: source_crate, target_crate: target_crate },
  *workspace_nodes { uid: source_uid, crate: source_crate },
  *workspace_nodes { uid: target_uid, crate: target_crate }
```

**Workspace Graph Integration**:
- **L0 Workspace Nodes**: Crates, packages, features, test targets
- **Edge Types**: WITHIN_CRATE, BETWEEN_CRATES, ENABLED_BY_FEATURE
- **Feature Filtering**: Respect active feature sets for current run
- **Test Impact Analysis**: Reverse closure from changed UIDs to dependent tests

### **Pattern-Enforced Validation Framework**

**Structured Pattern Records with Pre/Post Conditions**:
```json
{
  "id": "async_spawn_send",
  "applies_to": {
    "fn": "spawn",
    "generic": "F",
    "requires_present": ["F: Future"],
    "requires_missing": ["F: Send", "'static"]
  },
  "transform": {
    "action": "add_bounds",
    "site": "fn_where_clause",
    "target": "F",
    "bounds": ["Send", "'static"]
  },
  "preconditions": [
    "function signature analyzable",
    "generic type F identifiable",
    "no conflicting bounds present"
  ],
  "postconditions": [
    "compiles without errors",
    "calls_to_spawn_cross_thread_ok",
    "no_unintended_Unpin_bound_added"
  ],
  "validation_checklist": [
    "trait bounds added to correct generic",
    "lifetime annotations preserved",
    "public API compatibility maintained"
  ],
  "metrics": { "success_rate": 0.94 }
}
```

**Pattern Verification Engine**:
- **Pre-Condition Checking**: Verify pattern applicability before transformation
- **Transform Validation**: Ensure changes match pattern specification exactly
- **Post-Condition Verification**: Validate that all required outcomes are achieved
- **Semantic Consistency**: Check that changes maintain logical coherence

### **Deep Validation Matrix (Sequential, Abort-on-Fail)**

**Core Validators (Always Run)**:
1. **cargo fmt --check**: Code formatting consistency
2. **cargo clippy -D warnings**: Lint compliance with deny warnings
3. **cargo check**: Compilation validation with active features
4. **Impacted Tests**: Selective test execution based on ISG analysis

**Extended Validators (Optional, Accuracy-Max Mode)**:
5. **Test Amplification**: Auto-generate property tests around changed APIs
6. **Sanitizers**: Address/Thread sanitizers for targeted test subset
7. **Miri Analysis**: cargo miri test for specific changed functions
8. **Semver Checks**: API stability verification for library crates
9. **Unsafe Surface**: cargo geiger for unsafe usage regression detection

**Validation Configuration**:
```toml
[journey.bug_fixing]
mode = "accuracy-first"
features = "active-and-ci"
enable_clippy = true
enable_semver_checks = true
enable_test_amplification = true
enable_sanitizers = "auto"
enable_miri = "targeted"
```

### **Multi-Candidate Generation and Selection**

**Ensemble Approach**:
- **Generate 2-3 Candidates**: Different pattern specializations at low temperature
- **Self-Consistency Check**: Verify each candidate against pattern pre/post-conditions
- **Lightweight Verifier**: Quick validation pass to identify champion candidate
- **Confidence Calibration**: Mathematical model for final acceptance decision

**Final Acceptance Formula**:
```
p_final = calibrate(0.30·coverage + 0.25·agreement + 0.25·consistency + 0.20·static_check)
```

**Apply Only If**: `p_final ≥ 0.90` AND all validation gates pass

### **Comprehensive Acceptance Checklist**

**Compilation Requirements**:
- [ ] Compile passes on selected feature matrix
- [ ] All impacted tests pass
- [ ] Amplified tests pass (if enabled)
- [ ] Clippy clean with -D warnings

**Pattern Compliance**:
- [ ] Pattern post-conditions satisfied
- [ ] Bounds/lifetimes/traits on correct generics
- [ ] No unintended public API break (or user-allowed)

**Quality Standards**:
- [ ] Diff minimality ≥ 0.85
- [ ] Idiomaticity ≥ 0.95
- [ ] Calibrated confidence ≥ 0.90

### **Test Impact Analysis and Amplification**

**Impacted Test Selection**:
```datalog
?[test_uid] :=
  changed { uid: dep_uid },
  *isg_edges { from_uid: test_uid, to_uid: dep_uid, edge_type: "CALLS" },
  *isg_nodes { uid: test_uid, is_test: true }
```

**Test Amplification Strategy**:
- **Property Test Generation**: Create tests around changed API boundaries
- **Regression Protection**: Amplified tests prevent future regressions
- **Targeted Focus**: Amplify only for critical code paths
- **Performance Awareness**: Balance coverage gains with test execution time

### **Mode Configuration System**

**Three Operating Modes**:

| Mode | Retrieval | Candidates | Validators | Time | Accuracy |
|------|-----------|------------|------------|------|----------|
| **Accuracy-First** | 3 hops, K=30 | 2–3 | Full suite + amplifiers | 3–10+ min | 95-98% |
| **Balanced** | 2–3 hops, K=20 | 2 | Core validators | 90–180 s | 90-95% |
| **Fast** | 2 hops, K=15 | 1 | Minimal validators | 60–90 s | 85-90% |

**Configuration Management**:
- **Per-Project Settings**: Repository-specific accuracy requirements
- **Feature Union Modes**: Active vs CI feature union strategies
- **Validator Toggles**: Enable/disable specific validation layers
- **Time Budget Controls**: Automatic mode switching based on time constraints

---

## Adaptive Multi-Candidate Architecture (P34 Zero-Write Validation)

### **Single vs Multiple Candidates Decision Framework**

**Core Insight**: A single "current ISG → future ISG → code" transform works for many fixes, but multiple candidates address ambiguity in edit locus, feature conditioning, and trade-offs between semver safety vs minimal diff.

**When Single Transform Succeeds**:
- Clear pre/post-conditions with unambiguous edit locus
- Local, monotonic edits without public API impact
- Single valid solution space (e.g., add Send bounds to function where-clause)

**When Multiple Candidates Needed**:
- **Ambiguous Edit Locus**: Add Send bounds where? fn where-clause, impl block, trait method, or generic type
- **Feature-Conditioned Code**: Different cfg(feature) branches require different edits
- **Procedural Macro Handling**: Visible span vs real anchor differ (source vs post-expansion)
- **Lifetime Strategy Options**: Extend lifetime vs move/clone vs restructure return type
- **Executor Choice**: spawn vs spawn_local with Send constraints
- **API Stability Trade-offs**: Two valid fixes - one breaks public API, other keeps internal

### **Structured Candidate Enumeration**

**Parameterized Candidate Space**:
```rust
enum Locus { FnWhere, ImplWhere, TraitBound }
enum BoundSet { SendOnly, SendAndStatic }
enum ExecutorStrategy { Spawn, SpawnLocal }
enum LifetimeStrategy { Own, Clone, Extend }

fn enumerate_candidates(ctx: &TransformContext) -> Vec<Candidate> {
    let mut candidates = Vec::new();

    for locus in [Locus::FnWhere, Locus::ImplWhere] {
        for bounds in [BoundSet::SendOnly, BoundSet::SendAndStatic] {
            if preconditions_hold(ctx, locus, bounds) {
                candidates.push(apply_transform(ctx, locus, bounds));
            }
        }
    }
    candidates
}
```

**Adaptive Minimal Ensemble Policy**:
1. **Primary Deterministic**: Build FutureSpec from pattern pre/post-conditions
2. **Validation Pipeline**: Run full validator matrix on primary candidate
3. **Conditional Expansion**: If any gate fails or confidence near-threshold, enumerate up to 2 targeted alternates
4. **Champion Selection**: Choose first candidate passing all gates with optimal score

**Mathematical Selection Criteria**:
```
ĉ = argmax_{c ∈ C} s(c)  s.t.  V(c) = pass
```
- `V(c)`: Validators (compile, tests, clippy, semver, post-conditions) must pass
- `s(c)`: Score combining minimality, idiomaticity, historical pattern success

### **Zero-Write Validation Architecture**

**Rust-Analyzer LSP Preflight (In-Memory)**:
```rust
// LSP-based validation without disk writes
struct RustAnalyzerPreflight {
    lsp_client: LspClient,
    workspace_root: PathBuf,
}

impl RustAnalyzerPreflight {
    fn validate_candidate(&mut self, candidate: &Candidate) -> ValidationResult {
        // Send in-memory overlays via LSP
        for (file_path, content) in candidate.edits() {
            self.lsp_client.did_open(file_path, content);
            self.lsp_client.did_change(file_path, content);
        }

        // Wait for diagnostics without repo writes
        let diags = self.lsp_client.wait_for_diagnostics(Duration::from_secs(5));

        // Filter for blocking errors
        if diags.has_blocking_errors() {
            ValidationResult::Blocked(diags)
        } else {
            ValidationResult::Passed(diags)
        }
    }
}
```

**Shadow Workspace Validation (Ephemeral Overlay)**:
```rust
fn create_shadow_workspace(repo: &Path, candidate_id: &str, edits: &[(String, String)]) -> PathBuf {
    let overlay = temp_dir().join("parseltongue").join(hash(repo)).join(candidate_id);

    // Mirror repository with hardlinks/symlinks
    mirror_repository(repo, &overlay);

    // Materialize only edited files
    for (rel_path, content) in edits {
        let full_path = overlay.join(rel_path);
        fs::create_dir_all(full_path.parent().unwrap())?;
        fs::write(full_path, content)?;
    }

    overlay
}

fn validate_in_shadow(overlay: &Path, features: &[&str]) -> ValidationResult {
    let mut cargo = Command::new("cargo")
        .current_dir(overlay)
        .env("CARGO_TARGET_DIR", "/tmp/parseltongue/target") // Shared cache
        .arg("check")
        .arg("--message-format=json")
        .args(features.iter().flat_map(|f| ["--features", f]))
        .output()?;

    parse_cargo_output(&cargo.stdout)
}
```

### **Integration with Existing Architecture**

**Candidate Assembly in CozoDB**:
```json
{
  "candidate_id": "blake3(diffset_hash)",
  "future_code": {
    "src/runtime.rs": "modified content with Send bounds",
    "src/executor.rs": "unchanged content"
  },
  "metadata": {
    "locus": "FnWhere",
    "bounds": ["Send", "'static"],
    "pattern_id": "async_spawn_send",
    "confidence_estimate": 0.92
  }
}
```

**Validation Pipeline Integration**:
1. **Discovery**: Pattern-guided discovery → Micro-PRD → ISG_future
2. **Candidate Generation**: Enumerate 1-3 structured candidates from same FutureSpec
3. **LSP Preflight**: rust-analyzer overlays for immediate type/trait validation
4. **Shadow Validation**: Full cargo pipeline in ephemeral workspaces
5. **Champion Selection**: First candidate passing all gates with optimal score
6. **Real Repo Application**: Atomic git apply only for champion candidate

### **Concurrency and Caching Strategy**

**Parallel Candidate Processing**:
```rust
async fn validate_candidates_parallel(
    candidates: Vec<Candidate>,
    workspace_root: &Path
) -> Vec<ValidatedCandidate> {
    let mut tasks = Vec::new();

    for candidate in candidates {
        let root = workspace_root.to_path_buf();
        tasks.push(tokio::spawn(async move {
            // LSP preflight (fast)
            let lsp_result = rust_analyzer_preflight(&candidate, &root).await;
            if lsp_result.is_blocked() {
                return ValidatedCandidate::Rejected(candidate.id, lsp_result);
            }

            // Shadow validation (thorough)
            let overlay = create_shadow_workspace(&root, &candidate.id, &candidate.edits());
            let shadow_result = validate_in_shadow(&overlay, &candidate.features);

            ValidatedCandidate::Complete(candidate, lsp_result, shadow_result)
        }));
    }

    futures::future::join_all(tasks).await
}
```

**Cache Optimization**:
- **Shared Target Directory**: `/tmp/parseltongue/target` for all candidates
- **sccache Integration**: Reuse compiled dependencies across candidates
- **Incremental Compilation**: Leverage cargo incremental compilation
- **Feature Isolation**: Separate caches per feature combination if varying features

### **Test Amplification Strategy**

**Proptest Generation for Changed APIs**:
```rust
fn generate_amplified_tests(candidate: &Candidate) -> Vec<TestTemplate> {
    match candidate.pattern_id {
        "async_spawn_send" => vec![
            TestTemplate::spawn_send_proptest(),
            TestTemplate::concurrent_stress_test(),
        ],
        "lifetime_extend" => vec![
            TestTemplate::borrow_lifetime_proptest(),
            TestTemplate::ownership_transfer_test(),
        ],
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod amplified {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn spawn_does_not_panic_for_send_futures(n in 1u32..1000) {
            let fut = async move { n + 1 };
            // Protected by Send + 'static requirement
            spawn(fut);
        }
    }
}
```

### **Implementation Priority**

**Phase 1: Foundation (Week 1)**
1. **LSP Preflight Integration**: In-memory rust-analyzer validation
2. **Shadow Workspace Framework**: Ephemeral overlay creation
3. **Basic Candidate Enumeration**: Structured parameter space exploration

**Phase 2: Validation (Week 2)**
1. **Full Validator Matrix**: Complete validation pipeline integration
2. **Cache Optimization**: Shared target directories and sccache
3. **Parallel Processing**: Concurrent candidate validation

**Phase 3: Intelligence (Week 3)**
1. **Adaptive Expansion Logic**: Smart candidate generation based on failure modes
2. **Champion Selection Algorithm**: Optimal candidate scoring and selection
3. **Guardrail Enhancement**: Rust-specific validation rules

**Phase 4: Polish (Week 4)**
1. **Performance Tuning**: Optimized caching and parallelization
2. **Error Reporting**: Detailed validation failure diagnostics
3. **User Experience**: Transparent candidate evaluation and selection
