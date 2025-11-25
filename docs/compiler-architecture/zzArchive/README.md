# Archive: Detailed Research Documents

**Status**: Reference Material
**Total**: 350+ pages of rigorous analysis
**Date**: 2025-11-19 to 2025-11-24

---

## Purpose of Archive

These documents contain the **detailed research and analysis** that informed the strategic recommendations in `../MASTER-REFERENCE.md`.

**Use these documents when you need:**
- Deep technical specifications
- Mathematical proofs and derivations
- Empirical benchmark data
- Alternative approaches that were considered
- Step-by-step implementation examples

**For most readers**: The MASTER-REFERENCE.md provides sufficient detail.

---

## Document Inventory

### Strategic Analysis

**00-ARCHITECTURE-COMPARISON-TABLE.md** (64 KB)
- **Purpose**: Shreyas Doshi-style comparative analysis
- **Content**: 5 architectural approaches with detailed tradeoffs
- **Key Sections**:
  - Base-100 rating system across 12 dimensions
  - Weighted composite scores
  - Impact vs Effort matrix
  - Decision matrix for architecture selection
- **Read when**: Evaluating architectural alternatives

---

### System Design

**01-HLD-GRAPH-COMPILER.md** (31 KB)
- **Purpose**: High-level architectural overview
- **Content**: Complete system design from 30,000-foot view
- **Key Sections**:
  - Traditional vs graph-based comparison
  - Phase-by-phase breakdown (lexing → LLVM IR)
  - Memory optimization strategies
  - Incremental compilation design
- **Read when**: Understanding overall architecture

**02-LLD-IMPLEMENTATION.md** (50 KB)
- **Purpose**: Low-level implementation specifications
- **Content**: Concrete CozoDB schemas and Datalog queries
- **Key Sections**:
  - Complete schemas for all IR phases
  - Datalog transformation queries (executable)
  - Transaction boundaries and ACID properties
  - Parallel compilation patterns
  - Error handling and recovery
- **Read when**: Implementing the compiler

---

### Examples & Validation

**04-RUBBER-DUCK-SIMULATIONS.md** (31 KB)
- **Purpose**: Concrete walkthrough for understanding
- **Content**: Step-by-step compilation of `fn add(a: i32, b: i32) -> i32`
- **Key Sections**:
  - Exact graph states after each phase
  - Exact CozoDB queries executed
  - Memory usage tracking (byte-by-byte)
  - Incremental recompilation scenario
- **Read when**: Need concrete understanding of how it works

**06-PERFORMANCE-ANALYSIS.md** (58 KB)
- **Purpose**: Rigorous performance validation
- **Content**: Mathematical proofs + empirical benchmarks
- **Key Sections**:
  - Theoretical foundation for RAM reduction
  - CozoDB published benchmarks
  - Small/medium/large codebase analysis
  - Scaling laws (asymptotic complexity)
  - Memory-mapped I/O performance
- **Read when**: Validating performance claims

---

### Technical Deep Dives

**05-PATH-TO-LLVM.md** (49 KB)
- **Purpose**: Code generation strategy (graph → machine code)
- **Content**: MIR to LLVM IR transformation details
- **Key Sections**:
  - LLVM IR graph schema
  - Datalog transformation queries
  - Incremental code generation
  - Monomorphization deduplication
  - Export strategies (inkwell)
- **Read when**: Implementing code generation

**07-GRANULARITY-AND-MULTILANG.md** (49 KB)
- **Purpose**: AST granularity levels & multi-language support
- **Content**: Analysis of 5 granularity levels, 12 languages
- **Key Sections**:
  - Token-level to Module-level granularity
  - Memory/performance tradeoffs for each level
  - Multi-language support (which use LLVM, which don't)
  - Parseltongue upgrade path
- **Read when**: Deciding storage granularity or adding new languages

---

### Strategic Positioning

**08-C-CPP-RUST-STRATEGIC-ADVANTAGES.md** (58 KB)
- **Purpose**: Competitive advantages for C/C++/Rust
- **Content**: Shreyas Doshi-style advantage analysis
- **Key Sections**:
  - Strategic advantage table (11 dimensions)
  - Compilation time improvements (concrete examples)
  - Error detection improvements (with Datalog queries)
  - Comprehensive language feature support
  - Practical pipeline (commands and workflows)
- **Read when**: Pitching to stakeholders or competitive analysis

---

## Reading Strategies

### Strategy 1: Top-Down (Recommended for Most)
1. Read `../MASTER-REFERENCE.md` (complete understanding)
2. Deep-dive into specific archives as needed:
   - Need implementation details? → 02-LLD-IMPLEMENTATION.md
   - Need concrete example? → 04-RUBBER-DUCK-SIMULATIONS.md
   - Need performance proofs? → 06-PERFORMANCE-ANALYSIS.md

### Strategy 2: Bottom-Up (For Skeptics)
1. Start with concrete example: 04-RUBBER-DUCK-SIMULATIONS.md
2. Validate performance: 06-PERFORMANCE-ANALYSIS.md
3. Understand architecture: 01-HLD-GRAPH-COMPILER.md
4. Compare alternatives: 00-ARCHITECTURE-COMPARISON-TABLE.md
5. Synthesize: ../MASTER-REFERENCE.md

### Strategy 3: Implementation-Focused (For Builders)
1. High-level design: 01-HLD-GRAPH-COMPILER.md
2. Low-level specs: 02-LLD-IMPLEMENTATION.md
3. Concrete example: 04-RUBBER-DUCK-SIMULATIONS.md
4. Code generation: 05-PATH-TO-LLVM.md
5. Granularity decisions: 07-GRANULARITY-AND-MULTILANG.md

### Strategy 4: Complete (For Researchers)
Read all documents in order (00 → 08)
**Time**: 6-8 hours for full comprehension

---

## Key Cross-References

### Architecture Selection
- **Question**: Which architecture to build?
- **Answer**: 00-ARCHITECTURE-COMPARISON-TABLE.md (Architecture #4: Hybrid)
- **Validation**: 06-PERFORMANCE-ANALYSIS.md (proves 40-50% speedup)

### Granularity Decision
- **Question**: What level of AST detail to store?
- **Answer**: 07-GRANULARITY-AND-MULTILANG.md (Level 4: Function + ISG)
- **Validation**: 06-PERFORMANCE-ANALYSIS.md (20-50 MB per 100K LOC)

### Implementation Roadmap
- **Question**: How to build it?
- **Answer**: ../MASTER-REFERENCE.md Part IV (Phase-by-Phase)
- **Details**: 02-LLD-IMPLEMENTATION.md (schemas), 05-PATH-TO-LLVM.md (codegen)

### Performance Claims
- **Question**: Do the speedup numbers hold?
- **Answer**: 06-PERFORMANCE-ANALYSIS.md (mathematical proofs + benchmarks)
- **Example**: 04-RUBBER-DUCK-SIMULATIONS.md (30-60× for simple case)

---

## Document Lineage

These documents were created through rigorous research over November 2024:

**Phase 1: Architecture Exploration** (Nov 18-19)
- 00-ARCHITECTURE-COMPARISON-TABLE.md
- 01-HLD-GRAPH-COMPILER.md
- 02-LLD-IMPLEMENTATION.md

**Phase 2: Validation** (Nov 19)
- 04-RUBBER-DUCK-SIMULATIONS.md
- 05-PATH-TO-LLVM.md
- 06-PERFORMANCE-ANALYSIS.md

**Phase 3: Specialization** (Nov 20)
- 07-GRANULARITY-AND-MULTILANG.md
- 08-C-CPP-RUST-STRATEGIC-ADVANTAGES.md

**Phase 4: Synthesis** (Nov 24)
- ../MASTER-REFERENCE.md (Minto Pyramid synthesis)
- Archive organization

---

## Maintenance

**These documents are FROZEN as of 2025-11-24.**

Any updates or refinements should be made to `../MASTER-REFERENCE.md`, with archive documents preserved for historical reference.

**Exception**: If significant new research is conducted, create a new dated document (e.g., `09-NEW-RESEARCH-2025-12.md`) rather than modifying archived documents.

---

## Citation

When referencing this research:

**General citation**:
```
Graph-Based Compiler Architecture Research (2025)
docs/compiler-architecture/MASTER-REFERENCE.md
that-in-rust/parseltongue project
```

**Specific citation** (for detailed analysis):
```
[Topic] Analysis (2025)
docs/compiler-architecture/zzArchive/[XX-DOCUMENT-NAME].md
that-in-rust/parseltongue project
```

---

**This archive represents 350+ pages of rigorous, validated research.**

**The synthesis (MASTER-REFERENCE.md) is the actionable output.**

**Use the archive when you need to go deeper.**

🔬
