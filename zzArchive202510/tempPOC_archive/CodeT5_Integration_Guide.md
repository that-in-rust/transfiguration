# CodeT5-small Integration Guide

## Overview
This document outlines the path to integrate the real Salesforce/CodeT5-small model for actual code summarization instead of the current demonstration model.

## Current Status
✅ **Completed:**
- CodeT5-small model downloaded (233.8 MB)
- Model verified working in Python
- Basic tokenizer infrastructure implemented
- Pure ONNX inference pipeline established
- 20 parallel sessions working with demo model

🚧 **In Progress:**
- CodeT5-small to ONNX conversion (complex encoder-deoder architecture)

⏳ **Pending:**
- Real model inference performance testing
- 20 parallel sessions validation with real model

## Integration Path

### 1. Model Conversion (Complexity: High)
The main challenge is converting T5's encoder-decoder architecture to ONNX format.

**Approach A: Optimum Library (Recommended)**
```bash
pip install optimum[onnxruntime]
python3 convert_with_optimum.py
```

**Approach B: Custom ONNX Export**
- Export encoder and decoder separately
- Handle cross-attention mechanisms
- Manage cache states

**Approach C: Pre-converted Model**
- Download pre-converted CodeT5-small ONNX from HuggingFace
- Ensure compatibility with ONNX Runtime 2.0

### 2. Tokenizer Integration (Complexity: Medium)
**Current Status:** Basic tokenizer infrastructure exists but needs refinement.

**Required:**
- Fix JSON parsing for vocab.json (single-line format)
- Implement proper BPE tokenization
- Handle special tokens (<pad>, <s>, </s>, <unk>)
- Add prefix for summarization tasks: "summarize: "

### 3. Inference Pipeline Update (Complexity: Medium)
**Current Input/Output:**
```rust
// Current simple model
input: [1, 1, 28, 28] -> output: [1, 10]

// Required for CodeT5-small
input_ids: [batch, seq_len]
attention_mask: [batch, seq_len]
decoder_input_ids: [batch, 1]
decoder_attention_mask: [batch, 1]
output: [batch, 1, vocab_size]  // logits for next token
```

**Required Changes:**
1. Update `InferenceConfig` to use CodeT5 model path
2. Modify `preprocess_chunk_for_onnx()` for proper tokenization
3. Update `run_onnx_inference_static()` for 4-input model
4. Implement proper generation loop (auto-regressive)
5. Update `postprocess_output()` for vocabulary-based decoding

### 4. Memory and Performance (Complexity: Medium)
**CodeT5-small Specs:**
- Parameters: 60M (230MB model weights)
- Expected memory: ~400-500MB per instance
- Target: 20 parallel instances = ~8-10GB total

**Optimization Strategies:**
- Model quantization (int8) to reduce memory by ~4x
- Session pooling and reuse
- Batching where possible
- Memory-mapped model loading

## Implementation Steps

### Step 1: Complete Model Conversion
```bash
# Install optimum with proper version
pip install "optimum[onnxruntime]>=1.16"

# Convert model
python3 convert_with_optimum.py

# Verify conversion
python3 -c "
import onnxruntime as ort
import numpy as np
session = ort.InferenceSession('models/codet5-small-onnx/model.onnx')
print('Inputs:', session.get_inputs())
print('Outputs:', session.get_outputs())
"
```

### Step 2: Update Rust Inference Pipeline
```rust
// Update InferenceConfig
impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            model_path: "models/codet5-small-onnx/model.onnx".to_string(),
            input_name: "input_ids".to_string(),
            output_name: "logits".to_string(),
            // ... other fields
        }
    }
}

// Update preprocessing to use real tokenizer
fn preprocess_chunk_for_codet5(&self, chunk: &Chunk) -> Result<(Vec<i64>, Vec<i64>)> {
    let text = format!("summarize: {}", chunk.content);
    let tokenizer = create_codet5_tokenizer()?; // From tokenizer.rs

    let input_ids = tokenizer.encode_chunk_for_inference(&text)?;
    let attention_mask = vec![1; input_ids.len()]; // Simplified

    Ok((input_ids, attention_mask))
}
```

### Step 3: Implement Generation Loop
```rust
fn generate_summary(&self, input_ids: Vec<i64>, attention_mask: Vec<i64>) -> Result<String> {
    let mut decoder_input_ids = vec![0]; // decoder_start_token_id
    let mut generated_tokens = Vec::new();

    for _ in 0..self.config.max_output_length {
        // Run inference
        let logits = self.run_generation_inference(
            &input_ids,
            &attention_mask,
            &decoder_input_ids
        )?;

        // Get next token (argmax or sampling)
        let next_token = self.sample_next_token(&logits)?;

        // Check for end token
        if next_token == 2 { // </s> token
            break;
        }

        generated_tokens.push(next_token);
        decoder_input_ids.push(next_token);
    }

    // Decode tokens to text
    self.decode_tokens_to_text(&generated_tokens)
}
```

### Step 4: Performance Testing
```rust
#[tokio::test]
async fn test_codet5_parallel_performance() {
    let config = InferenceConfig::default();
    let pipeline = OnnxInferencePipeline::new(config)?;

    // Test 20 parallel sessions
    let tasks: Vec<_> = (0..20).map(|i| {
        let pipeline = &pipeline;
        let chunk = create_test_chunk(i);

        tokio::spawn(async move {
            pipeline.process_chunk(&chunk)
        })
    }).collect();

    let results = futures::future::join_all(tasks).await;

    // Verify all succeeded and measure performance
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    assert_eq!(success_count, 20);

    // Measure memory usage and throughput
    // ...
}
```

## Expected Results

### Performance Targets
- **Success Rate:** ≥95% for 300-line chunks
- **Latency:** ≤2 seconds per chunk
- **Throughput:** ≥50 chunks/second with 20 parallel sessions
- **Memory:** ≤9GB total for 20 sessions

### Quality Improvements
- **Current:** Pattern-based deterministic output
- **With CodeT5:** Actual neural code summarization
- **Expected:** Contextually relevant, human-like summaries

### Example Output
```
Input:  def calculate_fibonacci(n):
            if n <= 1: return n
            return calculate_fibonacci(n-1) + calculate_fibonacci(n-2)

Current Output: "Function definition with parameters"
CodeT5 Output: "Recursive function to calculate Fibonacci numbers"
```

## Troubleshooting

### Common Issues

1. **ONNX Export Failures**
   - T5 cache states not supported in legacy ONNX export
   - Solution: Use optimum library or export encoder/decoder separately

2. **Tokenizer Compatibility**
   - HuggingFace tokenizer format incompatible with Rust tokenizers crate
   - Solution: Implement custom BPE tokenizer or use Python bindings

3. **Memory Constraints**
   - 20 × 500MB = 10GB exceeds 9GB limit
   - Solution: Apply int8 quantization, reduce parallel sessions, or implement session pooling

4. **Performance Bottlenecks**
   - Sequential generation (auto-regressive) is slow
   - Solution: Batch generation, optimize tensor operations, use GPU if available

## Timeline Estimate

- **Model Conversion:** 2-4 hours (including debugging)
- **Tokenizer Integration:** 1-2 hours
- **Inference Pipeline Update:** 2-3 hours
- **Performance Testing:** 1-2 hours
- **Total:** 6-11 hours

## Success Criteria

✅ Model conversion completed successfully
✅ Real CodeT5 inference working in Rust
✅ 20+ parallel sessions operational
✅ Memory usage under 9GB
✅ Performance targets met
✅ Quality improvement demonstrated

## Next Steps

1. Complete the ONNX conversion using optimum library
2. Update the Rust inference pipeline for CodeT5 inputs/outputs
3. Implement proper tokenization and generation
4. Test and optimize parallel performance
5. Validate against performance targets

The foundation is solid - we have working ONNX parallel processing. The remaining work is primarily integration of the real model components.