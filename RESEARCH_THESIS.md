# Research Thesis: Graph-Based Incremental Compiler Architecture Using CozoDB

## Executive Summary

This document presents a comprehensive research thesis for building a graph-based compilation system that uses CozoDB as an incremental compilation cache with function-level dependency tracking. The architecture synthesizes concepts from multiple mature projects including rust-analyzer (incremental compilation), Joern (code property graphs), Salsa (incremental computation), and modern build systems (Buck2/Bazel).

**Core Innovation**: Treating the compilation process as a graph database problem where:
1. Source code is represented as a Code Property Graph (CPG)
2. Dependencies are explicit graph edges stored in CozoDB
3. Incremental compilation is reduced to Datalog queries
4. Function-level granularity enables fine-grained caching
5. Cross-file optimizations become graph queries

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Component Mapping](#component-mapping)
3. [Technology Stack](#technology-stack)
4. [Core Concepts](#core-concepts)
5. [System Design](#system-design)
6. [Implementation Phases](#implementation-phases)
7. [Key Challenges & Solutions](#key-challenges--solutions)
8. [Integration Strategy](#integration-strategy)
9. [Performance Considerations](#performance-considerations)
10. [Future Extensions](#future-extensions)

---

## Architecture Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Input Sources                            │
│         (Rust, C, C++, future: Python, JS, etc.)                │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Frontend Layer                                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ Tree-sitter  │  │     syn      │  │    clang     │         │
│  │   Parsers    │  │   (Rust)     │  │   (C/C++)    │         │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘         │
│         └───────────────────┼──────────────────┘                │
│                             ▼                                    │
│              ┌─────────────────────────┐                        │
│              │  AST Construction       │                        │
│              │  + Semantic Analysis    │                        │
│              └──────────┬──────────────┘                        │
└─────────────────────────┼───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                  Graph Construction Layer                        │
│         (CPG Builder - inspired by Joern)                       │
│  ┌──────────────────────────────────────────────────┐          │
│  │  Code Property Graph Generator                    │          │
│  │  - AST nodes → Graph nodes                        │          │
│  │  - Control Flow Graph (CFG)                       │          │
│  │  - Data Flow Graph (DFG)                          │          │
│  │  - Program Dependence Graph (PDG)                 │          │
│  │  - Call Graph                                     │          │
│  │  - Type Graph                                     │          │
│  └──────────────────────┬───────────────────────────┘          │
└─────────────────────────┼───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│              CozoDB - Graph Database Layer                       │
│                    (Core of the System)                          │
│  ┌──────────────────────────────────────────────────┐          │
│  │  Relations Stored:                                │          │
│  │  • ast_nodes(id, type, parent, file, span)       │          │
│  │  • functions(id, name, signature, hash, file)    │          │
│  │  • calls(caller_id, callee_id, call_site)        │          │
│  │  • data_flow(from_node, to_node, var_name)       │          │
│  │  • control_flow(from_block, to_block, condition) │          │
│  │  • dependencies(func_id, depends_on_func_id)     │          │
│  │  • compilation_artifacts(func_id, llvm_ir, obj)  │          │
│  │  • fingerprints(func_id, content_hash, deps_hash)│          │
│  └──────────────────────┬───────────────────────────┘          │
└─────────────────────────┼───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│            Incremental Computation Layer (Salsa)                 │
│  ┌──────────────────────────────────────────────────┐          │
│  │  Query System:                                    │          │
│  │  • parse(file) → AST                              │          │
│  │  • build_cpg(ast) → CPG                           │          │
│  │  • analyze_dependencies(cpg) → DepGraph           │          │
│  │  • compute_fingerprint(func) → Hash               │          │
│  │  • check_cache(func) → Option<Artifact>           │          │
│  │  • compile_function(func) → Artifact              │          │
│  │                                                    │          │
│  │  Memoization + Dependency Tracking                │          │
│  │  (Auto-invalidation on changes)                   │          │
│  └──────────────────────┬───────────────────────────┘          │
└─────────────────────────┼───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│               Analysis & Optimization Layer                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │  Datalog    │  │   petgraph  │  │     egg     │            │
│  │  Queries    │  │  Algorithms │  │  (e-graphs) │            │
│  │  (CozoDB)   │  │             │  │             │            │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘            │
│         │                 │                 │                   │
│         └────────────┬────┴────────────────┘                   │
│                      ▼                                          │
│         ┌─────────────────────────┐                            │
│         │  Optimization Passes:   │                            │
│         │  • Dead code elimination│                            │
│         │  • Inlining decisions   │                            │
│         │  • Constant propagation │                            │
│         │  • Escape analysis      │                            │
│         └──────────┬──────────────┘                            │
└────────────────────┼───────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Backend Layer                                 │
│  ┌──────────────┐        ┌──────────────┐                      │
│  │   LLVM IR    │   OR   │  Cranelift   │                      │
│  │ (via inkwell)│        │  (Fast JIT)  │                      │
│  └──────┬───────┘        └──────┬───────┘                      │
│         └───────────┬────────────┘                              │
│                     ▼                                            │
│         ┌────────────────────┐                                  │
│         │  Machine Code Gen  │                                  │
│         └──────────┬─────────┘                                  │
└────────────────────┼──────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Linking Layer                                 │
│            mold (Fast Parallel Linker)                          │
│  • Incremental linking                                          │
│  • Symbol resolution                                            │
│  • LTO support                                                  │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
                  ┌─────────┐
                  │ Binary  │
                  └─────────┘
```

---

## Component Mapping

### Core Components to Repository Mapping

| Component | Primary Repos | Role |
|-----------|--------------|------|
| **Graph Database** | CozoDB | Stores CPG, dependencies, cache |
| **Incremental Engine** | Salsa | Memoization, dependency tracking |
| **Parser (Rust)** | tree-sitter-rust, syn | Source → AST |
| **Parser (C/C++)** | tree-sitter-c/cpp, clangd | Source → AST |
| **CPG Generator** | Joern, codepropertygraph | AST → CPG |
| **Graph Algorithms** | petgraph, egg | Analysis, optimization |
| **Code Generation** | inkwell, cranelift | IR → machine code |
| **Linker** | mold, lld | Object files → binary |
| **Caching Strategy** | sccache, ccache | Distributed cache design |
| **Build System** | cargo, buck2, bazel | Dependency resolution |
| **Fingerprinting** | BLAKE3, rustc-hash | Content-addressed storage |
| **Parallelization** | rayon | Parallel compilation |
| **Datalog Engine** | CozoDB, souffle | Query language for analysis |
| **Visualization** | graphviz | Debug & exploration |

---

## Technology Stack

### Primary Stack

```rust
// Core Dependencies
cozo = "0.7"              // Graph database
salsa = "0.17"            // Incremental computation
tree-sitter = "0.20"      // Incremental parsing
petgraph = "0.6"          // Graph algorithms
inkwell = "0.4"           // LLVM bindings
rayon = "1.8"             // Parallelism
blake3 = "1.5"            // Hashing

// Language-specific
syn = "2.0"               // Rust parsing
tree-sitter-rust = "*"    // Rust grammar
tree-sitter-c = "*"       // C grammar
tree-sitter-cpp = "*"     // C++ grammar

// Optional
egg = "0.9"               // E-graph optimizations
```

### Language Choice: Rust

**Rationale**:
1. **Zero-cost abstractions**: Performance critical
2. **Memory safety**: Compiler stability
3. **Ecosystem**: All key libraries available
4. **Concurrency**: Rayon for parallel compilation
5. **LLVM integration**: Mature tooling (inkwell)
6. **Reference implementation**: rust-analyzer proves viability

---

## Core Concepts

### 1. Code Property Graph (CPG)

A CPG is a unified representation combining:
- **AST**: Abstract Syntax Tree
- **CFG**: Control Flow Graph
- **DFG**: Data Flow Graph
- **PDG**: Program Dependence Graph
- **Call Graph**: Function invocations
- **Type Graph**: Type relationships

**CPG Schema (CozoDB Relations)**:

```datalog
// Node types
:create ast_node {
    id: Uuid,
    type: String,          // "Function", "Expr", "Stmt", etc.
    parent: Uuid?,
    file_id: Uuid,
    span_start: Int,
    span_end: Int,
    => id
}

:create function {
    id: Uuid,
    name: String,
    signature: String,
    file_id: Uuid,
    content_hash: String,  // BLAKE3 hash
    => id
}

// Edge types
:create control_flow {
    from_block: Uuid,
    to_block: Uuid,
    condition: String?,
    => [from_block, to_block]
}

:create data_flow {
    from_node: Uuid,
    to_node: Uuid,
    variable: String,
    => [from_node, to_node, variable]
}

:create call_edge {
    caller_id: Uuid,
    callee_id: Uuid,
    call_site: Uuid,
    => [caller_id, callee_id, call_site]
}

// Dependency tracking
:create function_dependency {
    function_id: Uuid,
    depends_on: Uuid,
    dependency_type: String,  // "calls", "reads_global", "type_depends"
    => [function_id, depends_on]
}

// Compilation cache
:create compilation_artifact {
    function_id: Uuid,
    llvm_ir: String,
    object_code: Bytes,
    optimizations_applied: [String],
    timestamp: Int,
    => function_id
}

:create fingerprint {
    function_id: Uuid,
    content_hash: String,
    deps_hash: String,      // Hash of all dependencies
    transitive_hash: String, // Includes transitive deps
    => function_id
}
```

### 2. Incremental Compilation Strategy

**Key Insight**: Only recompile functions whose transitive closure of dependencies has changed.

**Algorithm**:

```rust
fn incremental_compile(changed_files: &[File]) -> Result<Binary> {
    // 1. Parse changed files incrementally (tree-sitter)
    let new_asts = parse_incrementally(changed_files)?;

    // 2. Update CPG in CozoDB
    let changed_functions = update_cpg_incremental(new_asts)?;

    // 3. Query CozoDB for transitive dependencies
    let affected_functions = query_transitive_deps(changed_functions)?;

    // 4. Check cache for each affected function
    let to_compile = affected_functions.iter()
        .filter(|f| !cache_valid(f))
        .collect();

    // 5. Compile in parallel (Rayon)
    let artifacts: Vec<_> = to_compile.par_iter()
        .map(|f| compile_function(f))
        .collect()?;

    // 6. Update cache
    for (func, artifact) in artifacts {
        store_artifact(func, artifact)?;
    }

    // 7. Incremental linking (mold)
    link_incremental(&artifacts)
}

fn cache_valid(function: &Function) -> bool {
    // Query CozoDB
    let stored_hash = db.query("
        ?[hash] := *fingerprint{function_id: $func_id, transitive_hash: hash}
    ")?;

    let current_hash = compute_transitive_hash(function)?;
    stored_hash == current_hash
}
```

**Salsa Integration**:

```rust
#[salsa::jar(db = CompilerDb)]
struct Jar(
    parse_file,
    build_cpg,
    compute_dependencies,
    compute_fingerprint,
    compile_function,
);

#[salsa::tracked]
fn parse_file(db: &dyn CompilerDb, file: File) -> Arc<Ast> {
    // Tree-sitter incremental parse
    tree_sitter::parse_incremental(file)
}

#[salsa::tracked]
fn build_cpg(db: &dyn CompilerDb, ast: Arc<Ast>) -> Arc<Cpg> {
    // Generate CPG from AST
    cpg_builder::build(ast)
}

#[salsa::tracked]
fn compute_dependencies(db: &dyn CompilerDb, func: Function) -> DependencySet {
    // Query CozoDB for dependencies
    query_cozo_dependencies(db, func)
}

#[salsa::tracked]
fn compute_fingerprint(db: &dyn CompilerDb, func: Function) -> Hash {
    let deps = compute_dependencies(db, func);
    let content = func.content_hash();
    let deps_hashes: Vec<_> = deps.iter()
        .map(|d| compute_fingerprint(db, *d))
        .collect();

    blake3::hash(&[content, deps_hashes].concat())
}

#[salsa::tracked]
fn compile_function(db: &dyn CompilerDb, func: Function) -> Artifact {
    // Check CozoDB cache first
    if let Some(cached) = check_cache(db, func) {
        return cached;
    }

    // Generate LLVM IR
    let ir = generate_llvm_ir(db, func);

    // Optimize
    let optimized = optimize_ir(db, ir);

    // Store in CozoDB
    store_artifact(db, func, optimized)
}
```

### 3. Dependency Tracking Granularity

**Function-Level Dependencies**:

```datalog
// Query: Find all functions that depend on `foo`
?[dependent_func] :=
    *function{id: $foo_id, name: "foo"},
    *function_dependency{depends_on: $foo_id, function_id: dependent_func}

// Query: Transitive dependencies (using recursive Datalog)
transitive_deps[func, dep] :=
    *function_dependency{function_id: func, depends_on: dep}

transitive_deps[func, dep] :=
    transitive_deps[func, intermediate],
    *function_dependency{function_id: intermediate, depends_on: dep}

// Query: All functions that need recompilation if `foo` changes
?[needs_recompile] :=
    *function{id: $foo_id, name: "foo"},
    transitive_deps[needs_recompile, $foo_id]
```

**Types of Dependencies**:
1. **Direct call**: Function A calls function B
2. **Data flow**: Function A reads global modified by B
3. **Type dependency**: Function A uses type defined in B
4. **Macro expansion**: Rust macro from B used in A
5. **Generic instantiation**: Generic B instantiated in A
6. **Trait implementation**: A implements trait from B

### 4. Fingerprinting Strategy

**Content-Addressed Storage**:

```rust
struct FunctionFingerprint {
    // Hash of function body only
    content_hash: Blake3Hash,

    // Hash of immediate dependencies
    direct_deps_hash: Blake3Hash,

    // Hash of transitive closure
    // (Includes all dependencies recursively)
    transitive_hash: Blake3Hash,

    // Metadata
    signature_hash: Blake3Hash,
    optimization_level: String,
}

fn compute_transitive_hash(db: &dyn CompilerDb, func: &Function) -> Blake3Hash {
    let mut hasher = blake3::Hasher::new();

    // 1. Hash function content
    hasher.update(func.content_hash());

    // 2. Query dependencies from CozoDB
    let deps = query_dependencies(db, func);

    // 3. Recursively hash dependencies (memoized by Salsa)
    let mut dep_hashes: Vec<_> = deps.iter()
        .map(|dep| compute_transitive_hash(db, dep))
        .collect();

    // Sort for determinism
    dep_hashes.sort();

    for hash in dep_hashes {
        hasher.update(hash.as_bytes());
    }

    hasher.finalize()
}
```

---

## System Design

### Database Schema Design

**CozoDB Relation Design**:

```datalog
// ============ CPG Schema ============

// Files
:create file {
    id: Uuid,
    path: String,
    content_hash: String,
    last_modified: Int,
    => id
}

// AST Nodes
:create ast_node {
    id: Uuid,
    type: String,
    parent_id: Uuid?,
    file_id: Uuid,
    span_start: Int,
    span_end: Int,
    text: String?,
    => id
}

// Functions (first-class entities)
:create function {
    id: Uuid,
    name: String,
    qualified_name: String,  // "module::path::function"
    signature: String,
    file_id: Uuid,
    ast_node_id: Uuid,
    content_hash: String,
    is_generic: Bool,
    is_inline: Bool,
    visibility: String,
    => id
}

// Types
:create type_def {
    id: Uuid,
    name: String,
    kind: String,  // "struct", "enum", "trait", "alias"
    file_id: Uuid,
    ast_node_id: Uuid,
    => id
}

// Control Flow
:create basic_block {
    id: Uuid,
    function_id: Uuid,
    entry: Bool,
    exit: Bool,
    => id
}

:create control_flow_edge {
    from_block: Uuid,
    to_block: Uuid,
    edge_type: String,  // "fallthrough", "branch_true", "branch_false", "return"
    condition: String?,
    => [from_block, to_block]
}

// Data Flow
:create variable {
    id: Uuid,
    name: String,
    scope_id: Uuid,
    type_id: Uuid?,
    => id
}

:create data_flow_edge {
    from_node: Uuid,
    to_node: Uuid,
    variable_id: Uuid,
    edge_type: String,  // "def", "use", "def-use"
    => [from_node, to_node, variable_id]
}

// Call Graph
:create call_site {
    id: Uuid,
    caller_id: Uuid,
    callee_id: Uuid?,  // Null for indirect calls
    ast_node_id: Uuid,
    is_direct: Bool,
    => id
}

// Dependencies
:create dependency {
    from_func: Uuid,
    to_func: Uuid,
    dep_type: String,  // "calls", "reads_global", "writes_global", "uses_type", "generic_instantiation"
    strength: String,  // "strong" (must recompile), "weak" (may recompile)
    => [from_func, to_func, dep_type]
}

// ============ Compilation Cache Schema ============

:create fingerprint {
    function_id: Uuid,
    content_hash: String,
    signature_hash: String,
    direct_deps_hash: String,
    transitive_hash: String,
    timestamp: Int,
    => function_id
}

:create compilation_artifact {
    function_id: Uuid,
    optimization_level: String,
    llvm_ir: String,
    llvm_bitcode: Bytes,
    object_code: Bytes,
    symbol_table: Json,
    size_bytes: Int,
    compile_time_ms: Int,
    timestamp: Int,
    => [function_id, optimization_level]
}

:create artifact_dependency {
    artifact_func: Uuid,
    depends_on_artifact: Uuid,
    => [artifact_func, depends_on_artifact]
}

// ============ Build Metadata ============

:create build_session {
    id: Uuid,
    timestamp: Int,
    git_commit: String?,
    changed_files: [Uuid],
    functions_recompiled: [Uuid],
    total_functions: Int,
    cache_hit_rate: Float,
    build_time_ms: Int,
    => id
}
```

### Query Examples

**1. Find functions to recompile after file change**:

```datalog
// Input: changed file ID
input_changed_file[file_id] <- [[to_uuid("...")]]

// Functions in changed file
changed_functions[func_id] :=
    input_changed_file[file_id],
    *function{id: func_id, file_id: file_id}

// Transitive reverse dependencies
affected[func] := changed_functions[func]

affected[func] :=
    affected[dep],
    *dependency{from_func: func, to_func: dep, strength: "strong"}

// Check cache validity
needs_recompile[func] :=
    affected[func],
    *function{id: func, content_hash: current_hash},
    *fingerprint{function_id: func, transitive_hash: cached_hash},
    current_hash != cached_hash

?[func, name] :=
    needs_recompile[func],
    *function{id: func, name: name}
```

**2. Detect circular dependencies**:

```datalog
// Find cycles in call graph
path[from, to] := *dependency{from_func: from, to_func: to}
path[from, to] := path[from, mid], *dependency{from_func: mid, to_func: to}

circular_dep[func1, func2] :=
    path[func1, func2],
    path[func2, func1],
    func1 != func2

?[func1_name, func2_name] :=
    circular_dep[func1, func2],
    *function{id: func1, name: func1_name},
    *function{id: func2, name: func2_name}
```

**3. Find dead code**:

```datalog
// Entry points (public functions, main, tests)
entry_point[func] :=
    *function{id: func, visibility: "pub"}

entry_point[func] :=
    *function{id: func, name: "main"}

// Reachable functions
reachable[func] := entry_point[func]
reachable[callee] :=
    reachable[caller],
    *dependency{from_func: caller, to_func: callee}

// Dead code = not reachable
dead_code[func] :=
    *function{id: func},
    not reachable[func]

?[func_name, file_path] :=
    dead_code[func],
    *function{id: func, name: func_name, file_id: file_id},
    *file{id: file_id, path: file_path}
```

**4. Cache statistics**:

```datalog
// Cache hit rate per build
?[session_id, hit_rate, recompiled_count, total_count] :=
    *build_session{
        id: session_id,
        cache_hit_rate: hit_rate,
        functions_recompiled: recompiled,
        total_functions: total
    },
    recompiled_count = count(recompiled)
```

---

## Implementation Phases

### Phase 1: Foundation (Months 1-2)

**Objective**: Build minimal viable compiler with basic incremental support

**Deliverables**:
1. CozoDB schema implementation
2. Basic parser integration (tree-sitter for one language)
3. Simple AST → CPG converter
4. Function extraction and fingerprinting
5. Naive compilation cache (no Salsa yet)

**Milestones**:
- [ ] Parse single Rust file into AST
- [ ] Convert AST to CPG graph in CozoDB
- [ ] Extract functions and compute content hashes
- [ ] Cache compiled function artifacts
- [ ] Detect changed functions and recompile only those

**Technology Focus**:
- CozoDB basics
- tree-sitter integration
- BLAKE3 hashing
- Basic inkwell LLVM IR generation

### Phase 2: Incremental Computation (Months 3-4)

**Objective**: Integrate Salsa for automatic dependency tracking

**Deliverables**:
1. Salsa query system implementation
2. Automatic invalidation on file changes
3. Parallel compilation with Rayon
4. Dependency tracking in CozoDB
5. Transitive dependency queries

**Milestones**:
- [ ] Wrap compilation steps in Salsa queries
- [ ] Implement incremental parsing
- [ ] Parallel function compilation
- [ ] Query transitive dependencies from CozoDB
- [ ] Cache validation using transitive hashes

**Technology Focus**:
- Salsa framework
- Rayon parallelism
- CozoDB recursive Datalog queries

### Phase 3: Advanced CPG (Months 5-6)

**Objective**: Full CPG with control flow, data flow, and call graphs

**Deliverables**:
1. Control Flow Graph generation
2. Data Flow Analysis
3. Call graph construction
4. Type dependency tracking
5. Cross-function analysis queries

**Milestones**:
- [ ] CFG generation from AST
- [ ] Reaching definitions analysis
- [ ] Use-def chains
- [ ] Interprocedural analysis
- [ ] Joern-style query interface

**Technology Focus**:
- petgraph for graph algorithms
- Dataflow analysis algorithms
- CozoDB advanced queries

### Phase 4: Multi-Language Support (Months 7-8)

**Objective**: Support Rust, C, and C++

**Deliverables**:
1. Tree-sitter grammars for C/C++
2. Unified CPG schema across languages
3. Cross-language call tracking
4. Language-specific semantic analysis

**Milestones**:
- [ ] Parse C/C++ with tree-sitter
- [ ] Map C/C++ to unified CPG
- [ ] Track Rust → C FFI calls
- [ ] Handle C++ templates
- [ ] Cross-language optimization

**Technology Focus**:
- tree-sitter-c, tree-sitter-cpp
- clang AST for complex C++ cases
- FFI handling

### Phase 5: Optimization (Months 9-10)

**Objective**: Advanced optimizations using graph queries

**Deliverables**:
1. Inlining decisions based on call graph
2. Dead code elimination
3. Constant propagation across functions
4. E-graph based optimizations (egg)
5. Profile-guided optimization hooks

**Milestones**:
- [ ] Implement inlining heuristics
- [ ] Global dead code elimination
- [ ] Interprocedural constant propagation
- [ ] E-graph optimizer for expression simplification
- [ ] Integration with PGO data

**Technology Focus**:
- egg e-graphs
- CozoDB optimization queries
- LLVM optimization passes

### Phase 6: Distributed Caching (Months 11-12)

**Objective**: Scale to large codebases and teams

**Deliverables**:
1. Distributed cache backend (S3/Redis)
2. Content-addressed artifact storage
3. Remote execution support
4. Build coordination across machines
5. Cache eviction policies

**Milestones**:
- [ ] Upload/download artifacts from remote cache
- [ ] Parallel builds across machines
- [ ] Deterministic builds for cache sharing
- [ ] Cache analytics and monitoring
- [ ] LRU/LFU cache eviction

**Technology Focus**:
- sccache architecture
- Buck2 remote execution
- Content-addressed storage

### Phase 7: Polish & Productionization (Month 13+)

**Objective**: Make it production-ready

**Deliverables**:
1. CLI tool (like cargo/rustc)
2. LSP integration for IDE support
3. Comprehensive error messages
4. Documentation and tutorials
5. Benchmarking suite

**Milestones**:
- [ ] User-friendly CLI
- [ ] LSP server for incremental diagnostics
- [ ] Error recovery in parser
- [ ] Performance benchmarks
- [ ] Migration guide from existing build systems

---

## Key Challenges & Solutions

### Challenge 1: Handling Cross-File Dependencies

**Problem**: Function A in file X calls function B in file Y. If B changes, A must be recompiled.

**Solution**:
1. Store call graph in CozoDB
2. Use Datalog recursive queries to find transitive dependencies
3. Salsa tracks file-level changes, triggers recompilation
4. CozoDB pinpoints exact functions affected

```datalog
// Efficient transitive dependency query
?[affected_func] :=
    *function{id: $changed_func_id},
    *dependency{to_func: $changed_func_id, from_func: affected_func}

// Recursive for transitive closure
transitive_affected[func] := ?[func] /* base case */
transitive_affected[func] :=
    transitive_affected[dep],
    *dependency{to_func: dep, from_func: func}
```

### Challenge 2: Generic Functions and Monomorphization

**Problem**: Rust generics are monomorphized per use. `Vec<T>` becomes `Vec<i32>`, `Vec<String>`, etc. Each is a separate compilation unit.

**Solution**:
1. Store generic functions with parametric hash
2. Track instantiations separately
3. Each instantiation gets own fingerprint
4. Cache per monomorphized version

```rust
// In CozoDB
:create generic_function {
    id: Uuid,
    name: String,
    type_params: [String],
    base_hash: String,
    => id
}

:create generic_instantiation {
    generic_id: Uuid,
    type_args: [String],
    instantiation_hash: String,
    artifact_id: Uuid,
    => [generic_id, type_args]
}
```

### Challenge 3: Macro Expansion

**Problem**: Rust macros can generate arbitrary code. Changes to macro definition affect all use sites.

**Solution**:
1. Expand macros during parsing (using syn)
2. Track macro definition → use site dependencies
3. Hash macro definition, include in dependent functions' fingerprints
4. On macro change, invalidate all users

```datalog
:create macro_def {
    id: Uuid,
    name: String,
    definition_hash: String,
    => id
}

:create macro_invocation {
    call_site: Uuid,
    macro_id: Uuid,
    expanded_code: String,
    => [call_site, macro_id]
}

// Dependency: function → macros it uses
?[func_id, macro_id] :=
    *function{id: func_id, ast_node_id: func_ast},
    *macro_invocation{call_site: invoc_site, macro_id: macro_id},
    // (invoc_site is descendant of func_ast)
```

### Challenge 4: Linker Symbol Resolution

**Problem**: Object files reference symbols. If symbol changes (even in unrelated function), linker may need to re-run.

**Solution**:
1. mold supports incremental linking
2. Track symbol table in CozoDB
3. Only re-link if:
   - New symbols added/removed
   - Symbol visibility changed
   - Address layout changed (rare with incremental)

```rust
:create symbol {
    id: Uuid,
    name: String,
    function_id: Uuid,
    visibility: String,
    address: Int?,
    => [name, function_id]
}

// Query: symbols changed since last link
?[symbol_name] :=
    *symbol{name: symbol_name, function_id: func},
    *fingerprint{function_id: func, timestamp: ts},
    *build_session{id: $last_session, timestamp: last_ts},
    ts > last_ts
```

### Challenge 5: Handling Side Effects

**Problem**: Not all functions are pure. File I/O, global state, etc. affect determinism.

**Solution**:
1. Track "purity" of functions
2. Impure functions always recompile (or use stricter invalidation)
3. Escape analysis to detect side effects
4. Store effect summary in CozoDB

```datalog
:create function_effects {
    function_id: Uuid,
    is_pure: Bool,
    reads_globals: [String],
    writes_globals: [String],
    performs_io: Bool,
    => function_id
}

// Conservative: recompile if any global read has changed
needs_recompile[func] :=
    *function_effects{function_id: func, reads_globals: globals},
    global_changed[global],
    contains(globals, global)
```

### Challenge 6: Deterministic Builds

**Problem**: For cache sharing across machines, builds must be deterministic.

**Solution**:
1. Normalize all file paths (use content addressing)
2. Sort all inputs (e.g., dep_hashes.sort())
3. Fix timestamps in artifacts
4. Disable non-deterministic optimizations
5. BLAKE3 ensures hash determinism

```rust
// Normalize during fingerprinting
fn compute_deterministic_hash(func: &Function) -> Blake3Hash {
    let mut hasher = blake3::Hasher::new();

    // 1. Content (already deterministic if AST-based)
    hasher.update(func.canonical_repr());

    // 2. Dependencies (sorted)
    let mut deps = get_dependencies(func);
    deps.sort_by_key(|d| d.id);
    for dep in deps {
        hasher.update(dep.hash());
    }

    // 3. Compilation flags (normalized)
    let flags = normalize_flags(&func.compile_flags);
    hasher.update(&flags);

    hasher.finalize()
}
```

### Challenge 7: Incremental Parsing Correctness

**Problem**: tree-sitter's incremental parsing might introduce bugs if not careful.

**Solution**:
1. tree-sitter is battle-tested (used in GitHub, Atom, Zed)
2. Always validate parse tree after incremental parse
3. Fall back to full parse if error nodes present
4. Unit test incremental parse extensively

```rust
fn parse_incremental(file: &File, old_tree: Option<Tree>) -> Result<Tree> {
    let new_tree = parser.parse_with(
        &file.content,
        old_tree.as_ref(),
    )?;

    // Validate
    if has_errors(&new_tree) {
        warn!("Incremental parse failed, falling back to full parse");
        return parser.parse(&file.content, None);
    }

    Ok(new_tree)
}
```

---

## Integration Strategy

### Integration Flow

```
1. Initialize System
   ├── Load CozoDB database
   ├── Initialize Salsa runtime
   └── Load cached artifacts

2. Watch File Changes (LSP or CLI)
   ├── Detect changed files (inotify, etc.)
   ├── Trigger Salsa query invalidation
   └── Mark affected functions dirty

3. Incremental Compilation
   ├── Parse changed files (tree-sitter)
   ├── Update CPG in CozoDB
   ├── Query affected functions (Datalog)
   ├── Recompile functions (parallel with Rayon)
   └── Update cache

4. Linking
   ├── Collect object files
   ├── Run mold incremental linker
   └── Produce binary

5. Result
   └── Emit diagnostics, binary, and stats
```

### API Design

```rust
// High-level API
pub struct GraphCompiler {
    db: CozoDb,
    salsa_db: SalsaDatabase,
    config: CompilerConfig,
}

impl GraphCompiler {
    pub fn new(config: CompilerConfig) -> Result<Self>;

    pub fn compile(&mut self, files: &[PathBuf]) -> Result<Binary> {
        // Full compilation
    }

    pub fn incremental_compile(&mut self, changed: &[PathBuf]) -> Result<Binary> {
        // Incremental compilation
    }

    pub fn query(&self, query: &str) -> Result<QueryResult> {
        // Run Datalog query on CPG
    }

    pub fn visualize_dependencies(&self, func: &str) -> Result<DotGraph> {
        // Generate Graphviz graph
    }

    pub fn cache_stats(&self) -> CacheStats {
        // Return cache hit rate, etc.
    }
}

// Lower-level APIs
pub mod parser {
    pub fn parse_rust(file: &Path) -> Result<Ast>;
    pub fn parse_c(file: &Path) -> Result<Ast>;
    pub fn parse_cpp(file: &Path) -> Result<Ast>;
}

pub mod cpg {
    pub fn build_cpg(ast: &Ast) -> Result<Cpg>;
    pub fn extract_functions(cpg: &Cpg) -> Vec<Function>;
    pub fn build_cfg(func: &Function) -> ControlFlowGraph;
    pub fn build_dfg(func: &Function) -> DataFlowGraph;
}

pub mod cache {
    pub fn compute_fingerprint(func: &Function) -> Fingerprint;
    pub fn check_cache(func: &Function) -> Option<Artifact>;
    pub fn store_artifact(func: &Function, artifact: Artifact);
}

pub mod codegen {
    pub fn generate_llvm_ir(func: &Function) -> LlvmIr;
    pub fn optimize_ir(ir: LlvmIr, level: OptLevel) -> LlvmIr;
    pub fn compile_to_object(ir: LlvmIr) -> ObjectFile;
}

pub mod linker {
    pub fn link(objects: &[ObjectFile]) -> Result<Binary>;
    pub fn incremental_link(objects: &[ObjectFile], prev_binary: &Binary) -> Result<Binary>;
}
```

---

## Performance Considerations

### Expected Performance Characteristics

**Clean Build**:
- Parse: ~10 files/sec (tree-sitter is fast)
- CPG generation: ~5 files/sec (graph construction overhead)
- Compilation: ~2 functions/sec (LLVM is slow)
- Overall: Comparable to rustc/clang

**Incremental Build**:
- File change detection: <10ms
- CPG update: ~50ms (Datalog queries)
- Affected function detection: ~100ms (graph traversal)
- Recompilation: Only changed functions
- Expected: **10-100x speedup** vs clean build for small changes

**Cache Hit Scenarios**:
- Local cache hit: <1ms (hash lookup in CozoDB)
- Remote cache hit: ~50-200ms (network + deserialization)
- Cache miss: Full compilation time

### Optimization Strategies

1. **Parallel Compilation**:
   - Use Rayon to compile functions in parallel
   - Dependency graph ensures correct ordering
   - Expected: Linear speedup with cores

2. **Lazy CPG Construction**:
   - Only build CPG for changed files
   - Reuse cached CPG for unchanged files
   - Store CPG in CozoDB for persistence

3. **Incremental Linking**:
   - mold is 10-50x faster than GNU ld
   - Incremental mode reuses previous link
   - Expected: Linking becomes negligible

4. **Smart Caching**:
   - Content-addressed storage (no false cache misses)
   - Distributed cache for team sharing
   - Expected: 80-95% cache hit rate after initial build

5. **Datalog Query Optimization**:
   - CozoDB uses magic set transformation
   - Indexes on frequently queried relations
   - Expected: Sub-100ms queries for 10k+ functions

### Benchmarking Plan

**Metrics to Track**:
- Clean build time
- Incremental build time (1%, 5%, 10% file changes)
- Cache hit rate
- Parse time
- CPG generation time
- Compilation time per function
- Link time
- Memory usage (CozoDB + Salsa)
- Disk space (cache size)

**Baseline Comparisons**:
- vs rustc (incremental mode)
- vs clang (ccache)
- vs cargo (clean vs incremental)

**Target Goals**:
- Clean build: Within 2x of rustc/clang
- Incremental build: 10-100x faster than rustc incremental
- Cache hit: <10ms
- Memory usage: <4GB for 100k LOC project

---

## Future Extensions

### 1. Cross-Project Caching

**Concept**: Share cache across different projects if they use same dependencies.

**Implementation**:
- Content-addressed storage for artifacts
- Global cache namespace (e.g., `hash://blake3/<function_hash>`)
- Projects reference shared artifacts
- Saves recompilation of common dependencies (e.g., serde, tokio)

### 2. Speculative Compilation

**Concept**: Predict which functions might be edited next, precompile them.

**Implementation**:
- Track edit patterns (ML model)
- Precompile frequently co-edited functions
- Run during idle time
- Reduces incremental build latency

### 3. Profile-Guided Optimization

**Concept**: Use runtime profiles to guide inlining, optimization decisions.

**Implementation**:
- Instrument binary, collect call counts
- Store profile in CozoDB
- Query hot paths during optimization
- Inline hot functions, optimize cold paths less

### 4. Distributed Compilation

**Concept**: Compile functions on remote machines.

**Implementation**:
- Similar to distcc/icecream
- Send CPG + dependencies to remote workers
- Workers check their cache, compile if needed
- Return artifacts
- Requires deterministic builds

### 5. IDE Integration

**Concept**: Real-time incremental compilation in editor.

**Implementation**:
- LSP server using Salsa
- On keystroke, incrementally parse + analyze
- CozoDB stores CPG, provides instant queries
- Red squiggles appear immediately
- Similar to rust-analyzer, but with full CPG

### 6. Verification Integration

**Concept**: Integrate formal verification using CPG.

**Implementation**:
- Export CPG to verification tools (e.g., CBMC, Kani)
- Store verification results in CozoDB
- Incremental verification (only re-verify changed functions)
- Proves correctness properties

### 7. Security Analysis

**Concept**: Detect vulnerabilities using graph queries.

**Implementation**:
- Joern-style security queries
- Datalog rules for vulnerability patterns
- Taint analysis (data flow from user input to sensitive sink)
- Store findings in CozoDB for tracking

**Example Query (SQL injection detection)**:
```datalog
// User input sources
user_input[node] :=
    *function{name: "read_user_input", id: func},
    *data_flow_edge{from_node: func, to_node: node}

// Sensitive sinks
sql_exec[node] :=
    *function{name: "execute_sql", id: func},
    *call_site{callee_id: func, ast_node_id: node}

// Taint propagation (transitive)
tainted[node] := user_input[node]
tainted[to] :=
    tainted[from],
    *data_flow_edge{from_node: from, to_node: to}

// Vulnerability: tainted data reaches SQL exec
sql_injection[site] :=
    tainted[data],
    sql_exec[site],
    *data_flow_edge{from_node: data, to_node: site}

?[file, line, func] :=
    sql_injection[site],
    *ast_node{id: site, file_id: file, span_start: line},
    *function{ast_node_id: func_ast},
    // (site is in func_ast)
```

### 8. Multi-Target Compilation

**Concept**: Compile for multiple platforms simultaneously.

**Implementation**:
- Store CPG once, compile to multiple targets
- Artifacts keyed by (function_hash, target_triple)
- Parallel backend compilation
- Expected: Near-linear speedup with targets

### 9. Hot Code Reloading

**Concept**: Replace functions in running program without restart.

**Implementation**:
- Generate dynamic libraries per function
- Hot-reload changed functions
- Requires careful state management
- Use cases: Game dev, web servers

### 10. Language Extension

**Concept**: Add support for more languages (Python, Go, JavaScript).

**Implementation**:
- tree-sitter has grammars for 50+ languages
- Unified CPG schema
- Language-specific semantic analysis plugins
- Expected: Python/JS support in months 12-18

---

## Conclusion

This architecture provides a solid foundation for building a graph-based incremental compiler using CozoDB. The key innovations are:

1. **Graph-native compilation**: CPG stored in graph database
2. **Incremental by default**: Salsa + CozoDB = automatic incrementality
3. **Function-level granularity**: Finer than file-level, coarser than instruction-level
4. **Datalog for analysis**: Expressive queries for optimization and analysis
5. **Content-addressed caching**: Deterministic, shareable across projects
6. **Multi-language support**: Unified CPG for interop

The phased implementation plan provides a clear roadmap from MVP to production-ready system. The cloned repositories provide all necessary reference implementations and libraries.

**Expected Timeline**: 12-18 months to production-ready compiler for Rust/C/C++.

**Expected Performance**: 10-100x faster incremental builds vs traditional compilers, with negligible overhead on clean builds.

**Next Steps**:
1. Study rust-analyzer and Salsa codebase
2. Implement Phase 1 MVP (basic incremental Rust compiler)
3. Benchmark against rustc incremental mode
4. Iterate based on performance data
5. Expand to C/C++ and advanced features

---

## References

### Academic Papers
1. "Adapton: Composable, Demand-Driven Incremental Computation" (Hammer et al., 2014)
2. "Build Systems à la Carte" (Mokhov et al., 2018)
3. "Modeling the Rust Borrow Checker" (Weiss et al., 2019)
4. "Code Property Graphs" (Yamaguchi et al., 2014)
5. "Equality Saturation: A New Approach to Optimization" (Tate et al., 2009)

### Key Projects
- rust-analyzer: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/rust-analyzer`
- Salsa: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/salsa`
- CozoDB: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/cozo`
- Joern: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/joern`

### Documentation
- CozoDB Manual: https://docs.cozodb.org/
- Salsa Book: https://salsa-rs.github.io/salsa/
- rustc Dev Guide: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/rustc-dev-guide`
- LLVM Documentation: https://llvm.org/docs/

---

**Document Version**: 1.0
**Last Updated**: 2025-11-24
**Author**: Research Team
**Project**: Transfiguration - Graph-Based Incremental Compiler
