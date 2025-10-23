# A02 Architecture V1: Parseltongue Interface-Centric Design

## Executive Summary (Pyramid Apex)

**Parseltongue transforms Claude Code into a reliable local LLM system through 4 key interface layers: System Gate (hardware compatibility), Model Orchestration (local LLM management), Code Graph (interface analysis), and User Dialog (interaction flow). Each layer exposes simple, conversational interfaces that take specific inputs and return predictable outputs, enabling complex code analysis while maintaining reliability.**

---

## Layer 1: System Gate Interface
**Purpose**: Prevent poor performance by validating hardware before any operations

### `check_system_compatibility()`
- **What it does**: Evaluates if user's hardware can run Parseltongue effectively
- **Input**: `{ detailed: boolean? }` - optional flag for verbose reporting
- **Output**: `{ compatible: boolean, architecture: string, memory_gb: number, recommendations: string[], block_reasons: string[] }`
- **When called**: Plugin startup, before any model downloads
- **Success flow**: Returns `compatible: true` with system specs
- **Failure flow**: Returns `compatible: false` with specific blocking reasons

### `get_system_status()`
- **What it does**: Real-time system resource monitoring
- **Input**: `{ }` (empty)
- **Output**: `{ memory_available_gb: number, cpu_usage_percent: number, active_processes: number }`
- **When called**: During model operations to prevent resource exhaustion
- **Behavior**: Continuously monitors and reports current system capacity

---

## Layer 2: Model Orchestration Interface
**Purpose**: Manage local LLM models like a restaurant kitchen manages chefs

### `install_models()`
- **What it does**: Downloads and verifies local models with checksums and basic tests
- **Input**: `{ models?: string[], force_reinstall?: boolean, parallel_downloads?: number }`
- **Output**: `{ success: boolean, installed_models: ModelInfo[], failed_models: ModelError[], total_time_ms: number }`
- **Behavior**: Downloads 3 models (Qwen2.5-Coder 1.5B, Gemma 270M, SmolLM2 135M) in parallel, runs 20-line validation tests
- **User experience**: "Models downloading, please have this butter beer while we have them ready"

### `spawn_agents()`
- **What it does**: Creates worker agents from available models based on current RAM
- **Input**: `{ agent_count: number, task_type: "summarization" | "generation" | "analysis" }`
- **Output**: `{ agent_handles: string[], resource_allocation: ResourceInfo }`
- **Behavior**: Spins up 5-8 parallel agents using available memory, each with specific model assignment
- **Resource logic**: More RAM = more parallel agents, fewer agents on constrained systems

### `execute_agent_task()`
- **What it does**: Sends specific tasks to agents and collects results
- **Input**: `{ agent_id: string, task_type: string, input_data: any, timeout_ms: number }`
- **Output**: `{ result: any, execution_time_ms: number, token_usage: TokenUsage }`
- **Common tasks**: Code summarization (300 lines → 1 line), code generation, dependency analysis

---

## Layer 3: Code Graph Interface
**Purpose**: Transform Rust codebase into queryable graph of interfaces

### `build_interface_signature_graph()`
- **What it does**: Converts entire Rust codebase into Interface Signature Graph (ISG)
- **Input**: `{ repository_path: string, include_patterns?: string[], exclude_patterns?: string[] }`
- **Output**: `{ node_count: number, processing_time_ms: number, memory_usage_mb: number }`
- **Process**: Parse all Rust files → Extract interfaces → Build dependency graph → Store in CozoDB
- **Duration**: ~10 minutes for large codebases
- **User experience**: "Code indexing has begun and will take 10 minutes"

### `query_interface_graph()`
- **What it does**: Retrieves specific interfaces and their relationships
- **Input**: `{ query_type: "find_interface" | "find_dependencies" | "blast_radius", parameters: any }`
- **Output**: `{ interfaces: InterfaceInfo[], relationships: DependencyInfo[], context: CodeContext }`
- **Key queries**:
  - Find all interfaces implementing specific trait
  - Get blast radius for interface changes (what breaks if I modify this?)
  - Find similar interfaces across codebase

### `update_interface_graph()`
- **What it does**: Incrementally updates ISG when code changes
- **Input**: `{ changed_files: string[], operation: "add" | "modify" | "delete" }`
- **Output**: `{ updated_nodes: number, processing_time_ms: number }`
- **Behavior**: Only re-parses changed files, updates existing graph incrementally

---

## Layer 4: User Dialog Interface
**Purpose**: Guide user through micro-PRD to code changes workflow

### `refine_micro_prd()`
- **What it does**: Iteratively improves user's requirements until clear and actionable
- **Input**: `{ prd_text: string, iteration_count?: number, context?: any }`
- **Output**: `{ accepted: boolean, refined_prd: string, suggested_clarifications: string[], confidence_score: number }`
- **Process**: Analyze PRD against ISG context → Ask clarifying questions → Refine until confident
- **Iterations**: Typically 2-3 rounds to reach acceptable PRD

### `simulate_code_changes()`
- **What it does**: Generates proposed code changes without touching actual files
- **Input**: `{ refined_prd: string, target_interfaces: string[], simulation_mode: "safe" | "aggressive" }`
- **Output**: `{ proposed_changes: CodeChange[], impact_analysis: ImpactInfo, confidence_scores: number[] }`
- **Process**: Use reasoning LLM + ISG context → Generate interface changes → Validate with rust-analyzer
- **Safety**: Never modifies real files, only shows what would change

### `apply_code_changes()`
- **What it does**: Writes validated changes to actual code files
- **Input**: `{ accepted_changes: CodeChange[], run_tests: boolean, create_backup: boolean }`
- **Output**: `{ success: boolean, compilation_result: BuildResult, test_results: TestResult }`
- **Validation flow**: Apply changes → Run cargo build → Run cargo test → Rollback on failure
- **Success criteria**: Code compiles AND tests pass before confirming changes

---

## Interface Coordination Flow

### Complete User Journey Interface Sequence

1. **System Validation**: `check_system_compatibility()` → If compatible, proceed
2. **Model Setup**: `install_models()` → `spawn_agents()`
3. **Code Analysis**: `build_interface_signature_graph()` → Show analytics
4. **Requirements**: `refine_micro_prd()` (2-3 iterations)
5. **Simulation**: `simulate_code_changes()` → User review
6. **Application**: `apply_code_changes()` → Validation and commit

### Error Recovery Interfaces

Each interface includes graceful degradation:
- **Resource errors**: Suggest reducing parallel agents or closing other applications
- **Model errors**: Restart specific model, keep others running
- **Code parsing errors**: Skip problematic files, continue with rest
- **Compilation errors**: Show specific error, suggest fixes, retry simulation

### Performance Monitoring Interfaces

### `get_performance_metrics()`
- **Input**: `{ metric_type: "memory" | "speed" | "accuracy" }`
- **Output**: `{ current_usage: number, historical_trend: number[], recommendations: string[] }`
- **Purpose**: Help users understand why operations take time and how to improve

### `optimize_performance()`
- **Input**: `{ optimization_target: "speed" | "memory" | "accuracy" }`
- **Output**: `{ applied_optimizations: Optimization[], expected_improvement: string }`
- **Behavior**: Automatically adjusts model counts, caching strategies, parallel processing

---

## Interface Design Principles

### 1. **Conversational Inputs/Outputs**
All interfaces use natural language descriptions, not technical parameters. Users describe what they want, system explains what it will do.

### 2. **Progressive Disclosure**
Start with simple interfaces, reveal complexity only when needed. Most users only interact with 4-5 main interfaces.

### 3. **Predictable Results**
Same input always produces same output. No randomness in interface behavior, only in model generation.

### 4. **Graceful Failure**
Every interface includes clear error messages and recovery suggestions. No cryptic technical errors.

### 5. **Resource Awareness**
Interfaces automatically adapt to available system resources. Users don't need to manage memory or model allocation.

---

## Integration Points

### Claude Code MCP Integration
- **Plugin definition**: `plugin.json` with MCP server configuration
- **Tool exposure**: All interfaces appear as MCP tools in Claude Code
- **Lifecycle**: Automatic start/stop with plugin enable/disable
- **Transport**: Stdio communication between Claude Code and Parseltongue

### Dobby Orchestrator Communication
- **Protocol**: JSON-RPC over local socket
- **Message types**: Model operations, agent lifecycle, task execution
- **Resource coordination**: Real-time resource sharing between components
- **Error propagation**: Structured error messages with recovery suggestions

### CozoDB Interface Layer
- **Query language**: Datalog for graph traversals
- **Performance**: 100K+ QPS for complex graph queries
- **Schema**: Fixed schema for ISG nodes and relationships
- **Transactions**: ACID compliance for graph updates

---

## Success Metrics for Interfaces

### Usability Metrics
- **Time to first success**: <5 minutes from plugin install to first code analysis
- **Interface success rate**: >95% of interface calls complete successfully
- **Error recovery**: <10% of errors require user intervention

### Performance Metrics
- **System check**: <30 seconds
- **Model installation**: <3 minutes on 5 MBPS connection
- **ISG construction**: <10 minutes for 1M+ LOC codebase
- **PRD processing**: <2 minutes per iteration
- **Code simulation**: <5 minutes for typical changes

### Reliability Metrics
- **Crash rate**: <0.1% of operations
- **Data corruption**: Zero tolerance for ISG data integrity
- **Rollback success**: 100% of failed changes correctly reverted
- **Resource exhaustion**: Zero incidents of system crashes due to memory

This interface-centric architecture ensures Parseltongue provides reliable, predictable code transformation capabilities while maintaining the flexibility to handle diverse Rust codebases and user requirements.