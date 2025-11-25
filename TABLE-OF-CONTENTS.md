# Master Table of Contents

**Purpose**: Complete navigation guide for all documentation in the transfiguration-cozo-graph-compiler project
**Last Updated**: 2025-11-25
**Total Documentation**: 26 files, ~32,000 lines, ~700 KB

---

## Quick Stats

| Category | Files | Lines | Size |
|----------|-------|-------|------|
| **Configuration** | 9 | 8,282 | ~275 KB |
| **Main Documentation** | 7 | 6,885 | ~201 KB |
| **Archives** | 3 | 4,844 | ~152 KB |
| **Root Documents** | 7 | 12,779 | ~72 KB |
| **TOTAL** | 26 | 32,790 | ~700 KB |

---

## Navigation Guide

### For First-Time Readers
**Start here** → [docs/compiler-architecture/README.md](#docs-compiler-architecture-readmemd) (10 min read)

Then read:
1. [MASTER-REFERENCE-v003.md](#docs-compiler-architecture-master-reference-v003md) - Complete specification (90-120 min)
2. [QUICK-REFERENCE.md](#docs-compiler-architecture-quick-referencemd) - Cheat sheet (5-10 min)

### For Implementers
**Start here** → [MASTER-REFERENCE-v003.md](#docs-compiler-architecture-master-reference-v003md) (Full implementation spec)

Then study:
1. [S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md](#docs-compiler-architecture-s07-loc-estimation-and-testing-strategymd) - Implementation roadmap
2. [.claude/S06-design101-tdd-architecture-principles.md](#claude-s06-design101-tdd-architecture-principlesmd) - TDD workflow
3. [.claude/S03-four-word-naming-thesis.md](#claude-s03-four-word-naming-thesismd) - Naming standards

### For Researchers
**Deep dive** → All documents in [docs/compiler-architecture/zzArchive/](#docs-compiler-architecture-zzarchive-350-pages) (6-8 hours)

---

## Configuration Files (.claude/)

Configuration files that define development standards, naming conventions, and quality enforcement.

### .claude.md
**Path**: `.claude.md`
**Size**: 207 lines | 8 KB
**Purpose**: Main development rules configuration for Claude Code

**Key Sections**:
- Four-word naming convention (research-backed: 96% LLM recall)
- TDD-First development cycle
- Quality enforcement checklist
- Mermaid-only diagram standards

**When to use**: Before writing any code - establishes all coding standards

---

### .claude/S01-README-MOSTIMP.md
**Path**: `.claude/S01-README-MOSTIMP.md`
**Size**: 90 lines | 3.8 KB
**Purpose**: Most important configuration principles

**Key Sections**:
- Codebase hygiene (cargo clean)
- TDD-First architecture principles
- Four-word naming convention (CRITICAL)
- Product thinking mindset
- Executable specifications concept

**When to use**: Daily - quick reminder of core principles

---

### .claude/S02-visual-summary-terminal-guide.md
**Path**: `.claude/S02-visual-summary-terminal-guide.md`
**Size**: 1,744 lines | 59 KB
**Purpose**: Terminal rendering and visual design guide

**Key Sections**:
- Terminal UI principles
- Mermaid diagram guidelines
- Visual formatting standards
- ASCII art alternatives

**When to use**: When creating terminal output or documentation visuals

---

### .claude/S03-four-word-naming-thesis.md
**Path**: `.claude/S03-four-word-naming-thesis.md`
**Size**: 959 lines | 29 KB
**Purpose**: Complete research thesis on four-word naming convention

**Key Sections**:
- Empirical validation (96% LLM recall vs 23% single-word)
- Cognitive science backing (Miller's Law, fNIRS brain imaging)
- BPE tokenization analysis (44% semantic density optimal)
- 13 peer-reviewed citations (ACM, IEEE, arXiv 2024-2025)
- Cost savings analysis ($600/year per developer)

**Key metrics**:
- 4.2× better LLM recall
- 2.25 tokens/word (most efficient)
- 0.34 fNIRS units (lowest cognitive load)

**When to use**: When justifying naming standards or training team

---

### .claude/S05-tone-style-guide.md
**Path**: `.claude/S05-tone-style-guide.md`
**Size**: 143 lines | 4.2 KB
**Purpose**: Communication style and tone guidelines

**Key Sections**:
- Professional objectivity principles
- Technical accuracy over validation
- Concise communication style
- Avoid superlatives and emotional language

**When to use**: When writing documentation or commit messages

---

### .claude/S06-design101-tdd-architecture-principles.md
**Path**: `.claude/S06-design101-tdd-architecture-principles.md`
**Size**: 982 lines | 30 KB
**Purpose**: Complete TDD and architecture design principles

**Key Sections**:
- Executable specifications (core concept)
- 8 architectural principles (L1→L2→L3, DI, RAII, etc.)
- TDD cycle: STUB → RED → GREEN → REFACTOR
- Performance validation requirements
- Error handling patterns (thiserror, anyhow)

**Key principles**:
1. Executable Specifications Over Narratives
2. Layered Rust Architecture (L1→L2→L3)
3. Dependency Injection for Testability
4. RAII Resource Management
5. Performance Claims Must Be Test-Validated
6. Structured Error Handling
7. Complex Domain Model Support
8. Concurrency Model Validation
9. MVP-First Rigor

**When to use**: Before designing any new component

---

### .claude/S44ReleaseCheckList.md
**Path**: `.claude/S44ReleaseCheckList.md`
**Size**: 903 lines | 23 KB
**Purpose**: Pre-release quality checklist

**Key Sections**:
- Version validation (ONE FEATURE PER INCREMENT)
- Test coverage requirements
- Documentation completeness
- Performance benchmarks
- Security audit checklist

**When to use**: Before every version release

---

### .claude/S77-IdiomaticRustPatterns.md
**Path**: `.claude/S77-IdiomaticRustPatterns.md`
**Size**: 2,326 lines | 100 KB
**Purpose**: Comprehensive idiomatic Rust patterns catalog

**Key Sections**:
- Iterator patterns (map, filter, fold)
- Error handling (Result<T,E>, ? operator)
- Type system patterns (newtype, builder)
- Ownership patterns (Cow, Arc, Mutex)
- Async patterns (tokio, futures)

**When to use**: When writing Rust code - reference for best practices

---

### .claude/FOUR-WORD-THESIS-INTEGRATION-SUMMARY.md
**Path**: `.claude/FOUR-WORD-THESIS-INTEGRATION-SUMMARY.md`
**Size**: 428 lines | 13 KB
**Purpose**: Integration summary for four-word naming into main documentation

**Key Sections**:
- Integration status (v1.1 → MASTER-REFERENCE-v001.md)
- Files modified summary
- Empirical validation recap
- Action items for implementers

**When to use**: Understanding how naming convention was integrated

---

## Main Documentation (docs/compiler-architecture/)

Core compiler architecture documentation, specifications, and implementation guides.

### docs/compiler-architecture/README.md
**Path**: `docs/compiler-architecture/README.md`
**Size**: 296 lines | 10 KB
**Purpose**: Entry point for all architecture documentation

**Key Sections**:
- Document structure overview
- What's in MASTER-REFERENCE-v003.md (Part I-VI breakdown)
- What's in zzArchive/ (8 detailed documents)
- How to use documentation (by role: Executive, Engineer, Researcher)
- Key insights (TL;DR)
- Success criteria (Year 1-3)
- Document history

**Latest version**: v003 (2025-11-25)
- Added Part VI: Complete Implementation Specification (HLD/LLD)
- 85+ CozoDB Query Catalog
- Rust Interface Definitions

**When to use**: First stop for understanding documentation structure

---

### docs/compiler-architecture/MASTER-REFERENCE-v003.md
**Path**: `docs/compiler-architecture/MASTER-REFERENCE-v003.md`
**Size**: 2,628 lines | 83 KB | ~260 pages
**Purpose**: **PRIMARY REFERENCE** - Complete implementation specification for graph-based compiler

**Version**: 3.0 (v003)
**Reading time**: 90-120 minutes
**Latest update**: 2025-11-25

**Structure**:

#### Part I: Strategic Context (30 pages)
- Situation/Complication/Question/Answer framework
- Strategic recommendation: Build Hybrid Graph-Native compiler
- Key claims: 100-250× faster incremental, 95% memory reduction
- ROI analysis: $122K/year/developer savings

#### Part II: Technical Foundation (40 pages)
- 5 architecture options compared (rated 100-165)
- Hybrid Graph-Native (#4) recommendation (165 rating)
- Function-level incremental compilation design
- CozoDB graph schema foundation

#### Part II.5: Development Methodology & Standards (30 pages) **v001**
- Versioning: ONE FEATURE PER INCREMENT - END TO END - SPIC AND SPAN
- Four-word naming convention (96% LLM recall, research-backed)
- TDD-First: STUB → RED → GREEN → REFACTOR
- Functional idiomatic Rust principles
- Layered architecture (L1→L2→L3)
- Quality enforcement checklist

#### Part III: Evidence (40 pages)
- Performance analysis (small/medium/large codebases)
- Memory usage (theoretical + empirical: 1.5 GB vs 32 GB)
- Error detection improvements (30-65% more errors caught)
- Parseltongue validation (proves graph storage works)

#### Part IV: Implementation (30 pages)
- 4-phase roadmap (24 months to production)
- Technology stack (CozoDB, tree-sitter, inkwell, LLVM)
- Success metrics (performance + DX targets)
- Risk mitigation strategies

#### Part V: Datalog Implementation Patterns (30 pages) **v002**
- Core Schema Foundation (function, type_def, scope, binding)
- Pattern 1: Name Resolution via Scope Chains (<100μs)
- Pattern 2: Type Inference via Constraints (<50ms)
- Pattern 3: Borrow Checking via Lifetime Graphs (<200μs)
- Pattern 4: Dependency Tracking (<1ms for 100K functions)
- Pattern 5: Red-Green Incremental Algorithm (100-250× speedup)
- Pattern 6: Trait Resolution (<500μs)
- Pattern 7: Control Flow & Dataflow Analysis (<1ms)
- 600+ lines of executable Datalog code

#### Part VI: Complete Implementation Specification (30 pages) **NEW in v003**
- High-Level Design (HLD) - System architecture
- Low-Level Design (LLD) - CozoDB Schema (40+ relations)
- Rust Interface Definitions (8 trait groups, 50+ methods)
- Comprehensive Query Catalog (85+ queries)
- Implementation Examples (concrete code)

#### Appendices (10 pages)
- Architecture comparison table
- Granularity levels (token to module)
- Multi-language support (12 languages)
- LLVM integration details

**Total**: ~260 pages of actionable specification

**Key Numbers to Memorize**:
- 100-250× faster incremental builds
- 95% memory reduction (1.5 GB vs 32 GB for Chromium)
- 30-65% more compile-time errors detected
- 30-50% smaller binaries
- 20-50 MB storage per 100K LOC
- <1ms CozoDB queries (2-hop graph traversal)

**When to use**: Primary reference for all implementation work

---

### docs/compiler-architecture/QUICK-REFERENCE.md
**Path**: `docs/compiler-architecture/QUICK-REFERENCE.md`
**Size**: 489 lines | 14 KB
**Purpose**: Fast lookup cheat sheet for key concepts

**Reading time**: 5-10 minutes

**Key Sections**:
- Core thesis (one sentence)
- Strategic decision summary
- Key numbers (memorize these)
- Architecture comparison table (quick lookup)
- Granularity levels (quick lookup)
- Multi-language support matrix
- Implementation phases timeline
- Technology stack
- CozoDB schema cheat sheet
- Compilation pipeline diagram
- Datalog patterns summary (7 patterns)
- Success metrics
- ROI calculation
- Risks & mitigations
- Elevator pitch (30 seconds)
- One-page summary (printable)
- Development standards (quick reference)

**When to use**: Daily reference, team presentations, executive briefings

---

### docs/compiler-architecture/VERSION-SUMMARY.md
**Path**: `docs/compiler-architecture/VERSION-SUMMARY.md`
**Size**: 396 lines | 11 KB
**Purpose**: Track changes across versions of MASTER-REFERENCE documentation

**Current Version**: 2.0 (v002)
**Note**: File tracks v002 as latest, but v003 is now current (needs update)

**Key Sections**:
- Version 2.0 (v002) details
- Version 1.1 (v001) details
- Version 1.0 initial synthesis
- Version comparison matrix
- Key metrics across versions
- Migration guide (v001 → v002)
- Future versions planned
- Version control best practices
- Archive policy

**Version Timeline**:
- v1.0 (2025-11-24): Initial synthesis (150 pages)
- v1.1/v001 (2025-11-25): Added Part II.5 Development Methodology
- v2.0/v002 (2025-11-25): Added Part V Datalog Patterns (180 pages)
- v3.0/v003 (2025-11-25): Added Part VI HLD/LLD Specification (~260 pages)

**When to use**: Understanding version history and migration paths

---

### docs/compiler-architecture/S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md
**Path**: `docs/compiler-architecture/S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md`
**Size**: 1,428 lines | 39 KB | ~80 pages
**Purpose**: Comprehensive LOC estimates and TDD testing strategy

**Reading time**: 60-90 minutes

**Key Sections**:

#### 1. Executive Summary
- Total LOC estimate: 80,000-150,000 (vs rustc 600,000)
- 75-83% LOC reduction expected
- 18-24 months to production
- 5-8 engineers required

#### 2. Phase-by-Phase LOC Breakdown
| Phase | Timeline | LOC | Team | Success Metric |
|-------|----------|-----|------|----------------|
| **POC** | 2-4 weeks | 500-2,000 | 1-2 engineers | Compile fibonacci |
| **MVP** | 1-2 months | 5,000-15,000 | 2-3 engineers | Basic borrow checking |
| **Bootstrap** | 3-6 months | 20,000-50,000 | 3-5 engineers | Self-hosting compiler |
| **Production** | 18-24 months | 80,000-150,000 | 5-8 engineers | Full Rust support |

#### 3. Component-Level Analysis
- Type Inference: 15,000 LOC (rustc) → 8,000 LOC (graph) = 47% reduction
- Borrow Checker: 25,000 LOC → 12,000 LOC = 52% reduction
- Dependency Tracking: 8,000 LOC → 3,000 LOC = 63% reduction

#### 4. Testing Strategy (Compiletest Framework)
- UI Tests: Type/borrow error tests with .stderr files
- Blessing Workflow: Auto-generate expected output with `--bless`
- Graph Consistency Queries: Validate invariants
- Incremental Differential Testing: Ensure incremental == clean build

#### 5. Minimal Proof-of-Concept
- Timeline: 2 weeks
- LOC: <2,000
- Goal: Compile fibonacci function to LLVM IR
- Validation: Three-tier (Correctness + Error Detection + Performance)

#### 6. Week-by-Week Implementation Plan
- Week 1-2: POC (fibonacci)
- Week 3-8: MVP (basic Rust subset)
- Month 3-6: Bootstrap (self-host compiler)
- Month 7-24: Production (full Rust)

#### 7. Research Findings
- rustc: ~600,000 LOC core compiler
- TCC: <10,000 LOC (proves minimal is viable)
- Salsa framework: Incremental compilation patterns
- Datalog conciseness: 1 query replaces 200-line function

**When to use**: Planning implementation, estimating effort, understanding testing approach

---

### docs/compiler-architecture/V002-VS-V003-ANALYSIS-COMPARISON.md
**Path**: `docs/compiler-architecture/V002-VS-V003-ANALYSIS-COMPARISON.md`
**Size**: 583 lines | 22 KB
**Purpose**: Detailed comparison analysis between v002 and v003

**Key Sections**:

#### Quantitative Changes
| Metric | v002 | v003 | Change |
|--------|------|------|--------|
| **Total Pages** | 180 | ~260 | +80 pages (+44%) |
| **Datalog Queries** | 7 patterns | **85+ queries** | **11× increase** |
| **Code Examples** | 600 lines | **2000+ lines** | +233% |
| **CozoDB Relations** | 4 core | **40+ complete** | **10× increase** |
| **Rust Interfaces** | None | **8 trait groups, 50+ methods** | **NEW** |

#### What v003 Adds
- Part VI: Complete Implementation Specification
- High-Level Design (HLD)
- Low-Level Design (LLD) with 40+ CozoDB relations
- 8 Rust trait groups (50+ methods)
- 85+ complete Datalog queries

#### Time Savings for Implementers
**6-8 weeks total**:
- Schema design: 2-3 weeks → immediate
- Interface design: 2-3 weeks → immediate
- Query development: 2-4 weeks → immediate

#### Migration Path
1. Review new Part VI (30 pages, 45-60 min)
2. Study CozoDB schema (40+ relations)
3. Review Rust interfaces (8 trait groups)
4. Integrate into Phase 1 (POC) implementation

**When to use**: Understanding value of v003, planning migration from v002

---

### docs/compiler-architecture/INTEGRATION-SUMMARY.md
**Path**: `docs/compiler-architecture/INTEGRATION-SUMMARY.md`
**Size**: 459 lines | 14 KB
**Purpose**: Integration summary for v1.1 (Part II.5 Development Methodology)

**Key Sections**:
- Changes made to MASTER-REFERENCE-v001.md
- Part II.5 content breakdown
- Files modified
- Empirical validation recap
- Integration with existing parts
- Action items for implementers

**When to use**: Understanding how Part II.5 was integrated (v1.0 → v1.1)

---

### docs/compiler-architecture/DATALOG-PATTERNS-INTEGRATION-SUMMARY.md
**Path**: `docs/compiler-architecture/DATALOG-PATTERNS-INTEGRATION-SUMMARY.md`
**Size**: 596 lines | 18 KB
**Purpose**: Integration summary for v2.0 (Part V Datalog Patterns)

**Key Sections**:
- Changes made to MASTER-REFERENCE-v002.md
- Part V content breakdown (7 patterns, 600+ lines)
- Core schema additions
- Performance guarantees per pattern
- Integration with existing parts
- Action items for implementers

**When to use**: Understanding how Part V was integrated (v1.1 → v2.0)

---

## Archives (docs/compiler-architecture/zzArchive/)

Previous versions of master references and detailed research documents (~350 pages total).

### docs/compiler-architecture/zzArchive/README.md
**Path**: `docs/compiler-architecture/zzArchive/README.md`
**Size**: 243 lines | 7.4 KB
**Purpose**: Navigation guide for archived documents

**Key Sections**:
- Archive purpose and structure
- Archived master references (v001, v002)
- Detailed research documents (8 files, 350+ pages)
- Document descriptions and use cases

**When to use**: Before diving into archived detailed research

---

### docs/compiler-architecture/zzArchive/MASTER-REFERENCE-v001.md
**Path**: `docs/compiler-architecture/zzArchive/MASTER-REFERENCE-v001.md`
**Size**: 1,987 lines | 63 KB | ~150 pages
**Version**: 1.1 (v001)
**Date**: 2025-11-25
**Status**: ✅ ARCHIVED

**Major Addition**: Part II.5 Development Methodology & Standards (30 pages)

**Structure**:
- Part I: Strategic Context (30 pages)
- Part II: Technical Foundation (40 pages)
- **Part II.5: Development Methodology (30 pages)** ← NEW
- Part III: Evidence (40 pages)
- Part IV: Implementation (30 pages)
- Appendices (10 pages)

**Total**: ~150 pages

**When to use**: Historical reference for v1.1, understanding development methodology integration

---

### docs/compiler-architecture/zzArchive/MASTER-REFERENCE-v002.md
**Path**: `docs/compiler-architecture/zzArchive/MASTER-REFERENCE-v002.md`
**Size**: 2,614 lines | 82 KB | ~180 pages
**Version**: 2.0 (v002)
**Date**: 2025-11-25
**Status**: ✅ ARCHIVED

**Major Addition**: Part V Datalog Implementation Patterns (30 pages, 600+ lines of code)

**Structure**:
- Part I: Strategic Context (30 pages)
- Part II: Technical Foundation (40 pages)
- Part II.5: Development Methodology (30 pages)
- Part III: Evidence (40 pages)
- Part IV: Implementation (30 pages)
- **Part V: Datalog Implementation Patterns (30 pages)** ← NEW
- Appendices (10 pages)

**Total**: ~180 pages

**7 Datalog Patterns**:
1. Name Resolution via Scope Chains (<100μs)
2. Type Inference via Constraints (<50ms)
3. Borrow Checking via Lifetime Graphs (<200μs)
4. Dependency Tracking (<1ms for 100K functions)
5. Red-Green Incremental Algorithm (100-250× speedup)
6. Trait Resolution (<500μs)
7. Control Flow & Dataflow Analysis (<1ms)

**When to use**: Historical reference for v2.0, understanding Datalog patterns integration

---

## Root Documents (Project Level)

Additional documentation at project root level (not in docs/compiler-architecture/).

### QUICK_REFERENCE.md
**Path**: `QUICK_REFERENCE.md` (root)
**Size**: 433 lines
**Purpose**: Duplicate/alternative quick reference at root level

---

### SETUP_COMPLETE.md
**Path**: `SETUP_COMPLETE.md` (root)
**Size**: 465 lines
**Purpose**: Setup completion summary and verification

---

### REPO_INDEX.md
**Path**: `REPO_INDEX.md` (root)
**Size**: 657 lines
**Purpose**: Repository structure index

---

### ARCHITECTURE_VISUAL.md
**Path**: `ARCHITECTURE_VISUAL.md` (root)
**Size**: 853 lines
**Purpose**: Visual architecture diagrams (Mermaid)

---

### RESEARCH_THESIS.md
**Path**: `RESEARCH_THESIS.md` (root)
**Size**: 1,461 lines
**Purpose**: Research thesis document

---

### refNotes.md
**Path**: `refNotes.md` (root)
**Size**: 9,420 lines
**Purpose**: Reference notes and research materials

---

## Document Relationships

### Reading Paths

#### Path 1: Quick Understanding (15-30 minutes)
```
README.md → QUICK-REFERENCE.md → Done
```

#### Path 2: Implementation Ready (2-3 hours)
```
README.md → MASTER-REFERENCE-v003.md → S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md → S06-design101-tdd-architecture-principles.md → Start coding
```

#### Path 3: Deep Research (6-8 hours)
```
README.md → MASTER-REFERENCE-v003.md → V002-VS-V003-ANALYSIS-COMPARISON.md → All zzArchive/ documents → Full comprehension
```

#### Path 4: Team Onboarding (1 day)
```
README.md (overview) → QUICK-REFERENCE.md (cheat sheet) → .claude.md (coding standards) → S03-four-word-naming-thesis.md (naming convention) → S06-design101-tdd-architecture-principles.md (TDD principles) → MASTER-REFERENCE-v003.md (full specification) → Ready to contribute
```

---

## Version History

| Version | Date | Major Changes | Files Added/Modified |
|---------|------|---------------|----------------------|
| **v003** | 2025-11-25 | Added Part VI: HLD/LLD Specification | MASTER-REFERENCE-v003.md, V002-VS-V003-ANALYSIS-COMPARISON.md |
| **v002** | 2025-11-25 | Added Part V: Datalog Patterns (600+ lines) | MASTER-REFERENCE-v002.md (→ archived), DATALOG-PATTERNS-INTEGRATION-SUMMARY.md |
| **v001** | 2025-11-25 | Added Part II.5: Development Methodology | MASTER-REFERENCE-v001.md (→ archived), INTEGRATION-SUMMARY.md, S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md |
| **v1.0** | 2025-11-24 | Initial synthesis (8 docs → 1 MASTER-REFERENCE) | MASTER-REFERENCE.md (superseded), README.md, QUICK-REFERENCE.md |

---

## Key Concepts Index

### Performance Metrics
- **100-250× faster incremental builds** → MASTER-REFERENCE-v003.md Part I, QUICK-REFERENCE.md
- **95% memory reduction** → MASTER-REFERENCE-v003.md Part III (1.5 GB vs 32 GB)
- **30-65% more errors detected** → MASTER-REFERENCE-v003.md Part III
- **30-50% smaller binaries** → MASTER-REFERENCE-v003.md Part I

### Architecture
- **Hybrid Graph-Native (#4)** → MASTER-REFERENCE-v003.md Part II
- **Function-level incremental compilation** → MASTER-REFERENCE-v003.md Part II
- **CozoDB + LLVM architecture** → MASTER-REFERENCE-v003.md Part II

### Development Methodology
- **ONE FEATURE PER INCREMENT** → .claude.md, MASTER-REFERENCE-v003.md Part II.5
- **Four-word naming convention** → S03-four-word-naming-thesis.md, .claude.md
- **TDD: STUB → RED → GREEN → REFACTOR** → S06-design101-tdd-architecture-principles.md
- **Layered Architecture (L1→L2→L3)** → S06-design101-tdd-architecture-principles.md

### Implementation Patterns
- **7 Datalog Patterns** → MASTER-REFERENCE-v003.md Part V
- **Red-Green Incremental Algorithm** → MASTER-REFERENCE-v003.md Part V, Pattern 5
- **Name Resolution (<100μs)** → MASTER-REFERENCE-v003.md Part V, Pattern 1
- **Borrow Checking (<200μs)** → MASTER-REFERENCE-v003.md Part V, Pattern 3

### Technology Stack
- **CozoDB (RocksDB-backed)** → MASTER-REFERENCE-v003.md Part II, Part VI
- **tree-sitter (12 languages)** → MASTER-REFERENCE-v003.md Part II
- **LLVM (inkwell)** → MASTER-REFERENCE-v003.md Part II
- **Datalog queries** → MASTER-REFERENCE-v003.md Part V, Part VI

### Testing & Validation
- **Compiletest framework** → S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md Section 4
- **UI tests + blessing workflow** → S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md Section 4
- **Graph consistency queries** → S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md Section 4
- **Incremental differential testing** → S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md Section 4

### LOC Estimates
- **POC: 500-2,000 LOC (2-4 weeks)** → S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md Section 2
- **MVP: 5,000-15,000 LOC (1-2 months)** → S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md Section 2
- **Production: 80,000-150,000 LOC (18-24 months)** → S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md Section 2
- **75-83% LOC reduction vs rustc** → S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md Section 3

---

## Search Index

Use your editor's search (Cmd+F / Ctrl+F) to find:

**Concepts**:
- `CozoDB` → MASTER-REFERENCE-v003.md (all parts), QUICK-REFERENCE.md
- `Datalog` → MASTER-REFERENCE-v003.md Part V, Part VI
- `LLVM` → MASTER-REFERENCE-v003.md Part II, Part IV
- `Incremental compilation` → MASTER-REFERENCE-v003.md Part II, Part V Pattern 5
- `Borrow checking` → MASTER-REFERENCE-v003.md Part V Pattern 3
- `Type inference` → MASTER-REFERENCE-v003.md Part V Pattern 2
- `Name resolution` → MASTER-REFERENCE-v003.md Part V Pattern 1

**Methodology**:
- `TDD` → S06-design101-tdd-architecture-principles.md, .claude.md
- `Four-word naming` → S03-four-word-naming-thesis.md, .claude.md
- `Layered architecture` → S06-design101-tdd-architecture-principles.md
- `RAII` → S06-design101-tdd-architecture-principles.md

**Implementation**:
- `LOC estimation` → S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md
- `Testing strategy` → S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md
- `Query catalog` → MASTER-REFERENCE-v003.md Part VI
- `Rust interfaces` → MASTER-REFERENCE-v003.md Part VI

---

## Maintenance

### Update This Document When:
- [ ] New documentation files are added
- [ ] Major versions are released (v004+)
- [ ] Document purposes change significantly
- [ ] New sections are added to major documents
- [ ] Archive structure changes

### Review Schedule:
- **Weekly**: Check for new files
- **Monthly**: Update statistics (lines, sizes)
- **Per version**: Update version history section
- **Per phase**: Update navigation guide based on user feedback

---

## FAQ

### Q: Where do I start?
**A**: Read [README.md](#docs-compiler-architecture-readmemd) (10 min), then [QUICK-REFERENCE.md](#docs-compiler-architecture-quick-referencemd) (10 min). This gives you 80% of what you need.

### Q: What's the single most important document?
**A**: [MASTER-REFERENCE-v003.md](#docs-compiler-architecture-master-reference-v003md) - it's the complete specification.

### Q: How do I understand the four-word naming convention?
**A**: Read [S03-four-word-naming-thesis.md](#claude-s03-four-word-naming-thesismd) for full research backing, or [.claude.md](#claudemd) for quick summary.

### Q: Where are the LOC estimates?
**A**: [S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md](#docs-compiler-architecture-s07-loc-estimation-and-testing-strategymd) - complete breakdown with testing strategy.

### Q: What's the difference between v001, v002, and v003?
**A**:
- v001: Added Part II.5 (Development Methodology)
- v002: Added Part V (Datalog Patterns, 600+ lines)
- v003: Added Part VI (HLD/LLD, 85+ queries, Rust interfaces)
See [VERSION-SUMMARY.md](#docs-compiler-architecture-version-summarymd) for details.

### Q: Where are the Datalog patterns?
**A**: [MASTER-REFERENCE-v003.md Part V](#docs-compiler-architecture-master-reference-v003md) - 7 complete patterns with 600+ lines of code.

### Q: Where are the archived research documents?
**A**: [docs/compiler-architecture/zzArchive/](#docs-compiler-architecture-zzarchive-350-pages) - 350+ pages of detailed research (8 documents).

### Q: How do I learn TDD for this project?
**A**: [S06-design101-tdd-architecture-principles.md](#claude-s06-design101-tdd-architecture-principlesmd) - complete TDD workflow and architectural principles.

### Q: What's the testing strategy?
**A**: [S07-LOC-ESTIMATION-AND-TESTING-STRATEGY.md Section 4](#docs-compiler-architecture-s07-loc-estimation-and-testing-strategymd) - compiletest framework, UI tests, blessing workflow.

### Q: Where are the CozoDB schemas?
**A**: [MASTER-REFERENCE-v003.md Part VI LLD](#docs-compiler-architecture-master-reference-v003md) - 40+ complete relations with 85+ queries.

---

## Contact & Contributions

**Repository**: [that-in-rust/parseltongue](https://github.com/that-in-rust/parseltongue)
**Issues**: Use GitHub issues for technical discussion
**Proposals**: Submit PRs for documentation improvements
**Questions**: Check this TABLE-OF-CONTENTS.md first, then ask

---

**Last Updated**: 2025-11-25
**Current Version**: v003
**Status**: ✅ COMPLETE - Ready for implementation

🚀 **Navigate with confidence. Build with precision.**
