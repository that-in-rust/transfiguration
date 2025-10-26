# Commmand Lines

## Parallel Summarizer

### Quick Start - Best for Code Analysis (Recommended)
```bash
# Use the built release binary for best performance
./target/release/parallel_summarizer \
    --file ./tests/fixtures/iggy_apache.txt \
    --output-file /tmp/iggy_summary.md \
    --results-file /tmp/iggy_progress.log \
    --loc 300 \
    --prompt "Create a concise 2-3 line summary of this code chunk focusing on its main functionality and purpose." \
    --agent-count 10 \
    --model-name qwen2.5-0.5b-int4 \
    --sampling-strategy beam \
    --num-beams 3 \
    --early-stopping \
    --temperature 0.30 \
    --max-new-tokens 60 \
    --min-length 35
```

### Large Scale Processing (1.25M LOC - 3 minutes)
```bash
# Process massive codebases with 10x parallelism
./target/release/parallel_summarizer \
    --file ./large_codebase/full_project.txt \
    --output-file /tmp/full_summary.md \
    --results-file /tmp/full_progress.log \
    --loc 300 \
    --prompt "Summarize this code in 2-3 lines, focusing on architecture patterns and key functionality." \
    --agent-count 10 \
    --model-name qwen2.5-0.5b-int4 \
    --sampling-strategy beam \
    --num-beams 3 \
    --early-stopping \
    --temperature 0.30 \
    --max-new-tokens 60 \
    --min-length 35
```


## localLLM browsing

Based on my research of Anthropic API patterns, LiteLLM proxy configurations, and local LLM server conventions, here's the recommended command line format for your Rust tool that creates an Anthropic-type key and local endpoint for claude-code integration:


Following your existing CLI patterns and the 4-word naming convention from your project memories, here's the suggested format:

```bash
./target/release/local_llm_endpoint \
    --model-path /absolute/path/to/model.onnx \
    --tokenizer-path /absolute/path/to/tokenizer \
    --host 127.0.0.1 \
    --port 4000 \
    --anthropic-version 2023-06-01 \
    --api-key sk-local-$(date +%s) \
    --max-tokens 4096 \
    --temperature 0.7 \
    --top-p 0.9 \
    --context-window 8192 \
    --enable-streaming \
    --enable-cors \
    --log-level info \
    --config-file /path/to/endpoint-config.yaml
```

## Key Design Decisions

**Anthropic API Compatibility:**
- Uses standard Anthropic headers (`x-api-key`, `anthropic-version`, `content-type`)
- Implements `/v1/messages` endpoint for claude-code compatibility
- Supports streaming responses for real-time interaction

**Model Configuration:**
- **Absolute paths** for both model and tokenizer (as you mentioned)
- Support for ONNX models (matching your existing inference pipeline)
- Configurable context window and generation parameters

**Network & Security:**
- Configurable host/port for local deployment
- Dynamic API key generation with timestamp for uniqueness
- CORS support for web-based claude-code usage

**Integration Features:**
- Compatible with LiteLLM proxy patterns I found in the documentation
- Environment variable support (`ANTHROPIC_BASE_URL`, `ANTHROPIC_API_KEY`)
- Structured logging for debugging


For different deployment scenarios:

```bash
# Development mode with debug logging
./target/release/local_llm_endpoint \
    --model-path ./models/qwen2.5-0.5b-int4.onnx \
    --host localhost \
    --port 11434 \
    --log-level debug

# Production deployment
./target/release/local_llm_endpoint \
    --config-file /etc/dobby/endpoint-config.yaml \
    --log-level warn

# Custom model with specific parameters
./target/release/local_llm_endpoint \
    --model-path /models/custom-reasoning-model.onnx \
    --tokenizer-path /models/custom-tokenizer \
    --context-window 16384 \
    --max-tokens 8192 \
    --temperature 0.3
```


The tool should also support these environment variables for seamless claude-code integration:

```bash
export ANTHROPIC_BASE_URL=http://localhost:4000
export ANTHROPIC_API_KEY=sk-local-$(date +%s)
export CLAUDE_CODE_API_KEY_HELPER_TTL_MS=3600000
```

This format aligns with your existing Rust CLI patterns using clap, provides comprehensive configuration options, and ensures full compatibility with claude-code's expected Anthropic API interface.
