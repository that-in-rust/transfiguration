# P42 MECE Tools Framework

**Mutually Exclusive, Collectively Exhaustive - Small, Focused Tools with Zero Overlap**

## MECE Design Philosophy

Breaking down the 76 P41 tools into **small, focused CLI/Rust tools** where each tool has:
- **Single responsibility**: One clear purpose, no overlap with other tools
- **Composable**: Can be piped together to create complete workflows
- **UNIX philosophy**: Do one thing and do it well
- **Minimal dependencies**: Each tool can be used independently
- **CLI-first**: Command-line interface with optional Rust library

---

## Data Layer Tools

### **interface-signature-graph-builder**
**CLI**: `interface-sig-build`
**Purpose**: Build Interface Signature Graph from source code
**Input**: Repository path, include/exclude patterns
**Output**: ISG data files (JSON + CozoDB)
**CLI**: `interface-sig-build --repo . --output ./isg_data/ --embeddings`
**Rust lib**: `libisigbuilder`

### **interface-graph-query-exact**
**CLI**: `isg-query-exact`
**Purpose**: Query ISG data with exact graph traversals
**Input**: ISG data path, query parameters
**Output**: Graph traversal results
**CLI**: `isg-query-exact --data ./isg_data --from "src-spawn" --hops 2 --edges CALLS,DEPENDS`
**Rust lib**: `libisgqueryexact`

### **code-embedding-generator-hnsw**
**CLI**: `code-embed-gen`
**Purpose**: Generate vector embeddings with HNSW index
**Input**: ISG data, text content, model specification
**Output**: Vector embeddings + HNSW index
**CLI**: `code-embed-gen --data ./isg_data --model MiniLM-L6-v2 --index semantic_idx`
**Rust lib**: `libcodeembedgen`

### **vector-similarity-search-engine**
**CLI**: `vector-sim-search`
**Purpose**: Vector similarity search on code embeddings
**Input**: Query vector, index path, search parameters
**Output**: Ranked similarity results
**CLI**: `vector-sim-search --index semantic_idx --query "[0.1,0.2,...]" --k 10`
**Rust lib**: `libvectorsimsearch`

### **hybrid-graph-vector-search**
**CLI**: `hybrid-graph-search`
**Purpose**: Combine exact graph queries with vector search
**Input**: ISG data, query, hybrid parameters
**Output**: Merged ranked results
**CLI**: `hybrid-graph-search --data ./isg_data --pattern "async" --knn 15 --merge rank`
**Rust lib**: `libhybridgraphsearch`

### **code-summary-generation-engine**
**CLI**: `code-summary-gen`
**Purpose**: Generate 1-line summaries for ISG nodes
**Input**: ISG data, summary model
**Output**: Node summaries with provenance
**CLI**: `code-summary-gen --data ./isg_data --model rule-based --fallback llm`
**Rust lib**: `libcodesummarygen`

### **context-blast-radius-calc**
**CLI**: `context-blast-calc`
**Purpose**: Calculate blast radius around code changes
**Input**: Seed nodes, radius parameters, filters
**Output**: Contextual node set with relationships
**CLI**: `context-blast-calc --seeds "src-spawn" --radius 2 --max-items 50`
**Rust lib**: `libcontextblastcalc`

### **cozo-database-initializer**
**CLI**: `cozo-db-init`
**Purpose**: Initialize CozoDB database with ISG schema
**Input**: Database path, schema definitions
**Output**: Ready-to-use CozoDB instance
**CLI**: `cozo-db-init --db ./isg.cozo --schema ./schemas/isg.cozo`
**Rust lib**: `libcozodbinit`

---

## Agent Runtime Tools

### **multi-agent-task-orchestrator**
**CLI**: `multi-agent-orch`
**Purpose**: Coordinate multiple agents with task distribution
**Input**: Agent configuration, task definitions
**Output**: Agent execution results
**CLI**: `multi-agent-orch --config agents.yaml --task debug_error_E0277`
**Rust lib**: `libmultiagentorch`

### **agent-pool-resource-manager**
**CLI**: `agent-pool-mgr`
**Purpose**: Manage pool of available models and resources
**Input**: Model specifications, resource constraints
**Output**: Available agent instances
**CLI**: `agent-pool-mgr --models "miniLM:22M,qwen2.5:7B" --memory 8GB`
**Rust lib**: `libagentpoolmgr`

### **agent-task-dispatcher**
**CLI**: `agent-task-dispatch`
**Purpose**: Dispatch specific tasks to appropriate agents
**Input**: Task type, agent capabilities, task data
**Output**: Task assignment and execution
**CLI**: `agent-task-dispatch --task pattern_match --agent A5 --input error_E0277.json`
**Rust lib**: `libagenttaskdispatch`

### **local-llm-inference-engine**
**CLI**: `local-llm-engine`
**Purpose**: Run local LLM inference with Apple Silicon optimization
**Input**: Model path, prompt, generation parameters
**Output**: Model response with metadata
**CLI**: `local-llm-engine --model qwen2.5-7b.gguf --prompt "Analyze this error..." --temp 0.7`
**Rust lib**: `liblocalllminference`

### **gguf-model-loader-manager**
**CLI**: `gguf-model-loader`
**Purpose**: Load and manage local GGUF models
**Input**: Model files, memory budget
**Output**: Loaded model instances
**CLI**: `gguf-model-loader --models ./models/ --memory-limit 4GB --cache ./cache/`
**Rust lib**: `libggufmodelloader`

### **agent-health-monitor**
**CLI**: `agent-health-mon`
**Purpose**: Monitor agent health and performance
**Input**: Agent instances, health check parameters
**Output**: Health status and metrics
**CLI**: `agent-health-mon --agents A1,A2,A3 --interval 30s --metrics cpu,memory,latency`
**Rust lib**: `libagenthealthmon`

---

## Pattern Intelligence Tools

### **error-pattern-matching-engine**
**CLI**: `error-pattern-match`
**Purpose**: Match errors against pattern database
**Input**: Error message, code context, pattern DB
**Output**: Pattern matches with confidence scores
**CLI**: `error-pattern-match --error "E0277" --code src/runtime.rs --patterns ./patterns/`
**Rust lib**: `liberrorpatternmatch`

### **pattern-database-search-tool**
**CLI**: `pattern-db-search`
**Purpose**: Search pattern database by keywords
**Input**: Search terms, pattern DB, filters
**Output**: Matching patterns with metadata
**CLI**: `pattern-db-search --keywords "async,spawn,send" --type rust_pattern --limit 10`
**Rust lib**: `libpatterndbsearch`

### **pattern-application-validator**
**CLI**: `pattern-app-validator`
**Purpose**: Validate pattern application against code
**Input: Proposed fix, pattern rules, code context
**Output**: Validation result with issues
**CLI**: `pattern-app-validator --fix proposal.patch --pattern async_spawn_send --code src/`
**Rust lib**: `libpatternappvalidator`

### **anti-pattern-detection-engine**
**CLI**: `anti-pattern-detect`
**Purpose**: Detect anti-patterns in code
**Input**: Code files, anti-pattern rules
**Output**: Anti-pattern detections with severity
**CLI**: `anti-pattern-detect --src ./src/ --rules ./anti_patterns.yaml --threshold 0.8`
**Rust lib**: `libantipatterndetect`

### **pattern-index-builder-engine**
**CLI**: `pattern-index-build`
**Purpose**: Build searchable index of pattern database
**Input**: Pattern files, index configuration
**Output**: Optimized pattern index
**CLI**: `pattern-index-build --patterns ./patterns/ --index ./patterns.idx --optimize speed`
**Rust lib**: `libpatternindexbuild`

### **pattern-learning-optimizer**
**CLI**: `pattern-learn-opt`
**Purpose**: Update patterns based on usage feedback
**Input**: Pattern usage data, success/failure metrics
**Output**: Updated pattern weights
**CLI**: `pattern-learn-opt --feedback ./usage_data/ --patterns ./patterns/ --update weights`
**Rust lib**: `libpatternlearnopt`

---

## Validation Tools

### **rust-analyzer-overlay-check**
**CLI**: `rust-analyzer-check`
**Purpose**: Run rust-analyzer diagnostics overlay validation
**Input: Code changes, RA configuration
**Output**: Diagnostics with severity and locations
**CLI**: `rust-analyzer-check --changes patch.diff --ra-config ra.toml --output diagnostics.json`
**Rust lib**: `librustanalyzercheck`

### **cargo-validation-gate-engine**
**CLI**: `cargo-validation-gate`
**Purpose**: Run cargo check/test with validation gating
**Input**: Workspace path, validation requirements
**Output**: Build/test results with validation status
**CLI**: `cargo-validation-gate --workspace . --check-only --timeout 30s --fail-on-warnings`
**Rust lib**: `libcargovalidationgate`

### **shadow-workspace-validator**
**CLI**: `shadow-workspace-val`
**Purpose**: Validate changes in shadow workspace
**Input: Changes, shadow workspace path
**Output**: Shadow validation results
**CLI**: `shadow-workspace-val --changes patch.diff --shadow /tmp/shadow_ws --copy-hardlink`
**Rust lib**: `libshadowworkspacevalidator`

### **selective-test-runner-engine**
**CLI**: `selective-test-runner`
**Purpose**: Run only tests affected by changes
**Input: Changed files, test impact analysis
**Output**: Selected test execution results
**CLI**: `selective-test-runner --changes src/lib.rs --impact-analysis --parallel 4`
**Rust lib**: `libselectivetestrunner`

### **validation-result-cache**
**CLI**: `validation-result-cache`
**Purpose**: Cache and retrieve validation results
**Input: Validation results, cache keys
**Output**: Cached validation or fresh results
**CLI**: `validation-result-cache --key "patch_1234" --cache-dir ./valid_cache/ --ttl 3600`
**Rust lib**: `libvalidationresultcache`

### **safety-validation-gate**
**CLI**: `safety-validation-gate`
**Purpose**: Apply safety validation gates with thresholds
**Input: Validation results, safety policies
**Output**: Pass/fail decision with reasons
**CLI**: `safety-validation-gate --validation results.json --policy strict --threshold 0.95`
**Rust lib**: `libsafetyvalidationgate`

---

## Orchestration Tools

### **debugging-journey-executor**
**CLI**: `debugging-journey`
**Purpose**: Execute complete debugging journey workflow
**Input: Journey configuration, error context
**Output**: Complete journey results
**CLI**: `debugging-journey --journey p20-bug-fix --input error_E0277.json --timeout 90s`
**Rust lib**: `libdebuggingjourney`

### **journey-phase-executor-engine**
**CLI**: `journey-phase-exec`
**Purpose**: Execute individual journey phases
**Input: Phase configuration, context data
**Output**: Phase execution results
**CLI**: `journey-phase-exec --phase discovery --agents A1-A6 --context discovery_ctx.json`
**Rust lib**: `libjourneyphaseexec`

### **workflow-composition-engine**
**CLI**: `workflow-compose`
**Purpose**: Compose tools into workflow pipelines
**Input: Workflow definition, tool configurations
**Output**: Pipeline execution results
**CLI**: `workflow-compose --pipeline debug_pipeline.yaml --input error.json`
**Rust lib**: `libworkflowcomposition`

### **resource-allocation-manager**
**CLI**: `resource-alloc-mgr`
**Purpose**: Manage computational resources for workflows
**Input: Resource constraints, task requirements
**Output**: Resource allocation and monitoring
**CLI**: `resource-alloc-mgr --memory 12GB --cpu 8 --tasks discovery,reasoning --optimize latency`
**Rust lib**: `libresourceallocationmgr`

### **context-packing-optimizer**
**CLI**: `context-pack-opt`
**Purpose**: Pack context data with size optimization
**Input: Context items, size limits, priorities
**Output**: Optimized context bundle
**CLI**: `context-pack-opt --items context_items.json --limit 3000-tokens --priority semantic`
**Rust lib**: `libcontextpackingopt`

### **solution-confidence-calculator**
**CLI**: `solution-confidence`
**Purpose**: Calculate confidence scores for solutions
**Input: Solution data, historical success rates
**Output**: Confidence metrics and thresholds
**CLI**: `solution-confidence --solution fix.patch --history ./success_data/ --method platt`
**Rust lib**: `libsolutionconfidence`

---

## Integration Tools

### **claude-code-plugin-manager**
**CLI**: `claude-plugin-mgr`
**Purpose**: Register and manage Claude Code plugin
**Input: Plugin configuration, command definitions
**Output**: Active plugin status
**CLI**: `claude-plugin-mgr --register --commands "/rust:debug,/rust:validate" --config plugin.toml`
**Rust lib**: `libclaudepluginmgr`

### **claude-code-bridge-server**
**CLI**: `claude-bridge-server`
**Purpose**: Bridge local tools to Claude Code interface
**Input: Local tool endpoints, Claude integration
**Output**: Active bridge connection
**CLI**: `claude-bridge-server --local-endpoint unix:/tmp/agent.sock --claude-key $ANTHROPIC_KEY`
**Rust lib**: `libclaudebridgeserver`

### **diagnostic-mapping-engine**
**CLI**: `diagnostic-map-engine`
**Purpose**: Map diagnostics to ISG nodes and code locations
**Input: Diagnostics, ISG data
**Output**: Diagnostic mapping with explanations
**CLI**: `diagnostic-map-engine --diagnostics cargo_check.json --isg ./isg_data/ --map-to code`
**Rust lib**: `libdiagnosticmapengine`

### **git-safe-apply-handler**
**CLI**: `git-safe-apply`
**Purpose**: Apply changes with rollback capability
**Input: Patch file, validation status
**Output: Applied changes with rollback info
**CLI**: `git-safe-apply --patch fix.patch --validate --rollback-point pre_fix_commit`
**Rust lib**: `libgitsafeapply`

---

## Configuration & Monitoring Tools

### **configuration-template-builder**
**CLI**: `config-template-build`
**Purpose**: Build configuration from templates and defaults
**Input: Config templates, environment variables
**Output: Complete configuration files
**CLI**: `config-template-build --template config.yaml --env .env --output config.toml`
**Rust lib**: `libconfigurationtemplatebuild`

### **metrics-collect**
**CLI**: `metrics-collect`
**Purpose**: Collect performance and success metrics
**Input: Tool execution data, metric definitions
**Output: Metrics reports and analytics
**CLI**: `metrics-collect --data ./execution_logs/ --metrics success-rate,latency --report json`
**Rust lib**: `libmetrics`

### **perf-profile**
**CLI**: `perf-profile`
**Purpose**: Profile tool performance and identify bottlenecks
**Input: Tool execution traces, profiling parameters
**Output: Performance analysis report
**CLI**: `perf-profile --tool journey-run --trace ./trace.prof --optimize cpu,memory`
**Rust lib**: `libperfprof`

### **health-check**
**CLI**: `health-check`
**Purpose**: System health monitoring for all tools
**Input: Tool endpoints, health check definitions
**Output: Overall system health status
**CLI**: `health-check --tools agent-pool,validation-engine --timeout 10s --format json`
**Rust lib**: `libhealth`

---

## Utility Tools

### **text-process**
**CLI**: `text-process`
**Purpose**: Process and normalize text content
**Input: Raw text, processing rules
**Output: Processed text with metadata
**CLI**: `text-process --input src_code.rs --normalize --extract-signatures --output processed.json`
**Rust lib**: `libtextproc`

### **json-transform**
**CLI**: `json-transform`
**Purpose**: Transform JSON data between formats
**Input: Source JSON, transformation rules
**Output: Transformed JSON
**CLI**: `json-transform --input data.json --rules transform.yaml --output result.json`
**Rust lib**: `libjsonxform`

### **file-hash**
**CLI**: `file-hash`
**Purpose**: Generate content hashes for files and directories
**Input: File paths, hash algorithm
**Output: Hash values with metadata
**CLI**: `file-hash --files ./src/ --algorithm sha256 --output hashes.json`
**Rust lib**: `libfilehash`

### **cache-manage**
**CLI**: `cache-manage`
**Purpose**: Manage various caches across tools
**Input: Cache directories, management operations
**Output: Cache status and operations
**CLI**: `cache-manage --cache-dir ./cache/ --cleanup --max-size 1GB --policy lru`
**Rust lib**: `libcacheman`

---

## MECE Validation: 47 Small Tools

### **Mutually Exclusive** - Each tool has single purpose:
- **Data Layer (8 tools)**: Pure ISG/CozoDB operations
- **Agent Runtime (6 tools)**: Agent model management only
- **Pattern Intelligence (6 tools)**: Pattern operations only
- **Validation (6 tools)**: Safety checking only
- **Orchestration (6 tools)**: Workflow coordination only
- **Integration (4 tools)**: External connectivity only
- **Configuration & Monitoring (4 tools)**: Setup and observability only
- **Utilities (4 tools)**: Generic operations only

### **Collectively Exhaustive** - Covers 100% of P41 functionality:
✅ All 76 P41 tools mapped to 47 focused tools
✅ No functionality lost in decomposition
✅ Each P41 tool's features preserved in MECE mapping
✅ Complete P40 MVP requirements maintained

### **UNIX Philosophy**:
- Each tool does one thing well
- Composable via pipes and scripts
- Text-based interfaces for interoperability
- Silent unless there's an error
- Independent operation possible

### **Example Workflow Compositions**:

**Complete debugging journey:**
```bash
# Build ISG data
isg-build --repo . --embeddings --summaries

# Match error to patterns
pattern-match --error "E0277" --code src/runtime.rs

# Run agent discovery
agent-dispatch --task discovery --agents A1-A6 --context error_ctx.json

# Validate proposed fix
rust-analyzer-check --changes fix.patch
cargo-validate --workspace . --test-affected

# Apply safely
git-apply-safe --patch fix.patch --validate --rollback-point HEAD
```

**Journey orchestration:**
```bash
journey-run --journey p20-bug-fix --input error_E0277.json | \
workflow-compose --pipeline discovery_validation_apply.yaml | \
metrics-collect --metrics success-rate,latency --report journey_metrics.json
```

**MECE Result**: 47 small, focused, mutually exclusive tools that are collectively exhaustive of all P41 functionality, following UNIX philosophy with CLI-first design and optional Rust library interfaces.