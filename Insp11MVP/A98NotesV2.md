amuldotexe: - local-llama-rust-orchestrator-elf will be a command line tool with default installation config of naming & downloading following models for our current scope
    - StarCoder2 3B
- local-llama-rust-orchestrator-elf will identify current free RAM & suggest number of subagents that can be activated based on 90% of free RAM based on StarCoder2 3B


can this be done by an OSS tool like Ray https://github.com/ray-project/ray

---

Raycast AI: 
## Short answer

Yes. Ray can absolutely orchestrate “subagents” based on available memory and can manage their lifecycle. You can:

-  Detect free RAM, compute how many workers fit under a 90% cap, and launch one Ray actor per subagent.
-  Use Ray’s resource-aware scheduling with per-task/actor memory requirements via `@ray.remote(memory=...)`.
-  Scale locally or across a Ray cluster.

Key caveat: Ray is Python-first (also Java), not Rust. If you need a pure Rust CLI, Ray won’t be a native dependency; you’d either call Ray via its Jobs API (HTTP) or implement a lightweight scheduler in Rust.

## What you want to do, mapped to Ray

-  Download model (StarCoder2 3B): handle with `huggingface_hub` in Python, or do it once outside Ray; Ray doesn’t download for you but can run the download step.
-  Detect free RAM and suggest subagent count:
    - Get host free RAM (e.g., `psutil.virtual_memory().available`).
    - Estimate per-agent memory for StarCoder2 3B (depends on dtype/quantization and KV cache).
    - Compute:
        \[n = \left\lfloor \frac{0.9 \cdot \text{free\_RAM}}{\text{mem\_per\_agent}} \right\rfloor\]
-  Launch subagents:
    - One Ray actor per subagent with `@ray.remote(memory=per_agent_bytes, num_cpus=...)`.
    - Optionally use placement groups if you want to co-locate/pack resources.

## Rough memory sizing for StarCoder2 3B

| Variant | Param bytes | Base model RAM (approx) | Notes |
|:---|:---|:---|:---|
| FP16 | 2 B/param | ~6.0 GB | plus framework/loader overhead and KV cache |
| Int8 | 1 B/param | ~3.0 GB | plus overhead and KV cache |
| GGUF Q4-ish | ~0.5 B/param | ~1.5–2.0 GB | depends on quant; good for CPU via llama.cpp |
| KV cache | — | + a few hundred MB per concurrent context | grows with sequence length and concurrency |

These are ballpark figures. Actual resident RAM depends on runtime (PyTorch vs. llama.cpp), quantization, and context length.

## Minimal Ray example (local)

This Python example:
-  Downloads the model folder (once).
-  Estimates memory per agent.
-  Suggests subagent count.
-  Optionally spawns actors with a memory reservation.

Install: `pip install ray psutil huggingface_hub`

```python
import math
import os
import ray
import psutil
from huggingface_hub import snapshot_download

MODEL_ID = "bigcode/starcoder2-3b"
# Choose one of: "fp16", "int8", "q4_gguf"
MODEL_VARIANT = os.environ.get("MODEL_VARIANT", "int8")

# Very rough estimates (bytes) for the base model in memory
BYTES_PER_PARAM = {
    "fp16": 2.0,
    "int8": 1.0,
    "q4_gguf": 0.6,  # adjust for your GGUF build
}
PARAMS = 3_000_000_000

def estimate_model_ram_bytes(variant: str) -> int:
    base = PARAMS * BYTES_PER_PARAM[variant]
    framework_overhead = 600 * (1024**2)  # ~600 MB
    kv_cache = 400 * (1024**2)            # ~400 MB per agent (depends on seq len)
    return int(base + framework_overhead + kv_cache)

def suggest_agent_count(per_agent_bytes: int) -> int:
    free_bytes = psutil.virtual_memory().available
    usable = int(0.9 * free_bytes)
    return max(0, usable // per_agent_bytes)

def ensure_model():
    # If using GGUF/llama.cpp, you’d download a GGUF asset instead.
    snapshot_download(repo_id=MODEL_ID, local_dir="models/starcoder2-3b", local_dir_use_symlinks=False)

@ray.remote
class SubAgent:
    def __init__(self, idx: int):
        self.idx = idx
    def ping(self):
        return f"subagent {self.idx} ready"

if __name__ == "__main__":
    ensure_model()

    per_agent = estimate_model_ram_bytes(MODEL_VARIANT)
    n = suggest_agent_count(per_agent)
    print(f"Per-agent memory (approx): {per_agent/1024**3:.2f} GB")
    print(f"Suggested subagents (90% of free RAM): {n}")

    if n > 0:
        ray.init()  # local Ray; use ray.init(address="auto") for a cluster
        actors = [
            SubAgent.options(memory=per_agent, num_cpus=1).remote(i)
            for i in range(n)
        ]
        print(ray.get([a.ping.remote() for a in actors]))
```

Notes:
-  `options(memory=per_agent, ...)` asks Ray to place each actor on a node with enough available RAM, preventing oversubscription.
-  For GPU inference, reserve `num_gpus=1` per agent (Ray does not schedule by GPU VRAM amount; you typically allocate a whole GPU per actor, or create custom resources).
-  For distributed setups, run a Ray cluster and call `ray.init(address="auto")`.

## If you need a Rust-first CLI

Ray has no official Rust SDK. You have three practical options:

-  Keep the orchestrator in Rust and:
    - Use `sysinfo` or `heim` to read free RAM.
    - Compute \(n\) via the same formula.
    - Spawn subagents as processes (e.g., llama.cpp servers) with a per-process model/quant and context settings to keep RAM within budget.
    - Use `hf-hub` crate to download models at install time.

-  Hybrid: Rust CLI + Ray Jobs API
    - Rust invokes a Python “Ray bundle” via Ray Jobs (HTTP) to schedule/scale subagents.
    - Keeps UX in Rust, orchestration in Ray.

-  Use a general scheduler (Nomad/Kubernetes) if you anticipate broader orchestration needs beyond a single node.

## When Ray is a good fit

-  You anticipate scaling beyond one machine.
-  You want simple, resource-aware task/actor scheduling and elasticity.
-  You prefer Python-based orchestration and ecosystems (Serve, Train, Datasets).

For a single-node, Rust-only CLI, Ray may be overkill; a small in-process scheduler is simpler.

## Quick questions to finalize the design

-  Will subagents run on CPU (GGUF/llama.cpp) or GPU (PyTorch/vLLM)?
-  What quantization/precision do you plan to use for StarCoder2 3B?
-  Single machine or a small cluster?
-  Do subagents share a model in memory (e.g., server multiplexing) or load one per process? Sharing can drastically reduce RAM per agent.

---

amuldotexe: My use case is to browse a codebase using tree-sitter and while we put it in a cozoDB at Interface Signature Graph level - that is all top level interfaces inside a code file - as Primary key of CozoDB - then code blobs being stored and then they are being parsed by This tool to summarize in 1 lines and store results in CozoDB

---

Raycast AI: 

## Proposed architecture

-  Source scanning and parsing (tree-sitter)
    - Walk the repo, detect language per file.
    - Use per-language tree-sitter queries to extract top-level interface signatures and their spans.
-  Storage (CozoDB)
    - Store a normalized “Interface Signature” entity as the primary key for your graph level.
    - Keep code blobs (content-addressed) and a mapping from interfaces to the minimal code slice (span) they originate from.
    - Store one-line summaries keyed by the interface ID and model/version.
-  Summarization (LLM)
    - Generate a single-line description per interface using StarCoder2 3B (or another code-tuned model).
    - Strongly recommended: run a single shared inference server and call it from workers, instead of loading the model in every worker.
-  Orchestration (Ray or Rust-only)
    - Ray (Python): schedule parsing tasks per file and summarization tasks per interface; use an actor pool for inference clients.
    - Rust-only: parallelize parsing with rayon, enqueue summarization jobs against a local model server, write back to CozoDB.

If you still want to compute a max subagent count from free RAM for local mode, use:
\[
n = \left\lfloor \frac{0.9 \cdot \text{free\_RAM}}{\text{mem\_per\_agent}} \right\rfloor
\]

## Data model (conceptual)

| Entity | Suggested key | Required fields | Notes |
|:---|:---|:---|:---|
| Interface | interface_id | repo_id, commit_sha, lang, file_path, name, signature_norm, span_start, span_end, code_hash | interface_id = blake3(repo_id:commit:lang:file:signature_norm) |
| Blob | code_hash | content, size_bytes | Content-addressed blobs for dedupe |
| Summary | (interface_id, model_id) | one_line, created_at, prompt_hash, params_hash | Cache by (interface_id, model_id, prompt/params) |
| Edge (graph) | (src_id, dst_id, kind) | kind, attrs | Optional: interface-to-interface references |

Notes:
-  signature_norm should be a canonical string (no whitespace, normalized generic params, etc.) per language.
-  For multi-repo or branch awareness, include repo_id and commit_sha in the ID or as separate indexed fields.

## Orchestration choices

-  Ray (Python-first)
    - Pros: resource-aware scheduling, easy parallelism, local-to-cluster portability.
    - Pattern: parsing tasks (light CPU) + summarization actor pool that talks to a single inference server.
    - Cozo access: from Python via an HTTP wrapper you provide, or call a small Rust microservice/CLI that executes Cozo queries.
-  Rust-only
    - Pros: single binary, direct tree-sitter and CozoDB integration, low overhead.
    - Pattern: rayon for parallel parse; a bounded threadpool for inference requests to a shared model server (or embed via candle if feasible).

Given your workflow, a single model server is more memory-efficient than multiple “subagents” each loading StarCoder2 3B.

## Minimal Ray pipeline skeleton (Python)

This keeps the model in a separate inference server (HTTP) and uses Ray to parallelize extraction and summarization. Replace the inference URL with your server (e.g., Hugging Face TGI, vLLM, or a custom FastAPI around transformers).

```python
import os
import glob
import json
import time
from pathlib import Path
import ray
import requests
from tree_sitter import Language, Parser

# Build a multi-lang tree-sitter library (one-time step outside code)
# Language.build_library("build/my-langs.so", ["tree-sitter-typescript", "tree-sitter-java", ...])

TS_LIB = os.environ.get("TS_LIB", "build/my-langs.so")
TS_LANGS = {
    ".ts": ("typescript", "tsx"),
    ".tsx": ("typescript", "tsx"),
    ".java": ("java",),
    # add more…
}
LANG_MAP = {
    "typescript": Language(TS_LIB, "typescript"),
    "java": Language(TS_LIB, "java"),
}

INFER_URL = os.environ.get("INFER_URL", "http://127.0.0.1:8000/generate")  # replace with your server
MODEL_ID = os.environ.get("MODEL_ID", "bigcode/starcoder2-3b")

def detect_lang(path: str):
    ext = Path(path).suffix.lower()
    return TS_LANGS.get(ext, (None,))[0]

def normalize_signature(lang: str, sig: str) -> str:
    # implement per-language normalization: strip whitespace, normalize generics, etc.
    return " ".join(sig.split())

def blake3_hex(s: str) -> str:
    import blake3
    return blake3.blake3(s.encode()).hexdigest()

def extract_interfaces_from_source(lang: str, source: bytes):
    parser = Parser()
    parser.set_language(LANG_MAP[lang])
    tree = parser.parse(source)
    root = tree.root_node

    # Per-language queries (examples below). For simplicity here, do a light traversal.
    # Replace with tree-sitter queries for accuracy and speed.

    interfaces = []
    def walk(node):
        # TypeScript example: "interface_declaration"
        if lang == "typescript" and node.type == "interface_declaration":
            start, end = node.start_byte, node.end_byte
            sig_text = source[start:end].decode(errors="ignore")
            interfaces.append((start, end, sig_text))
        # Java example: "interface_declaration"
        if lang == "java" and node.type == "interface_declaration":
            start, end = node.start_byte, node.end_byte
            sig_text = source[start:end].decode(errors="ignore")
            interfaces.append((start, end, sig_text))
        for child in node.children:
            walk(child)
    walk(root)
    return interfaces

def summarize_one_liner(context: str, name_hint: str):
    prompt = (
        "You are a senior engineer. Summarize the following interface in one concise line "
        "(<= 20 words), active voice, start with a verb, no code, no quotes.\n\n"
        f"Name: {name_hint}\n"
        "Interface:\n"
        f"```\n{context}\n```\n"
        "One-line summary:"
    )
    resp = requests.post(INFER_URL, json={
        "model": MODEL_ID,
        "prompt": prompt,
        "max_new_tokens": 40,
        "temperature": 0.2,
        "stop": ["\n\n", "\n#"],
    }, timeout=60)
    resp.raise_for_status()
    out = resp.json()
    # adapt to your server’s schema
    return out.get("generated_text") or out.get("text") or ""

@ray.remote
def parse_file(path: str):
    lang = detect_lang(path)
    if not lang:
        return []
    src = Path(path).read_bytes()
    interfaces = extract_interfaces_from_source(lang, src)
    results = []
    for (start, end, sig_text) in interfaces:
        signature_norm = normalize_signature(lang, sig_text.split("{")[0])
        interface_id = blake3_hex(f"{path}|{lang}|{signature_norm}")
        code_hash = blake3_hex(src.decode(errors="ignore"))
        results.append({
            "interface_id": interface_id,
            "lang": lang,
            "file_path": path,
            "signature_norm": signature_norm,
            "span_start": start,
            "span_end": end,
            "code_hash": code_hash,
        })
    return results

@ray.remote
class Summarizer:
    def summarize(self, interface_row, source_text):
        start = interface_row["span_start"]; end = interface_row["span_end"]
        snippet = source_text[start:end]
        name_hint = interface_row["signature_norm"].split()[1] if " " in interface_row["signature_norm"] else interface_row["signature_norm"]
        one_line = summarize_one_liner(snippet, name_hint)
        return {
            "interface_id": interface_row["interface_id"],
            "model_id": MODEL_ID,
            "one_line": one_line.strip(),
            "created_at": int(time.time()),
        }

def main(root: str):
    ray.init()
    files = [str(p) for p in Path(root).rglob("*") if p.is_file()]
    parsed = ray.get([parse_file.remote(p) for p in files])
    iface_rows = [r for batch in parsed for r in batch]

    # Load file contents once (for summarization snippets)
    source_map = {str(p): Path(p).read_text(errors="ignore") for p in set(r["file_path"] for r in iface_rows)}

    # Start a small pool of summarizers (they call the shared model server)
    pool = [Summarizer.remote() for _ in range(os.cpu_count() or 4)]
    tasks = []
    for i, row in enumerate(iface_rows):
        actor = pool[i % len(pool)]
        tasks.append(actor.summarize.remote(row, source_map[row["file_path"]]))
    summaries = ray.get(tasks)

    # TODO: write iface_rows, blobs, and summaries to CozoDB here
    print(f"Interfaces: {len(iface_rows)}; Summaries: {len(summaries)}")

if __name__ == "__main__":
    main(root=os.environ.get("CODE_ROOT", "."))
```

Notes:
-  Replace the naive tree walk with tree-sitter queries for robustness and speed (see below).
-  Summarizers talk to a shared inference server, so memory stays stable regardless of worker count.

## Per-language tree-sitter query hints

-  TypeScript top-level interfaces:
    - node type: interface_declaration
    - fields: name, type_parameters, extends_clause
-  Java top-level interfaces:
    - node type: interface_declaration
    - ensure parent is the compilation_unit root (not nested)

Pattern: use tree-sitter queries to extract names and headers rather than slicing entire bodies, then slice source spans as needed.

## Inference and memory

-  Single shared server (recommended):
    - Run StarCoder2 3B once (e.g., with transformers or TGI/vLLM if supported).
    - Ray or Rust workers send HTTP/gRPC requests.
    - RAM is dominated by one model instance; horizontal scaling is via server concurrency.
-  Per-worker model loads (not recommended locally):
    - Use the earlier RAM heuristic to bound worker count:
      \[n = \left\lfloor \frac{0.9 \cdot \text{free\_RAM}}{\text{mem\_per\_agent}} \right\rfloor\]
    - Works but wastes memory; avoid unless isolation is required.

## One-line summary prompt template

-  Constraints keep outputs terse and uniform. Example:

    - System: “You are a senior software engineer who writes concise summaries.”
    - User content:
        - Name and header of the interface
        - Optional docstring if available
        - Instruction: “One line, <= 20 words, start with a verb, no quotes, no code.”

Tune temperature (0.1–0.3) and max_new_tokens (~40). Add stop sequences to avoid extra text.

## Incremental updates

-  Derive interface_id from (repo_id, commit/Tree hash, lang, file, signature_norm). If unchanged, you can skip re-summarization by looking up an existing summary.
-  When a file changes:
    - Parse again, diff interface_id sets per file.
    - Delete stale summaries for removed interfaces; add for new/changed ones.
-  Optionally store prompt_hash and params_hash to invalidate caches when you change prompts/settings.

## Rust-only variant (outline)

-  tree-sitter:
    - Use the tree-sitter crates directly per language.
-  CozoDB:
    - Use the Rust crate to open the DB; define relations and indexes.
-  Parallelism:
    - rayon for parse stage.
    - A bounded async task pool (Tokio) to call the inference server.
-  Model:
    - Run a Python transformers server or TGI/vLLM, and call over HTTP.
    - If you must embed, consider candle with an instruction-tuned, small code model; but StarCoder2 3B support/quantization is simpler via Python.

## What I need to tailor this further

-  Which languages do you need first (e.g., TS/Java/Go/Java/.NET)?
-  Do you plan to run a shared inference server (GPU/CPU) or load models inside workers?
-  CozoDB access pattern: embedded in-process, or via an HTTP service you control?
-  Any constraints on output format (length, style, tags) beyond “one line”?

---

amuldotexe: ELI5 HLD LLD  in details

---

Raycast AI: 

---

amuldotexe: ELI5 HLD LLD  in details (of what you suggested above)

---

Raycast AI: 

## ELI5 overview

-   Imagine your codebase is a big library.
-   A Scanner walks through every book (file) and finds the table of contents items we care about: top-level interfaces.
-   A Librarian makes index cards for each interface and files them in drawers (CozoDB).
-   A Summarizer reads the small interface snippet and writes a one-line blurb.
-   A Traffic Cop makes sure we don’t read too many books at once or run out of memory.
-   Later, when books change, we only re-scan the changed parts and update the relevant index cards.

## High-Level Design (HLD)

-   Components
    -  Repository Scanner
        - Walks the code tree, detects language per file.
        - Feeds file paths to parsing workers.
    -  Parser (tree-sitter)
        - Uses language grammars to extract top-level interface signatures and spans (byte offsets).
        - Produces normalized signature strings and a stable interface_id.
    -  Blob Store
        - Content-addressed storage of code blobs (file contents), using a hash for deduplication.
    -  Summarizer
        - Calls a shared inference server to create a single-line summary per interface.
        - Stateless client; no model weights loaded in the worker process.
    -  CozoDB
        - Stores Interfaces, Blobs, Summaries, and optional graph edges.
        - Enforces idempotency (upserts) using stable keys.
    -  Orchestrator
        - Option A: Ray (Python) to parallelize parsing and summarization; resource-aware scheduling.
        - Option B: Rust-only: rayon/Tokio to parallelize; call the same inference server.
-   Why a shared inference server?
    -  Loading StarCoder2 3B once saves RAM and speeds up requests.
    -  Workers send HTTP/gRPC requests; you control concurrency centrally.
-   Data flow
    -  Scan files → Parse with tree-sitter → Normalize signature → Compute IDs/hashes → Upsert Interface + Blob records → Summarize via model server → Upsert Summary record.
-   Resource model
    -  Parsing is light CPU; summarization is network-bound (to model server).
    -  If you still want multiple “subagents,” bound the count with:
        \[n = \left\lfloor \frac{0.9 \cdot \text{free\_RAM}}{\text{mem\_per\_agent}} \right\rfloor\]
-   Deployment
    -  Single machine: run DB + model server + orchestrator locally.
    -  Scale out: run Ray cluster; keep a single model server per GPU, and increase server concurrency.

## Low-Level Design (LLD)

### Data model and identifiers

| Entity | Primary key | Fields | Indexes | Notes |
|---|---|---|---|---|
| Interface | interface_id | repo_id, commit_sha, lang, file_path, signature_norm, span_start, span_end, code_hash | repo_id, commit_sha, file_path | interface_id = blake3(repo_id:commit:lang:file:signature_norm) |
| Blob | code_hash | content, size_bytes, mime, created_at | size_bytes | code_hash = blake3(file_bytes) |
| Summary | (interface_id, model_id) | one_line, created_at, prompt_hash, params_hash | prompt_hash | Cacheable by interface+model+prompt/params |
| Edge (optional) | (src_id, dst_id, kind) | attrs | kind | For future graph relations |

-   Normalization rules (signature_norm)
    -  Trim whitespace, collapse spaces.
    -  Strip bodies; keep only the header/signature.
    -  Canonicalize generics and extends/implements lists deterministically.

### Algorithms and pipelines

-   File discovery
    -  Input: repository root.
    -  Steps:
        - Recursively list files; skip vendor/build folders via a configurable ignore list.
        - Detect language by extension or a language map.
        - Emit work items: (file_path, lang).
-   Parse with tree-sitter
    -  Input: (file_path, lang).
    -  Steps:
        - Load bytes; create parser for lang; parse into AST.
        - Run language-specific queries to find top-level interfaces.
        - For each match:
            - Extract header text and span (start_byte, end_byte).
            - signature_norm = normalize(header_text).
            - interface_id = blake3(repo_id|commit|lang|file|signature_norm).
            - code_hash = blake3(file_bytes) for the Blob.
        - Output: list of interface rows.
-   Upsert to CozoDB
    -  Blob: if code_hash not present, insert code blob; else skip.
    -  Interface: upsert by interface_id; if code_hash changed (file changed), update span info.
-   Summarization
    -  For each interface row lacking a current summary for (model_id, prompt_hash, params_hash):
        - Extract the snippet using span from file content (keep in-memory cache).
        - Build prompt; call inference server.
        - Upsert Summary keyed by (interface_id, model_id).
-   Incremental updates
    -  Compute diff by comparing existing interface_ids for a file with newly parsed ones:
        - New IDs → insert + summarize.
        - Missing IDs → mark summaries stale or delete.
        - Same IDs → skip summarization unless prompt_hash/params_hash changed.

### Orchestrator details

-   Ray option (Python)
    -  Parse stage: stateless Ray tasks for each file.
    -  Summarization stage: small pool of Ray actors that call the inference server (to reuse HTTP sessions, throttle load).
    -  Backpressure:
        - Limit max in-flight summaries with a queue size or by actor pool size.
    -  Memory-safety:
        - If you must load models in actors (not recommended), set `.options(memory=per_agent_bytes, num_cpus=1)`.
-   Rust-only option
    -  Parallel parsing with rayon over file list.
    -  Async summarization with a bounded Tokio semaphore to control concurrency to the model server.
    -  CozoDB access via Rust crate; batch upserts for throughput.

### Inference server contract

-   Endpoint: POST /generate
-   Request (example)
    -  Fields: model, prompt, max_new_tokens, temperature, stop
-   Response (example)
    -  Fields: generated_text, usage (optional)
-   Example JSON

    - Request:
        - 
            {
                "model": "bigcode/starcoder2-3b",
                "prompt": "You are a senior engineer... One-line summary:",
                "max_new_tokens": 40,
                "temperature": 0.2,
                "stop": ["\n\n", "\n#"]
            }
    - Response:
        - 
            {
                "generated_text": "Describe what the interface does in one crisp line."
            }

-   Timeouts and retries
    -  Client timeout: 60 s.
    -  Retries: 2 with exponential backoff for 5xx or connection errors.
    -  Idempotency: caller can reissue safely; upsert overwrites the same key.

### Tree-sitter extraction patterns

-   TypeScript (top-level interfaces)
    -  Node type: interface_declaration
    -  Query example:

        - 
            (interface_declaration
                name: (type_identifier) @name)

-   Java (top-level interfaces)
    -  Node type: interface_declaration
    -  Ensure parent is compilation_unit, not nested.

-   Span handling
    -  Use node.start_byte and node.end_byte for consistent slicing.
    -  Keep header-only by truncating at the opening brace.

### Normalization examples

-   Remove redundant whitespace:
    -  `interface   Foo<T>   extends A,  B {` → `interface Foo<T> extends A,B`
-   Deterministic generics/list ordering only if semantically safe; otherwise preserve order and just normalize spacing.

### Memory planning and subagents

-   If you do consider per-process model loading, estimate subagents by:
    \[n = \left\lfloor \frac{0.9 \cdot \text{free\_RAM}}{\text{mem\_per\_agent}} \right\rfloor\]
-   StarCoder2 3B rough sizes:

| Variant | Base model RAM | Notes |
|---|---|---|
| FP16 | ~6.0 GB | + framework overhead and KV cache |
| Int8 | ~3.0 GB | + overhead and KV cache |
| GGUF Q4-ish | ~1.5–2.0 GB | CPU via llama.cpp |

-   Recommendation: one shared server, many lightweight clients.

### API and function shapes (pseudo)

-   Parser

    - 
        interface ParseResult {
            interface_id: string
            repo_id: string
            commit_sha: string
            lang: string
            file_path: string
            signature_norm: string
            span_start: number
            span_end: number
            code_hash: string
        }

-   Upserts

    - 
        upsert_blob(code_hash: string, content: bytes, size_bytes: number)
        upsert_interface(row: ParseResult)
        upsert_summary(interface_id: string, model_id: string, one_line: string, prompt_hash: string, params_hash: string)

-   Summarization

    - 
        summarize_one_line(snippet: string, name_hint: string, model_id: string) -> string

### Concurrency and backpressure

-   Bound concurrent summarizations (e.g., 16) to avoid overloading the model server.
-   Cache file contents in memory keyed by file_path to avoid rereads during summarization bursts.
-   Batch DB writes:
    -  Group up to 100 upserts per transaction to reduce overhead.
-   Idempotency
    -  Keys ensure that retries do not create duplicates.

### Configuration

-   config.yaml example

    - 
        repo_root: "./repo"
        ignore:
            - "node_modules"
            - "target"
            - "dist"
        languages:
            ".ts": "typescript"
            ".tsx": "typescript"
            ".java": "java"
        model:
            id: "bigcode/starcoder2-3b"
            endpoint: "http://127.0.0.1:8000/generate"
            timeout_ms: 60000
            max_concurrent: 16
        cozo:
            path: "./data/cozo.db"
        summarization:
            max_new_tokens: 40
            temperature: 0.2

### Observability and quality

-   Logs
    -  Per file: parsed interfaces count.
    -  Per summary: latency, tokens (if available), status.
-   Metrics
    -  P95/P99 latency for summarize.
    -  Error rates, retry counts.
    -  Queue depth for pending summaries.
-   Tracing (optional)
    -  TraceID from file → interface → summary call.

### Failure handling

-   Parse errors: capture and mark file as failed; continue others.
-   DB conflicts: retry transaction backoff.
-   Inference failures: retry N times; if still failing, store a placeholder error in Summary with an error flag.
-   Timeouts: surface as warnings; re-queue with lower priority.

### Testing strategy

-   Unit tests
    -  Normalization edge cases.
    -  Tree-sitter queries against small fixtures.
-   Integration tests
    -  End-to-end on a tiny repo; verify Interfaces and Summaries populated.
-   Performance tests
    -  Large repo scan with dry-run summarization to validate throughput.

### Minimal snippets

-   Normalization (Python)

    - 
        def normalize_signature(header: str) -> str:
            header = header.strip()
            header = " ".join(header.split())
            # strip body if present
            if "{" in header:
                header = header.split("{", 1)[0].strip()
            # collapse spaces around commas
            header = header.replace(", ", ",")
            return header

-   Interface ID

    - 
        def interface_id(repo_id, commit, lang, path, signature_norm):
            import blake3
            s = f"{repo_id}|{commit}|{lang}|{path}|{signature_norm}"
            return blake3.blake3(s.encode()).hexdigest()

-   One-line prompt

    - 
        PROMPT = """You are a senior engineer. Summarize the following interface in one concise line (<= 20 words).
        Start with a verb. No code, no quotes.

        Name: {name}
        Interface:
        ```
        {snippet}
        ```
        One-line summary:"""

## Recommended next steps

-   Confirm initial languages (e.g., TypeScript and Java).
-   Choose orchestrator path (Ray vs Rust-only). For a single-node CLI, Rust-only with a shared model server is simplest.
-   Stand up the inference server for StarCoder2 3B and validate the one-line prompt quality.
-   Implement the parse → upsert → summarize loop and verify idempotency on repeated runs.
-   Add incremental update logic keyed by commit_sha and code_hash.

---

amuldotexe: Ok Can we actually call you built something like these 2 tools in Rust while taking maximum help from existing tools like Ray and your ideation

### **code-summary-generation-engine**
**CLI**: `code-summary-gen`
**Purpose**: Generate terse, lossless 1-line summaries for ISG nodes using rule-based heuristics with LLM fallback
**Input**: ISG data, summary model configuration, target nodes
**Output**: Node summaries with provenance tracking and confidence scores
**Actions**: Extract signature patterns → apply rule-based generation → validate quality → use LLM fallback if needed → store with provenance
**CLI**: `code-summary-gen --data ./isg_data --model rule-based --fallback llm --batch-size 50`
**Rust lib**: `libcodesummarygen`

**Variants**:
- (a) Rule-only heuristics for speed and determinism
- (b) LLM-assisted for complex cases with budget limits
- (c) Hybrid approach with confidence-based fallback
- (d) Domain-specific templates for different code patterns

- interface-summary-generator
  - Purpose: Generate terse, lossless 1-line summaries for ISGL1 nodes.
  - Inputs: ISG_current.
  - Outputs: summaries table with provenance (rule_based | llm_assisted).
  - Actions: extract signature verbs/nouns → generate ≤120 char summaries; backfill with LLM if rule-based fails.
  - Variants: (a) rule-only heuristics; (b) LLM backfill under budget.
  - Notes: Summaries are hints, never authority; used to reduce tokens.
  - Example CLI: interface-summary-generator --cozo cozo://isg --write summaries
  - Example Input (JSON): {"isgl1_keys":["src/lib.rs-foo-parse_cfg"]}
  - Example Output (JSON): {"isgl1_key":"src/lib.rs-foo-parse_cfg","summary":"parse TOML to Cfg; returns Result<Cfg,E>","provenance":"rule_based"}


- interface-graph-builder
  - Purpose: Build ISGL1 (filepath-filename-InterfaceName) as the canonical interface layer with L2/L3 constituents strictly "under" L1 for understanding.
  - Inputs: repo path, include/exclude globs.
  - Outputs: ISG_current (Cozo) and JSON snapshot.
  - Actions: parse crates, resolve items, derive ISGL1 keys, attach L2/L3 facets, persist to Cozo + JSON.
  - Variants: (a) rust-analyzer LSP overlays; (b) rustdoc JSON; (c) syn-based AST for macro-lite repos.
  - Notes: Zero code writes; stable ISGL1 keys enable cross-tool composition.
  - Example CLI: interface-graph-builder --repo . --out cozo://isg
  - Example Input (JSON): {"repo":".","include":["crates/**"],"exclude":["target/**"]}
  - Example Output (JSON): {"isgl1_key":"src/lib.rs-foo-parse_cfg","path":"src/lib.rs","kind":"fn","facets":{"generics":["T"],"where":["T: DeserializeOwned"]}}

  > **Reliability-First Summary**: ISG builder implements deterministic validation through rust-analyzer overlays and cargo check gates. Uses bounded concurrency (Tokio runtime) with cooperative yields for UI responsiveness. Caching strategies (ISG + HNSW persisted) ensure sub-100ms query performance. Error recovery uses thiserror patterns with complete diagnostic surfaces. Memory pressure managed via quantization (Q4_K_M) and dynamic parallelism throttling. Mode profiles select latency vs coverage vs rigor without changing contracts.

---

Raycast AI: 

## Yes—let’s build them in Rust (with optional Ray assist)

Below is a concrete, buildable plan for:

-  interface-graph-builder: constructs ISGL1 keys and attaches L2/L3 facets into Cozo and a JSON snapshot.
-  code-summary-generation-engine: generates terse, lossless, one-line summaries using rule-based heuristics with LLM fallback; includes a focused CLI alias interface-summary-generator.

Optional Ray assist:
-  Keep Rust as the UX and main orchestrator.
-  Use Ray only as a remote inference/service backend (Ray Serve or Jobs) via HTTP/gRPC. This gives you scalable LLM capacity without pulling Ray into your Rust runtime.

## Repo layout

| Path | Type | Purpose |
|---|---|---|
| crates/isg-core | Rust lib | Shared types, IDs, config, hashing, JSON codecs |
| crates/isg-cozo | Rust lib | CozoDB adapter (embedded or HTTP), upserts/queries |
| crates/isg-parse | Rust lib | Language adapters: rust-analyzer overlays, rustdoc JSON, syn fallback; tree-sitter adapters if needed |
| crates/isg-heuristics | Rust lib | Rule-based summary generation, confidence scoring, validators |
| crates/isg-infer-client | Rust lib | LLM client (HTTP; supports local transformers, TGI, vLLM, Ray Serve) |
| crates/interface-graph-builder | Rust bin | CLI to build ISGL1 and facets into Cozo + JSON snapshot |
| crates/code-summary-gen | Rust bin + lib | CLI/library to produce summaries with provenance and budget control |
| examples/ | Data | Small fixtures and sample configurations |

## CLI specifications

-  interface-graph-builder
    - Purpose: Build ISGL1 (filepath-filename-InterfaceName) as the canonical interface layer, attach L2/L3 facets.
    - CLI:
        - 
            interface-graph-builder --repo . --out cozo://isg --snapshot ./isg_snapshot.json --include 'crates/**' --exclude 'target/**'
    - JSON input (optional override):
        - 
            {"repo":".","include":["crates/**"],"exclude":["target/**"]}
    - JSON output (per-item example):
        - 
            {"isgl1_key":"src/lib.rs-foo-parse_cfg","path":"src/lib.rs","kind":"fn","facets":{"generics":["T"],"where":["T: DeserializeOwned"]}}

-  code-summary-gen (alias: interface-summary-generator)
    - Purpose: One-line summaries for ISGL1 nodes; rule-based with LLM fallback.
    - CLI (generic):
        - 
            code-summary-gen --cozo cozo://isg --model rule-based --fallback llm --batch-size 50 --max-llm 200 --write summaries
    - Focused CLI (alias):
        - 
            interface-summary-generator --cozo cozo://isg --write summaries
    - JSON input:
        - 
            {"isgl1_keys":["src/lib.rs-foo-parse_cfg"]}
    - JSON output:
        - 
            {"isgl1_key":"src/lib.rs-foo-parse_cfg","summary":"parse TOML to Cfg; returns Result<Cfg,E>","provenance":"rule_based","confidence":0.86}

## High-Level Design (HLD)

-  interface-graph-builder
    - Inputs: repo path, include/exclude globs.
    - Pipeline:
        - Discover workspace (cargo metadata) → filter files → parse items → derive ISGL1 keys → compute facets (L2/L3) → persist to Cozo + write JSON snapshot.
    - Parsing variants:
        - rust-analyzer overlays (most robust; needs r-a crates/LSP).
        - rustdoc JSON (cargo rustdoc -Z unpretty-json; stable, but larger).
        - syn-based AST (fast for macro-lite repos).
    - Guarantees:
        - Deterministic IDs, bounded concurrency, caching by code_hash, zero code writes.

-  code-summary-gen
    - Inputs: ISG_current (Cozo) and a list of target nodes or a query.
    - Pipeline:
        - Fetch target nodes → rule-based summary → validate and score → if below threshold, call LLM → upsert Summary with provenance and confidence.
    - Variants:
        - Rule-only, LLM-only, hybrid (confidence-based fallback), domain templates (e.g., HTTP, serde).
    - Budgeting:
        - Global limits on LLM calls and tokens per run; hard stop when exceeded.

-  Optional Ray
    - You run a Ray Serve deployment that exposes /generate.
    - Rust code talks HTTP to that endpoint; Ray handles autoscaling, batching, rate limiting.
    - No Ray dependency in Rust binary.

## Low-Level Design (LLD)

### Core identifiers and data

-  ISGL1 key: filepath-filename-InterfaceName
    - Example: src/lib.rs-foo-parse_cfg
-  Stable IDs and hashes:
    - interface_id = blake3(repo_id|commit|lang|path|signature_norm)
    - code_hash = blake3(file_bytes)

### Cozo logical schema (conceptual)

| Relation | Primary key | Fields | Notes |
|---|---|---|---|
| interfaces | interface_id | repo_id, commit_sha, lang, file_path, isgl1_key, kind, signature_norm, span_start, span_end, code_hash | ISGL1 is also stored |
| blobs | code_hash | content, size_bytes, mime, created_at | Optional content storage if you want dedupe |
| summaries | (interface_id, model_id, prompt_hash, params_hash) | one_line, provenance, confidence, created_at, error | Idempotent upsert |
| facets | (interface_id, facet_kind) | value (JSON) | L2/L3 facets like generics, where-clauses |

Persist snapshot: Write interfaces + facets to a JSON array file for portability.

### Concurrency and reliability

-  Tokio runtime with bounded semaphores for parse and summary.
-  Cooperative yields during long database or network operations.
-  Caching:
    - In-memory code cache by file_path → content.
    - DB-level: skip parsing/summarization if interface_id present with same code_hash and current prompt_hash/params_hash.
-  Error handling:
    - thiserror + anyhow for rich diagnostics.
    - Summary errors recorded in summaries.error; continue processing others.

### Heuristics (rule-based summaries)

Goal: ≤ 120 chars, start with a verb, include main object and return shape; avoid claims beyond signature.

-  Feature extraction from signature_norm:
    - Name: snake_case → “verb object” (split at underscores).
        - parse_cfg → “parse cfg”
        - compute_sha256 → “compute SHA-256”
    - Inputs: parameter types → pick dominant nouns.
        - (input: &str) → “string”
        - (reader: impl Read) → “reader”
    - Returns:
        - Result<T, E> → “returns Result<T,E>”
        - Option<T> → “returns Option<T>”
        - () → omit returns clause.
    - Async/traits:
        - async fn → “async”
        - trait Foo → “define Foo contract”
        - interface (TS/Java) → “define <Name> interface; …”
-  Assembly:
    - Pattern: “[verb object][; qualifiers][; returns …]”
    - Limit length to 120 chars; prefer removing qualifiers before the returns clause.
-  Validation and confidence:
    - Checks:
        - Starts with a verb.
        - No code/quotes.
        - Contains at least one noun or return hint if available.
        - Length ≤ 120.
    - Confidence scoring (0–1):
        - +0.4 if verb found, +0.2 if object found, +0.2 if return parsed, +0.1 if qualifiers detected, +0.1 if passed all validators.
        - Threshold (e.g., 0.75) triggers “good enough.”
-  Fallback:
    - If confidence < threshold, call LLM with a constrained prompt; cap tokens and retries.

### Budgeting

-  Config:
    - max_llm_calls: hard cap per run.
    - max_tokens_per_call: for predictable latency and cost.
-  Accounting:
    - Maintain a counter; refuse fallback once exhausted; mark summary as rule_based_insufficient if needed.

### LLM client (isg-infer-client)

-  Configuration:
    - endpoint: http://127.0.0.1:8000/generate (local TGI/vLLM/Ray Serve)
    - model_id: bigcode/starcoder2-3b
    - timeout_ms, retries, backoff, stop sequences.
-  Request/response:
    - 
        POST /generate
        {
            "model": "bigcode/starcoder2-3b",
            "prompt": "You are a senior engineer... One-line summary:",
            "max_new_tokens": 40,
            "temperature": 0.2,
            "stop": ["\n\n", "\n#"]
        }
    - 
        200 OK
        {
            "generated_text": "Parse TOML into Cfg; returns Result<Cfg,E>."
        }

Optional Ray:
-  Run Ray Serve with a deployment exposing the same contract.
-  Rust stays unchanged; only endpoint switches.

### interface-graph-builder: algorithm details

-  Workspace discovery:
    - cargo metadata to enumerate packages/targets.
    - Apply include/exclude globs.
-  Parsing variants:
    - rust-analyzer overlays:
        - Start r-a in proc or connect via LSP; request symbol tree; filter top-level functions/traits/structs/interfaces as “interfaces” for ISGL1.
    - rustdoc JSON:
        - Generate once; parse items; map to files via spans; extract signatures and generics/where clauses.
    - syn fallback:
        - Parse files; limited macro expansion; good for macro-lite crates.
-  ISGL1 key derivation:
    - isgl1_key = "{path}-{symbol-name}" (normalized path and symbol name).
-  Facets extraction (L2/L3):
    - generics: type params; where-clauses; visibility; async/unsafe; attributes (derive/serde); doc presence.
-  Persistence:
    - Upsert interfaces and facets; write snapshot JSON.

### code-summary-gen: algorithm details

-  Selection:
    - From CLI: explicit isgl1_keys or “all missing summaries.”
-  For each node:
    - Fetch interface row; load file content; slice span for snippet if needed.
    - Run heuristics → validate/score.
    - If score < threshold and budget remains → LLM:
        - Prompt includes name, normalized signature, and small code header snippet.
    - Upsert summaries:
        - Key: (interface_id, model_id, prompt_hash, params_hash).
        - Fields: one_line, provenance (rule_based|llm_assisted), confidence, created_at.
-  Idempotency:
    - If a summary exists with same keys, skip unless overwrite flag is set.

## Rust skeletons

-  interface-graph-builder (CLI)

    ```rust
    use clap::Parser;
    use isg_core::{Config, Isgl1Item};
    use isg_cozo::CozoClient;
    use isg_parse::{parse_workspace_isgl1};
    use std::path::PathBuf;

    #[derive(Parser, Debug)]
    struct Args {
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        #[arg(long)]
        out: String, // e.g., "cozo://isg"
        #[arg(long)]
        snapshot: Option<PathBuf>,
        #[arg(long, num_args = 1.., value_delimiter = ',')]
        include: Vec<String>,
        #[arg(long, num_args = 1.., value_delimiter = ',')]
        exclude: Vec<String>,
        #[arg(long, default_value_t = 8)]
        concurrency: usize,
    }

    #[tokio::main]
    async fn main() -> anyhow::Result<()> {
        let args = Args::parse();
        let cfg = Config::from_args(&args)?;
        let cozo = CozoClient::connect(&args.out).await?;

        let items: Vec<Isgl1Item> =
            parse_workspace_isgl1(&args.repo, &args.include, &args.exclude, args.concurrency).await?;

        cozo.upsert_interfaces(&items).await?;
        if let Some(path) = args.snapshot {
            isg_core::write_snapshot_json(&items, path)?;
        }
        Ok(())
    }
    ```

-  code-summary-gen (CLI)

    ```rust
    use clap::Parser;
    use isg_core::{SummaryParams};
    use isg_cozo::CozoClient;
    use isg_heuristics::{summarize_rule_based, validate_and_score};
    use isg_infer_client::InferClient;

    #[derive(Parser, Debug)]
    struct Args {
        #[arg(long)]
        cozo: String,
        #[arg(long, num_args = 1.., value_delimiter = ',')]
        isgl1_keys: Vec<String>,
        #[arg(long, default_value = "rule-based")]
        model: String,          // "rule-based" | "llm" | "hybrid"
        #[arg(long, default_value = "llm")]
        fallback: String,       // "none" | "llm"
        #[arg(long, default_value_t = 50)]
        batch_size: usize,
        #[arg(long, default_value_t = 200)]
        max_llm: usize,
        #[arg(long)]
        write: Option<String>,  // "summaries"
    }

    #[tokio::main]
    async fn main() -> anyhow::Result<()> {
        let args = Args::parse();
        let cozo = CozoClient::connect(&args.cozo).await?;
        let infer = InferClient::from_env()?; // endpoint/model from env or config

        let mut llm_remaining = args.max_llm;
        let targets = cozo.fetch_targets(&args.isgl1_keys).await?;

        for chunk in targets.chunks(args.batch_size) {
            for item in chunk {
                let rb = summarize_rule_based(&item)?;
                let (ok, conf, text) = validate_and_score(&rb);
                let (summary, prov, conf) = if ok {
                    (text, "rule_based", conf)
                } else if args.fallback == "llm" && llm_remaining > 0 {
                    llm_remaining -= 1;
                    let prompt = isg_core::one_line_prompt(&item, &rb.hints);
                    let llm_text = infer.generate(&prompt).await.unwrap_or_default();
                    let llm_text = isg_heuristics::sanitize(llm_text);
                    let (_, conf2, clean) = validate_and_score(&llm_text);
                    (clean, "llm_assisted", conf2.max(conf))
                } else {
                    (rb.fallback_text, "rule_based_insufficient", conf)
                };
                let params = SummaryParams::current();
                cozo.upsert_summary(&item.interface_id, &params, &summary, prov, conf).await?;
            }
        }
        Ok(())
    }
    ```

-  Heuristics sketch

    ```rust
    pub struct RuleBased {
        pub text: String,
        pub hints: Vec<String>,
        pub fallback_text: String,
    }

    pub fn summarize_rule_based(item: &Isgl1Item) -> anyhow::Result<RuleBased> {
        let sig = &item.signature_norm;
        let name = item.symbol_name.as_str(); // e.g., "parse_cfg"
        let (verb, obj) = verb_object_from_name(name);
        let returns = returns_from_sig(sig); // e.g., "Result<Cfg,E>"
        let qualifiers = qualifiers_from_sig(sig); // e.g., "async"
        let mut parts = vec![format!("{} {}", verb, obj)];
        if let Some(q) = qualifiers { parts.push(q); }
        if let Some(r) = returns { parts.push(format!("returns {}", r)); }
        let mut text = parts.join("; ");
        if text.len() > 120 {
            text = truncate_smart(text, 120);
        }
        Ok(RuleBased { text: capitalize(&text), hints: vec![returns.unwrap_or_default()], fallback_text: capitalize(&format!("{} {}", verb, obj)) })
    }

    pub fn validate_and_score(s: &str) -> (bool, f32, String) {
        let clean = sanitize(s);
        let mut score = 0.0;
        if starts_with_verb(&clean) { score += 0.4; }
        if has_object_noun(&clean) { score += 0.2; }
        if mentions_returns(&clean) { score += 0.2; }
        if clean.len() <= 120 { score += 0.2; }
        (score >= 0.75, score.min(1.0), clean)
    }
    ```

-  Inference client sketch

    ```rust
    pub async fn generate(&self, prompt: &str) -> anyhow::Result<String> {
        let body = serde_json::json!({
            "model": self.model_id,
            "prompt": prompt,
            "max_new_tokens": self.max_new_tokens,
            "temperature": self.temperature,
            "stop": ["\n\n", "\n#"]
        });
        let resp = self.http.post(format!("{}/generate", self.endpoint))
            .json(&body)
            .timeout(self.timeout)
            .send()
            .await?
            .error_for_status()?;
        let v: serde_json::Value = resp.json().await?;
        Ok(v.get("generated_text").and_then(|x| x.as_str()).unwrap_or_default().to_string())
    }
    ```

## Optional Ray integration

-  Why: scale the LLM serving without changing Rust code.
-  How:
    - Deploy a Ray Serve app exposing /generate with your StarCoder2 3B backend (PyTorch/vLLM).
    - The Rust isg-infer-client points to that endpoint.
    - If you need queue control, add a simple /health and /config endpoints.

## Configuration example (YAML)

-  config.yaml

    ```yaml
    repo_root: .
    include:
        - crates/**
    exclude:
        - target/**
    cozo:
        uri: cozo://isg
    summarization:
        mode: hybrid
        confidence_threshold: 0.75
        max_llm_calls: 200
        model_id: bigcode/starcoder2-3b
        endpoint: http://127.0.0.1:8000
        max_new_tokens: 40
        temperature: 0.2
    runtime:
        parse_concurrency: 8
        summarize_concurrency: 16
    ```

## Reliability-first profile

-  Deterministic IDs and normalization for signatures.
-  Bounded concurrency (Tokio Semaphore) for parse and LLM calls.
-  Cooperative yields during large writes and HTTP calls.
-  Caching:
    - Skip unchanged items via (interface_id, code_hash).
    - Skip summaries if existing with same (model_id, prompt_hash, params_hash).
-  Observability:
    - tracing for spans: parse_file, extract_interfaces, summarize_rule_based, summarize_llm.
    - metrics: summaries_total, summaries_llm_total, summarize_latency_ms_hist.
-  Error surfaces:
    - Rich diagnostics on parse and network errors; summaries.error captures failures for later retries.

## Milestone plan

-  M1: Scaffolding
    - Crates, shared types, hashing, config loader, CLI skeletons.
-  M2: Graph builder (Rust crates)
    - rustdoc JSON path first; ISGL1 key extraction; facets: generics/where; Cozo upserts; snapshot JSON.
-  M3: Heuristics engine
    - Verb-object extraction, return parsing, validators, confidence scoring; Cozo summaries upsert.
-  M4: LLM fallback
    - Inference client; endpoint config; budget control; retries; provenance/confidence.
-  M5: Optional Ray backend
    - Stand up Ray Serve; load StarCoder2 3B; benchmark and tune concurrency.
-  M6: Incremental updates and caching
    - code_hash checks; prompt_hash/params_hash invalidation; CLI flags for overwrite/dry-run.
-  M7: QA and tuning
    - Fixtures; tests; performance; observability dashboards.

## What I need from you to finalize scaffolding

-  Initial target: Rust-only repos (using rustdoc JSON), or include other languages now (tree-sitter)?
-  LLM endpoint preference: local transformers server vs Ray Serve vs external.
-  CozoDB mode: embedded or server URI.
-  Confidence threshold and budget defaults.

---

amuldotexe: Integrate this brilliant ideation with below - focus on HLD LLD Interface reasoning and not on exact code which you can write once you figure out HLD LLD

- Constraints of tech stack
    - llama.cpp because Ollama does not allow parallelism
    - CozoDB because it is a graph database
    - We will be based on Claude Code as a plugin or skill or something because we want to focus on the core differentiation which is ISG & similar Aggregated Code Context which can ensure reliable bug solving with high accuracy
        - Hence ideally whatever we want to be part of Parseltongue could be a Cargo Library or Tool, it ensures we can experminent with combination permutations
        - We need to FOCUS HARD on MVP 1.0 Scope and note down everything else as Backlog
    - Core Data Model: Interface Signature Graph (ISG)
        - Nodes: function signatures, impl blocks, trait items, types, public APIs, macro-expanded interfaces.
        - Edges: CALLS, IMPLEMENTS, USES, DEPENDS, REQUIRES_BOUND, FEATURE_GATED_BY.
        - Levels: ISGL1 (interface node keyed as filepath-filename-InterfaceName, 1 level below file/module), ISGL2/ISGL3 are constituents under ISGL1 used for understanding only.
        - Store: CozoDB (Datalog + HNSW) with columnar payloads for fast filters and range scans.
    - CodeGraph (single write surface)
        - indexed by ISGL1 key (filepath-filename-InterfaceName)
        - columns (minimal, opinionated):
            - Current_Code (canonical pre-edit slice),
            - Future_Code (candidate patch slice, ephemeral until approval),
            - Future_Action (None|Create|Edit|Delete),
            - TDD_Classification (TEST_IMPLEMENTATION, CODE_IMPLEMENTATION)
            - current_id (0/1: 0 meaning NOT in current code, 1 meaning in current code),
            - future_id (0/1: 0 meaning NOT in future code, 1 meaning in future code)
        - Rule: All code-iteration writes happen only in CodeGraph. All other CozoDB tables (ISG nodes/edges, embeddings, pattern KB indices) are read-only context stores and never mutate code.
        - Flow: PreFlight compiles Future_Code via RA overlay; on approval, flip Future→Current, clear future_* flags
    - Many types of ISGs can be created
        - ISGL1 (interface node keyed as filepath-filename-InterfaceName, 1 level below file/module)
        - ISGL2 is 1 extra distance below file / module for e.g. if a function is inside main function in main.rs that will be ISGL3
        - ISGL3 is 1 extra distance below ISGL2 for e.g. if a function is inside main function and then further inside another function in main.rs that will be ISGL3
        - ISGs are a proxy for what we called Aggregated Code Context
            - It is not just about interface signatures, it can be any pyramidal way to aggregate the codebase
            - for e.g. we can using sub-agents generically summarize all ISGL1 related code blobs to 1 liner summaries, and that will be significantly HQ code context and much lesser than the actual codebase
                - a variation of this can be running small agents with context of the PRD
            - for e.g. we can try to get HIR or other Rust analyzer meta-data for all ISGL1 related code blobs and that will be significantly HQ code context and much lesser than the actual codebase


    - Table Codebase has following columns
        - id
        - ISG_current_ind (0 or 1)
        - ISG_future_ind (0 or 1)
        - filePath-fileName-interfaceName as unique identifier
        - Current_Code
        - Future_Code (empty at initiation)
        - Future_Action (empty at initiation, but filled with whether to suit the PRD change we will edit or delete or create this new interface)


---

Raycast AI: 

## MVP 1.0 scope (laser-focused)

-  Languages: Rust only.
-  ISG levels: ISGL1 required; ISGL2/ISGL3 stored as read-only “facets” under L1 for understanding (no writes).
-  Graph content:
    - Nodes: top-level functions, impl items (methods), trait items, types, public APIs, macro-expanded interfaces if resolvable.
    - Edges: CALLS, IMPLEMENTS, USES, DEPENDS (coarse), REQUIRES_BOUND (basic bounds), FEATURE_GATED_BY (feature flags).
-  Single write surface: CodeGraph table only (for code slices and actions).
-  Summaries: rule-based first; LLM fallback via llama.cpp server; provenance and confidence recorded.
-  Claude Code integration: expose both CLIs; optional library entrypoints callable by a plugin/skill; no UI work—return JSON and file links.
-  Performance: bounded concurrency; stable IDs; idempotent upserts; sub-100ms reads for common queries using Cozo indexes and columnar payloads.

Everything else goes to backlog.

## High-Level Design (HLD)

-  Components
    - interface-graph-builder (Rust CLI/lib)
        - Build ISGL1 keys, attach ISGL2/ISGL3 facets, derive edges, upsert to Cozo (read-only ISG spaces), and emit JSON snapshot.
    - code-summary-gen (Rust CLI/lib; alias: interface-summary-generator)
        - Generate one-line summaries for ISGL1 nodes using rule-based heuristics with llama.cpp fallback; write to Summaries relation.
    - llama.cpp server (shared inference)
        - GGUF model (e.g., StarCoder2 3B quantized); serves HTTP endpoints with parallel sessions.
    - CozoDB
        - Stores ISG nodes/edges, facets, summaries (read-only for code), and CodeGraph (the only write surface for code slices/intent).
    - Claude Code plugin/skill wrapper
        - Calls CLIs/libs, shows summaries and ISG context, and writes proposed edits into CodeGraph (Future_* fields), never mutating code directly.

-  Trust and write boundaries
    - ISG nodes/edges, embeddings, pattern KB: read-only context stores.
    - CodeGraph: sole table accepting code writes (Current_Code/Future_Code and action flags).
    - PreFlight compile happens against Future_Code via rust-analyzer overlay; approval gate flips Future → Current.

-  Data flow overview
    - Build ISG:
        - Scan repo → parse (rustdoc JSON or rust-analyzer overlays) → compute ISGL1 keys → extract L2/L3 facets → compute edges → upsert to Cozo → snapshot JSON.
    - Summarize:
        - Query ISGL1 nodes missing a summary → run heuristics → validate → if low-confidence, send prompt to llama.cpp → upsert Summary with provenance/confidence.
    - Code iteration:
        - Claude Code proposes changes → writes only to CodeGraph.Future_Code + Future_Action + future_id=1 → PreFlight compiles → on approval, flip to Current_*, clear Future_*.

-  Deployment (single machine)
    - CozoDB embedded or local server.
    - llama.cpp server process with configured n_parallel, threads, context.
    - Two Rust CLIs and shared libraries (crates) for programmatic integration.

## Low-Level Design (LLD)

### Core keys and identifiers

-  isgl1_key: "{filePath}-{fileName}-{interfaceName}" (normalized path + symbol)
-  interface_id: blake3("{repo_id}|{commit_sha}|{lang}|{path}|{signature_norm}")
-  code_hash: blake3(file_bytes)
-  summary key: (interface_id, model_id, prompt_hash, params_hash)

### Cozo logical schema (conceptual)

| Relation | Primary key | Columns (selected) | Purpose |
|---|---|---|---|
| isg_nodes | interface_id | isgl1_key, repo_id, commit_sha, lang, file_path, kind, signature_norm, span_start, span_end, code_hash, level (L1/L2/L3) | Canonical interface nodes (read-only) |
| isg_edges | (src_id, dst_id, kind) | kind in {CALLS, IMPLEMENTS, USES, DEPENDS, REQUIRES_BOUND, FEATURE_GATED_BY}, attrs | Graph edges (read-only) |
| isg_facets | (interface_id, facet_kind) | facet_value (JSON) | Store ISGL2/ISGL3 and extra attributes (read-only) |
| summaries | (interface_id, model_id, prompt_hash, params_hash) | one_line, provenance in {rule_based, llm_assisted, rule_based_insufficient}, confidence, created_at, error | One-line summaries cache |
| embeddings (optional) | (entity_id, model) | vector, dim, created_at | For HNSW semantic search (read-only) |
| codegraph | isgl1_key | Current_Code, Future_Code, Future_Action, TDD_Classification, current_id, future_id, last_updated, author | Only write surface for code slices |

-  Invariants
    - isg_nodes/isg_edges/summaries/embeddings are never used to write code.
    - codegraph.current_id ∈ {0,1}, future_id ∈ {0,1}; (current_id, future_id) transitions follow a strict state machine.
    - isgl1_key unique across codegraph; interface_id unique across isg_nodes.

### CodeGraph row state machine

-  Fields involved: Current_Code, Future_Code, Future_Action, current_id, future_id.
-  States (simplified):
    - Clean: current_id=1, future_id=0, Future_Action=None, Future_Code=""
    - Draft: current_id can be 0 or 1, future_id=1, Future_Action ∈ {Create|Edit|Delete}, Future_Code set for Create/Edit
    - Approved: transition event flips Future→Current
-  Transitions:
    - Propose:
        - Clean → Draft: set future_id=1; set Future_Action; set Future_Code (except Delete).
    - PreFlight (compile with RA overlay):
        - Draft → Draft: attach diagnostics result; block Approval if errors (unless override).
    - Approve:
        - Draft → Clean: Current_Code = Future_Code (or empty on Delete); update current_id (1 for Create/Edit, 0 for Delete); clear Future_*; future_id=0.
    - Reject:
        - Draft → Clean: clear Future_*; future_id=0; no change to Current_*.

### Interfaces and contracts

-  CLIs (stable surfaces)
    - interface-graph-builder
        - Inputs: repo path, include/exclude globs.
        - Outputs: upserts to isg_nodes/isg_edges/isg_facets, JSON snapshot file.
        - Flags: --repo, --out cozo://path, --snapshot, --include, --exclude, --concurrency.
    - code-summary-gen (alias: interface-summary-generator)
        - Inputs: Cozo URI, optional list of isgl1_keys or a selection rule (“missing summaries”).
        - Outputs: upserts to summaries with provenance/confidence; JSON lines to stdout for integration.
        - Flags: --cozo, --mode (rule|llm|hybrid), --confidence-threshold, --max-llm, --batch-size, --write summaries.

-  Library traits (for Claude Code plugin integration)
    - IsgBuilder
        - build_isg(repo_root, include, exclude) → {nodes, edges, facets, snapshot_path}
    - Summarizer
        - summarize_targets(isgl1_keys, mode, budget) → stream of {isgl1_key, one_line, provenance, confidence}
    - CodeGraphApi
        - get(isgl1_key) → row
        - propose(isgl1_key, action, future_code, tdd_class) → row (Draft)
        - preflight(isgl1_key) → diagnostics
        - approve(isgl1_key) → row (Clean)
        - reject(isgl1_key) → row (Clean)

-  llama.cpp server endpoint (assumed contract)
    - Endpoint: POST /completion (or /v1/completions depending on build)
    - Request:
        - 
            {
                "model": "starcoder2-3b-q4_0.gguf",
                "prompt": "<one-line-summary-prompt>",
                "n_predict": 40,
                "temperature": 0.2,
                "stop": ["\n\n", "\n#"],
                "cache_prompt": true
            }
    - Response:
        - 
            {
                "content": "Parse TOML into Cfg; returns Result<Cfg,E>."
            }
    - Notes:
        - Use server’s parallel sessions (n_parallel) for concurrent requests.
        - Keep prompts short, deterministic, and with stop tokens.

-  Claude Code plugin/skill boundaries
    - Calls library or CLI with:
        - Build ISG for current repo on-demand or cached.
        - Request summaries for visible files or selection.
        - Propose Future_Code edits into CodeGraph.
        - Show PreFlight diagnostics; offer approve/reject which call CodeGraphApi.

### Pipelines (end-to-end)

-  ISG build (interface-graph-builder)
    - Discover: cargo metadata → filter include/exclude.
    - Parse:
        - Primary: rustdoc JSON for item inventory + spans (stable).
        - Overlay: rust-analyzer for validation and macro-expanded interfaces (where available).
    - Derive:
        - ISGL1: top-level items keyed as filepath-filename-InterfaceName; compute signature_norm and spans.
        - L2/L3 facets: child items “under” L1 for understanding (no writes).
        - Edges: CALLS (from HIR/rustdoc refs), IMPLEMENTS (impl to trait), USES/DEPENDS (imports and crate deps), REQUIRES_BOUND (where-clauses), FEATURE_GATED_BY (cfg attrs).
    - Persist:
        - Upsert into isg_nodes, isg_edges, isg_facets; write snapshot JSON.
    - Guarantees:
        - Idempotent upserts keyed by interface_id; consistent isgl1_key mapping; zero code writes.

-  Summarization (code-summary-gen)
    - Select targets:
        - ISGL1 nodes without a recent summary for (model_id, prompt_hash, params_hash).
        - Or explicit isgl1_keys from CLI or plugin.
    - Rule-based pass:
        - Extract verb and object from symbol name (snake_case → words).
        - Parse returns from signature_norm (Result<T,E>, Option<T>, etc.).
        - Add qualifiers: async/unsafe/trait/interface; optionally input nouns.
        - Compose ≤ 120 chars, start with a verb, no code/quotes; validate and score.
    - LLM fallback (llama.cpp):
        - Triggered if confidence < threshold and budget allows.
        - Prompt includes: Name, normalized header, and minimal snippet around signature.
        - Use small n_predict, low temperature, and stop tokens; sanitize output; revalidate.
    - Persist:
        - Upsert summary with provenance (rule_based|llm_assisted) and confidence.
    - Budget and throttling:
        - Hard cap max_llm calls; concurrency bounded to llama.cpp n_parallel; exponential backoff on 429/5xx.

-  Code iteration via CodeGraph
    - Propose:
        - Plugin writes Future_Code + Future_Action + future_id=1 for isgl1_key.
    - PreFlight:
        - Run rust-analyzer overlay build using future slices; attach diagnostics to the row (non-persistent or a diagnostics side table).
    - Approve:
        - Flip: Current_Code ← Future_Code; set current_id accordingly; clear Future_*; future_id=0.
    - Reject:
        - Clear Future_*; future_id=0.
    - Rule:
        - All edits go through CodeGraph; ISG and Summaries stay read-only.

## Capacity planning and concurrency with llama.cpp

-  One shared llama.cpp server can handle parallel requests. Size your client concurrency to match server capacity.
-  Compute safe parallelism:
    \[
    n_{\text{safe}} = \min\left(n_{\text{parallel\_server}},\ \left\lfloor \frac{0.9 \cdot \text{free\_RAM}}{\text{mem\_per\_session}} \right\rfloor\right)
    \]
    - mem_per_session ≈ base_model_resident_overhead + KV_cache_per_context
    - For a 3B GGUF Q4-ish: base 1.5–2.0 GB; KV cache a few hundred MB per long context; keep contexts short for summaries.

-  Practical defaults (CPU):
    - n_parallel_server: 4–8
    - n_threads per request: half to full core count divided by n_parallel
    - context length: 2k–4k tokens is plenty for signatures/snippets

## Observability and reliability

-  Metrics
    - ISG build: files processed, nodes/edges counts, failures.
    - Summarization: rule vs LLM hit ratios, P95 latency, error rates, budget usage.
    - CodeGraph: PreFlight pass/fail rates, approval latency.
-  Logs and tracing
    - Trace IDs per isgl1_key through parse → summarize → codegraph changes.
-  Caching
    - Skip summarization if a valid summary exists for the same (model_id, prompt_hash, params_hash).
    - Skip ISG node rebuild if code_hash unchanged for the file.
-  Failure handling
    - ISG parse errors → record diagnostics; continue.
    - llama.cpp timeouts/5xx → retry with backoff; if still failing, record rule_based_insufficient.
    - CodeGraph approval must be transactional: flip and clear in a single commit.

## Configuration surfaces

-  isg.yaml
    - repo_root, include, exclude
    - cozo.uri
    - runtime: parse_concurrency, summarize_concurrency
-  summarization.yaml
    - mode: rule|llm|hybrid
    - confidence_threshold
    - llama:
        - endpoint
        - model_id (gguf filename)
        - n_predict, temperature, stop
        - client_concurrency (≤ n_safe)
    - budgeting:
        - max_llm_calls per run
-  codegraph policy
    - preflight_required: true
    - approve_requires_clean_diagnostics: true|false
    - tdd_default: TEST_IMPLEMENTATION|CODE_IMPLEMENTATION

## Example I/O contracts (JSON)

-  interface-summary-generator input
    - 
        {"isgl1_keys":["src/lib.rs-foo-parse_cfg"]}
-  interface-summary-generator output
    - 
        {"isgl1_key":"src/lib.rs-foo-parse_cfg","summary":"Parse TOML into Cfg; returns Result<Cfg,E>","provenance":"rule_based","confidence":0.86}

-  CodeGraph propose (plugin → library)
    - 
        {"isgl1_key":"src/lib.rs-foo-parse_cfg","Future_Action":"Edit","Future_Code":"pub fn parse_cfg(...){...}","TDD_Classification":"CODE_IMPLEMENTATION"}

-  CodeGraph approve (library → DB)
    - 
        {"isgl1_key":"src/lib.rs-foo-parse_cfg","event":"approve","result":"flipped_future_to_current"}

## Interfaces reasoning recap

-  Stable identity: isgl1_key bridges ISG and CodeGraph; interface_id ties summaries and facets to a specific code version via code_hash and commit_sha.
-  Single-writer rule: only CodeGraph mutates code; ISG and summaries remain authoritative context snapshots aligned to commits and code_hash—safe for read-heavy AI workflows.
-  Determinism: signature_norm and ID derivations are canonical; rule-based summaries are deterministic; LLM fallback is constrained and recorded with provenance/confidence for auditability.
-  Isolation: llama.cpp is a shared service; clients are stateless and bounded; no model loading in worker processes.
-  Claude Code plugin is a client of these stable interfaces; it composes tools without breaking the single-writer and read-only guarantees.

## MVP 1.0 acceptance checklist

-  Build ISGL1 for Rust repos; edges for CALLS, IMPLEMENTS, USES; basic REQUIRES_BOUND and FEATURE_GATED_BY.
-  Persist ISG nodes/edges/facets to Cozo; write JSON snapshot.
-  Generate one-line summaries for ISGL1 with rule-first, llama.cpp fallback; store provenance and confidence.
-  Expose CodeGraph with the specified columns and state transitions; enforce PreFlight compile before approve.
-  Bounded concurrency; configurable llama.cpp endpoint; idempotent upserts; basic metrics logs.

## Backlog (not in MVP 1.0)

-  ISGL2/ISGL3 full graph traversal and cross-file edges beyond immediate scope.
-  Multi-language support (TS/Java/Go) via tree-sitter; generalized ISG schema for polyglot repos.
-  HNSW embeddings for semantic search across ISG; pattern KB indexes.
-  Domain-specific summary templates; PRD-aware small agents; reranking.
-  GPU-backed llama.cpp and dynamic quantization switching.
-  Advanced rust-analyzer overlays for macro-heavy code; full HIR extraction.
-  Batch PreFlight for multiple CodeGraph rows; change-impact analysis via graph traversal.
-  Formal Cozo views for common queries and analytics dashboards.

This HLD/LLD keeps the system coherent under your constraints (llama.cpp, CozoDB, Claude Code plugin) while preserving the single-writer contract and enabling iterative expansion once MVP 1.0 lands.