# Repository Index: Graph-Based Incremental Compiler Architecture

This document catalogs all reference repositories cloned for building a graph-based compilation system using CozoDB as an incremental compilation cache with function-level dependency tracking.

**Total Repositories Cloned: 39**

---

## Category 1: Graph Database & Query Engines

### 1. CozoDB
- **URL**: https://github.com/cozodb/cozo
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/cozo`
- **Purpose**: Primary graph database using Datalog for incremental compilation cache
- **Key Features**:
  - Embedded graph database with Datalog query language
  - Transactional, supports ACID properties
  - Multi-language bindings (Rust, Python, Node.js, Java, Swift, Go, C)
  - Supports graph algorithms out of the box
  - Time-travel queries and versioning
- **License**: MPL-2.0
- **Activity**: Active (last commit Dec 2024)
- **Stars**: ~3k
- **Use Case**: Core graph database for storing CPG, dependency graphs, and compilation artifacts

### 2. Souffle Datalog
- **URL**: https://github.com/souffle-lang/souffle
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/souffle`
- **Purpose**: High-performance Datalog engine for static analysis
- **Key Features**:
  - Compiled Datalog (translates to C++)
  - Parallel execution
  - Used extensively in program analysis
  - Incremental evaluation support
- **License**: UPL-1.0
- **Activity**: Active
- **Use Case**: Reference for advanced Datalog patterns in static analysis

---

## Category 2: Parser & Frontend Tools

### 3. Tree-sitter
- **URL**: https://github.com/tree-sitter/tree-sitter
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/tree-sitter`
- **Purpose**: Incremental parsing library
- **Key Features**:
  - Incremental parsing (reparse only changed sections)
  - Error recovery
  - Language-agnostic parser generator
  - Concrete syntax tree with full fidelity
- **License**: MIT
- **Activity**: Very active
- **Stars**: ~18k
- **Use Case**: Frontend parser for multi-language support with incremental parsing

### 4. Tree-sitter-rust
- **URL**: https://github.com/tree-sitter/tree-sitter-rust
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/tree-sitter-rust`
- **Purpose**: Rust grammar for tree-sitter
- **Key Features**: Complete Rust language grammar
- **License**: MIT
- **Use Case**: Rust language parsing

### 5. Tree-sitter-c
- **URL**: https://github.com/tree-sitter/tree-sitter-c
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/tree-sitter-c`
- **Purpose**: C grammar for tree-sitter
- **Key Features**: Complete C language grammar
- **License**: MIT
- **Use Case**: C language parsing

### 6. Tree-sitter-cpp
- **URL**: https://github.com/tree-sitter/tree-sitter-cpp
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/tree-sitter-cpp`
- **Purpose**: C++ grammar for tree-sitter
- **Key Features**: Complete C++ language grammar
- **License**: MIT
- **Use Case**: C++ language parsing

### 7. syn
- **URL**: https://github.com/dtolnay/syn
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/syn`
- **Purpose**: Rust parser for proc macros
- **Key Features**:
  - Full Rust syntax parsing
  - Used by rust-analyzer and rustc
  - Excellent error messages
  - Zero-copy parsing
- **License**: MIT/Apache-2.0
- **Activity**: Very active
- **Stars**: ~6k
- **Use Case**: Reference Rust parser implementation

### 8. rust-analyzer
- **URL**: https://github.com/rust-lang/rust-analyzer
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/rust-analyzer`
- **Purpose**: Rust language server - reference incremental compiler architecture
- **Key Features**:
  - Incremental compilation using Salsa
  - IDE integration (LSP)
  - Type inference and checking
  - Macro expansion
  - Semantic analysis
- **License**: MIT/Apache-2.0
- **Activity**: Extremely active (official Rust project)
- **Stars**: ~14k
- **Use Case**: Primary reference for incremental compilation architecture

### 9. clangd
- **URL**: https://github.com/clangd/clangd
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/clangd`
- **Purpose**: C/C++ language server
- **Key Features**:
  - LLVM-based
  - Incremental compilation
  - LSP support
- **License**: Apache-2.0
- **Activity**: Active
- **Use Case**: Reference C/C++ frontend architecture

---

## Category 3: Incremental Computation Framework

### 10. Salsa
- **URL**: https://github.com/salsa-rs/salsa
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/salsa`
- **Purpose**: Generic framework for on-demand, incremental computation
- **Key Features**:
  - Memoization with automatic invalidation
  - Dependency tracking
  - Parallel execution
  - Used by rust-analyzer
  - Inspired by adapton and glimmer
- **License**: MIT/Apache-2.0
- **Activity**: Active
- **Stars**: ~2k
- **Use Case**: CORE framework for incremental compilation logic

---

## Category 4: Code Property Graph (CPG) & Static Analysis

### 11. Joern
- **URL**: https://github.com/joernio/joern
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/joern`
- **Purpose**: Code property graph analysis platform
- **Key Features**:
  - Multi-language CPG generation
  - Scala-based query DSL
  - Combines AST, CFG, DFG, PDG
  - Vulnerability discovery
  - Custom graph database
- **License**: Apache-2.0
- **Activity**: Very active
- **Stars**: ~2k
- **Use Case**: Reference CPG design and implementation

### 12. Code Property Graph (Reference)
- **URL**: https://github.com/ShiftLeftSecurity/codepropertygraph
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/codepropertygraph`
- **Purpose**: CPG specification and schema
- **Key Features**:
  - Defines CPG node types
  - Edge types for program dependencies
  - Schema definitions
- **License**: Apache-2.0
- **Use Case**: CPG schema reference

### 13. Static Analysis Tools Collection
- **URL**: https://github.com/analysis-tools-dev/static-analysis
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/static-analysis`
- **Purpose**: Curated list of static analysis tools
- **Key Features**: Comprehensive catalog of analysis tools
- **License**: CC0-1.0
- **Use Case**: Reference for analysis techniques and tools

---

## Category 5: LLVM Integration & Code Generation

### 14. LLVM Project
- **URL**: https://github.com/llvm/llvm-project
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/llvm-project`
- **Purpose**: Compiler infrastructure
- **Key Features**:
  - LLVM IR
  - Optimization passes
  - Multiple backends
  - clang, lld included
- **License**: Apache-2.0 with LLVM exceptions
- **Activity**: Extremely active
- **Stars**: ~28k
- **Use Case**: Backend code generation and optimization
- **Note**: Shallow clone to save space

### 15. inkwell
- **URL**: https://github.com/TheDan64/inkwell
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/inkwell`
- **Purpose**: Safe LLVM bindings for Rust
- **Key Features**:
  - Type-safe LLVM IR builder
  - Rust-idiomatic API
  - Supports LLVM 4-18
- **License**: Apache-2.0
- **Activity**: Active
- **Stars**: ~2.4k
- **Use Case**: Rust interface to LLVM for IR generation

### 16. Cranelift
- **URL**: https://github.com/bytecodealliance/cranelift
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/cranelift`
- **Purpose**: Fast code generator (alternative to LLVM)
- **Key Features**:
  - Fast compilation times
  - Used by Wasmtime
  - Simpler than LLVM
  - Good for JIT scenarios
- **License**: Apache-2.0
- **Activity**: Very active
- **Stars**: ~8k
- **Use Case**: Alternative fast code generation backend

---

## Category 6: Fast Linkers

### 17. mold
- **URL**: https://github.com/rui314/mold
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/mold`
- **Purpose**: Modern fast linker
- **Key Features**:
  - 10-50x faster than GNU ld
  - Parallel linking
  - Compatible with GNU ld
  - Supports LTO
- **License**: MIT
- **Activity**: Very active
- **Stars**: ~14k
- **Use Case**: Fast incremental linking

---

## Category 7: Graph Algorithms & Visualization

### 18. petgraph
- **URL**: https://github.com/petgraph/petgraph
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/petgraph`
- **Purpose**: Graph data structure library for Rust
- **Key Features**:
  - Generic graph types
  - Standard graph algorithms (DFS, BFS, dijkstra, etc.)
  - Serialization support
  - Stable node/edge indices
- **License**: MIT/Apache-2.0
- **Activity**: Active
- **Stars**: ~2.8k
- **Use Case**: In-memory graph manipulation and algorithms

### 19. Graphviz (GitLab)
- **URL**: https://gitlab.com/graphviz/graphviz
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/graphviz-gitlab`
- **Purpose**: Graph visualization toolkit
- **Key Features**:
  - DOT language
  - Multiple layout algorithms
  - Export to various formats
- **License**: EPL-1.0
- **Activity**: Active
- **Use Case**: Visualizing dependency graphs and CPG

### 20. egg (e-graphs)
- **URL**: https://github.com/egraphs-good/egg
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/egg`
- **Purpose**: E-graph library for program optimization
- **Key Features**:
  - Equality saturation
  - Term rewriting
  - Used in compiler optimizations
  - Fast congruence closure
- **License**: MIT
- **Activity**: Active
- **Stars**: ~1.2k
- **Use Case**: Advanced program optimization using e-graphs

---

## Category 8: Caching & Build Systems

### 21. sccache
- **URL**: https://github.com/mozilla/sccache
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/sccache`
- **Purpose**: Shared compilation cache (Rust)
- **Key Features**:
  - Distributed caching
  - Supports Rust, C/C++
  - Cloud storage backends (S3, GCS, Redis)
  - Incremental compilation cache
- **License**: Apache-2.0
- **Activity**: Active
- **Stars**: ~6k
- **Use Case**: Reference distributed caching architecture

### 22. ccache
- **URL**: https://github.com/ccache/ccache
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/ccache`
- **Purpose**: Compiler cache for C/C++
- **Key Features**:
  - Local caching
  - Fast recompilation
  - Widely used in Linux development
  - Hash-based cache keys
- **License**: GPL-3.0
- **Activity**: Active
- **Stars**: ~2k
- **Use Case**: Reference caching strategies

### 23. Buck2
- **URL**: https://github.com/facebook/buck2
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/buck2`
- **Purpose**: Modern build system (Meta)
- **Key Features**:
  - Written in Rust
  - Incremental builds
  - Remote execution
  - Fine-grained dependencies
  - Starlark configuration
- **License**: Apache-2.0/MIT
- **Activity**: Very active
- **Stars**: ~3.5k
- **Use Case**: Reference incremental build system architecture

### 24. Bazel
- **URL**: https://github.com/bazelbuild/bazel
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/bazel`
- **Purpose**: Google's build system
- **Key Features**:
  - Multi-language support
  - Incremental builds
  - Remote caching and execution
  - Reproducible builds
- **License**: Apache-2.0
- **Activity**: Extremely active
- **Stars**: ~23k
- **Use Case**: Reference build system architecture
- **Note**: Shallow clone

### 25. Cargo
- **URL**: https://github.com/rust-lang/cargo
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/cargo`
- **Purpose**: Rust package manager and build tool
- **Key Features**:
  - Incremental compilation
  - Dependency resolution
  - Build script support
  - Fingerprinting for cache invalidation
- **License**: MIT/Apache-2.0
- **Activity**: Extremely active (official Rust)
- **Stars**: ~12k
- **Use Case**: Reference incremental build and caching system

---

## Category 9: Hashing & Fingerprinting

### 26. BLAKE3
- **URL**: https://github.com/blake3-team/BLAKE3
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/BLAKE3`
- **Purpose**: Cryptographic hash function
- **Key Features**:
  - Extremely fast (faster than SHA-3, MD5)
  - Parallel hashing
  - Tree-based structure
  - Ideal for content-addressing
- **License**: CC0-1.0/Apache-2.0
- **Activity**: Active
- **Stars**: ~5k
- **Use Case**: Fast fingerprinting for cache keys

### 27. rustc-hash
- **URL**: https://github.com/rust-lang/rustc-hash
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/rustc-hash`
- **Purpose**: Fast non-cryptographic hash
- **Key Features**:
  - Used in rustc
  - FxHash algorithm
  - Optimized for hash tables
- **License**: MIT/Apache-2.0
- **Activity**: Stable
- **Use Case**: Fast hashing for internal data structures

---

## Category 10: Parallel Computing

### 28. Rayon
- **URL**: https://github.com/rayon-rs/rayon
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/rayon`
- **Purpose**: Data parallelism library
- **Key Features**:
  - Work-stealing scheduler
  - Easy parallelization of iterators
  - Zero overhead
  - Composable
- **License**: MIT/Apache-2.0
- **Activity**: Active
- **Stars**: ~10k
- **Use Case**: Parallel compilation and analysis

---

## Category 11: Reference Compilers & Interpreters

### 29. SWC
- **URL**: https://github.com/swc-project/swc
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/swc`
- **Purpose**: Rust-based JavaScript/TypeScript compiler
- **Key Features**:
  - 20x faster than Babel
  - Incremental compilation
  - Written in Rust
  - Used in Next.js
- **License**: Apache-2.0
- **Activity**: Very active
- **Stars**: ~31k
- **Use Case**: Reference fast compiler architecture

### 30. RustPython
- **URL**: https://github.com/RustPython/RustPython
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/RustPython`
- **Purpose**: Python interpreter in Rust
- **Key Features**:
  - Complete Python 3 implementation
  - Can be embedded
  - WASM support
- **License**: MIT
- **Activity**: Active
- **Stars**: ~19k
- **Use Case**: Reference interpreter architecture

### 31. HHVM
- **URL**: https://github.com/facebook/hhvm
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/hhvm`
- **Purpose**: HipHop VM for PHP
- **Key Features**:
  - JIT compilation
  - Incremental compilation
  - Profile-guided optimization
  - Repository-wide analysis
- **License**: PHP/Zend
- **Activity**: Active
- **Stars**: ~18k
- **Use Case**: Reference JIT and incremental compilation
- **Note**: Shallow clone

### 32. Miri
- **URL**: https://github.com/rust-lang/miri
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/miri`
- **Purpose**: Rust interpreter for undefined behavior detection
- **Key Features**:
  - MIR interpreter
  - Memory safety checks
  - Tracks provenance
- **License**: MIT/Apache-2.0
- **Activity**: Very active (official Rust)
- **Use Case**: Reference for MIR-level analysis

### 33. Chalk
- **URL**: https://github.com/rust-lang/chalk
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/chalk`
- **Purpose**: Trait system implementation
- **Key Features**:
  - Logic programming for traits
  - Used in rust-analyzer
  - Based on Prolog-style resolution
- **License**: MIT/Apache-2.0
- **Activity**: Active
- **Use Case**: Reference for trait/type system

### 34. Gluon
- **URL**: https://github.com/gluon-lang/gluon
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/gluon`
- **Purpose**: Functional programming language
- **Key Features**:
  - Incremental compilation
  - Embeddable
  - Type inference
- **License**: MIT
- **Activity**: Moderately active
- **Use Case**: Reference incremental functional language compiler

---

## Category 12: Code Transformation & Analysis

### 35. c2rust
- **URL**: https://github.com/immunant/c2rust
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/c2rust`
- **Purpose**: C to Rust transpiler
- **Key Features**:
  - Automated C → Rust translation
  - Refactoring tools
  - Cross-checking
- **License**: BSD-3-Clause
- **Activity**: Active
- **Stars**: ~3.8k
- **Use Case**: Reference for AST transformation

### 36. difftastic
- **URL**: https://github.com/Wilfred/difftastic
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/difftastic`
- **Purpose**: Structural diff tool
- **Key Features**:
  - AST-based diffing
  - Uses tree-sitter
  - Language-aware
- **License**: MIT
- **Activity**: Active
- **Stars**: ~20k
- **Use Case**: Reference for AST-level change detection

---

## Category 13: Documentation & Learning Resources

### 37. rustc-dev-guide
- **URL**: https://github.com/rust-lang/rustc-dev-guide
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/rustc-dev-guide`
- **Purpose**: rustc compiler development guide
- **Key Features**:
  - Comprehensive compiler documentation
  - Architecture explanations
  - Incremental compilation details
- **License**: MIT/Apache-2.0
- **Activity**: Active
- **Use Case**: Learning resource for compiler architecture

---

## Category 14: Package Management & Release Tools

### 38. cargo-release
- **URL**: https://github.com/crate-ci/cargo-release
- **Path**: `/Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos/cargo-release`
- **Purpose**: Cargo subcommand for releases
- **Key Features**:
  - Automated versioning
  - Publishing workflow
  - Dependency updates
- **License**: MIT/Apache-2.0
- **Activity**: Active
- **Use Case**: Reference release automation

---

## Repository Dependency Matrix

```
Core Dependencies:
├── CozoDB (graph database)
│   └── Used by: All components for storing CPG and dependencies
├── Salsa (incremental computation)
│   ├── Used by: rust-analyzer, compilation engine
│   └── Depends on: rustc-hash
├── Tree-sitter (parsing)
│   ├── Used by: Frontend parsers, difftastic
│   └── Grammars: tree-sitter-{rust,c,cpp}
└── LLVM (code generation)
    ├── Used by: Backend, inkwell
    └── Includes: lld (linker)

Analysis Layer:
├── Joern (CPG framework)
│   └── Depends on: codepropertygraph schema
├── petgraph (graph algorithms)
│   └── Used by: Analysis passes
└── egg (e-graphs)
    └── Used by: Optimizer

Build & Cache:
├── Buck2/Bazel (build system reference)
├── sccache/ccache (caching reference)
└── Cargo (Rust build reference)
    └── Depends on: incremental compilation strategies

Supporting:
├── Rayon (parallelization)
├── BLAKE3 (hashing)
└── Graphviz (visualization)
```

---

## License Summary

- **MIT/Apache-2.0 Dual**: rust-analyzer, salsa, syn, inkwell, cranelift, petgraph, cargo, rayon, chalk, miri, difftastic, rustc-hash, rustc-dev-guide, gluon, cargo-release
- **MIT**: tree-sitter, mold, egg, RustPython
- **Apache-2.0**: joern, codepropertygraph, sccache, buck2, bazel, swc
- **MPL-2.0**: CozoDB
- **UPL-1.0**: souffle
- **GPL-3.0**: ccache
- **CC0-1.0**: BLAKE3, static-analysis
- **EPL-1.0**: graphviz
- **BSD-3-Clause**: c2rust
- **PHP/Zend**: hhvm

Most repositories use permissive open-source licenses compatible with commercial use.

---

## Repository Activity Assessment

**Extremely Active (Official/Critical Projects)**:
- rust-analyzer, cargo, miri, chalk (Rust official)
- llvm-project, bazel, swc

**Very Active (Maintained & Growing)**:
- CozoDB, joern, tree-sitter, inkwell, cranelift, mold, buck2, sccache, hhvm, salsa, rayon, difftastic

**Active (Regular Updates)**:
- syn, petgraph, egg, souffle, clangd, ccache, RustPython, c2rust, gluon, rustc-dev-guide, static-analysis, cargo-release

**Stable (Mature, Fewer Updates)**:
- rustc-hash, BLAKE3, graphviz, codepropertygraph

---

## Notes

1. **Empty/Failed Clones**:
   - graphviz (GitHub mirror is empty, use graphviz-gitlab instead)

2. **Shallow Clones** (to save space):
   - llvm-project (very large repo)
   - bazel (very large repo)
   - hhvm (very large repo)
   - graphviz-gitlab

3. **Organization Changes**:
   - c2rust moved from c2rust/c2rust to immunant/c2rust
   - souffle moved from google/souffle to souffle-lang/souffle

---

## Next Steps for Integration

1. Study **rust-analyzer** and **salsa** architecture for incremental compilation patterns
2. Examine **Joern** CPG schema and adapt to CozoDB
3. Integrate **tree-sitter** for frontend parsing
4. Use **petgraph** for in-memory graph operations before persisting to CozoDB
5. Implement **BLAKE3** for fast fingerprinting
6. Reference **sccache/ccache** for distributed caching strategies
7. Use **inkwell** for LLVM IR generation
8. Apply **rayon** for parallel compilation passes
9. Use **mold** for fast linking
10. Implement **egg**-based optimizations in later phases
