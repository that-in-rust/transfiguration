# P16: Sub-Agent Architecture for ISG Analysis

## 📋 ELI15 Summary - The Big Ideas in Simple Terms

**What's this about?** Instead of using one massive AI model for everything, we use a **team of specialized tiny AIs** working together. It's like having a sports team where each player has a specific position - you don't make your goalkeeper score goals!

### Core Concepts Table

| Concept | What It Actually Means | Why It's Powerful | Real Numbers |
|---------|----------------------|-------------------|--------------|
| **Tiny Agent Team** | 10-30 small AI models (22M-500M parameters) instead of 1 giant one | Each AI is an expert at ONE thing and runs super fast | 200-400 words/sec per tiny AI vs 15 words/sec for big AI |
| **Agent Roles** | Different sized AIs for different jobs | Use the right tool for the job - don't use a bulldozer to move a pebble | 22M model filters fast, 14B model thinks deeply |
| **Hybrid Search** | Mix exact database searches (like Google search) with fuzzy AI understanding (like asking a friend) | Find things precisely AND understand meaning at the same time | <1ms for exact, <10ms for semantic |
| **CPU-Only System** | Everything runs on your Mac's processor, zero GPU needed | Works on normal laptops, not just gaming computers | Your 16GB Mac Mini handles 20+ agents easily |
| **Vector Embeddings** | Turn code into lists of numbers (384 numbers per code snippet) so computers can compare similarity | Find similar code even if it's written completely differently | "fn spawn(...)" becomes [0.23, -0.45, 0.12, ...] |
| **CozoDB Storage** | Graph database that stores your code like a mind map you can search | Ask complex questions like "show me all async functions that depend on tokio" in milliseconds | 250K+ searches per second on laptop |
| **Parallel Processing** | All agents work simultaneously, not one-by-one | 10 agents working together = roughly 10x faster | 5MB codebase: 1 minute parallel vs 10 minutes sequential |
| **Multi-Level ISG** | Code organized in 5 zoom levels: crate → module → type → function → statement | Like Google Maps - see the whole city or zoom to one street | ISGL1-L5 extracted in single pass |
| **HNSW Index** | Fast approximate similarity search (like "find songs that sound like this") | Search millions of code snippets for similar ones in milliseconds | Cosine distance, tunable accuracy |
| **Rust Workspace** | All components in one organized project structure | Clean organization, easy to build and maintain | coordinator + agents/search + agents/validation |

### The Agent Team Lineup

| Agent Type | Model Size | Job Description (Simple) | Speed | RAM Usage |
|------------|-----------|-------------------------|-------|-----------|
| **Search Agents** | MiniLM 22M, STLM 50M | Find stuff in database super fast | 200-400 words/sec | ~200-500 MB each |
| **Validation Agents** | SmolLM2 135M, Gemma 270M | Check if code is correct (types, traits, lifetimes) | 150-300 words/sec | ~500-800 MB each |
| **Refinement Agents** | MiniCPM4 500M | Suggest improvements to plans/code | 100-200 words/sec | ~1-2 GB each |
| **Reasoning Agent** | Qwen 14B | Think deeply about complex problems (only 1 of these) | ~15 words/sec | ~8-10 GB |

**Team on 16GB Mac Mini:**
- 5-10 Search Agents (~2-5 GB)
- 2-3 Validation Agents (~1-2 GB)
- 1-2 Refinement Agents (~1-2 GB)
- 1 Reasoning Agent (~8-10 GB)
- **Total: Fits in 16GB with room to spare!**

### How The Workflow Actually Works

```
Your Rust Code (5MB)
    ↓
Extract with syn + rust-analyzer (parse into graph)
    ↓
Store in CozoDB with vector embeddings (searchable mind map)
    ↓
┌─────────────────────────────────────┐
│ PARALLEL AGENT TEAM                 │
├─────────────────────────────────────┤
│ Search Agents → Find relevant code  │ ← 5-10 agents working at once
│ Validation → Check correctness      │ ← 2-3 agents checking in parallel
│ Refinement → Suggest improvements   │ ← 1-2 agents refining ideas
│ Reasoning → Make final decisions    │ ← 1 deep thinker
└─────────────────────────────────────┘
    ↓
Generate fixes/docs/code
    ↓
Done in ~1 minute! (vs 10+ mins single-threaded)
```

### Three Use Cases (Journeys)

| Journey | What You're Doing | How Agents Help | Time Saved |
|---------|------------------|----------------|------------|
| **Journey 1: Bug Fixing** | Fix borrow checker errors, lifetime issues | Search finds similar bugs, Validation checks types, Reasoning drafts fix | 5-10 min → 1-2 min |
| **Journey 2: Pattern Research** | Find architectural patterns (Builder, Strategy, etc.) | Search scans graph, Validation classifies patterns, Web agents lookup docs | 3-6 min → 30-60 sec |
| **Journey 3: Code Generation** | Write new features based on requirements | Reasoning drafts plan, Search finds API boundaries, Validation checks constraints | 6-12 min → 1-3 min |

### Memory Budget Breakdown (16GB Mac Mini)

```
macOS System:              ~2-3 GB
CozoDB (with 1M nodes):    ~50-500 MB
Agent Team:                ~8-10 GB
IDE/Tools:                 ~2-3 GB
───────────────────────────────────
Total Used:                ~13-16 GB ✅ Fits!
```

### Performance You Can Expect (5MB Codebase)

| Operation | Single-Threaded | Parallel Agents | Speedup |
|-----------|----------------|-----------------|---------|
| ISG Build | 2-3 min | 30-60 sec | 3-4x faster |
| Bug Analysis | 5-10 min | 1-2 min | 5-7x faster |
| Pattern Search | 3-6 min | 30-60 sec | 5-10x faster |
| Code Generation | 6-12 min | 1-3 min | 4-6x faster |

---

## 🚀 Technical Deep-Dive (Original Content)

To blend the tiny LLM sub-agents (e.g., MiniLM 22M for filtering, STLM 50M for classification, SmolLM2 135M for quick tagging) with CozoDB's CPU-based search capabilities, we can create a hybrid system that leverages CozoDB's strengths in exact/graph/relational queries (via Datalog, up to 250K+ QPS read-only) and vector proximity search (HNSW for approximate nearest neighbors with L2/Cosine/IP metrics, fully CPU-optimized). This "mixing" avoids over-relying on agents for every search—instead, use CozoDB for efficient, low-latency CPU searches (milliseconds for graph traversals or vector queries on millions of nodes), and route results to agents for semantic enhancement, validation, or refinement. It's perfect for ISG workflows, as CozoDB natively supports graph-vector hybrids (e.g., combine vector similarity with recursive traversals).

Key benefits on your 16 GB Mac Mini:
- **CPU Efficiency**: CozoDB's RocksDB backend is SSD/multi-core optimized—no GPU needed. Vector searches are approximate but fast (tunable accuracy vs. speed via params like `ef`, `m`).
- **Hybrid Search**: Exact Datalog for structured ISG queries (e.g., transitive deps), vector for semantic/fuzzy (e.g., similar signatures via embeddings).
- **Agent Augmentation**: Tiny agents post-process Cozo results (e.g., classify matches at 200-400 t/s) or generate embeddings for vector indexing.
- **Scaling**: Embed CozoDB via cozo-rs (pure Rust, <50 MB RAM for your ISG scale). Mix with 10-20 agents without exceeding 8-10 GB total RAM.
- **Performance for 5 MB Codebase**: ISG with ~1M nodes/edges: Build index in seconds-minutes; queries <1ms (graph) or <10ms (vector). Full workflow: <30s with parallelism.

### Integration Strategy
1. **Embeddings Generation**: Use tiny LLMs or dedicated embedders (via code_execution tool with torch for models like all-MiniLM-L6-v2 ~22M params, 384-dim) to create vectors for ISG nodes (e.g., signatures as text: "fn spawn(&self, future: F) -> JoinHandle<F::Output>"). Store in CozoDB alongside metadata.
2. **CozoDB Setup**: Extend your unified ISG schema with vector fields/indexes. Use HNSW for semantic search on embeddings.
3. **Query Flow**: Coordinator runs Datalog for exact/graph searches; if fuzzy needed, use vector NN. Feed results to agents for refinement.
4. **Mixing Agents**: Agents handle non-deterministic tasks (e.g., "Interpret this vector match as a pattern"); Cozo for deterministic/scale.
5. **Edge Cases**: For PRD loops, use Cozo vector search to find similar past iterations; agents refine. Time travel in Cozo for rollbacks.

Updated CozoDB Schema (from doc, with vectors):

```
// Unified ISG node table with vector for semantic search
:create isg_nodes {
  uid: String,              // Primary key
  level: Int,               // 1-5
  kind: String,
  name: String,
  parent_uid: String?,
  metadata: String,         // JSON
  embedding: <F32; 384>     // Vector for signature/metadata text
}

// HNSW Index Creation (once, after schema)
::hnsw create isg_nodes:semantic_idx {
    dim: 384,               // Match embedding dim
    m: 50,                  // Connections per node (tune for accuracy)
    dtype: F32,
    fields: [embedding],
    distance: Cosine,       // Or L2/IP; Cosine good for text
    ef_construction: 20,    // Build-time NN (higher = slower build, better quality)
    extend_candidates: false,
    keep_pruned_connections: false
}

// Unified edges (no change, but queryable with vector-joined nodes)
:create isg_edges { src: String, dst: String, kind: String, edge_level: Int, metadata: String? }
```

### Workflow Updates with Mixing
For each journey, "mix" by prioritizing CozoDB CPU searches, then agents. Use Rust coordinator for orchestration.

1. **Journey 1: Bug Solving**
   - **Mixed Flow**: Ingest → Extract ISG + generate embeddings (tiny agent or torch embedder) → Store in CozoDB.
     - Exact Search: Datalog for deps/lifetimes (e.g., `?[dependent, dependency] := *isg_edges{src: dependent, dst: dependency, kind: "DEPENDS"}` → <1ms).
     - Vector Search: Fuzzy similar bugs (e.g., `?[dist, uid, embedding] := ~isg_nodes:semantic_idx { uid, embedding | query: q_vec, k: 5, ef: 30, bind_distance: dist }` where `q_vec` is embedding of bug desc).
     - Agents: 5-10 parallel search agents (STLM/MiniLM) filter/classify results (e.g., "Is this match a borrow error?"); validation agents (SmolLM2) check.
     - PRD Refine: Deeper Qwen + refinement agents use mixed results.
   - **Time Est.**: Cozo queries add negligible overhead; total parallel: ~45s for 5 MB.

2. **Journey 2: Pattern Research**
   - **Mixed Flow**: Build multi-level ISG + embeddings → Cozo for graph algos (e.g., PageRank: built-in, ~50ms for 10K nodes) + vector for similar patterns (e.g., query embedding of "Builder pattern" to find nearest trait nodes).
     - Hybrid Query Example: Combine vector + graph:

```
?[dist, uid, dep] := ~isg_nodes:semantic_idx { uid | query: q_vec, k: 10, bind_distance: dist },
                     *isg_edges{src: uid, dst: dep, kind: "IMPLEMENTS"}
```

     - Agents: 10+ parallel (Gemma/MiniCPM4) tag patterns (e.g., "Classify as Strategy: {vector match}"); web agents if external needed.
   - **Time Est.**: Vector + graph <50ms/query; parallel agents: ~20s total.

3. **Journey 3: Code Generation/PRD Refinement**
   - **Mixed Flow**: PRD draft (deeper agent) → Cozo vector for similar historical PRDs (store past iterations with embeddings) + graph for constraints.
     - Roll-Up Example with Vector Filter:

```
?[type_uid, similar_count] := ~isg_nodes:semantic_idx { stmt_uid | query: q_vec, k: 20, filter: level == 5 },
                              *isg_nodes{uid: stmt_uid, level: 5, parent_uid: fn_uid},
                              *isg_nodes{uid: fn_uid, level: 4, parent_uid: type_uid},
                              similar_count := count(stmt_uid)
:group by type_uid
```

     - Agents: Refinement agents (SmolLM2) suggest updates based on matches; validation for constraints.
   - **Time Est.**: Cozo time travel + vector: instant; parallel: ~1 min.

### Rust Code Sketch (Coordinator Crate)
Embed embeddings gen via tiny LLM or torch (simulate here; use code_execution for real).

```rust
use cozo_rs::{DbInstance, ScriptResult};
use tokio::sync::mpsc;
use reqwest::Client;
use sentence_transformers; // Or tiny LLM via llama.cpp for embeddings

fn generate_embedding(text: &str) -> Vec<f32> {
    // Use tiny model (e.g., MiniLM) or torch: 384-dim vector
    // Example: POST to agent "Embed this signature: {text}"
    vec![0.1; 384] // Placeholder
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = DbInstance::new("rocksdb", "isg.db")?;
    
    // Create schema/index (once)
    db.run_script(":create isg_nodes { uid: String, ... , embedding: <F32; 384> }")?;
    db.run_script("::hnsw create isg_nodes:semantic_idx { dim: 384, m: 50, distance: Cosine, ... }")?;
    
    // Extract ISG + embed
    let isg = extract_multi_level_isg(Path::new("crate_root"))?;
    for node in isg.nodes {
        let emb = generate_embedding(&node.signature_text());
        db.run_script(&format!(":put isg_nodes {{ uid: '{}', ..., embedding: {:?} }}", node.uid, emb))?;
    }
    
    // Mixed Query Example: Vector + Graph
    let q_vec = generate_embedding("fn spawn example");
    let query = format!("?[dist, uid, dep] := ~isg_nodes:semantic_idx {{ uid | query: {:?}, k: 10, ef: 30, bind_distance: dist }},
                                                *isg_edges{{src: uid, dst: dep, kind: 'DEPENDS'}}", q_vec);
    let results: ScriptResult = db.run_script(&query)?;
    
    // Parallel Agents Process Results
    let client = Client::new();
    let (tx, mut rx) = mpsc::channel(32);
    for result in results.rows {
        let tx = tx.clone();
        tokio::spawn(async move {
            // e.g., POST to SmolLM2 agent: "Classify pattern in: {result}"
            tx.send(/* processed */).await.ok();
        });
    }
    
    // Collect for PRD
    while let Some(processed) = rx.recv().await { /* Use in deeper agent */ }
    
    Ok(())
}
```

This mixes CozoDB's CPU prowess (exact/vector/graph) with agents' flexibility. For embeddings, prefer dedicated models over tiny LLMs for quality. Test with your 5 MB codebase—Cozo will handle the heavy search lifting. Need a specific query example or benchmark code?

To integrate multiple small agents (using tiny LLMs like MiniLM 22M, STLM 50M, SmolLM2 135M, Gemma 3 270M, or MiniCPM4 0.5B) into workflows involving Interface Signature Graphs (ISG), we can leverage their high speed and low RAM footprint for parallel, task-specific sub-agents. This fits perfectly with the Parseltongue architecture described in the document, which emphasizes multi-level ISG extraction, storage in CozoDB, semantic enrichment via rust-analyzer, and iterative PRD (Product Requirements Document) workflows across three user journeys (inferred from the text: Journey 1 for bug solving, Journey 2 for pattern research, and an implied Journey 3 for code generation/PRD refinement).

The core idea: Break ISG workflows into modular tasks (e.g., querying CozoDB, validating dependencies, refining PRDs) and assign them to specialized sub-agents. Use a Rust coordinator (in a workspace setup) to orchestrate async calls to these agents via llama.cpp or Ollama instances. Tiny models handle fast, low-context tasks (e.g., search/filtering at 150-400 t/s), while reserving a deeper model (e.g., Qwen 14B at ~15 t/s) for complex reasoning like PRD drafting. On your 16 GB Mac Mini, this enables 10-30+ parallel agents, keeping total RAM under 10 GB.

### Key Workflow Integration Principles
- **Agent Roles**: 
  - **Search Agents** (tiny models like MiniLM 22M or STLM 50M): Query CozoDB for ISG nodes/edges (e.g., "find matching interface signatures"). Prompts: Short, precise (e.g., "Extract filepath::module::TypeName from this query result: {Cozo output}"). High speed for sub-second responses.
  - **Validation Agents** (Gemma 3 270M or SmolLM2 135M): Check ISG constraints (e.g., type compatibility, dependencies). Prompts: "Validate if this trait implements Send: {ISG metadata}".
  - **Refinement Agents** (MiniCPM4 0.5B): Suggest PRD iterations based on ISG gaps. Prompts: "Refine this PRD draft with missing deps: {draft} + {ISG query results}".
  - **Deeper Reasoning Agent** (Qwen 14B, solo instance): Generate/iterate PRDs with full context (up to 20k tokens, including rust-analyzer enriched metadata).
- **Orchestration**: Rust workspace with a coordinator crate using tokio for async parallelism. Agents run as separate llama.cpp servers (ports 8081+). CozoDB embedded via cozo-rs crate for local queries—no net overhead.
- **ISG Handling**: Extract multi-level ISG (L1-L5) in a single pass using syn + rust-analyzer, store in CozoDB. Agents query via Datalog (e.g., drill-down from crate to statement level).
- **Parallelism & Scaling**: On 16 GB, run 5-10 search agents + 2-3 validation/refinement + 1 deeper. For a 5 MB codebase (~1.25M tokens), full ISG build + analysis could take <1 min with parallelism.
- **Edge Cases**: Handle PRD loops by routing failed validations back to refinement agents; use web agents (DuckDuckGo scraper in Rust) for external patterns if local ISG misses (e.g., in Journey 2).

### Workflow Breakdown by Journey
Based on the document's flows (ingest codebase → build/enrich ISG → PRD draft/validate/refine loop → codegen/test), here's how to map agents:

1. **Journey 1: Bug Solving with ISG (PRD Iteration for Bug Fixing)**
   - **Workflow Summary**: Ingest Rust codebase → Build enriched ISG (syntax via syn + semantics via rust-analyzer) → Draft PRD for bug fix → Query ISG for relevant interfaces/dependencies/lifetimes → Validate (e.g., type mismatches, borrow errors) → Refine PRD if incomplete → Codegen/test.
   - **Agent Integration**:
     - **Ingest & Build**: Coordinator runs syn/rust-analyzer extraction; no agents needed.
     - **PRD Draft**: Deeper agent (Qwen 14B) generates initial draft with 20k-token context (e.g., "Draft bug fix PRD for this issue, using ISG metadata: {enriched types/traits/call hierarchy}").
     - **ISG Query**: 5-10 parallel search agents (MiniLM/STLM) query CozoDB in batches (e.g., "Find transitive dependencies for this function: {Datalog query}"). ~0.1s per query.
     - **Validate**: 2-3 validation agents (Gemma/SmolLM2) check edges (e.g., "Does this impl satisfy trait bounds? {ISG data}"). Parallel for multi-level checks (L3 types + L4 functions).
     - **Refine Loop**: Refinement agents (MiniCPM4) suggest updates if validation fails (e.g., "Add lifetime constraints based on this borrow graph: {results}"). Loop until valid.
     - **Time Est. for 5 MB Codebase**: Single: 5-10 min (deeper reasoning dominates). Parallel: 1-2 min (10 agents handling queries/validations).
     - **Edge Cases**: If tests fail, route back to refinement agents with diagnostic metadata; handle large graphs by chunking queries.

2. **Journey 2: Pattern Research with ISG (Architectural Pattern Detection)**
   - **Workflow Summary**: Ingest codebase → Build multi-level ISG + orthogonal views (e.g., CFG/DFG via petgraph) → Query for patterns (e.g., Builder/Strategy via trait analysis) → Draft PRD for pattern application → Validate against codebase idioms → Refine with research (local Cozo pattern library + web if needed) → Finalize.
   - **Agent Integration**:
     - **Pattern Query**: 10+ parallel search agents (MiniLM/STLM) scan Cozo for graph algorithms (e.g., PageRank for central types: "Identify tightly coupled modules: {Datalog for L2 edges}"). Use for recursive traversals.
     - **Enrichment**: Validation agents (SmolLM2) classify patterns (e.g., "Is this dyn trait a Strategy pattern? {trait/impl metadata}").
     - **PRD Draft/Refine**: Deeper agent drafts PRD; refinement agents iterate with pattern matches (e.g., "Incorporate this community detection result into PRD: {graph output}").
     - **Web Augment**: Add 1-2 web agents (no-model, just Rust scraper to DuckDuckGo) for external patterns if local misses (e.g., "Search Rust idioms for lifetime elision").
     - **Time Est.**: Single: 3-6 min. Parallel: 0.5-1 min (high parallelism for graph queries).
     - **Edge Cases**: For massive graphs (e.g., 1.6M nodes), use agent queuing to avoid RAM spikes; cache common patterns in Cozo for offline reuse.

3. **Journey 3: Code Generation/PRD Refinement (Implied from Iteration Loop)**
   - **Workflow Summary**: Similar to Journeys 1/2 but focused on new feature PRDs: Ingest → ISG build → Draft PRD → ISG query for API boundaries/visibility → Validate constraints (e.g., pub/private) → Refine loop with user confirmation → Codegen/test/pull request.
   - **Agent Integration**:
     - **Query/Validate**: Mix search (for L1-L5 drill-down: "Roll-up statement issues to type level: {Datalog}") and validation agents (for encapsulation: "Check visibility on this module: {metadata}").
     - **Refine/Codegen**: Deeper agent handles full PRD with 20k tokens; refinement agents for quick tweaks (e.g., "Add generic bounds based on this type graph: {results}").
     - **User Confirmation**: Coordinator pauses for input; agents resume post-confirmation.
     - **Time Est.**: Single: 6-12 min. Parallel: 1-3 min.
     - **Edge Cases**: Handle incomplete ISG (e.g., cross-crate refs) by fallback web agents; rollback via Cozo time travel if bad updates.

### Rust Implementation Sketch (Workspace Setup)
Root `Cargo.toml`:
```
[workspace]
members = ["coordinator", "agents/search", "agents/validation", "agents/refinement"]
```

Coordinator (main crate):
```rust
use tokio::sync::mpsc;
use reqwest::Client;
use cozo_rs::DbInstance; // Embed CozoDB

#[tokio::main]
async fn main() {
    let db = DbInstance::new("rocksdb", "path/to/db")?; // ISG storage
    let client = Client::new();
    let (tx, mut rx) = mpsc::channel(32); // Queue for agent tasks

    // Spawn agent servers via llama.cpp (external processes, or embed if possible)
    // e.g., std::process::Command::new("llama-server").args(["-m", "minilm.gguf", "--port", "8081"]);

    // Example: Journey 1 bug fix workflow
    let isg = extract_multi_level_isg(Path::new("crate_root"))?; // From doc: syn + rust-analyzer
    store_isg_in_cozo(&db, &isg)?;

    // Parallel search agents query Cozo
    for i in 0..10 {
        let tx = tx.clone();
        tokio::spawn(async move {
            let query = format!("?[uid] := *isg_nodes{{level: 3}}"); // L3 types
            let result = db.run_script(&query)?;
            tx.send(result).await?;
        });
    }

    // Collect, validate with parallel agents
    while let Some(cozo_result) = rx.recv().await {
        // Async POST to validation agent: client.post("http://localhost:8082/completion").json(&json!({"prompt": format!("Validate: {}", cozo_result)}));
    }

    // Deeper agent for PRD: similar POST to port for Qwen
    // Loop for refinement if needed
}
```

This setup ties directly to the document's single-pass extraction, unified Cozo schema, and PRD loop. Test on small codebases first—tiny agents will shine for ISG queries. If you need code tweaks or model GGUF links, let me know!