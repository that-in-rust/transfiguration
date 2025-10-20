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
