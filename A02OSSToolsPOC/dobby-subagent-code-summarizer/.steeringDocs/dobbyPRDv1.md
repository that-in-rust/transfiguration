# Dobby Code Summarizer - Product Requirements

## Project Overview

Dobby is a Rust-based code summarization tool that uses intelligent parsing and parallel AI inference to generate concise summaries of codebases. The project is organized as a Cargo workspace with two focused crates:

1. **`semantic-code-chunker`** - Tree-sitter based semantic code parsing and chunking
2. **`parallel-inference-engine`** - 10x parallel AI model inference for summarization

## Architecture

```
dobby-subagent-code-summarizer/
├── Cargo.toml                    # Workspace root
├── semantic-code-chunker/        # Crate 1: Parsing & chunking
│   ├── src/
│   │   ├── lib.rs               # Main chunking interface
│   │   ├── tree_sitter.rs       # Tree-sitter integration
│   │   ├── languages.rs         # Language-specific parsers
│   │   ├── chunking.rs          # Semantic chunking logic
│   │   └── text_chunker.rs      # Non-code text chunking
│   └── Cargo.toml
└── parallel-inference-engine/    # Crate 2: AI processing
    ├── src/
    │   ├── lib.rs               # Main inference interface
    │   ├── models.rs            # Model management
    │   ├── parallel.rs          # 10x parallel processing
    │   ├── inference.rs         # AI inference logic
    │   └── cli.rs               # Command-line interface
    └── Cargo.toml
```

---

## Feature 1: Semantic Code Chunking (`semantic-code-chunker`)

### Purpose
Intelligently parse and segment codebases into semantic chunks rather than arbitrary line counts, using Tree-sitter for language-aware analysis.

### Key Capabilities
- **Tree-sitter Integration**: AST-based parsing for 20+ programming languages
- **Semantic Boundaries**: Chunk based on functions, classes, modules, and logical blocks
- **Token-based Sizing**: Optimize chunks by token count rather than lines
- **Language Awareness**: Different chunking strategies per language
- **Non-code Support**: Text chunking for documentation and configuration files

### Chunking Strategies

#### Code Files (Tree-sitter based):
```rust
// Semantic boundaries for different languages
Rust: fn, impl, struct, enum, mod, trait blocks
Python: def, class, async def, with blocks
JavaScript: function, class, method, arrow functions
TypeScript: interface, type, class, function
Go: func, method, struct, interface definitions
Java: class, method, interface, enum definitions
```

#### Non-code Files:
```rust
// Smart text chunking preserving context
Markdown: Section headers (# ## ###), code blocks
JSON/YAML: Logical object/group boundaries
Plain Text: Paragraph and sentence boundaries
Configuration: File section boundaries
```

### CLI Interface

```bash
# Basic semantic chunking
dobby chunk \
    --input ./src \
    --output ./chunks.json \
    --max-tokens 1000 \
    --preserve-context

# Language-specific chunking
dobby chunk \
    --input ./rust-project \
    --output ./rust-chunks.json \
    --languages rust,toml \
    --strategy semantic \
    --include-exports

# Mixed content processing
dobby chunk \
    --input ./mixed-repo \
    --output ./all-chunks.json \
    --code-strategy semantic \
    --text-strategy paragraph \
    --context-window 2048
```

### Parameters
- `--input`: Path to file or directory (required)
- `--output`: Output file for chunk metadata (required)
- `--max-tokens`: Maximum tokens per chunk (default: 1000)
- `--languages`: Comma-separated languages to process
- `--strategy`: Chunking strategy (semantic, hybrid, lines)
- `--preserve-context`: Include surrounding context for each chunk
- `--include-exports`: Only chunk public/exported functions and types
- `--context-window`: Model context window size for optimization

---

## Feature 2: Parallel Inference Engine (`parallel-inference-engine`)

### Purpose
Process semantic code chunks in parallel using AI models to generate concise, contextually aware summaries.

### Key Capabilities
- **10x Parallel Processing**: Concurrent inference with semaphore control
- **Multiple Model Support**: TinyLlama-1.1B, DeepSeek-Coder-1.3B, Gemma-2-2B
- **Intelligent Routing**: Route chunks to appropriate models based on content type
- **Session Reuse**: Shared model sessions for memory efficiency
- **Progress Tracking**: Real-time processing status and error recovery

### Model Routing Strategy

```rust
// Intelligent model selection based on chunk content
Code Chunks: deepseek-coder-1.3b (code-specialized)
Documentation: gemma-2-2b (highest quality)
Large Chunks: tinyllama-1.1b (fast processing)
Mixed Content: auto-select based on content analysis
```

### CLI Interface

```bash
# Process pre-chunked files
dobby summarize \
    --chunks ./chunks.json \
    --output ./summary.md \
    --model auto \
    --parallel 10

# Direct file processing with integrated chunking
dobby process \
    --input ./src \
    --output ./analysis.md \
    --chunking-strategy semantic \
    --model deepseek-coder-1.3b \
    --parallel 15

# High-quality documentation generation
dobby process \
    --input ./lib \
    --output ./docs/README.md \
    --model gemma-2-2b \
    --quality-mode thorough \
    --include-signatures

# Batch processing multiple repositories
dobby batch \
    --input ./repos/ \
    --output ./summaries/ \
    --pattern "*/src/*.rs" \
    --model deepseek-coder-1.3b \
    --parallel 20
```

### Parameters
- `--chunks`: Pre-computed chunk file (from `dobby chunk`)
- `--input`: Direct file/directory input (triggers integrated chunking)
- `--output`: Summary output file (required)
- `--model`: Model selection or "auto" for intelligent routing
- `--parallel`: Number of parallel agents (default: 10, max: 100)
- `--chunking-strategy`: Strategy for on-the-fly chunking
- `--quality-mode`: Processing quality (fast, balanced, thorough)
- `--include-signatures`: Include function/type signatures in summaries
- `--pattern`: Glob pattern for file selection (batch mode)

---

## Configuration

### Chunking Configuration (`chunking.yaml`)
```yaml
chunking:
  max_tokens: 1000
  preserve_context: true
  context_overlap: 100

languages:
  rust:
    boundaries: ["fn", "impl", "struct", "enum", "mod", "trait"]
    include_tests: false
  python:
    boundaries: ["def", "class", "async def", "with"]
    preserve_docstrings: true
  javascript:
    boundaries: ["function", "class", "method", "=>"]
    include_exports: true

strategies:
  code: "semantic"
  text: "paragraph"
  mixed: "hybrid"
```

### Model Configuration (`models.yaml`)
```yaml
models:
  tinyllama-1.1b:
    path: "./models/tinyllama-1.1b"
    ram_mb: 550
    context_length: 2048
    best_for: ["speed", "large_files"]

  deepseek-coder-1.3b:
    path: "./models/deepseek-coder-1.3b"
    ram_mb: 500
    context_length: 4096
    best_for: ["code", "technical_accuracy"]

  gemma-2-2b:
    path: "./models/gemma-2-2b"
    ram_mb: 680
    context_length: 8192
    best_for: ["quality", "documentation"]

routing:
  code_heavy: "deepseek-coder-1.3b"
  documentation: "gemma-2-2b"
  mixed_content: "auto"
  large_files: "tinyllama-1.1b"
```

