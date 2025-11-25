# Quick Reference: Cloned Repositories

## Summary

**Total Repositories Cloned**: 39

**Categories**:
- Graph Database & Query: 2
- Parser & Frontend: 8
- Incremental Computation: 1
- CPG & Static Analysis: 3
- LLVM & Code Generation: 3
- Linkers: 1
- Graph Algorithms: 3
- Build Systems & Caching: 6
- Hashing: 2
- Parallel Computing: 1
- Reference Compilers: 6
- Code Transformation: 2
- Documentation: 1

---

## Clone Commands (for reference)

```bash
cd /Users/neetipatni/Projects20251124/transfiguration-cozo-graph-compiler/refRepos

# Graph Database
git clone https://github.com/cozodb/cozo.git
git clone https://github.com/souffle-lang/souffle.git

# Parsers
git clone https://github.com/tree-sitter/tree-sitter.git
git clone https://github.com/tree-sitter/tree-sitter-rust.git
git clone https://github.com/tree-sitter/tree-sitter-c.git
git clone https://github.com/tree-sitter/tree-sitter-cpp.git
git clone https://github.com/dtolnay/syn.git
git clone https://github.com/rust-lang/rust-analyzer.git
git clone https://github.com/clangd/clangd.git

# Incremental Computation
git clone https://github.com/salsa-rs/salsa.git

# CPG & Analysis
git clone https://github.com/joernio/joern.git
git clone https://github.com/ShiftLeftSecurity/codepropertygraph.git
git clone https://github.com/analysis-tools-dev/static-analysis.git

# Code Generation
git clone https://github.com/llvm/llvm-project.git --depth 1
git clone https://github.com/TheDan64/inkwell.git
git clone https://github.com/bytecodealliance/cranelift.git

# Linkers
git clone https://github.com/rui314/mold.git

# Graph Libraries
git clone https://github.com/petgraph/petgraph.git
git clone https://gitlab.com/graphviz/graphviz.git graphviz-gitlab --depth 1
git clone https://github.com/egraphs-good/egg.git

# Build & Cache
git clone https://github.com/mozilla/sccache.git
git clone https://github.com/ccache/ccache.git
git clone https://github.com/facebook/buck2.git
git clone https://github.com/bazelbuild/bazel.git --depth 1
git clone https://github.com/rust-lang/cargo.git
git clone https://github.com/crate-ci/cargo-release.git

# Hashing
git clone https://github.com/blake3-team/BLAKE3.git
git clone https://github.com/rust-lang/rustc-hash.git

# Parallel Computing
git clone https://github.com/rayon-rs/rayon.git

# Reference Compilers
git clone https://github.com/swc-project/swc.git
git clone https://github.com/RustPython/RustPython.git
git clone https://github.com/facebook/hhvm.git --depth 1
git clone https://github.com/rust-lang/miri.git
git clone https://github.com/rust-lang/chalk.git
git clone https://github.com/gluon-lang/gluon.git

# Code Transformation
git clone https://github.com/immunant/c2rust.git
git clone https://github.com/Wilfred/difftastic.git

# Documentation
git clone https://github.com/rust-lang/rustc-dev-guide.git
```

---

## Priority Reading Order

### 1. Start Here (Essential)
1. **rust-analyzer** - Study incremental compilation architecture
   - Path: `rust-analyzer/docs/dev/architecture.md`
   - Focus: Salsa integration, query system

2. **Salsa** - Understand incremental computation framework
   - Path: `salsa/book/`
   - Focus: Memoization, dependency tracking

3. **CozoDB** - Learn Datalog query language
   - Path: `cozo/README.md`, online docs
   - Focus: Graph queries, schema design

4. **Joern** - Study CPG design
   - Path: `joern/docs/`
   - Focus: CPG schema, graph traversals

### 2. Implementation Details (Important)
5. **tree-sitter** - Parser integration
   - Path: `tree-sitter/docs/`
   - Focus: Incremental parsing API

6. **inkwell** - LLVM bindings
   - Path: `inkwell/examples/`
   - Focus: IR generation

7. **petgraph** - Graph algorithms
   - Path: `petgraph/src/`
   - Focus: Graph data structures

8. **mold** - Fast linking
   - Path: `mold/README.md`
   - Focus: Incremental linking

### 3. Advanced Topics (Later)
9. **egg** - E-graph optimizations
10. **cranelift** - Alternative codegen
11. **Buck2/Bazel** - Build system architecture
12. **sccache** - Distributed caching

---

## Critical Files to Read

```
refRepos/
├── rust-analyzer/
│   ├── docs/dev/architecture.md          ⭐ MUST READ
│   ├── crates/salsa/
│   └── crates/hir/
│
├── salsa/
│   ├── book/src/                         ⭐ MUST READ
│   └── examples/
│
├── cozo/
│   ├── README.md                         ⭐ MUST READ
│   └── cozo-core/src/
│
├── joern/
│   ├── docs/cpg.md                       ⭐ MUST READ
│   └── schema/
│
├── codepropertygraph/
│   └── schema/                           ⭐ IMPORTANT
│
├── tree-sitter/
│   ├── lib/src/parser.c
│   └── docs/section-3-using-parsers.md
│
├── inkwell/
│   └── examples/                         ⭐ IMPORTANT
│
├── rustc-dev-guide/
│   ├── src/incremental-compilation.md    ⭐ MUST READ
│   └── src/queries/
│
└── mold/
    └── docs/design.md
```

---

## Dependency Graph

```
Core Stack (Start with these):
    CozoDB
    ├── Used for: Graph storage, Datalog queries
    └── Dependencies: None (embedded DB)

    Salsa
    ├── Used for: Incremental computation
    └── Dependencies: rustc-hash

    tree-sitter
    ├── Used for: Parsing
    └── Dependencies: None

Supporting Libraries (Add next):
    petgraph (graph algorithms)
    inkwell (LLVM bindings)
    rayon (parallelism)
    BLAKE3 (hashing)

Reference Implementations (Study for patterns):
    rust-analyzer (incremental compiler architecture)
    Joern (CPG design)
    sccache (caching strategy)
    cargo (build system)

Future Extensions (Later phases):
    egg (e-graph optimizations)
    cranelift (fast codegen)
    Buck2/Bazel (distributed builds)
    mold (fast linking)
```

---

## Key Cargo Dependencies

```toml
[dependencies]
# Core
cozo = { version = "0.7", features = ["graph-algo"] }
salsa = "0.17"
tree-sitter = "0.20"

# Parsing
syn = { version = "2.0", features = ["full", "extra-traits"] }
tree-sitter-rust = "0.21"

# Graphs
petgraph = "0.6"

# LLVM
inkwell = { version = "0.4", features = ["llvm18-0"] }

# Hashing
blake3 = "1.5"
rustc-hash = "1.1"

# Parallelism
rayon = "1.8"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Utilities
uuid = { version = "1.6", features = ["v4", "serde"] }
dashmap = "5.5"
parking_lot = "0.12"

# CLI
clap = { version = "4.4", features = ["derive"] }

# Optional (advanced features)
egg = "0.9"  # For e-graph optimizations
```

---

## Common Patterns Found Across Repos

### 1. Incremental Computation (rust-analyzer, salsa)
```rust
#[salsa::tracked]
fn compute_something(db: &dyn Db, input: Input) -> Output {
    // Automatically memoized and invalidated
}
```

### 2. Graph Queries (CozoDB, Joern)
```datalog
// Find transitive dependencies
transitive[x, y] := direct[x, y]
transitive[x, z] := transitive[x, y], direct[y, z]
```

### 3. Content Hashing (cargo, sccache)
```rust
let hash = blake3::hash(content.as_bytes());
let key = format!("cache/{hash}");
```

### 4. Parallel Processing (rayon)
```rust
items.par_iter()
    .map(|item| process(item))
    .collect()
```

### 5. Tree Traversal (tree-sitter)
```rust
let mut cursor = node.walk();
for child in node.children(&mut cursor) {
    visit_node(child);
}
```

---

## Quick Start Guide

### Phase 1: Learn (Week 1-2)
1. Read rust-analyzer architecture docs
2. Go through Salsa book
3. Try CozoDB tutorial
4. Study Joern CPG schema

### Phase 2: Prototype (Week 3-4)
1. Parse a Rust file with tree-sitter
2. Build simple CPG in CozoDB
3. Implement basic Salsa queries
4. Generate LLVM IR with inkwell

### Phase 3: Integrate (Week 5-6)
1. Connect all components
2. Implement function fingerprinting
3. Add caching logic
4. Test incremental compilation

### Phase 4: Optimize (Week 7-8)
1. Add parallel compilation
2. Improve Datalog queries
3. Optimize CPG generation
4. Benchmark vs rustc

---

## Useful Commands

### Explore CozoDB Schema
```rust
// In cozo/
cargo run --example tutorial
```

### Test Salsa Incremental Behavior
```rust
// In salsa/
cargo test --test incremental
```

### Parse with tree-sitter
```rust
// In tree-sitter/
cargo run --example parse_rust
```

### Generate LLVM IR
```rust
// In inkwell/
cargo run --example kaleidoscope
```

### Analyze with Joern
```bash
# In joern/
./joern-parse <source-file>
./joern-query
```

---

## Benchmarking Setup

### Baseline Measurements
```bash
# Measure rustc clean build
time cargo build --release

# Measure rustc incremental build
touch src/main.rs
time cargo build --release

# Measure cache hit (ccache)
ccache -z  # Zero stats
make clean && make
make clean && make  # Should be cached
ccache -s  # Show stats
```

### Target Metrics
- Clean build: < 2x rustc time
- Incremental (1% files changed): < 5% of clean build
- Cache hit: < 10ms
- Memory usage: < 4GB for 100k LOC

---

## Notes on Repository Status

### Active & Well-Maintained
✅ rust-analyzer, cargo, salsa, CozoDB, tree-sitter, mold, inkwell, petgraph, rayon

### Mature & Stable
✅ syn, BLAKE3, rustc-hash, llvm-project

### Experimental but Promising
⚠️ egg, cranelift, gluon

### For Reference Only
📚 rustc-dev-guide, static-analysis, codepropertygraph

### Large Repos (Shallow Cloned)
📦 llvm-project, bazel, hhvm

---

## Contact & Resources

- CozoDB Docs: https://docs.cozodb.org/
- Salsa Zulip: https://salsa.zulipchat.com/
- rust-analyzer Matrix: https://rust-lang.zulipchat.com/#narrow/stream/185405-t-compiler.2Frust-analyzer
- LLVM Discourse: https://discourse.llvm.org/

---

## License Compatibility Matrix

Most repos use MIT/Apache-2.0 (compatible with commercial use):
- ✅ Permissive: CozoDB (MPL-2.0), rust-analyzer, salsa, tree-sitter, inkwell, mold, petgraph
- ⚠️ Copyleft: ccache (GPL-3.0) - reference only, don't link
- ✅ Public Domain: BLAKE3 (CC0)

---

**Last Updated**: 2025-11-24
