# Pensieve Evolution: Multi-Modal Intelligence Platform

## Core Thesis
Transform Pensieve from a content ingester into a structure-aware analysis platform that handles code (Rust/non-Rust) and documents (PDF/MD/TXT) through a pluggable Content Processor engine backed by CozoDB's graph model.

---

## Strategic Architecture

### The Fundamental Mutation
**From:** Content-agnostic text blob ingester  
**To:** Language-aware structure analyzer with semantic understanding

**Key Insight:** Make Content Processor pluggable—swap parsers based on file type while maintaining unified workflow: **ingest → process → store → generate tasks**

---

## Three Use Cases (Unified Platform)

### 1. Non-Rust Code: "The Generalist Soil"
**Goal:** Map architectural DNA across JS/Python/etc.

**Implementation:**
- **Parser:** tree-sitter for universal AST generation
- **Extraction:** Function signatures, class definitions, import/export statements
- **Schema:** `file_path`, `function_name`, `signature`, `dependencies`, `code_block`
- **AI Tasks:** "Find functions with >5 arguments", "Identify callback chains via dependency graphs"

### 2. Rust Code: "The Rich, Native Soil"
**Goal:** Maximum fidelity + safety analysis beyond general parsers

**Implementation:**
- **Parsers:** 
  - `syn` for perfect round-trippable syntax parsing
  - `rust-analyzer` libraries for semantic depth (HIR, types, traits, borrow-checker)
- **Schema:** `is_async`, `trait_impls`, `lifetime_annotations`, `borrow_risks`, `test_coverage_gaps`
- **AI Tasks:** "Refactor to reduce lifetime complexity", "Identify data races via Send/Sync traits"

### 3. Documents: "The Fertile Research Soil"
**Goal:** Knowledge management system / "second brain"

**Implementation:**
- **Processor:** Extract text from PDFs, identify sections via MD headers/NLP, extract citations
- **Schema:** Knowledge graph with `documents`, `claims`/`sections`, `citations` + relationships
- **AI Tasks:** "Assess argument strength via claims + citations", "Find thematic overlaps between documents"

---

## Document Chunking Strategy (Use Case 3 Deep Dive)

### Problem Statement
How to break down large documents (PDF/MD/TXT) for effective retrieval and analysis?

### Solution: Two-Pass Structure-First Pipeline

#### What Good Chunking Looks Like
1. **Structure-aware:** Respect headings, code fences, lists, tables, figures, footnotes, citations
2. **Token-budgeted:** Target 400-800 tokens/chunk with 10-15% overlap
3. **Citation-aware:** Keep sentences + supporting citations together
4. **Multi-granular:** Maintain section/paragraph/sentence hierarchy
5. **Stable:** Deterministic IDs via path + content hash

#### Recommended Defaults
| Setting | Default | Notes |
|---------|---------|-------|
| Token estimator | tiktoken-like | Approximate tokens per sentence/paragraph |
| Target tokens/chunk | 600 | Range 400-800 works well |
| Min tokens/chunk | 300 | Avoids overly small chunks |
| Overlap tokens | 80 | Or 10-15% of target |
| Hard boundaries | Headings, code fences, tables, figures, page headers/footers | Never cross |
| Atomic units | List items, table+caption, figure+caption, footnote group | Keep intact |
| Fallback segmentation | Lexical cohesion (TextTiling-like) | For flat TXT/PDF-without-structure |

---

## End-to-End Chunking Pipeline

### Step 1: Normalize and Parse
**Markdown:**
- Parse headings, fenced code blocks, lists, tables

**PDF:**
- Extract text with layout features (font size, bold/italic, x/y positions, page numbers)
- Reconstruct paragraphs by merging lines with compatible font/width/spacing
- Detect lists and captions

**Plain Text:**
- Split on blank lines
- Infer headings via patterns (all-caps lines, underlines `====`, numeric/roman prefixes)

### Step 2: Build Section Tree
- Produce hierarchy: `document → sections → subsections → blocks`
- Assign stable path: `["2", "1", "Background"]` with ordinal numbering

### Step 3: Citation and Asset Linking
- **Detect citations:** `[1]`, `(Author, 2020)`, DOI/arXiv/URL, reference labels
- Attach citations to sentences/blocks where they appear
- Bind figures/tables with captions as single atomic blocks
- Link "Figure X" mentions back to asset

### Step 4: Pack into Retrieval Chunks (Budget + Overlap)
**Within each section subtree:**
1. Accumulate blocks until near target tokens
2. If next block would overflow:
   - Close chunk
   - Start new chunk with overlap window from tail of previous chunk
3. **Never split atomic blocks** (code fences, tables, figures, single list item)
4. If single block exceeds budget:
   - Split at sentence boundaries
   - Keep citation sentences together
   - Provide internal overlap within that block

### Step 5: Fallback for Low-Structure Text (TextTiling-like)
**When structure is weak:**
1. Compute lexical cohesion over sliding windows of sentences
2. Boundary score at position i:
   ```
   b(i) = 1 - cos(v_L(i), v_R(i))
   ```
3. Place boundaries at top-k local maxima of b(i) subject to minimum segment size
4. Pack segments using same token-budget rules

### Step 6: Stable IDs and Versioning
**Segment ID:**
```
seg_id = hash(doc_id + path + ordinal + normalized_text_short_hash)
```

**Chunk ID:**
```
chunk_id = hash(doc_id + section_path + chunk_index + concat(seg_ids))
```

Store both raw text and normalized text hash; recompute deltas on re-ingest to avoid churn.

---

## CozoDB Data Model (Graph-First)

### Entities
```
document(doc_id, title, type, source_uri, version_hash, meta_json)
segment(seg_id, doc_id, type, section_path, order_in_parent, text, token_count, content_hash, meta_json)
chunk(chunk_id, doc_id, section_path, start_seg_order, end_seg_order, text, token_count, meta_json)
citation(cite_id, doc_id, raw, normalized_key, meta_json)
asset(asset_id, doc_id, kind, caption, meta_json)
```

### Edges
```
contains(parent_id, child_id, kind, order)     // document→segment, section→subsection, chunk→segment
next(prev_id, next_id, kind)                   // linear traversal within siblings
refers_to(holder_id, target_id, role)          // segment/chunk → citation/asset
anchors(doc_id, section_path, seg_id)          // quick lookup for path-to-segment
```

### Critical Indexes
- By `(doc_id, section_path, order_in_parent)`
- By `content_hash` (for idempotent upserts)
- By `chunk_id`, `seg_id`
- By citation `normalized_key`

### Example: Writing Chunks and Edges
```cozo
// Upsert a segment
input: [
    {seg_id: "seg_...", doc_id: "doc_...", type: "paragraph", 
     section_path: ["2","1"], order_in_parent: 3, text: "...", 
     token_count: 142, content_hash: "sha256:...", meta_json: {...}}
]
:put segment {seg_id, doc_id, type, section_path, order_in_parent, text, token_count, content_hash, meta_json}

// Chunk referencing its member segments
input: [
    {chunk_id: "chk_...", doc_id: "doc_...", section_path: ["2","1"], 
     start_seg_order: 1, end_seg_order: 6, text: "...", 
     token_count: 605, meta_json: {...}}
]
:put chunk {chunk_id, doc_id, section_path, start_seg_order, end_seg_order, text, token_count, meta_json}

// Edges
input: [
    {parent_id: "chk_...", child_id: "seg_a", kind: "chunk_contains", order: 1},
    {parent_id: "chk_...", child_id: "seg_b", kind: "chunk_contains", order: 2}
]
:put contains {parent_id, child_id, kind, order}
```

---

## Packing Algorithm (Practical Implementation)

```python
def build_chunks(blocks, target=600, min_tokens=300, overlap=80):
    chunks = []
    cur, cur_tokens = [], 0

    for b in blocks:
        b_tokens = tokens(b.text)

        # Never split atomic blocks
        if b_tokens > target:
            if cur_tokens >= min_tokens:
                chunks.append(pack(cur))
                cur, cur_tokens = [], 0
            for sub in split_block_by_sentence(b, target, overlap):
                chunks.append(pack([sub]))
            continue

        if cur_tokens + b_tokens > target:
            # close current with overlap
            if cur_tokens >= min_tokens:
                chunks.append(pack(cur))
                cur = tail_with_overlap(cur, overlap)
                cur_tokens = tokens_concat(cur)

        cur.append(b)
        cur_tokens += b_tokens

    if cur:
        chunks.append(pack(cur))

    return chunks
```

**Key Implementation Notes:**
- `split_block_by_sentence` must keep citation-bearing sentences with support sentences
- `tail_with_overlap` should reconstruct overlap window by sentences, not characters (avoid mid-sentence splits)

---

## Format-Specific Handling

### Markdown (Best Case)
- **Hard boundaries:** headings, fenced code, tables
- **Atomic units:** single list item with sublist, code fence, table, figure block (image + caption)
- **Advantage:** Section tree comes free from headings; then pack

### PDF (Layout-Aware)
- Merge lines to paragraphs using font/indent/line-gap heuristics
- Detect lists (bullets/numbers), captions (small font near image/table)
- **Hard boundaries:** page headers/footers, big font jumps, captioned assets
- If headings unclear, apply lexical-cohesion fallback within pages/regions

### Plain Text
- **Paragraphs:** blank-line blocks
- **Headings via patterns:** all-caps, underline with `====` or `----`, numeric/roman prefixes
- If weak signal, skip to lexical-cohesion fallback

---

## Lexical-Cohesion Fallback (TextTiling-like)

**When to use:** Low-structure documents where heading detection fails

**Algorithm:**
1. Represent each sentence window by term vector (tf-idf or simple tf)
2. Boundary score at position i:
   ```
   b(i) = 1 - cos(v_L(i), v_R(i))
   ```
3. Find local maxima above z-score threshold; enforce min segment length
4. After boundaries identified, pack segments with same budget/overlap rules

---

## Query Patterns in Cozo

### Common Queries You'll Need

**Get all chunks under a section path:**
```
From anchors(section_path) → segments → chunks referencing those segments
```

**Walk neighbors for context stitching:**
```
next(chunk_i) → chunk_{i+1} to show expanded context
```

**Citation back-links:**
```
refers_to(segment/chunk → citation) and reverse to find all places citing X
```

**Stable re-chunking:**
```
Use content_hash and section_path to detect unchanged segments
Don't rebuild edges or embeddings for unchanged segments
```

---

## Quality Checks (Fast Heuristics)

### Metrics
1. **Length score:** within `[min, target * 1.2]`
2. **Structure score:** no split through hard boundaries; atomic blocks preserved
3. **Cohesion score:** cosine similarity between first and last 3 sentences above threshold
4. **Orphan check:** avoid chunks with only code/table and no explanatory text (unless necessary)

### Overall Quality Score
```
score = w1·s_len + w2·s_structure + w3·s_cohesion + w4·(1 - s_orphan)
```

---

## Python Implementation Libraries

### Core Libraries by Function
| Need | Libraries | Why/Notes |
|------|-----------|-----------|
| PDF parsing with structure | PyMuPDF (fitz), pdfplumber, unstructured | PyMuPDF is fast and layout-aware; pdfplumber excels at tables; unstructured returns typed elements (Title, NarrativeText, Table, ListItem, Caption) |
| Markdown parsing | markdown-it-py (+ mdit-py-plugins), mistune, mdformat | Token/AST-level access to headings, code fences, lists, tables |
| Plain text heuristics | regex, NLTK, spaCy | Paragraphs by blank lines; headings by patterns; sentence segmentation |
| Sentence segmentation | spaCy (en_core_web_sm), NLTK sent_tokenize | Avoid mid-sentence splits and split oversized blocks |
| Token estimation | tiktoken, HuggingFace tokenizers | Token-budgeted packing and overlap |
| Lexical-cohesion fallback | NLTK TextTilingTokenizer | Classic topic-boundary detector for low-structure text |
| Citation extraction/normalization | GROBID (service, with grobid-client-python), manubot.cite, regex | Robust bibliography/citation parsing (GROBID) or light DOI/URL normalization |
| Tables | camelot, tabula-py, pdfplumber | Extract tables as atomic blocks |
| OCR for scanned PDFs | pytesseract, PaddleOCR, docTR | Use when PDF lacks real text |
| Semantic segmentation (optional) | LlamaIndex SemanticSplitterNodeParser, LangChain RecursiveCharacterTextSplitter/MarkdownHeaderTextSplitter, Haystack PreProcessor | Embedding-based or structure-aware chunkers |

### Drop-in Splitters (Higher-Level APIs)
**LangChain text splitters:**
- `MarkdownHeaderTextSplitter`: honors headers for MD
- `RecursiveCharacterTextSplitter`: strong baseline with custom separators and overlap
- `TokenTextSplitter`: respects a tokenizer budget

**LlamaIndex:**
- `SentenceSplitter` and `SemanticSplitterNodeParser`: embedding-based semantic boundaries and overlap

**Haystack:**
- `PreProcessor`: word/char/token length and overlap; can preserve sentence boundaries and headers

### Recommended Starting Points
- **PDF-heavy corpus:** Unstructured + PyMuPDF + TextTiling fallback + tiktoken; optionally GROBID for proper citation parsing
- **Markdown-heavy corpus:** markdown-it-py for AST + LangChain's MarkdownHeaderTextSplitter + tiktoken
- **Mixed corpus with limited engineering time:** Unstructured + LangChain RecursiveCharacterTextSplitter + tiktoken

---

## Product Thinking: Utilities for Senior Tech Professionals

### Strategic Utilities (Shreyas Doshi Framework)

| Utility | Wedge (10x moment) | Who cares | MVP surface |
|---------|-------------------|-----------|-------------|
| **Paper triage autopilot** | 2-minute decision: read, skim, or skip | Senior ICs, founders | Raycast/CLI + browser extension |
| **Claim→Evidence graph** | Extracts claims; links to citations and figures | Researchers, PMs, staff engineers | Local app + graph view |
| **SOTA comparator** | Side-by-side matrix of baselines, datasets, metrics, deltas, licenses | Builders scouting tech | Web dashboard/CLI |
| **Protocol cards** | One-page "how-to": assumptions, inputs, steps, pitfalls, compute budget | Engineers prototyping | Markdown generator + VS Code |
| **Product lens on papers** | Maps research to problems, ICPs, "why now", wedge hypotheses | PMs/founders | PRD template generator |
| **Watchlists + triggers** | Alerts when claim gets replicated, retracted, or outperformed | Decision-makers | Notifications + digest |
| **Replicability radar** | Flags shaky evals, missing code/data, incompatible licenses | Eng leads/legal | Report in CI/CLI |
| **Idea recombinator** | Cross-pollinates techniques across domains | Innovators | Interactive notebook/app |
| **Paper→PRD/RFC generator** | Turns claim graph + protocol cards into structured PRD/RFC | PMs/tech leads | One-command generator |
| **Reading pipeline & TTI metrics** | Measures "Time-To-Insight", shows where time is spent | Everyone | Dashboard + weekly review |

**Key Insight:** Start with one sharp wedge (Paper→PRD generator + SOTA comparator). Ship fast value, then layer in graph and watchlists.

**North-star metric:** Time-To-Insight (TTI) per paper; aim for sub-2 minutes for triage.

### OSS Product Surfaces That Compound
- CLI and Raycast command for instant triage
- Browser extension for one-click capture of papers, PDFs, arXiv pages
- VS Code extension to insert protocol cards and PRD skeletons
- Lightweight local web UI for claim/evidence graph
- GitHub Action to run replicability/risk checks on literature lists

---

## Storage Architecture Decision

### CozoDB vs Alternatives

| Pattern | When to pick | Pros | Cons |
|---------|--------------|------|------|
| **CozoDB (local-first graph)** | Individual/small-team OSS, offline-first, rich relationships | Simple embed, expressive graph/Datalog queries | Smaller ecosystem; pair with vector index |
| **Postgres + pgvector** | Servered, multi-tenant SaaS, broad tooling | Ubiquitous, transactions, SQL, vectors in one place | Graph queries clunkier; model edges manually |
| **SQLite + FTS5 + local vector** | Minimal deps, pure local portable DB | Zero infra, great DX for OSS | Limited concurrency; graph queries manual |
| **Neo4j/JanusGraph + vector** | Heavy graph analytics at scale | Mature graph tooling, visualization | Infra overhead, not local-first friendly |
| **All-in-one vector DB** | Retrieval-first, simple pipeline | Easy ANN, filters, ops tooling | Weaker native graph semantics |

### Recommendation for OSS Local-First
**Primary:** CozoDB as source of truth for knowledge graph (documents, sections, claims, citations, assets, edges)

**Retrieval layer (pluggable):**
- Start with local vector index (LanceDB or hnswlib) for embeddings
- Add FTS for keyword search (Cozo text index or SQLite FTS for hybrid)

**Future scaling:** If going multi-tenant/servered, add Postgres read model (ETL from Cozo)—don't abandon the graph, mirror it.

**Why this fits Shreyas-style product thinking:**
- Sharp wedge that removes grind (paper triage → PRD/RFC)
- Compounding advantage: every captured claim/citation enriches future triage
- "Why now": explosion of papers + LLMs enable structure extraction + local-first graph makes it private, fast, OSS-friendly

---

## User Journeys (Shreyas Framework)

### Journey Overview

| Journey | Trigger | Key question | Output | Surface | TTI |
|---------|---------|--------------|--------|---------|-----|
| **2-minute triage** | New paper/URL | Is this worth my time now? | One-pager summary | Raycast/CLI | ≤ 2 min |
| **Deep skim** | I decided to skim | What are main ideas? | Section map + claim highlights | Web/VS Code | ≤ 10 min |
| **Claim→Evidence** | I'm validating | Which claims, evidence support what? | Claim graph with citations | Web graph view | ≤ 15 min |
| **SOTA comparator** | I need to benchmark | How does this compare? | Comparison table + deltas | Web/CLI | ≤ 5 min |
| **Paper→PRD/RFC** | I see product potential | Can this be a product wedge? | PRD/RFC draft | VS Code/CLI | ≤ 10 min |
| **Protocol card** | I want to try it | What's the minimal recipe? | Method card: inputs, steps, pitfalls | VS Code/Markdown | ≤ 10 min |
| **Idea recombinator** | I'm exploring | What combos unlock value? | Concept briefs + prompts | Web/Notebook | ≤ 15 min |
| **Watchlist + alerts** | I follow a topic | What changed that matters? | Digest with upgrades/retractions | Notifications | < 1 min |
| **Literature review** | I need coherent narrative | What's the storyline? | Structured review | Web/Markdown | ≤ 60 min |
| **Replication radar** | I'm evaluating feasibility | What breaks in real world? | Risk report: eval gaps, licenses | CLI/CI | ≤ 5 min |

### Flagship Journeys

#### 1. 2-Minute Triage (Sharp Wedge)
**Trigger:** Paste arXiv/URL/PDF into Raycast or CLI

**Steps:**
1. Grab metadata, abstract, section headers
2. Pull top-3 claims and strongest citations/figures
3. Detect code/data availability and license; flag red flags

**Output:** One-pager: what it is, why it matters, read/skim/skip decision

**Moment of truth:** Confident decision in ≤ 2 minutes

**Metric:** TTI_triage, % of triaged items leading to next action

#### 2. Paper→PRD/RFC
**Trigger:** Triage says "read" or "product potential"

**Steps:**
1. Use claim graph to auto-fill ICP, problem, assumptions, differentiation
2. Pull SOTA deltas and license constraints into risks
3. Generate PRD sections: overview, user problem, why-now, wedge hypothesis, success metrics, risks, milestones

**Output:** PRD.md or RFC.md committed to repo

**Moment of truth:** "I could share this PRD with my team" with minimal editing

**Metric:** TTI_PRD, PRD acceptance rate, follow-up issues created

#### 3. SOTA Comparator
**Trigger:** Candidate approaches need benchmark view

**Steps:**
1. Query graph for methods touching your dataset/metric
2. Build table with baselines, metrics, confidence intervals, compute cost, license, reproduction status
3. Highlight meaningful deltas and suspect comparisons

**Output:** Comparator table with callouts and links

**Moment of truth:** Clear "we should test X first" decision

**Metric:** TTI_compare, # of decisions made without further digging

#### 4. Watchlist + Alerts (Compounding Loop)
**Trigger:** Follow topics/methods/repos

**Steps:**
1. Monitor incoming papers; diff claims/metrics vs. watchlist
2. Alert on new SOTA, retractions, code releases, license changes
3. Tie alerts back to PRDs and experiments

**Output:** Weekly digest or instant alert with "what changed" and "what to do"

**Moment of truth:** Catch meaningful shifts early; take action within the week

**Metric:** Alert usefulness score, actions per alert, reduced time-to-react

### MVP Surfaces

**Raycast/CLI:**
```bash
pensieve triage <url_or_pdf>
pensieve prd --from <doc_id> --out PRD.md
pensieve compare --topic "document VLM" --dataset "DocVQA"
pensieve watchlist add --query "diffusion inversion" --mode weekly
```

**VS Code:** Insert protocol cards and PRD skeletons; inline claim/citation previews

**Web UI:** Claim graph view, SOTA comparator, literature review composer

### Metrics to Instrument

**Time-To-Insight:** TTI_triage, TTI_PRD, TTI_compare

**Adoption:** % triage leading to PRD/protocol, weekly active queries

**Quality:** Claim correctness rate, citation integrity, PRD edit distance

**Compounding:** Reuse of claims across artifacts, actions per alert

---

## Graph Traversal Mental Model

### Core Concept
**Document as tree:**
```
document → section → subsection → block (paragraph/list/table/code) → sentence
```

**Chunks:** Sliding windows over contiguous blocks/sentences sized by token budget with overlap

**Graph movement:**
- **Up/down:** contains edges
- **Left/right:** next edges
- **Across:** refers_to edges for citations/figures

**Retrieval pattern:** search (keyword/embedding) → land on chunk → traverse neighbors for context

### Two-Tier Segmentation

#### 1. Structural Segmentation (Always First)
**Hard boundaries (never split):** Headings, fenced code, tables, figures+captions, footnotes, page headers/footers

**Atomic units (keep intact):** One list item (including sublist), one table with caption, one code fence

**Build section tree** from headings/layout (MD/PDF) or heuristics (TXT)

#### 2. Token-Budgeted Packing
- Accumulate blocks until approaching target token budget
- Add 10–15% overlap
- If single block exceeds budget, split by sentence with internal overlap (keep citation sentences together)

**Fallbacks:** Lexical cohesion (TextTiling-like) or semantic splitter (embedding similarity)

### Minimal Schema

**Entities:**
```
document(doc_id, title, source_uri, version_hash, meta)
section(sec_id, doc_id, path, title, order, meta)
segment(seg_id, sec_id, kind, order, text, token_count, hash, meta)
chunk(chunk_id, doc_id, section_path, start_order, end_order, text, token_count, meta)
citation(cite_id, doc_id, norm_key, raw, meta)
asset(asset_id, doc_id, kind, caption, meta)
```

**Edges:**
```
contains(parent_id, child_id, kind, order)      // doc→section, section→segment, chunk→segment
next(prev_id, next_id, kind)                    // sibling order
refers_to(holder_id, target_id, role)           // segment/chunk → citation/asset
anchors(doc_id, section_path, node_id)          // fast path→node lookup
```

**Stable IDs:**
```
seg_id = hash(doc_id + section_path + order + normalized_text_short_hash)
chunk_id = hash(doc_id + section_path + chunk_index + concat(seg_ids))
```

### Common Traversal Patterns

**1. Jump to section's chunks:**
```sql
SELECT c.* FROM chunk c
WHERE c.doc_id = $doc AND c.section_path = $path
ORDER BY c.start_order
```

**2. Expand context (neighbors):**
```sql
-- Next chunk
SELECT c2.* FROM next n JOIN chunk c2 ON c2.chunk_id = n.next_id
WHERE n.prev_id = $chunk

-- Previous chunk
SELECT c1.* FROM next n JOIN chunk c1 ON c1.chunk_id = n.prev_id
WHERE n.next_id = $chunk
```

**3. Get segments for chunk:**
```sql
SELECT s.* FROM contains rel JOIN segment s ON s.seg_id = rel.child_id
WHERE rel.parent_id = $chunk ORDER BY rel.order
```

**4. Follow citations:**
```sql
SELECT cit.* FROM refers_to r JOIN citation cit ON cit.cite_id = r.target_id
WHERE r.holder_id = $chunk AND r.role = 'citation'
```

**5. Search → traverse:**
```python
def retrieve_with_context(query, k=6, neighbors=1):
    hits = vector_store.search(query, top_k=k)
    results = []
    for cid, score in hits:
        center = db.get_chunk(cid)
        left = db.get_prev_chunk(cid, steps=neighbors)
        right = db.get_next_chunk(cid, steps=neighbors)
        bundle = [*left, center, *right]
        results.append({"score": score, "chunks": bundle})
    return results
```

### Why Graph DB?

**Yes—for this use case.** You have hierarchy, sequence, and cross-references.

**Pattern:**
- **CozoDB** = source of truth for structure and edges
- **Vector store** = retrieval index pointing to Cozo IDs
- **Your app** = composes "search → traverse → answer"

### Sanity Checks
- No chunk crosses hard boundaries
- Chunk length between min and ~1.2×target
- Overlap exists between consecutive chunks
- Segments referenced by chunk are contiguous in order
- Hashes stable; unchanged segments/chunks keep IDs across re-ingest

---

## Line-First Chunking Approach

### Simple, Configurable Alternative

**Core idea:** Build pages by lines, never break mid-paragraph or mid-sentence. If paragraph too large, split at sentence boundaries.

### Recommended Defaults

| Setting | Default | Notes |
|---------|---------|-------|
| max_lines_per_page | 500 | Hard cap for each page |
| min_lines_per_page | 250 | Avoid tiny pages unless at end |
| overlap_lines | 10 | Carry last N lines into next page |
| prefer_paragraph_boundary | true | Close pages only at paragraph ends when possible |
| respect_code_fences | true | Treat triple-backtick blocks as atomic |
| sentence_splitter | auto | Use spaCy/NLTK if present; else regex fallback |

### Implementation Details

**Core page builder function:**
```python
def build_pages_by_lines(
    lines: List[str],
    max_lines_per_page: int = 500,
    min_lines_per_page: int = 250,
    overlap_lines: int = 10,
    prefer_paragraph_boundary: bool = True,
    respect_code_fences: bool = True,
) -> List[Dict]:
```

**Key features:**
- Returns pages as dicts with: `page_index`, `start_line`, `end_line` (exclusive), `lines`, `note`
- Enforces: no mid-paragraph break unless paragraph itself exceeds `max_lines_per_page`
- If paragraph too large, split at sentence boundaries
- Respects code fences as atomic blocks

**What this gives you:**
- Line-first, page-like chunks capped by chosen line count
- Pages end at paragraph boundaries whenever possible
- If paragraph too big, split at sentence boundaries close to limit
- Optional overlap in lines ensures retrieval robustness

### Storing Pages in CozoDB

**Page as node:**
```
page(page_id, doc_id, start_line, end_line, text, meta)
```

**Edges for order and containment:**
```
contains(doc_id->page_id)
next(page_i->page_{i+1})
```

**Optional segments for finer traversal:**
```
segment(seg_id, doc_id, start_line, end_line, kind)
contains(page_id->segment_id) for segments included in a page
```

**Example inserts:**
```cozo
// Pages
input: [
    {page_id: "p_0001", doc_id: "doc_1", start_line: 0, end_line: 480, text: "...", meta: {overlap_lines: 10}},
    {page_id: "p_0002", doc_id: "doc_1", start_line: 470, end_line: 920, text: "...", meta: {overlap_lines: 10}}
]
:put page {page_id, doc_id, start_line, end_line, text, meta}

// Order
input: [
    {prev_id: "p_0001", next_id: "p_0002", kind: "page_next"}
]
:put next {prev_id, next_id, kind}
```

**Tip:** Keep `page_id` deterministic, e.g., `hash(doc_id + start_line + end_line)` for stable re-ingest.

### Notes and Tradeoffs

- **Lines are coarse measure:** Two files with different line lengths can have very different token counts. If needed, add optional `max_tokens_per_page` check that supersedes line rule
- **PDF-to-text extraction:** Often yields short, wrapped lines. If needed, pre-merge lines into paragraphs before applying line-based pager
- **Lists:** Usually come as separate lines; since they're within one paragraph group bordered by blank lines, they'll stay together

---

## txt-sectumsempra: 1000-IQ Improvements

### Current State
Minimal Rust CLI that splits large text files by size (MB) into parts named `input-part-0..N`. Great baseline for raw splitting, but ripe for "smart chunking" and product polish.

### A) Make Chunking "Smart" by Default

**Paragraph/sentence aware:**
- Prefer paragraph boundaries (blank-line separated)
- If paragraph exceeds budget, split at sentence boundaries

**Configurable budgets:**
- Support bytes, lines, tokens, and approximate tokens

**Overlap windows:**
- Add configurable overlap lines/sentences/tokens between chunks

**Hard-boundary preservation:**
- Never split inside fenced code blocks (` ``` `), tables (simple ASCII), or custom regex-delimited regions

**Deterministic IDs:**
- Compute stable chunk IDs from source path + byte span + short content hash
- Store in manifest

**Multimode strategies:**
- Modes: `bytes | lines | paragraphs | sentences | tokens | mixed` (structure-first, then tokens)

**Huge-paragraph safety:**
- Greedy sentence packing with internal overlap to avoid mid-sentence cuts

### B) Product/UX That Compounds

**Dry runs and previews:**
- Show proposed boundaries, sizes, and summaries before writing

**In-place or streamed:**
- Write to stdout or target folder
- Support piping: `cat big.txt | txt-sectumsempra ...`

**Resumable/Idempotent runs:**
- Skip unchanged outputs by content hash
- Write `manifest.jsonl` for reproducibility

**Join/unite command:**
- `txt-sectumsempra join manifest.jsonl > original.txt` (round-trip confidence)

**Rich naming templates:**
- `--name-template "{basename}-{index:03}-{start}-{end}-{id8}"`

### C) Performance and Scale

**Streaming, zero-copy where possible:**
- Use `BufRead` with large buffers
- Optional `memmap` for read-only inputs

**Fast scans:**
- Use `memchr/memchr2` for newline scanning
- Avoid UTF-8 decoding until needed

**Parallel writes:**
- Buffer chunk decisions
- Write files concurrently (bounded Rayon pool)

**Low memory on TB-scale:**
- Two-pass option: first build line/byte offset index; second pass emit chunks

### D) Reliability and Edge Cases

**Newline normalization and detection:**
- Handle LF/CRLF consistently
- Preserve original by default

**BOM and encoding handling:**
- Detect BOM
- Optionally transcode via `encoding_rs`
- Default to pass-through

**Windows path lengths and permissions:**
- Use canonical paths
- Fallbacks and friendly errors

**Atomic writes:**
- Write to temp files, fsync, then rename

### E) Developer Experience and Testing

**CLI polish:**
- `clap` with clear subcommands
- Rich help; examples
- Config file support (TOML/YAML)

**Observability:**
- Progress bars (`indicatif`)
- `--json-logs` for machine consumption
- Metrics summary at end

**Tests:**
- Property tests (`proptest`) for "split then join equals original"
- Golden tests for fence/paragraph/sentence behaviors
- Fuzz sentence splitter on odd punctuation/Unicode

**Benchmarks:**
- `criterion` for throughput on 1 GB, 10 GB files
- CPU and I/O bound scenarios

### F) Outputs and Metadata

**Write sidecar manifest:**
- One JSONL per chunk: `{chunk_id, src_path, start_byte, end_byte, start_line, end_line, start_sent, end_sent, tokens, hash, created_at}`

**Embedding-ready:**
- Optional `.chunks` directory with text and minimal metadata
- No surprises in shell scripts

**Checksums:**
- Per-chunk SHA-256 + whole-run manifest hash for reproducibility

### G) Integrations (Real Workflows)

**CozoDB export:**
- `--export cozo`: create nodes (document, chunk) and edges (next, contains)
- Store spans and hashes

**Vector index handoff:**
- `--emit-ids file` to align with external embedding/indexing pipelines (Qdrant/LanceDB/pgvector)

**Git-friendly runs:**
- Stable IDs prevent churn in diffs
- Optional deterministic output ordering and naming

### H) Extensibility

**Plugins for boundary detection:**
- Simple trait `BoundaryDetector` with built-ins: Lines, Paragraphs, Sentences, Tokens, RegexRegions

**Heuristics via DSL:**
- `--rules file.kdl` to describe "don't split between pattern A and B" or "close chunk on header regex"

**Token backends:**
- Optional `tiktoken-rs` or `tokenizers` (HuggingFace) for accurate token budgeting
- Fallback to heuristic

### Proposed CLI Surface

```bash
# Basic: split by 50 MB with overlap of 128 lines, prefer paragraph breaks
txt-sectumsempra split input.txt \
    --by bytes --size 50MB \
    --prefer paragraph --overlap 128 \
    --out ./out

# Line-first "pages" capped at 500 lines, avoid mid-sentence splits
txt-sectumsempra split input.txt \
    --by lines --size 500 \
    --avoid mid-sentence --overlap 10

# Structure-first, then token budget with internal sentence overlaps
txt-sectumsempra split input.txt \
    --by mixed --target-tokens 600 --min-tokens 300 --overlap-tokens 80 \
    --hard-boundaries codefence,table --manifest manifest.jsonl

# Dry run with preview
txt-sectumsempra split input.txt --dry-run --preview 5

# Export chunk graph to CozoDB
txt-sectumsempra export cozo manifest.jsonl --db ./graph.cozo
```

### Config File (TOML) Example

```toml
[input]
path = "input.txt"

[strategy]
mode = "mixed"                # bytes|lines|paragraphs|sentences|tokens|mixed
target_tokens = 600
min_tokens = 300
overlap_tokens = 80
prefer_paragraph_boundary = true
avoid_mid_sentence = true
hard_boundaries = ["codefence", "table"]

[output]
dir = "./out"
name_template = "{basename}-{index:03}-{id8}"
manifest = "manifest.jsonl"
atomic_writes = true
```

### Internal Architecture (Clean and Fast)

**Pipeline stages:**
1. **Scanner:** builds byte and line offset index with `memchr` (streaming)
2. **Grouper:** groups paragraphs (blank-line separated) and fenced regions; optional simple sentence index using lightweight splitter
3. **Packer:** greedy packer honoring budget (bytes/lines/tokens) and hard boundaries; adds overlap
4. **Writer:** atomic file writes + manifest writer; optional concurrent writes

**Core traits:**
- `BoundaryDetector`, `Packer`, `Sink` (FileSink, StdoutSink, CozoSink)

**IDs and hashes:**
- `chunk_id = xxhash64(doc_path + start_byte + end_byte + short_sha(text))`
- Store both xxhash (fast) and sha256 (strong)

### CozoDB Export Shape

**Nodes:**
```
document(doc_id, path, size_bytes, hash)
chunk(chunk_id, doc_id, start_byte, end_byte, start_line, end_line, tokens, hash)
```

**Edges:**
```
contains(document -> chunk, order)
next(chunk_i -> chunk_{i+1})
```

Keep deterministic ordering by `start_byte`.

**Example manifest.jsonl row:**
```json
{
    "chunk_id": "chk_9f3a1cde",
    "src_path": "input.txt",
    "start_byte": 1048576,
    "end_byte": 2097151,
    "start_line": 1204,
    "end_line": 2387,
    "tokens": 587,
    "hash": "sha256:3c0f...",
    "created_at": "2025-10-15T10:00:00+05:30"
}
```

### Tests and Benchmarks

**Golden tests:**
- Paragraph/sentence-aware splitting across punctuation, Unicode, CRLF/LF, code fences

**Property tests:**
- `join(split(x)) == x`
- Monotonic spans
- No overlaps except configured window

**Stress tests:**
- 1 GB and 10 GB files, low memory
- Charsets with BOM
- Mixed line endings

**Benchmarks:**
- Throughput (MB/s), peak RSS, chunks/s
- Test both HDD and SSD

### Name Relevance

**txt-sectumsempra:**
- **Pros:** Memorable, fun HP reference, "cuts text"
- **Cons:** Obscure and violent reference; weak SEO/clarity; enterprise users may hesitate; not self-explanatory on CLI listings

**Recommendation:** Keep the codename internally, but use a descriptive binary/package name for adoption and SEO.

**Name alternatives:**

| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| txt-segment | Clear, searchable | Generic | Strong |
| txt-paginator | Implies line/page logic | Less general than "segment" | Good |
| txt-sharder | Tech-y | DB connotation | Good |
| txt-chunker | Descriptive | Common | Fine |
| txt-paginate | Page metaphor | Slightly narrow | Good |
| segmenta | Short, brandable | Not obviously "text" | Consider |
| granulate | Unique | Less literal | Consider |

**Tagline ideas:**
- "Smart, reproducible chunking for absurdly large text files."
- "Page-, paragraph-, and sentence-aware splitting, at streaming speed."

### 1-Week Punch List

**Day 1–2:**
- Refactor into Scanner/Grouper/Packer/Writer
- Add line/byte index and overlap windows
- Manifest writer; deterministic IDs

**Day 3:**
- Paragraph and code-fence preservation
- Sentence-aware split for oversized paragraphs

**Day 4:**
- CLI polish (clap), dry-run preview
- Progress and JSON logs
- Atomic writes

**Day 5:**
- Tests (golden + property)
- Criterion bench
- Docs with examples

**Day 6–7:**
- Cozo export
- Name template
- Basic token budgeting (heuristic now, tokenizers behind feature flag)

---

## Parseltongue: Enriching Interface Signature Graph with rust-analyzer

### Core Enrichment Opportunities

#### 1. Type Information & Trait Relationships (High Impact)

**Enhanced ISG node structure:**
```rust
pub struct EnrichedNode {
    // Original data
    id: NodeId,
    name: String,
    node_type: NodeType,
    
    // New enriched data
    type_signature: Option<TypeSignature>,        // Function signatures, struct fields
    trait_implementations: Vec<TraitImplementation>, // What traits this implements
    implemented_by: Vec<NodeId>,                  // Who implements this trait
    generic_parameters: Vec<GenericParameter>,    // Generic constraints
    associated_types: Vec<AssociatedType>,        // Associated types for traits
}
```

**Why this matters for LLMs:**
- Provides complete type context without needing to parse code
- Shows trait relationships at a glance
- Reveals generic constraints that affect usage

#### 2. Semantic Context from IDE Features (High Impact)

```rust
pub struct SemanticContext {
    documentation: Option<String>,           // Extracted doc comments
    usage_count: usize,                     // How often this is used
    call_relationships: Vec<CallRelationship>, // Who calls whom
    location: FileLocation,                 // Where defined
    visibility: Visibility,                 // Public/private/module visibility
    deprecated: bool,                       // Is this deprecated?
}
```

**Why this matters for LLMs:**
- Provides immediate context about importance and usage patterns
- Shows visibility to understand API boundaries
- Identifies deprecated elements to avoid in new code

#### 3. Diagnostic Information (Medium Impact)

```rust
pub struct DiagnosticInfo {
    errors: Vec<Diagnostic>,      // Type errors, missing implementations
    warnings: Vec<Diagnostic>,    // Unused imports, dead code
    suggestions: Vec<String>,     // Common fixes
}
```

**Why this matters for LLMs:**
- Flags problematic interfaces before use
- Suggests alternatives or fixes
- Helps avoid introducing new issues

#### 4. Macro Expansion Context (Medium Impact)

```rust
pub struct MacroContext {
    is_macro_generated: bool,
    macro_source: Option<String>,      // What macro generated this
    expansion_details: Option<String>,
}
```

**Why this matters for LLMs:**
- Helps understand actual code vs. macro-generated code
- Provides context for debugging macro-related issues
- Shows true structure after preprocessing

### Minimal Implementation Strategy

#### Phase 1: Essential Type Information (Maximum Impact)

Focus on extracting high-value elements:

1. **Type Signatures**
   - Function parameters and return types
   - Struct/enum field types
   - Generic parameters and bounds

2. **Trait Implementations**
   - Which traits a type implements
   - Associated types and constants
   - Trait bounds for generics

```rust
impl RustAnalyzerIntegration {
    pub fn enrich_isg(&self, isg: &mut InterfaceSignatureGraph) {
        for node in isg.nodes() {
            if let Some(type_info) = self.extract_type_info(node) {
                node.type_signature = Some(type_info);
            }
            
            if let Some(trait_info) = self.extract_trait_implementations(node) {
                node.trait_implementations = trait_info;
            }
        }
    }
}
```

#### Phase 2: Semantic Context

Add documentation and usage patterns:

```rust
impl RustAnalyzerIntegration {
    pub fn add_semantic_context(&self, isg: &mut InterfaceSignatureGraph) {
        for node in isg.nodes() {
            node.documentation = self.extract_doc_comments(node);
            node.usage_count = self.count_usage(node);
            node.visibility = self.determine_visibility(node);
        }
    }
}
```

#### Phase 3: Quality Indicators

Add lightweight quality metrics:

```rust
impl RustAnalyzerIntegration {
    pub fn add_quality_indicators(&self, isg: &mut InterfaceSignatureGraph) {
        for node in isg.nodes() {
            node.diagnostics = self.check_diagnostics(node);
            node.complexity = self.assess_complexity(node);
            node.deprecated = self.is_deprecated(node);
        }
    }
}
```

### Benefits for LLM Context Generation

**Complete Type Understanding:**
```json
{
  "name": "User",
  "type": "struct",
  "fields": [
    {"name": "name", "type": "String"},
    {"name": "age", "type": "u32"}
  ],
  "implements": ["Display", "Clone"],
  "doc": "Represents a user with personal information"
}
```

**Usage Context:**
```json
{
  "name": "create_user",
  "usage_count": 12,
  "called_by": ["main", "setup_module"],
  "visibility": "public",
  "doc": "Creates a new user instance"
}
```

**Quality Indicators:**
```json
{
  "name": "old_function",
  "deprecated": true,
  "diagnostics": ["warning: unused parameter"],
  "suggestion": "Consider using new_function instead"
}
```

---

## Parseltongue Product Strategy (Shreyas Doshi Framework)

### TL;DR (The "Shreyas Take")

**Your wedge is not "AI code nav."** Your wedge is **"First-PR-in-hours, not weeks"** for Rust OSS, powered by an interface-first graph and risk-scored simulation. Keep everything else subordinate to that outcome.

**Ship a P0** that proves one thing end-to-end: given a curated Rust repo, a new contributor can go from install to an accepted PR in under 4 hours with >80% acceptance rate. Cut anything that doesn't make that loop reliable.

**Technically,** resist jumping to MIR/trait solver now. Your ISG v1 is enough to power high-signal impact analysis when paired with strict templates, opinionated changes, and compile/test gates. Add cfg/type/trait depth only when it moves acceptance rate or time-to-first-PR.

**Distribution:** Maintainers are the real buyer. Build "Maintainer Mode" first to seed good-first-PRs and get guaranteed acceptances. Contributors come next.

### Positioning and Wedge

**One-line:** "Parseltongue helps Rust devs submit a safe, high-quality first PR in hours by simulating the change at the interface level before touching code."

**What you're not:** A general-purpose IDE agent, or a generic call-graph visualizer. Your outcome is accepted PRs fast.

**Why you win:**
- Interface Signature Graph (ISG) → stable, small, fast, predictable
- Dual-state "future overlay" → risk scoring without churn
- Opinionated, template-driven changes → less variance, higher acceptance

### P0: What to Ship in 6–8 Weeks

Ship this single loop, end-to-end, for 3 curated OSS Rust repos:
- **Candidate repos:** Medium-sized, active maintainers, clear CONTRIBUTING.md, tests run in CI

**P0 scope:**

**CLI:**
- `analyze`: build ISG from repo; index names; produce "opportunity shortlist"
- `propose "<known pattern>"`: return a bounded plan (files, interfaces, tests)
- `implement "<known pattern>"`: scaffold patch from templates, open a branch
- `check`: run cargo fmt/clippy/test locally; summarize violations and fix suggestions
- `submit`: push branch, open PR with structured PR body

**Known patterns (opinionated, high-acceptance):**
- Add rate limiting middleware skeleton to a single endpoint (if HTTP framework present)
- Harden error handling for existing Result-returning functions (add anyhow/thiserror wrappers, propagate or instrument)
- Add missing tests for public functions (template-based)

**Simulation:**
- ISG_current vs ISG_future overlay: changed interfaces list, predicted blast radius (outgoing 'Uses' and 'Calls' edges), predicted test surface
- Risk score = (fanout × publicness × touched-files) with simple thresholds. No MIR/trait solver in P0

**Guardrails:**
- Only Rust, only one framework class per repo cohort (e.g., axum or actix), only interface-level ops (no deep refactors)
- Compile gate + tests gate must pass before submit

**GitHub integration:**
- Branch, PR creation, PR body builder, CI link capture. No auto-merge

**Cut for P0:**
- Full MIR/CFG, trait solver, full cfg-eval. Not needed to prove the wedge
- Multi-language parsing. Focus Rust-only
- WASM visualizations and Playwright UI tests as default pathway. Keep as demo, not on critical path
- Arbitrary free-form "ask" changes. Limit to whitelisted patterns

**P0 acceptance criteria:**
- TTFPR (install → merged PR) p50 ≤ 4 hours across 3 curated repos
- ≥ 80% PR acceptance without major rework
- Simulation "changed interfaces" precision ≥ 90% (predicted vs actual touched interfaces)
- Local compile/test pass rate ≥ 95% pre-submit

### Technical Calls (Ruthlessly Practical)

**Parsing/graph:**
- Keep syn + tree-sitter as you have, but standardize on one hot path. For P0, syn is enough for item discovery; your Visit-based call edge heuristic is fine for confidence scoring, not absolute truth
- Add an O(1) name index: hash map from name → SigHash (you already flagged it; do it now to remove O(n) lookups)

**Dual-state overlay:**
- Implement ISG_future as a delta store over ISG_current (Create/Edit/Delete interface ops). Don't re-materialize the whole graph; apply a view layer

**Risk model v1:**
- Score = public API change? + transitive dependents count + touching files in critical dirs + test coverage presence
- Output: red/yellow/green plus "what changes can I make to move into green" (ex: limit scope to one endpoint)

**Compile/test:**
- Before PR: cargo fmt, clippy (deny=warnings), cargo test. Fail fast with actionable diffs

**Templates:**
- Put project-specific templates in a ".parseltongue/" folder if available, else fall back to your opinionated defaults. This is the hook for maintainers to tune acceptance

**Snapshotting:**
- Your SerializableISG and snapshot save/load are good. Make snapshot part of the loop so "analyze" is instant on second run

**Performance contracts:**
- Keep your current micro-bench claims internally but relax public promises. The contract that matters for P0 is "analyze ≤ 5s, check ≤ 60s, propose ≤ 5s"

### Enrichment Roadmap (When It Moves Core Metrics)

Sequence only when it increases acceptance rate or shortens TTFPR:

1. **cfg awareness** (feature/target toggles) → reduces false positives in blast radius
2. **Basic trait resolution** for method calls on common patterns (inherent vs trait methods) → improves caller/callee precision
3. **Unsafe/safety flags + FFI entry-point mapping** → unlocks security-class PRs
4. **MIR-lite for control-flow hot spots** (panic edges, assert paths) → supports "fix potential panic" templates
5. **Salsa-like on-demand recomputation** if/when graphs grow

### Adoption Loops and GTM

**"Maintainer Mode" (seed the market):**
- Maintainers get a `/parseltongue.yml` with:
  - Approved change patterns and templates
  - Guardrails (disallowed files/areas)
  - PR rubric and labels to auto-apply
- Output: Parseltongue can propose pre-approved changes with high acceptance odds

**"First PR Fridays":**
- Weekly rotation of 3–5 tasks surfaced as "parseltongue-ready" issues. Record conversion

**Distribution:**
- Start as a CLI plus optional Claude Code plugin:
  - Slash commands: `/propose`, `/implement`, `/submit` mapped to your CLI
  - Reuse your commit/PR plugin flows for push + PR

**Pricing (later):**
- Free for contributors
- "Maintainer Seats" paid (policy packs, template packs, PR scoring, CI gates)
- Optional sponsorware for OSS

### Core Metrics (Only a Few, All User-Outcome-Aligned)

- Time to first accepted PR (TTFPR) p50/p90
- PR acceptance rate without rework
- Simulation accuracy (predicted vs actual touched interfaces/files)
- Maintainer NPS for Parseltongue-generated PRs
- Retention: #PRs per contributor in week 2–4 post install

### Risks and Explicit Kill Criteria

**Risk:** Overbuilding static analysis before product fit
- **Mitigation:** P0 pattern whitelist + compile/test gates. Kill if after 6–8 weeks TTFPR p50 > 6h and acceptance < 60% despite curated repos

**Risk:** Maintainers reject "drive-by" PRs
- **Mitigation:** Maintainer Mode first, repository policy files, project-owned templates. Kill if no maintainers adopt after 10 targeted outreaches

**Risk:** Repo idiosyncrasies destroy template reuse
- **Mitigation:** per-repo template folder support; measure reuse rate; invest in 3–5 framework adapters only if reuse > 50%

### 30/60/90 Execution Plan

**30 days (P0 skeleton):**
- Name index for O(1) entity lookup
- ISG_future overlay with Create/Edit/Delete ops
- Pattern whitelist: rate limiting middleware; error-handling hardening; test scaffolds
- CLI: analyze, propose, implement, check, submit
- Local compile/test and PR opening
- Curate 3 repos and secure maintainer buy-in + templates

**60 days (reliability + Maintainer Mode):**
- Repository policy file (.parseltongue.yml) + template overrides (.parseltongue/)
- Risk scoring v1 shipped; "make it green" tips
- CI signals in PR body (format/clippy/test matrix), label auto-apply
- First public "First PR Friday" with measured TTFPR

**90 days (confidence and scale):**
- cfg awareness (feature flags per template)
- Simple trait resolution for common patterns; better who-calls precision
- Add one more high-acceptance pattern (e.g., structured logging instrumentation)
- Publish early metrics; expand to 10 repos

### Code-Level Deltas (Surgical)

**daemon.rs:**
- Implement `parse_rust_file` or remove the test path that depends on it; current tests reference an unimplemented method

**isg.rs:**
- Add a `HashMap<String, SigHash>` name_index; fill on upsert_node; use in find_entity_by_name for O(1)

**cli.rs:**
- Add "propose" and "implement" subcommands that map to pattern templates (start with 2–3)
- Add "check" to run cargo fmt/clippy/test and summarize
- Add "submit" to wrap gh PR creation and embed simulation summary

**templates/:**
- Introduce opinionated Rust templates per pattern with placeholders; allow repo override via .parseltongue/

**risk/:**
- New module with simple scoring based on EdgeKind fanout + publicness + file criticality

**tests:**
- Add end-to-end integration tests: analyze → propose → implement → check (mock repo)
- Loosen perf assertions in CI to avoid flakes; keep micro-bench tests as ignored/bench

---

## Graph-Based Code Generation Insights (Research Paper Analysis)

### The Shreyas Take: If This Paper Is Right, What Becomes Possible?

**Your LLM doesn't need to "understand all Rust" to be useful.** It needs a small, structural prefix that stabilizes generation for the exact change you're proposing. Encode structure; freeze the model; get more reliable diffs.

**You don't need full MIR/trait solvers to move your metric.** Start with types + dataflow on the narrow slice you will touch; control flow can wait. The paper's ablation agrees: type/dataflow edges deliver most of the gain.

**"Graph-invariant PRs."** If your ISG prefix is invariant to naming/formatting, your propose/implement pipeline becomes robust against refactors on main. That directly improves acceptance rate and reduces rework.

**Graph-to-prompt is a scalpel.** Use it only where text fails (risky edits, tests generation, error-handling hardening). Keep the fast path text+templates; selectively switch on structural prefixes when the risk score is high.

**You can ship value without training a big model.** Learn a tiny adapter (graph→prefix), freeze the LLM, and iterate. If you can't train now, use deterministic "graph signatures" as a surrogate prefix and A/B it.

**Maintainers will trust "structurally justified" PRs more.** A PR that includes "structural delta and blast radius (type/dataflow)" looks like engineering, not guessing. That's your wedge into Maintainer Mode.

**The dual-state simulation gets sharper with dataflow edges.** Predict test impact and propose focused tests for the nodes on the path; you'll raise merge-confidence with minimal extra analysis.

### What to Adopt Now vs Defer

| Area | Paper signal | A011 implication | Decision |
|------|--------------|------------------|----------|
| **Type edges** | High impact in ablation | Add lightweight type info per interface; include in risk scoring and prompt prefix | **Adopt now** |
| **Dataflow edges** | High impact | Build function-level dataflow stubs (params→return, simple var propagation, call IO) for changed files | **Adopt now** |
| **Control flow (CFG)** | Lower marginal impact | Keep for security/panic patterns later, not P0 | **Defer** |
| **Graph→LLM soft prompt** | Lifts generation reliability | Add a structural prefix ("GraphHints") for propose/implement only when risk≥Y | **Adopt selectively** |
| **LLVM IR for Rust** | Heavy to compile; precise | Prefer HIR/MIR-lite or AST-derived types/dataflow; only compile IR for "ultra confidence" mode | **Defer to P1/P2** |
| **Full trait/method resolution** | Complex | Implement minimal method resolution for common frameworks if/when false positives block merges | **Defer til it moves acceptance** |
| **Pretraining GNN** | Expensive | Start with deterministic graph signatures; consider small LoRA adapter later | **Defer training; prototype heuristics** |

### Two Cheap Experiments (2 Weeks)

**A/B "structural prefix":**
- **A:** Current text+templates
- **B:** Same, plus a 512–1,024 token GraphHints prefix for only the touched files: top-K symbols, type table for changed interfaces, dataflow pairs (producer→consumer), outdegree stats
- **Measure:** test pass pre-submit, reviewer requested changes, acceptance rate, token cost, latency

**Narrow-scope dataflow:**
- Build a simple param/return and call-IO dataflow over the changed functions and their immediate neighbors
- Use it to (a) rank tests to update/add and (b) lower risk by pruning edits that touch high outdegree nodes
- **Measure:** fewer failing tests; fewer "you touched too much" reviews

### What to Stop Doing (For Now)

- Stop chasing full MIR/CFG before you've proven acceptance lift from types+dataflow. It's delay without proof
- Stop aiming for repository-wide precision. Do on-demand extraction for the diff's neighborhood only
- Stop generating free-form large edits. Keep to patterns and let structure steer small, high-confidence changes

### Minimal Technical Deltas to A011

**ISG enrichments:**
- Add a name→hash index (O(1) lookup)
- Extend NodeData with simple type info (from syn/tree-sitter; start with function signatures, struct fields)
- Add DataflowEdge: Param→Return (function-level), CallSite→Callee IO (who writes/reads)

**GraphHints prefix (deterministic):**
- Per change set: "Interfaces Touched," "Type Table," "Dataflow Pairs," "Blast Radius (top-10 by outdegree)," "Forbidden touch-list" (from maintainer policy)
- Size-budgeted and stable ordering to be diff-friendly

**CLI path:**
- `propose --risk-aware` mode: inject GraphHints when risk score ≥ threshold
- `implement --validate`: run fmt/clippy/tests, compare predicted "interfaces changed" vs actual; rollback if mismatch > N%
- `submit`: include "Structural Delta" section in PR body (trust signal)

### Maintainer Mode (Ties to Paper's Advantage)

**Opt-in .parseltongue.yml:**
- Approved patterns, disallowed regions, required tests, "structural guardrails" (e.g., do not change public API types)

**Output in PR:**
- Short "Structural Justification" based on type/dataflow deltas

**Benefit:** Higher initial acceptance and a feedback loop to improve your risk model

### "When to Escalate Analysis" Rule-of-Thumb

**If risk score ≥ red OR predicted interfaces changed ≠ actual (>20%):**
- Enable deeper structure:
  - Add one-hop callee/caller dataflow scan
  - Add type mismatches to hints
  - For panic-fix PR types, enable minimal control-flow markers (assert/panic edges) for the touched functions only
- **Otherwise:** Stay fast-path (text+templates)

### 30/60-Day Structure-Aware Roadmap

**30 days:**
- Type metadata + name index; DataflowEdge (function-level); GraphHints v1; risk-aware propose/implement; PR "Structural Delta"
- A/B on 3 curated repos; target +10–20% acceptance lift or −30% rework

**60 days:**
- Maintainer Mode policies; test selector/prioritizer using dataflow; "make-it-green" tips driven by structure (reduce fanout, avoid public API changes)
- Decide on tiny adapter training (graph→prefix) vs deterministic prefix only, based on A/B gains

### How This Sharpens Your Wedge

- **Faster, safer:** Structure narrows ambiguity for the exact edits you ask the model to make
- **Trust-building:** "Structural justification" in PRs is a maintainer's native language
- **Bounded cost:** You stay in the P0 promise (≤ 5s analyze, sub-minute check) by scoping structure to the patch neighborhood and prioritizing types/dataflow

### One-Page Checklist Before You Integrate

- Does this structural prefix reduce rework on your top 3 change patterns?
- Can you keep the prefix within 1k tokens and make it stable across runs?
- Do your risk scores actually change behavior (e.g., choose a smaller edit) and improve acceptance?
- Did acceptance or TTFPR improve in A/B on curated repos? If not, don't add MIR—fix patterns or templates first

**In short:** Borrow the paper's core insight—LLMs get much better when conditioned with compact, structural signals—and apply it surgically: types + dataflow, on-demand, only where your PR quality or acceptance needs the help. This keeps A011 focused on what matters: accepted PRs in hours, with structural receipts maintainers can trust.

---

## Parseltongue: Complete Project Structure

### Workspace Structure

```
parseltongue/
├── Cargo.toml
└── crates/
    ├── parseltongue-core/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── errors.rs
    │       ├── util/
    │       │   └── ids.rs
    │       ├── isg/
    │       │   ├── types.rs
    │       │   ├── store.rs
    │       │   └── analysis.rs
    │       ├── prd/
    │       │   ├── types.rs
    │       │   ├── refinement.rs
    │       │   ├── feasibility.rs
    │       │   ├── solutioning.rs
    │       │   ├── simulation.rs
    │       │   └── approval.rs
    │       ├── parse/
    │       │   ├── types.rs
    │       │   └── parser.rs
    │       ├── persist/
    │       │   ├── json_snapshot.rs
    │       │   ├── graph_db.rs
    │       │   └── sqlite_repo.rs
    │       ├── visualize/
    │       │   ├── mermaid.rs
    │       │   └── html.rs
    │       ├── orchestrator/
    │       │   ├── pipeline.rs
    │       │   └── events.rs
    │       ├── build/
    │       │   └── test_runner.rs
    │       ├── vcs/
    │       │   └── vcs.rs
    │       └── llm/
    │           └── provider.rs
    ├── parseltongue-daemon/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── file_watch.rs
    │       └── ingest.rs
    ├── parseltongue-cli/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs
    │       └── commands.rs
    └── parseltongue-adapters/
        ├── Cargo.toml
        └── src/
            ├── rust_analyzer.rs
            ├── treesitter_rust.rs
            ├── sqlite.rs
            ├── cosdata.rs
            ├── llm_ollama.rs
            ├── llm_anthropic.rs
            ├── graphviz.rs
            ├── html_renderer.rs
            ├── cargo_runner.rs
            └── git.rs
```

### Interface Signatures by File

**Core Library (parseltongue-core):**

**errors.rs:**
- `type CoreError`

**util/ids.rs:**
- `type InterfaceId`
- `type SnapshotId`
- `type GraphKey`

**isg/types.rs:**
- `type ISG`
- `type ISGNode`
- `type ISGEdge`
- `enum InterfaceKind`
- `enum EdgeKind`
- `struct InterfaceMeta`

**isg/store.rs:**
- `trait ISGStore`
  - `get(&self) -> &ISG`
  - `set(&mut self, next: ISG) -> Result<(), CoreError>`
  - `upsert_node(&mut self, node: ISGNode) -> Result<(), CoreError>`
  - `upsert_edge(&mut self, edge: ISGEdge) -> Result<(), CoreError>`
  - `remove_node(&mut self, id: InterfaceId) -> Result<(), CoreError>`
  - `save_snapshot(&self) -> Result<SnapshotId, CoreError>`
  - `load_snapshot(&mut self, id: SnapshotId) -> Result<(), CoreError>`

**isg/analysis.rs:**
- `trait ISGAnalyzer`
  - `enrich_metadata(&mut self) -> Result<(), CoreError>`
  - `classify_test_interfaces(&mut self) -> Result<(), CoreError>`
  - `dependencies_of(&self, id: InterfaceId) -> Result<Vec<InterfaceId>, CoreError>`
  - `callers_of(&self, id: InterfaceId) -> Result<Vec<InterfaceId>, CoreError>`
  - `blast_radius(&self, id: InterfaceId) -> Result<Vec<InterfaceId>, CoreError>`
  - `detect_cycles(&self) -> Result<Vec<Vec<InterfaceId>>, CoreError>`
  - `diff(a: &ISG, b: &ISG) -> ISGDiff`
- `type ISGDiff`

**prd/types.rs:**
- `struct PRD`
- `struct FeasibilityReport`
- `struct SolutionPlan`
- `struct ChangeSummary`
- `enum FutureAction`
- `struct TestReport`
- `struct BuildReport`
- `struct Outcome`
- `enum ApprovalDecision`

**prd/refinement.rs:**
- `trait PRDRefiner`
  - `refine(&self, initial: PRD, isg: &ISG) -> Result<PRD, CoreError>`

**prd/feasibility.rs:**
- `trait FeasibilityAssessor`
  - `assess(&self, prd: &PRD, isg: &ISG) -> Result<FeasibilityReport, CoreError>`

**prd/solutioning.rs:**
- `trait FutureISGDesigner`
  - `propose(&self, isg_current: &ISG, prd: &PRD) -> Result<ISG, CoreError>`
- `trait DeltaPlanner`
  - `plan_changes(&self, isg_current: &ISG, isg_future: &ISG) -> Result<SolutionPlan, CoreError>`

**prd/simulation.rs:**
- `trait SQLiteReflector`
  - `reflect_isg(&self, isg_future: &ISG) -> Result<(), CoreError>`
- `trait RubberDuckDebugger`
  - `validate(&self, prd: &PRD, isg_current: &ISG, isg_future: &ISG) -> Result<ChangeSummary, CoreError>`

**prd/approval.rs:**
- `trait HumanApprovalGate`
  - `request_approval(&self, summary: &ChangeSummary) -> Result<ApprovalDecision, CoreError>`

**parse/types.rs:**
- `struct FileUnit`
- `struct HirInfo`
- `enum LanguageId`

**parse/parser.rs:**
- `trait CodeParser`
  - `parse_dump(&self, dump: &FileUnit) -> Result<Vec<ISGNode>, CoreError>`
  - `parse_project(&self, root: &std::path::Path) -> Result<Vec<FileUnit>, CoreError>`
- `trait MetadataProvider`
  - `hir_for(&self, path: &std::path::Path) -> Result<HirInfo, CoreError>`
- `trait LanguageSupport`
  - `supported_languages(&self) -> Vec<LanguageId>`

**persist/json_snapshot.rs:**
- `trait SnapshotStore`
  - `save(&self, isg: &ISG) -> Result<SnapshotId, CoreError>`
  - `load(&self, id: SnapshotId) -> Result<ISG, CoreError>`

**persist/graph_db.rs:**
- `trait GraphDb`
  - `persist(&self, isg: &ISG) -> Result<GraphKey, CoreError>`
  - `load(&self, key: GraphKey) -> Result<ISG, CoreError>`
  - `neighbors(&self, id: InterfaceId, depth: u8) -> Result<Vec<InterfaceId>, CoreError>`

**persist/sqlite_repo.rs:**
- `struct CodeRow`
- `trait CodebaseRepository`
  - `upsert(&self, row: CodeRow) -> Result<(), CoreError>`
  - `set_future_code(&self, id: InterfaceId, code: String) -> Result<(), CoreError>`
  - `set_future_action(&self, id: InterfaceId, action: FutureAction) -> Result<(), CoreError>`
  - `mark_isg_flags(&self, id: InterfaceId, current: bool, future: bool) -> Result<(), CoreError>`
  - `rows_with_future_changes(&self) -> Result<Vec<CodeRow>, CoreError>`

**visualize/mermaid.rs:**
- `trait MermaidExporter`
  - `to_mermaid(&self, isg: &ISG) -> String`

**visualize/html.rs:**
- `struct HtmlOptions`
- `trait HtmlExporter`
  - `to_html(&self, isg: &ISG, options: &HtmlOptions) -> String`

**orchestrator/pipeline.rs:**
- `trait JourneyOrchestrator`
  - `run_issue_resolution(&self, prd: PRD) -> Result<Outcome, CoreError>`
  - `run_feature_development(&self, prd: PRD) -> Result<Outcome, CoreError>`
  - `run_refactoring(&self, prd: PRD) -> Result<Outcome, CoreError>`

**orchestrator/events.rs:**
- `enum Event`
- `enum Topic`
- `struct Subscription`
- `trait EventBus`
  - `publish(&self, event: Event) -> Result<(), CoreError>`
  - `subscribe(&self, topic: Topic) -> Result<Subscription, CoreError>`

**build/test_runner.rs:**
- `trait BuildSystem`
  - `compile(&self) -> Result<BuildReport, CoreError>`
- `trait TestRunner`
  - `run_all(&self) -> Result<TestReport, CoreError>`

**vcs/vcs.rs:**
- `struct VcsStatus`
- `type CommitId`
- `type PrUrl`
- `trait Vcs`
  - `status(&self) -> Result<VcsStatus, CoreError>`
  - `create_branch(&self, name: &str) -> Result<(), CoreError>`
  - `commit(&self, message: &str) -> Result<CommitId, CoreError>`
  - `push(&self) -> Result<(), CoreError>`
  - `open_pr(&self, title: &str, body: &str) -> Result<PrUrl, CoreError>`

**llm/provider.rs:**
- `struct Prompt`
- `struct Completion`
- `trait LlmClient`
  - `complete(&self, prompt: Prompt) -> Result<Completion, CoreError>`
  - `stream(&self, prompt: Prompt) -> Result<(), CoreError>`

**Daemon (parseltongue-daemon):**

**lib.rs:**
- `trait Daemon`
  - `start(&self) -> Result<(), CoreError>`
  - `shutdown(&self) -> Result<(), CoreError>`

**file_watch.rs:**
- `struct WatchHandle`
- `trait FileWatcher`
  - `watch(&self, root: &std::path::Path) -> Result<WatchHandle, CoreError>`

**ingest.rs:**
- `struct IngestStats`
- `trait IngestService`
  - `ingest_dump(&self, path: &std::path::Path) -> Result<IngestStats, CoreError>`
  - `ingest_live_change(&self, path: &std::path::Path) -> Result<IngestStats, CoreError>`

**CLI (parseltongue-cli):**

**commands.rs:**
- `trait CliCommand`
  - `run(&self, args: Vec<String>) -> Result<(), CoreError>`
- Commands: `ingest`, `daemon`, `query`, `generate_context`, `export_mermaid`, `export_html`, `prd_refine`, `plan`, `apply`, `test`, `approve`, `commit_push_pr`

**Adapters (parseltongue-adapters):**

- `trait RustAnalyzerAdapter : MetadataProvider`
- `trait TreeSitterRustAdapter : CodeParser`
- `trait SQLiteAdapter : CodebaseRepository`
- `trait CosDataAdapter : GraphDb`
- `trait OllamaClient : LlmClient`
- `trait AnthropicClient : LlmClient`
- `trait GraphvizExporter : MermaidExporter`
- `trait HtmlRendererAdapter : HtmlExporter`
- `trait CargoRunner : BuildSystem + TestRunner`
- `trait GitAdapter : Vcs`

---

## Ollama Integration: User Journey & HLD Support

### Coverage Summary for Ollama

**Interfaces:**
- `parseltongue-core/src/llm/provider.rs`: LlmClient, Prompt, Completion
- `parseltongue-core/src/prd/refinement.rs`: PRDRefiner (consumes LlmClient)
- `parseltongue-core/src/prd/solutioning.rs`: FutureISGDesigner, DeltaPlanner (optionally consumes LlmClient)
- `parseltongue-core/src/prd/simulation.rs`: RubberDuckDebugger (consumes LlmClient)
- `parseltongue-core/src/orchestrator/pipeline.rs`: JourneyOrchestrator (drives loops using above)

**Adapter:**
- `parseltongue-adapters/src/llm_ollama.rs`: OllamaClient : LlmClient (local Ollama)

**CLI/Daemon integration:**
- `parseltongue-cli/src/commands.rs`: CLI commands invoke JourneyOrchestrator
- `parseltongue-daemon/*`: optional live parsing; orthogonal to LLM

### User Journey with Ollama

**Step 1: Select local model and connect**
- **Core interfaces:** LlmClient
- **Adapter:** OllamaClient : LlmClient
- **Outcome:** LLM availability for subsequent loops (stream or complete)

**Step 2: Create/refine PRD in architecture context**
- **Core interfaces:** PRDRefiner, ISGStore, ISGAnalyzer, LlmClient
- **Adapter:** OllamaClient
- **Outcome:** Refined PRD aligned to ISG_current

**Step 3: Feasibility assessment and risk**
- **Core interfaces:** FeasibilityAssessor, ISGAnalyzer
- **Adapter:** (Optional) OllamaClient if using LLM-aided risk notes
- **Outcome:** FeasibilityReport with risk/benefit

**Step 4: Future ISG design and delta planning**
- **Core interfaces:** FutureISGDesigner, DeltaPlanner, ISGStore, LlmClient (optional prompt-driven design heuristics)
- **Adapter:** OllamaClient
- **Outcome:** ISG_future + SolutionPlan

**Step 5: Reflect changes to SQLite "bridge"**
- **Core interfaces:** SQLiteReflector, CodebaseRepository
- **Adapter:** SQLiteAdapter : CodebaseRepository
- **Outcome:** Rows populated (Future_Code, Future_Action, flags)

**Step 6: Rubber duck check (consistency pass)**
- **Core interfaces:** RubberDuckDebugger, LlmClient, ISGAnalyzer (cross-check)
- **Adapter:** OllamaClient
- **Outcome:** ChangeSummary (may iterate PRD or ISG_future)

**Step 7: Apply, compile, and test**
- **Core interfaces:** BuildSystem, TestRunner, ISGStore (finalize)
- **Adapter:** CargoRunner : BuildSystem + TestRunner
- **Outcome:** BuildReport, TestReport

**Step 8: Visualize and present for approval**
- **Core interfaces:** MermaidExporter, HtmlExporter, HumanApprovalGate, LlmClient (optional summarization)
- **Adapters:** GraphvizExporter, HtmlRendererAdapter, OllamaClient (for summary phrasing)
- **Outcome:** Diagram(s), decision (ApprovalDecision)

**Step 9: Commit and PR**
- **Core interfaces:** Vcs
- **Adapter:** GitAdapter : Vcs
- **Outcome:** CommitId, PrUrl

### Journey-to-Interface Mapping

| Step | Core interfaces | Adapter(s) | Primary outputs |
|------|-----------------|------------|-----------------|
| Connect | LlmClient | OllamaClient | LLM ready (stream/complete) |
| PRD refine | PRDRefiner, ISGStore, ISGAnalyzer, LlmClient | OllamaClient | Refined PRD |
| Feasibility | FeasibilityAssessor, ISGAnalyzer | (OllamaClient optional) | FeasibilityReport |
| Future design | FutureISGDesigner, DeltaPlanner, ISGStore, LlmClient | OllamaClient | ISG_future, SolutionPlan |
| Reflect SQLite | SQLiteReflector, CodebaseRepository | SQLiteAdapter | Future rows populated |
| Rubber duck | RubberDuckDebugger, LlmClient, ISGAnalyzer | OllamaClient | ChangeSummary |
| Build/Test | BuildSystem, TestRunner | CargoRunner | BuildReport, TestReport |
| Visualize/Approve | MermaidExporter, HtmlExporter, HumanApprovalGate, LlmClient | Graphviz/HTML, OllamaClient | Diagrams, ApprovalDecision |
| VCS | Vcs | GitAdapter | CommitId, PrUrl |

### HLD Support by Levels

**L1 Core (pure traits, domain contracts):**
- **Location:** `parseltongue-core/src/llm/provider.rs`, `parseltongue-core/src/prd/*.rs`, `parseltongue-core/src/isg/*.rs`, `parseltongue-core/src/orchestrator/*.rs`
- **Responsibilities:**
  - Define Prompt/Completion and LlmClient (complete, stream) without transport concerns
  - Express PRD loops: PRDRefiner, FeasibilityAssessor, FutureISGDesigner, DeltaPlanner, RubberDuckDebugger, HumanApprovalGate
  - ISG operations: ISGStore, ISGAnalyzer
  - Orchestration: JourneyOrchestrator

**L2 Services (composition, business logic):**
- **Location:** Implementations of core traits (not shown in interface signatures, but implied)
- **Responsibilities:**
  - Compose L1 traits into end-to-end flows
  - Handle error recovery, retries, logging
  - Manage state transitions across PRD lifecycle

**L3 Adapters (concrete implementations):**
- **Location:** `parseltongue-adapters/src/llm_ollama.rs`
- **Responsibilities:**
  - Implement LlmClient for Ollama HTTP API
  - Handle model selection, temperature, token limits
  - Stream or batch completions
  - Map Ollama-specific errors to CoreError

---

