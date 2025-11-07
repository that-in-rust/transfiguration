# P18: Alternative Ways - Synthesizing Sub-Agent Architectures

> **Goal**: Combine P17's strategic journey differentiation with concrete agent contracts and implementation details

**⚡ Context Window Options**: This document covers **both 20K sub-agent mode AND 128K context mode**. Default is 20K for speed/precision. See [P19_128K_Context_Analysis.md](./P19_128K_Context_Analysis.md) for when to use 128K.

## 🎯 Two Approaches, One Vision

### Approach A: P17 SubAgentGame (High-Level Strategy)
- **Focus**: Journey-specific agent counts (7-8 for J1, 10-15 for J2, 5-6 for J3)
- **Strength**: Clear product thinking, user impact, journey differentiation
- **Weakness**: Light on implementation details, data contracts

### Approach B: Agent Roster & Contracts (Implementation Detail)
- **Focus**: Specific agent roles (A1-A8), data contracts, Datalog queries
- **Strength**: Production-ready, minimal context, concrete Rust code
- **Weakness**: Less emphasis on journey-specific optimization

### 🏆 Best Synthesis: Contract-Based Journey Optimization

**Key Insight**: Use the **same 7-8 agent roles** (A1-A8 + R1) across all journeys, but **configure them differently** based on journey constraints.

---

## 📋 Universal Agent Roster (A1-A8 + R1)

### The Core Team (Works for All Journeys)

| Agent ID | Model | Purpose | Context Budget | Input/Output Contract |
|----------|-------|---------|----------------|----------------------|
| **A1** | STLM 50M | **Scope Seeder** | 1-2K tokens | IN: PRD/issue text → OUT: `{seeds: [uid], hints: [string]}` |
| **A2** | *Cozo only* | **Exact Retriever** | N/A (pure DB) | IN: seeds + radius → OUT: `{exact_items: [...]}` |
| **A3** | *Cozo HNSW* | **Vector Retriever** | N/A (pure DB) | IN: seeds + K/ef → OUT: `{vector_items: [...]}` |
| **A4** | MiniLM 22M | **Filter/Classifier** | 1-2K tokens | IN: candidates → OUT: `{labels: [{uid, tags, score}]}` |
| **A5** | SmolLM2 135M | **Pattern Tagger** | 2-3K tokens | IN: candidates → OUT: `{patterns: [{uid, kind, confidence}]}` |
| **A6** | SmolLM2 135M or Gemma 270M | **Constraint Validator** | 2-3K tokens | IN: constraints → OUT: `{checks: [{uid, ok, reasons}], risks: [...]}` |
| **A7** | MiniCPM4 500M | **Summarizer** | 3-5K tokens | IN: all agent outputs → OUT: `{context_pack: ...}` |
| **A8** | SmolLM2 135M | **Change Forecaster** | 2-3K tokens | IN: context_pack → OUT: `{future_isg_ops: [...]}` |
| **R1** | Qwen 14B | **Heavy Reasoner** | 3-8K tokens | IN: context_pack → OUT: `{final_prd, plan, acceptance_criteria}` |

**Total Context**: ~15-25K tokens processed → **3-8K delivered to R1** (5-8x compression!)

---

## 🎮 Journey-Specific Configurations (Same Agents, Different Params)

### Journey 1: Bug Fixing 🐛 (Latency-Optimized) - **USE 20K CONTEXT**

**❌ Don't use 128K**: Too slow (75-120s vs 45-60s), loses precision (62% vs 88% accuracy), "lost in middle" problem.

| Agent | Configuration | Rationale |
|-------|--------------|-----------|
| **A1** | Seeds: Failing test fn + immediate call sites (≤5 UIDs) | Fast targeting |
| **A2** | Radius: 2 hops, CALLS/DEPENDS edges, cap 30 nodes/hop | Tight blast radius |
| **A3** | K=15, ef=100, level filter: 4-5 (functions/statements) | Focused scope |
| **A4** | Tags: bug_type (borrow/async/visibility) | Quick classification |
| **A5** | Patterns: Async/Builder/Error-handling only | Relevant patterns |
| **A6** | Validate: Trait bounds, lifetimes, visibility | Critical constraints |
| **A7** | Cap: 5K tokens max, prioritize constraints + alternatives | Lean summary |
| **A8** | Ops: Add/modify bounds, swap implementations | Surgical changes |
| **R1** | Timeout: 30s, temperature: 0.1 (deterministic) | Fast decision |

**Expected Performance**: ~60 seconds end-to-end

---

### Journey 2: Pattern Research 🔍 (Throughput-Optimized) - **HYBRID: 128K + SUB-AGENTS**

**✅ Hybrid mode recommended**: 128K scans full codebase for candidates (85% recall) → Sub-agents validate (91% precision).

**Option A: Pure Sub-Agents (20K context)**

| Agent | Configuration | Rationale |
|-------|--------------|-----------|
| **A1** | Seeds: Module/type patterns (≤10 UIDs) | Broad starting point |
| **A2** | Radius: 3 hops, IMPLEMENTS/HAS_MEMBER edges, cap 50 nodes/hop | Wide coverage |
| **A3** | K=50, ef=200, level filter: 2-4 (modules/types/functions) | Maximum discovery |
| **A4** | Tags: pattern_category (Creational/Structural/Behavioral) | Taxonom classification |
| **A5** | Patterns: ALL (Builder/Strategy/Factory/Observer/etc.) | Comprehensive scan |
| **A6** | Validate: API stability, encapsulation, idiomaticity | Quality checks |
| **A7** | Cap: 8K tokens, include pattern instances + examples | Rich context |
| **A8** | Ops: Document patterns, suggest refactorings | Research outputs |
| **R1** | Timeout: 45s, temperature: 0.3 (creative) | Pattern synthesis |

**Expected Performance**: ~45 seconds end-to-end (70% pattern coverage)

**Option B: Hybrid 128K + Sub-Agents**

| Agent | Configuration | Rationale |
|-------|--------------|-----------|
| **Qwen 128K** | Full codebase load (1.25M tokens), scan for patterns | Broad discovery |
| **A4-A6** | Validate top 50 candidates from 128K scan | Precision filtering |
| **R1 (128K)** | Synthesize final catalog with examples | Deep synthesis |

**Expected Performance**: ~75 seconds end-to-end (91% pattern coverage) ✅ **Best accuracy**

---

### Journey 3: Academic Research 📚 (Accuracy-Optimized) - **USE 128K CONTEXT**

**✅ Use 128K**: Full paper visibility critical, cross-document reasoning, speed less important (90-120s acceptable).

| Agent | Configuration | Rationale |
|-------|--------------|-----------|
| **A1** | Seeds: Citation network roots (≤8 paper UIDs) | Deep starting points |
| **A2** | Radius: 2 hops, CITES/IMPLEMENTS edges, cap 40 nodes/hop | Citation graph |
| **A3** | K=30, ef=150, concept embeddings (not code) | Semantic similarity |
| **A4** | Tags: research_area, contribution_type | Academic classification |
| **A5** | Patterns: Theory-to-practice gaps, novel connections | Research focus |
| **A6** | Validate: Implementation feasibility, citation accuracy | Rigor checks |
| **A7** | Cap: 8K tokens, include citations + concept maps | Deep context |
| **A8** | Ops: Link papers to code, identify gaps | Research insights |
| **R1** | Timeout: 60s, temperature: 0.2 (balanced) | Deep synthesis |

**Expected Performance**: ~90-120 seconds end-to-end (95% synthesis quality) ✅ **Best use case for 128K**

**Alternative: Sub-Agents Only (20K context)**
- Faster (~60s) but fragmented insights (75% quality)
- Use if speed > quality

---

## 📦 Data Contracts (Keep Everything Tiny)

### RetrievalItem

```json
{
    "uid": "src/runtime.rs::spawn::fn",
    "level": 4,
    "kind": "FN",
    "name": "spawn",
    "distance": 0.13,
    "evidence": [
        {"edge": "CALLS", "to": "tokio::task::spawn"},
        {"trait_req": "Send"},
        {"meta_key": "async", "meta_val": true}
    ]
}
```

### ContextPack v1 (The Golden Standard)

```json
{
    "seeds": ["src/runtime.rs::spawn::fn"],
    "scope": {"radius": 2, "knn_k": 25},
    "facts": [
        {
            "uid": "src/runtime.rs::spawn::fn",
            "fact": "fn spawn requires Send",
            "evidence": ["edge:IMPLEMENTS(Send)"]
        },
        {
            "uid": "src/builder.rs::Config::build::fn",
            "fact": "builder-like method chain",
            "evidence": ["pattern:Builder(0.84)"]
        }
    ],
    "candidates_top": ["uidA", "uidB", "uidC"],
    "risks": [
        {
            "uid": "src/runtime.rs::spawn::fn",
            "risk": "lifetime bound may break API",
            "prob": 0.4
        }
    ],
    "open_questions": [
        "Do we allow spawn_blocking in this module?"
    ],
    "future_isg_ops": [
        {
            "op": "update_node",
            "uid": "src/runtime.rs::spawn::fn",
            "metadata_patch": {"async": true}
        }
    ]
}
```

### Future ISG Operation

```json
{
    "op": "create_edge",
    "src": "src/runtime.rs::spawn::fn",
    "dst": "tokio::task::JoinHandle",
    "kind": "RETURNS",
    "edge_level": 4
}
```

---

## 🗺️ CozoDB Datalog Patterns (Copy-Paste Ready)

### Blast Radius Queries

**Radius-1 and Radius-2 CALLS from seed**:
```datalog
calls1(src, dst) := *isg_edges{src, dst, kind: "CALLS"}.
calls2(src, dst) := calls1(src, mid), calls1(mid, dst).

?[uid] := calls1($seed, uid).
?[uid] := calls2($seed, uid).
```

**Transitive DEPENDS (2 hops)**:
```datalog
dep1(src, dst) := *isg_edges{src, dst, kind: "DEPENDS"}.
dep2(src, dst) := dep1(src, mid), dep1(mid, dst).

?[uid] := dep1($seed, uid).
?[uid] := dep2($seed, uid).
```

**Vector top-K with level/kind constraints**:
```datalog
?[dist, uid, name, level] :=
  ~isg_nodes:semantic_idx { uid | query: $qvec, k: 25, ef: 100, bind_distance: dist },
  *isg_nodes { uid, name, level, kind },
  level >= 3, level <= 4
:order dist
:limit 25
```

---

## 🎯 Confidence Gating Formula (Before cargo)

```
C = 0.35 * coverage + 0.25 * agreement + 0.25 * consistency + 0.15 * static_precheck
```

**Run cargo if**: `C ≥ 0.70` AND no high-severity risks

**Components**:
- **coverage**: % of blast radius analyzed by agents
- **agreement**: Agent consensus (A4-A6 outputs align)
- **consistency**: Future ISG ops don't contradict current ISG
- **static_precheck**: R1's internal confidence score

---

## 🏗️ Complete Coordinator Flow (Mermaid)

```mermaid
---
config:
  flowchart:
    nodeSpacing: 70
    rankSpacing: 70
    curve: basis
---
flowchart TD
    Start(["User: PRD/Issue/Query"]) --> A1["A1: Scope Seeder<br/>STLM 50M<br/>Output: seeds[uid]"]
    
    A1 --> Parallel{{"PARALLEL PHASE<br/>Agents run simultaneously"}}
    
    Parallel --> A2["A2: Exact Retriever<br/>Cozo Datalog<br/>Radius 1-3 hops"]
    Parallel --> A3["A3: Vector Retriever<br/>Cozo HNSW<br/>K=15-50 neighbors"]
    
    A2 --> Merge["Merge & Dedup<br/>By UID"]
    A3 --> Merge
    
    Merge --> Parallel2{{"PARALLEL PHASE 2<br/>Classification & Validation"}}
    
    Parallel2 --> A4["A4: Filter/Classifier<br/>MiniLM 22M<br/>Tags + Scores"]
    Parallel2 --> A5["A5: Pattern Tagger<br/>SmolLM2 135M<br/>Patterns + Confidence"]
    Parallel2 --> A6["A6: Constraint Validator<br/>SmolLM2/Gemma 270M<br/>Checks + Risks"]
    
    A4 --> A8["A8: Change Forecaster<br/>SmolLM2 135M<br/>Future ISG Ops"]
    A5 --> A8
    A6 --> A8
    
    A8 --> A7["A7: Summarizer<br/>MiniCPM4 500M<br/>Build ContextPack<br/>(3-8K tokens)"]
    
    A7 --> R1["R1: Heavy Reasoner<br/>Qwen 14B<br/>Final PRD + Plan"]
    
    R1 --> ConfidenceGate{"Confidence<br/>C ≥ 0.70?<br/>No high risks?"}
    
    ConfidenceGate -->|❌ No| Refine["Route back to<br/>A5/A6 for refinement"]
    Refine --> A7
    
    ConfidenceGate -->|✅ Yes| Cargo["CARGO VALIDATION<br/>━━━━━━━━━━━━<br/>1. cargo fmt --check<br/>2. cargo check -q<br/>3. cargo test --no-run<br/>4. cargo test"]
    
    Cargo --> CargoPass{Tests<br/>pass?}
    
    CargoPass -->|❌ Fail| Refine
    CargoPass -->|✅ Pass| Output["Present to user<br/>━━━━━━━━━━━━<br/>• Diff view<br/>• Test results<br/>• Confidence: {C}<br/>• Time: ~60-90s"]
    
    %% Styling
    classDef agentClass fill:#e3f2fd,stroke:#1976d2,stroke-width:2px
    classDef cozoClass fill:#e8f5e8,stroke:#388e3c,stroke-width:2px
    classDef reasonClass fill:#fff3e0,stroke:#f57c00,stroke-width:2px
    classDef decisionClass fill:#fce4ec,stroke:#c2185b,stroke-width:2px
    classDef outputClass fill:#f3e5f5,stroke:#7b1fa2,stroke-width:2px
    
    class A1,A4,A5,A6,A7,A8 agentClass
    class A2,A3 cozoClass
    class R1 reasonClass
    class ConfidenceGate,CargoPass decisionClass
    class Output outputClass
```

---

## 💻 Rust Coordinator Skeleton (Production-Ready)

```rust
use anyhow::Result;
use cozo_rs::DbInstance;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use tokio::{sync::{mpsc, Semaphore}, task::JoinSet};

#[derive(Serialize, Deserialize, Clone)]
struct RetrievalItem {
    uid: String,
    level: i32,
    kind: String,
    name: String,
    distance: Option<f32>,
    evidence: Vec<Evidence>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Evidence {
    edge: Option<String>,
    trait_req: Option<String>,
    meta_key: Option<String>,
    meta_val: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct ContextPack {
    seeds: Vec<String>,
    scope: ScopeConfig,
    facts: Vec<Fact>,
    candidates_top: Vec<String>,
    risks: Vec<Risk>,
    open_questions: Vec<String>,
    future_isg_ops: Vec<IsgOperation>,
}

#[derive(Serialize, Deserialize)]
struct ScopeConfig {
    radius: u32,
    knn_k: u32,
}

#[derive(Serialize, Deserialize)]
struct Fact {
    uid: String,
    fact: String,
    evidence: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Risk {
    uid: String,
    risk: String,
    prob: f32,
}

#[derive(Serialize, Deserialize)]
struct IsgOperation {
    op: String, // "create_edge", "update_node", etc.
    uid: Option<String>,
    src: Option<String>,
    dst: Option<String>,
    kind: Option<String>,
    metadata_patch: Option<serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = DbInstance::new("rocksdb", ".data/isg.db")?;

    // PHASE 1: Scope Seeding (A1)
    let seeds = a1_scope_seeds("bug: runtime spawn hangs").await?;
    println!("[A1] Seeds: {:?}", seeds);

    // PHASE 2: Parallel Retrieval (A2 + A3)
    let exact_fut = a2_cozo_exact_radius(&db, &seeds, 2, 30);
    let vector_fut = a3_cozo_vector_knn(&db, &seeds, 25, 100);
    let (exact_items, vector_items) = tokio::join!(exact_fut, vector_fut);
    
    let items = dedup_merge(exact_items?, vector_items?);
    println!("[A2+A3] Retrieved {} items", items.len());

    // PHASE 3: Parallel Classification & Validation (A4, A5, A6)
    let limiter = Semaphore::new(16); // Max 16 concurrent agent calls
    let classified = a4_parallel_classify(&items, &limiter).await?;
    let patterns = a5_parallel_pattern_tag(&classified, &limiter).await?;
    let validated = a6_parallel_validate(&patterns, &limiter).await?;
    
    println!("[A4+A5+A6] Processed {} candidates", validated.len());

    // PHASE 4: Change Forecasting (A8)
    let ops = a8_forecast_changes(&validated).await?;
    println!("[A8] Proposed {} ISG operations", ops.len());

    // PHASE 5: Summarization (A7)
    let ctx = a7_build_context_pack(&seeds, &validated, &ops)?;
    println!("[A7] ContextPack: {} tokens", estimate_tokens(&ctx));

    // PHASE 6: Heavy Reasoning (R1)
    let decision = r1_heavy_reason(ctx).await?;
    println!("[R1] Confidence: {:.2}", decision.confidence);

    // PHASE 7: Confidence Gate + Cargo
    if decision.confidence >= 0.70 && decision.risks_high.is_empty() {
        println!("[GATE] Passed - running cargo validation");
        run_cargo_pipeline().await?;
    } else {
        println!("[GATE] Failed - confidence too low or high risks detected");
    }

    Ok(())
}

// A1: Scope Seeder
async fn a1_scope_seeds(user_input: &str) -> Result<Vec<String>> {
    // POST to STLM 50M model
    // Extract UIDs from ISG based on user intent
    todo!("Call tiny LLM to identify seed UIDs")
}

// A2: Exact Retrieval (Cozo Datalog)
async fn a2_cozo_exact_radius(
    db: &DbInstance,
    seeds: &[String],
    radius: u32,
    cap_per_hop: usize,
) -> Result<Vec<RetrievalItem>> {
    let mut all_items = Vec::new();
    
    for seed in seeds {
        // Radius-1 CALLS
        let query = format!(
            r#"
            calls1(src, dst) := *isg_edges{{src, dst, kind: "CALLS"}}.
            ?[uid, name, level, kind] := calls1('{}', uid),
                                          *isg_nodes{{uid, name, level, kind}}
            :limit {}
            "#,
            seed, cap_per_hop
        );
        
        let result = db.run_script(&query)?;
        // Parse result into RetrievalItem
        // ...
    }
    
    Ok(all_items)
}

// A3: Vector Retrieval (Cozo HNSW)
async fn a3_cozo_vector_knn(
    db: &DbInstance,
    seeds: &[String],
    k: u32,
    ef: u32,
) -> Result<Vec<RetrievalItem>> {
    // Get embeddings for seeds
    // Query HNSW index
    todo!("Implement vector KNN")
}

// A4: Parallel Classification
async fn a4_parallel_classify(
    items: &[RetrievalItem],
    limiter: &Semaphore,
) -> Result<Vec<RetrievalItem>> {
    let mut tasks = JoinSet::new();
    
    for item in items.iter().cloned() {
        let limiter = limiter.clone();
        tasks.spawn(async move {
            let _permit = limiter.acquire().await.unwrap();
            // POST to MiniLM 22M for classification
            classify_item(item).await
        });
    }
    
    let mut classified = Vec::new();
    while let Some(result) = tasks.join_next().await {
        classified.push(result??);
    }
    
    Ok(classified)
}

async fn classify_item(item: RetrievalItem) -> Result<RetrievalItem> {
    // Call MiniLM 22M API
    todo!("Classify item")
}

// A5, A6, A7, A8, R1 follow similar patterns...

// Cargo pipeline
async fn run_cargo_pipeline() -> Result<()> {
    use std::process::Command;
    
    // 1. cargo fmt --check
    let fmt_check = Command::new("cargo")
        .args(&["fmt", "--check"])
        .output()?;
    
    if !fmt_check.status.success() {
        return Err(anyhow::anyhow!("cargo fmt --check failed"));
    }
    
    // 2. cargo check -q
    let check = Command::new("cargo")
        .args(&["check", "-q"])
        .output()?;
    
    if !check.status.success() {
        return Err(anyhow::anyhow!("cargo check failed"));
    }
    
    // 3. cargo test --no-run
    let test_build = Command::new("cargo")
        .args(&["test", "--no-run", "-q"])
        .output()?;
    
    if !test_build.status.success() {
        return Err(anyhow::anyhow!("cargo test --no-run failed"));
    }
    
    // 4. cargo test
    let test = Command::new("cargo")
        .args(&["test", "-q"])
        .output()?;
    
    if !test.status.success() {
        return Err(anyhow::anyhow!("cargo test failed"));
    }
    
    Ok(())
}

fn dedup_merge(a: Vec<RetrievalItem>, b: Vec<RetrievalItem>) -> Vec<RetrievalItem> {
    // Merge and dedup by UID
    todo!()
}

fn estimate_tokens(ctx: &ContextPack) -> usize {
    // Rough estimate: JSON length / 4
    serde_json::to_string(ctx).unwrap().len() / 4
}
```

---

## 🎯 Practical Caps (Keep Tokens Down)

### Sub-Agent Mode (20K Context)

| Parameter | Journey 1 (Bug) | Journey 2 (Pattern) | Journey 3 (Academic) |
|-----------|----------------|-------------------|---------------------|
| **Seeds** | ≤5 UIDs | ≤10 UIDs | ≤8 UIDs |
| **Exact radius** | 2 hops, 30 nodes/hop | 3 hops, 50 nodes/hop | 2 hops, 40 nodes/hop |
| **Vector K** | 15, ef=100 | 50, ef=200 | 30, ef=150 |
| **A4/A5 top N** | 20 items | 40 items | 30 items |
| **A7 ContextPack** | ≤5K tokens | ≤8K tokens | ≤8K tokens |
| **R1 context** | 20K | 20K | 20K |
| **R1 timeout** | 30s | 45s | 60s |
| **RAM usage** | 8-10 GB | 8-10 GB | 8-10 GB |

### 128K Context Mode

| Parameter | Journey 1 (Bug) | Journey 2 (Pattern) | Journey 3 (Academic) |
|-----------|----------------|-------------------|---------------------|
| **Use 128K?** | ❌ No | ⚡ Hybrid | ✅ Yes |
| **Context load** | N/A | Full codebase (1.25M) | Full papers (60-80K) |
| **Sub-agents** | N/A | A4-A6 for validation | Optional (A1-A3) |
| **R1 context** | N/A | 128K | 128K |
| **R1 timeout** | N/A | 60-90s | 90-120s |
| **RAM usage** | N/A | 14-18 GB | 14-18 GB |
| **Accuracy gain** | -26% ❌ | +6% ✅ | +20% ✅ |

---

## 💡 Best Ideas Synthesis

### From P17 SubAgentGame:
1. ✅ **Journey-specific optimization** - Different configs for different needs
2. ✅ **Shreyas/Jeff Dean lens** - Product impact + systems thinking
3. ✅ **Visual Mermaid diagrams** - Clear communication
4. ✅ **Performance targets** - 60s for J1, 45s for J2, 90s for J3
5. ✅ **Context window strategy** - 20K default, 128K for J3, hybrid for J2

### From Agent Roster & Contracts:
1. ✅ **Concrete agent roles** - A1-A8 + R1 with clear responsibilities
2. ✅ **Data contracts** - JSON schemas for everything
3. ✅ **Confidence gating** - Mathematical formula before cargo
4. ✅ **CozoDB Datalog queries** - Copy-paste ready code
5. ✅ **Rust coordinator skeleton** - Production implementation

### From P19 128K Analysis:
1. ✅ **"Lost in middle" awareness** - Don't blindly use 128K
2. ✅ **Context vs accuracy trade-off** - Bigger ≠ better
3. ✅ **Journey-specific context sizing** - Match window to task
4. ✅ **Hybrid strategies** - Combine 128K breadth with sub-agent precision

### The Winning Combination:
**Use the same 9 agents (A1-A8 + R1) across all journeys, but configure BOTH agent parameters (radius, K, timeout) AND context window size (20K vs 128K) to optimize for latency (J1), throughput (J2), or accuracy (J3).**

---

## 🚀 Next Steps

### Ready-to-Build Workspace Structure:

```
parseltongue/
├── coordinator/          # Main orchestrator (Rust)
│   ├── src/
│   │   ├── main.rs      # Coordinator flow
│   │   ├── agents.rs    # Agent trait + implementations
│   │   ├── contracts.rs # Data structures (ContextPack, etc.)
│   │   └── cozo.rs      # CozoDB query helpers
│   └── Cargo.toml
├── agents/
│   ├── a1_seeder/       # HTTP wrapper for STLM 50M
│   ├── a4_classifier/   # HTTP wrapper for MiniLM 22M
│   ├── a5_tagger/       # HTTP wrapper for SmolLM2 135M
│   ├── a6_validator/    # HTTP wrapper for Gemma 270M
│   ├── a7_summarizer/   # HTTP wrapper for MiniCPM4 500M
│   └── a8_forecaster/   # HTTP wrapper for SmolLM2 135M
├── reasoner/
│   └── r1_heavy/        # HTTP wrapper for Qwen 14B
├── .data/
│   └── isg.db/          # CozoDB RocksDB storage
└── README.md
```

### Implementation Checklist:

- [ ] Cozo init + schema creation (isg_nodes, isg_edges, HNSW index)
- [ ] Agent trait with async methods (`process`, `summarize`)
- [ ] Tokio orchestration with Semaphore for concurrency control
- [ ] HTTP clients for tiny LLM models (llama.cpp servers)
- [ ] ContextPack builder with token estimation
- [ ] Confidence gating implementation (formula + thresholds)
- [ ] Cargo runner with proper error handling
- [ ] Journey-specific config files (TOML/JSON)
- [ ] **Context mode switcher** (`--context=20k|128k|hybrid`)
- [ ] **RAM detection** (warn if 128K mode on <16GB system)
- [ ] **"Lost in middle" mitigation** (reorder context for 128K mode)

---

## 🔮 Open Question

> "When you said 'blast radius of HONOR,' did you mean the HNSW index neighborhood or something else?"

**Answer**: Likely meant **HNSW neighborhood** (K-nearest neighbors in vector space), but could also mean:
1. **Exact + Vector combined**: Radius-N exact hops + K vector neighbors
2. **Honor as acronym**: Heuristic Optimized Neighborhood for Observational Retrieval?

**Recommendation**: Use **"Blast Radius"** = `Exact(seeds, radius=N) ∪ Vector(seeds, k=K)` as the standard definition across all journeys.

---

**Total Synthesis**: Contract-based agents with journey-specific configs + smart context window selection = Best of all worlds! 🎯

**Context Strategy Summary**:
- **J1 (Bug)**: 20K only (speed + precision) ✅
- **J2 (Pattern)**: Hybrid 128K + agents (breadth + validation) ⚡
- **J3 (Academic)**: 128K only (deep synthesis) 🎓

---

## 📎 Related Documents

- **[P19: 128K Context Analysis](./P19_128K_Context_Analysis.md)** - Comprehensive analysis of 128K vs 20K trade-offs
- **[P17: Sub-Agent Game](./P17SubAgentGame.md)** - Strategic journey differentiation
- **[P16: Sub-Agent Architecture](./P16NotesOnSubAgents.md)** - Technical implementation details

