# Domain01: ONNX Model Replacement for 1-Line Code Summarization

## 🎯 **Executive Summary**

This document provides comprehensive research and implementation guidance for replacing the current CodeT5 encoder-only model with **SmolLM2-135M-Instruct** for generating meaningful 1-line code summaries using real ONNX neural inference.

**Current Problem**: CodeT5 encoder-only model outputs "token 1" (BOS) for all inputs because it's designed for classification, not text generation.

**Solution**: Replace with SmolLM2-135M-Instruct, a 135M parameter model specifically designed for summarization and text generation.

---

## 📊 **Model Comparison**

| Feature | Current CodeT5 | SmolLM2-135M-Instruct |
|---------|----------------|------------------------|
| **Parameters** | 60M (encoder-only) | 135M (full) |
| **Size** | ~240MB | ~500MB |
| **Architecture** | Encoder-only | Encoder-Decoder |
| **Purpose** | Code understanding | Text generation, summarization |
| **Training** | Code-specific | 2T tokens, diverse data |
| **Output** | Classification tokens | Generated text |
| **License** | Apache 2.0 | Apache 2.0 |

---

## 🔍 **Why SmolLM2-135M-Instruct?**

### **✅ Advantages:**
1. **Designed for Summarization**: Built specifically for text rewriting and summarization tasks
2. **Modern Architecture**: Full encoder-decoder for text generation
3. **Compact Size**: 135M parameters (~500MB) - well under 800MB limit
4. **High Performance**: Trained on 2 trillion tokens with instruction tuning
5. **Apache 2.0 License**: Commercial friendly
6. **ONNX Available**: Direct ONNX support
7. **Instruction Tuned**: Responds well to prompting for specific tasks

### **📈 Performance Expectations:**
- **Inference Time**: Similar ~200ms per chunk (current baseline)
- **Output Quality**: Meaningful 1-line summaries instead of "Empty decoded text"
- **Diversity**: Different chunks will produce different summaries
- **Parallel Ready**: Same infrastructure supports 20 concurrent agents

---

## 💻 **Model Downloading & Setup**

### **1. HuggingFace Download Methods**

#### **Method A: HuggingFace Hub CLI**
```bash
# Install HuggingFace Hub CLI
pip install -U "huggingface_hub[cli]"

# Download model files
hf download HuggingFaceTB/SmolLM2-135M-Instruct --include "*.onnx" --include "*.json" --include "*.txt"
```

#### **Method B: Git Clone**
```bash
# Clone the model repository
git clone https://huggingface.co/HuggingFaceTB/SmolLM2-135M-Instruct

# Navigate to model directory
cd SmolLM2-135M-Instruct
```

#### **Method C: Python API**
```python
from huggingface_hub import hf_hub_download
import os

# Download specific files
model_file = hf_hub_download(
    repo_id="HuggingFaceTB/SmolLM2-135M-Instruct",
    filename="model.onnx",
    local_dir="./models/smolLM2-onnx"
)

tokenizer_file = hf_hub_download(
    repo_id="HuggingFaceTB/SmolLM2-135M-Instruct",
    filename="tokenizer.json",
    local_dir="./models/smolLM2-onnx"
)
```

### **2. Required Model Files**

```
models/smolLM2-onnx/
├── model.onnx              # Main ONNX model file
├── tokenizer.json          # Tokenizer configuration
├── config.json             # Model configuration
├── generation_config.json  # Generation parameters
└── special_tokens_map.json # Special token mappings
```

### **3. Directory Structure Setup**

```bash
# Create model directories
mkdir -p models/backup
mkdir -p models/smolLM2-onnx

# Backup current model
mv models/codet5-onnx models/backup/codet5-onnx-backup

# Download new model
# (Use one of the methods above)
```

---

## ⚙️ **ONNX Integration in Rust**

### **1. Current ONNX Setup Analysis**

**Current Model Input/Output:**
```rust
// Current CodeT5 (encoder-only)
inputs: ["input_ids", "attention_mask"]
outputs: ["logits"]  // Shape: [batch, seq, vocab_size]
```

**New Model Expected I/O:**
```rust
// SmolLM2 (text generation)
inputs: ["input_ids", "attention_mask"]
outputs: ["logits"]  // Shape: [batch, seq, vocab_size]
```

### **2. Rust ONNX API Patterns**

#### **Model Loading**
```rust
use ort::{Session, SessionBuilder};

// Load SmolLM2 model
let session = SessionBuilder::new()?
    .with_optimization_level(ort::GraphOptimizationLevel::Level3)?
    .commit_from_file("models/smolLM2-onnx/model.onnx")?;
```

#### **Tensor Creation**
```rust
use ort::Value;

// Create input tensors
let input_tensor = Value::from_array(([1, seq_len], input_ids))?;
let attention_tensor = Value::from_array(([1, seq_len], attention_mask))?;

// Create input mapping
let inputs = ort::inputs! {
    "input_ids" => input_tensor,
    "attention_mask" => attention_tensor
};
```

#### **Inference Execution**
```rust
// Run inference
let outputs = session.run(inputs)?;

// Extract logits
let logits_output = &outputs[0];
let (_shape, logits_data) = logits_output.try_extract_tensor::<f32>()?;
```

---

## 🎯 **Text Generation Pipeline**

### **1. Architecture Differences**

**Current (CodeT5 Encoder-Only):**
```rust
// Takes input, produces representation
fn run_encoder_inference(input_ids, attention_mask) -> Vec<u32> {
    // Single forward pass
    // Returns best token (always token 1/BOS)
}
```

**New (SmolLM2 Text Generation):**
```rust
// Takes input, generates text sequentially
fn run_text_generation(input_ids, attention_mask, max_length: usize) -> Vec<u32> {
    let mut generated_tokens = Vec::new();
    let mut current_input = input_ids;

    for step in 0..max_length {
        // Forward pass
        let logits = model_forward(&current_input, &attention_mask);

        // Sample next token
        let next_token = sample_next_token(&logits);
        generated_tokens.push(next_token);

        // Check for EOS
        if next_token == eos_token_id { break; }

        // Update input for next step
        current_input.push(next_token);
    }

    generated_tokens
}
```

### **2. Generation Strategies**

#### **Greedy Search (Simplest)**
```rust
fn greedy_search(logits: &[f32]) -> u32 {
    // Find token with highest probability
    let max_value = logits.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let max_index = logits.iter().position(|&x| x == max_value).unwrap_or(0);
    max_index as u32
}
```

#### **Temperature Sampling**
```rust
fn temperature_sample(logits: &[f32], temperature: f32) -> u32 {
    // Apply temperature
    let scaled_logits: Vec<f32> = logits.iter()
        .map(|&x| x / temperature)
        .collect();

    // Softmax normalization
    let exp_logits: Vec<f32> = scaled_logits.iter()
        .map(|&x| (x - scaled_logits.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b))).exp())
        .collect();
    let sum: f32 = exp_logits.iter().sum();
    let probs: Vec<f32> = exp_logits.iter().map(|&x| x / sum).collect();

    // Sample from distribution
    sample_from_distribution(&probs)
}
```

### **3. Prompt Engineering for Code Summarization**

#### **Effective Prompt Templates**
```rust
// Template 1: Direct instruction
fn create_summary_prompt(code: &str) -> String {
    format!("Summarize this code in one line: {}", code)
}

// Template 2: Role-based
fn create_role_prompt(code: &str) -> String {
    format!("You are a code expert. Provide a one-line summary: {}", code)
}

// Template 3: Task-specific
fn create_task_prompt(code: &str) -> String {
    format!("Code: {}\nOne-line summary:", code)
}
```

---

## 🔤 **Tokenizer Integration**

### **1. Tokenizer Differences**

**Current CodeT5 Tokenizer:**
- **Type**: BPE (Byte-Pair Encoding)
- **Vocabulary**: Code-specific
- **Special Tokens**: `<pad>`, `<s>`, `</s>`, `<unk>`

**SmolLM2 Tokenizer:**
- **Type**: BPE (SentencePiece-based)
- **Vocabulary**: General English + code
- **Special Tokens**: `<s>`, `</s>`, `<pad>`, `<unk>`

### **2. Rust Tokenizer Implementation**

#### **Using Tokenizers Crate**
```rust
use tokenizers::Tokenizer;

fn load_smollm_tokenizer() -> Result<Tokenizer, Box<dyn std::error::Error>> {
    let tokenizer = Tokenizer::from_file("models/smolLM2-onnx/tokenizer.json")?;
    Ok(tokenizer.with_padding(None)) // Disable padding for generation
}

fn encode_text(tokenizer: &Tokenizer, text: &str) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let encoding = tokenizer.encode(text, false)?;
    Ok(encoding.get_ids().to_vec())
}

fn decode_tokens(tokenizer: &Tokenizer, tokens: &[u64]) -> Result<String, Box<dyn std::error::Error>> {
    let decoded = tokenizer.decode(tokens, true)?;
    Ok(decoded)
}
```

### **3. Input/Output Processing**

#### **Input Preprocessing**
```rust
fn preprocess_input(tokenizer: &Tokenizer, code: &str, max_length: usize) -> Result<(Vec<i64>, Vec<i64>), Box<dyn std::error::Error>> {
    // Create prompt
    let prompt = format!("Summarize this code in one line: {}", code);

    // Encode text
    let token_ids = encode_text(tokenizer, &prompt)?;

    // Truncate to max_length
    let truncated_ids: Vec<u64> = token_ids.into_iter().take(max_length).collect();

    // Convert to i64 and create attention mask
    let input_ids: Vec<i64> = truncated_ids.iter().map(|&x| x as i64).collect();
    let attention_mask: Vec<i64> = vec![1; input_ids.len()];

    Ok((input_ids, attention_mask))
}
```

#### **Output Postprocessing**
```rust
fn postprocess_output(tokenizer: &Tokenizer, tokens: &[u32]) -> Result<String, Box<dyn std::error::Error>> {
    // Convert to u64
    let token_ids: Vec<u64> = tokens.iter().map(|&x| x as u64).collect();

    // Decode to text
    let decoded = decode_tokens(tokenizer, &token_ids)?;

    // Clean up output
    let cleaned = decoded
        .trim()
        .strip_prefix("Summarize this code in one line:")
        .unwrap_or(&decoded)
        .trim()
        .lines()
        .next()
        .unwrap_or("")
        .to_string();

    Ok(cleaned)
}
```

---

## 🚀 **Implementation Step-by-Step**

### **Phase 1: Model Replacement**

#### **Step 1.1: Backup Current Model**
```bash
# Create backup directory
mkdir -p models/backup

# Backup current CodeT5 model
cp -r models/codet5-onnx models/backup/codet5-onnx-backup-$(date +%Y%m%d)

echo "✅ Current model backed up successfully"
```

#### **Step 1.2: Download SmolLM2 Model**
```bash
# Install HuggingFace Hub CLI if not already installed
pip install -U "huggingface_hub[cli]"

# Create model directory
mkdir -p models/smolLM2-onnx

# Download model files
hf download HuggingFaceTB/SmolLM2-135M-Instruct \
  --local-dir models/smolLM2-onnx \
  --include "*.onnx" \
  --include "*.json" \
  --include "*.txt"

echo "✅ SmolLM2 model downloaded successfully"
```

#### **Step 1.3: Verify Model Files**
```bash
# Check required files exist
required_files=("model.onnx" "tokenizer.json" "config.json")

for file in "${required_files[@]}"; do
    if [ -f "models/smolLM2-onnx/$file" ]; then
        echo "✅ $file found"
    else
        echo "❌ $file missing"
        exit 1
    fi
done

# Check model size
model_size=$(du -h models/smolLM2-onnx/model.onnx | cut -f1)
echo "📊 Model size: $model_size"
```

### **Phase 2: Code Updates**

#### **Step 2.1: Update Configuration**
```rust
// src/inference.rs - Update InferenceConfig
impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            max_input_length: 512,
            max_output_length: 64,  // Shorter for 1-line summaries
            timeout: Duration::from_secs(5),
            model_path: "models/smolLM2-onnx/model.onnx".to_string(),
            input_name: "input_ids".to_string(),
            output_name: "logits".to_string(),
        }
    }
}
```

#### **Step 2.2: Create SmolLM2 Tokenizer Module**
```rust
// src/tokenizer_smollm.rs
use tokenizers::Tokenizer;
use crate::errors::{ProcessingError, Result};

pub fn create_smollm_tokenizer() -> Result<Tokenizer> {
    let tokenizer_path = "models/smolLM2-onnx/tokenizer.json";

    if !std::path::Path::new(tokenizer_path).exists() {
        return Err(ProcessingError::TokenizerLoadFailed {
            source: format!("Tokenizer file not found: {}", tokenizer_path).into(),
        });
    }

    let tokenizer = Tokenizer::from_file(tokenizer_path)
        .map_err(|e| ProcessingError::TokenizerLoadFailed {
            source: format!("Failed to load tokenizer: {}", e).into(),
        })?;

    Ok(tokenizer.with_padding(None))
}
```

#### **Step 2.3: Update Inference Pipeline**
```rust
// src/inference.rs - Replace run_simple_encoder_inference
fn run_text_generation_inference(
    &self,
    input_ids: Vec<i64>,
    attention_mask: Vec<i64>,
    chunk: &Chunk
) -> Result<Vec<u32>> {
    println!("🔧 Running SmolLM2 text generation for chunk {}", chunk.id);

    // Load SmolLM2 tokenizer
    let tokenizer = crate::tokenizer_smollm::create_smollm_tokenizer()
        .map_err(|e| ProcessingError::TokenizerLoadFailed {
            source: format!("Failed to load SmolLM2 tokenizer: {}", e).into(),
        })?;

    // Initialize generation
    let mut generated_tokens = Vec::new();
    let mut current_input_ids = input_ids;

    // Generation loop
    for step in 0..self.config.max_output_length {
        // Create tensors
        let input_len = current_input_ids.len();
        let input_array = current_input_ids.clone().into_boxed_slice();
        let attention_array = attention_mask.clone().into_boxed_slice();

        let input_tensor = Value::from_array(([1, input_len], input_array))?;
        let attention_tensor = Value::from_array(([1, attention_mask.len()], attention_array))?;

        // Run inference
        let inputs = ort::inputs! {
            "input_ids" => input_tensor,
            "attention_mask" => attention_tensor
        };

        let mut session = self.session.lock().unwrap();
        let outputs = session.run(inputs)?;

        // Extract logits and sample next token
        let logits_output = &outputs[0];
        let next_token = self.sample_next_token(logits_output)?;

        // Check for EOS token
        if next_token == 2 {  // EOS token ID
            println!("  ✅ EOS token reached at step {}", step);
            break;
        }

        generated_tokens.push(next_token);
        current_input_ids.push(next_token as i64);

        // Safety limit
        if generated_tokens.len() >= 50 {
            println!("  ⚠️ Reached 50 token limit");
            break;
        }
    }

    println!("✅ SmolLM2 generation completed for chunk {}: {} tokens", chunk.id, generated_tokens.len());
    Ok(generated_tokens)
}
```

### **Phase 3: Testing & Validation**

#### **Step 3.1: Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smollm_tokenizer_creation() {
        let tokenizer = create_smollm_tokenizer();
        assert!(tokenizer.is_ok(), "SmolLM2 tokenizer should load successfully");
    }

    #[test]
    fn test_text_generation() {
        let config = InferenceConfig::default();
        let pipeline = OnnxInferencePipeline::new(config);
        assert!(pipeline.is_ok(), "Pipeline with SmolLM2 should create successfully");

        let pipeline = pipeline.unwrap();
        let chunk = Chunk {
            id: 0,
            line_start: 0,
            line_end: 3,
            line_count: 3,
            content: "fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
        };

        let result = pipeline.process_chunk(&chunk);
        assert!(result.is_ok(), "Should process simple function");

        let summary = result.unwrap();
        println!("Generated summary: {}", summary);
        assert!(!summary.is_empty(), "Summary should not be empty");
        assert_ne!(summary, "Empty decoded text", "Should generate meaningful summary");
    }
}
```

#### **Step 3.2: Integration Tests**
```bash
# Test single chunk
cargo test --lib test_text_generation -- --nocapture

# Test multiple chunks
cargo run --bin test_all_chunks

# Performance test
time cargo run --bin test_all_chunks
```

#### **Step 3.3: Expected Results Validation**
```bash
# Check for meaningful outputs
echo "Testing diverse outputs..."

# Run on different chunks and verify variety
for chunk in chunks/chunk_{aa,ab,ac,ad}; do
    echo "Testing $chunk..."
    cargo run --bin test_single_chunk -- $chunk
done

# Expected: Different summaries for each chunk
# Not expected: Same "Empty decoded text" for all
```

---

## 🔧 **Troubleshooting Guide**

### **Common Issues & Solutions**

#### **1. Model Loading Issues**
```
Error: "Failed to load model: File not found"
```
**Solution:**
- Verify model files exist in `models/smolLM2-onnx/`
- Check file permissions
- Ensure ONNX model is compatible with ort crate version

#### **2. Tokenizer Issues**
```
Error: "Tokenizer file not found"
```
**Solution:**
- Download `tokenizer.json` from HuggingFace
- Verify tokenizer format compatibility
- Check for special token mappings

#### **3. Memory Issues**
```
Error: "Out of memory"
```
**Solution:**
- Reduce `max_input_length` in config
- Implement batch processing
- Use CPU instead of GPU if memory limited

#### **4. Performance Issues**
```
Error: "Inference timeout"
```
**Solution:**
- Increase timeout in `InferenceConfig`
- Optimize tensor operations
- Consider model quantization

#### **5. Output Quality Issues**
```
Problem: "Repetitive or meaningless output"
```
**Solution:**
- Adjust temperature in sampling
- Improve prompt engineering
- Implement beam search
- Fine-tune generation parameters

### **Debugging Tools**

#### **Model Inspection**
```python
# Python script to inspect model
import onnxruntime as ort
import json

sess = ort.InferenceSession('models/smolLM2-onnx/model.onnx')
print("Model inputs:")
for input_meta in sess.get_inputs():
    print(f"  {input_meta.name}: {input_meta.shape}")

print("Model outputs:")
for output_meta in sess.get_outputs():
    print(f"  {output_meta.name}: {output_meta.shape}")

# Load config
with open('models/smolLM2-onnx/config.json', 'r') as f:
    config = json.load(f)
    print(f"Model type: {config.get('model_type')}")
    print(f"Vocab size: {config.get('vocab_size')}")
```

#### **Tokenizer Testing**
```rust
// Test tokenizer functionality
fn test_tokenizer() -> Result<()> {
    let tokenizer = create_smollm_tokenizer()?;

    let test_text = "fn test() { println!(\"hello\"); }";
    let encoded = tokenizer.encode(test_text, false)?;
    let decoded = tokenizer.decode(encoded.get_ids(), true)?;

    println!("Original: {}", test_text);
    println!("Encoded: {:?}", encoded.get_ids());
    println!("Decoded: {}", decoded);

    Ok(())
}
```

---

## 📈 **Performance Optimization**

### **1. Memory Management**
```rust
// Efficient tensor handling
fn create_tensors_efficiently(input_ids: Vec<i64>, attention_mask: Vec<i64>) -> Result<(Value, Value)> {
    // Avoid unnecessary allocations
    let input_array = input_ids.into_boxed_slice();
    let attention_array = attention_mask.into_boxed_slice();

    let input_tensor = Value::from_array(([1, input_array.len()], input_array))?;
    let attention_tensor = Value::from_array(([1, attention_array.len()], attention_array))?;

    Ok((input_tensor, attention_tensor))
}
```

### **2. Batch Processing**
```rust
// Process multiple chunks in parallel
async fn process_chunks_parallel(chunks: Vec<Chunk>, pipeline: &OnnxInferencePipeline) -> Vec<Result<String>> {
    let futures: Vec<_> = chunks.into_iter()
        .map(|chunk| async {
            pipeline.process_chunk(&chunk)
        })
        .collect();

    futures::future::join_all(futures).await
}
```

### **3. Caching Strategy**
```rust
use std::collections::HashMap;
use lru::LruCache;

struct CachedInference {
    pipeline: OnnxInferencePipeline,
    cache: LruCache<String, String>,
}

impl CachedInference {
    fn process_chunk_cached(&mut self, chunk: &Chunk) -> Result<String> {
        let cache_key = format!("{}-{}", chunk.id, chunk.content.len());

        if let Some(cached_result) = self.cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }

        let result = self.pipeline.process_chunk(chunk)?;
        self.cache.put(cache_key, result.clone());
        Ok(result)
    }
}
```

---

## 🎯 **Success Criteria**

### **Metrics for Success**

#### **Functional Requirements**
- ✅ **Meaningful Output**: Generate actual 1-line summaries (not "Empty decoded text")
- ✅ **Diverse Results**: Different chunks produce different summaries
- ✅ **Content Relevance**: Summaries relate to code functionality
- ✅ **Length Control**: Outputs fit within reasonable 1-line limits

#### **Performance Requirements**
- ✅ **Speed**: Maintain ~200ms per chunk processing time
- ✅ **Memory**: Stay under 2GB memory usage
- ✅ **Reliability**: 95%+ success rate on diverse code inputs
- ✅ **Scalability**: Ready for 20 concurrent agents

#### **Quality Requirements**
- ✅ **Accuracy**: Summaries correctly describe code functionality
- ✅ **Conciseness**: Clear, brief 1-line descriptions
- ✅ **Consistency**: Similar quality across different code patterns
- ✅ **Readability**: Human-understandable English summaries

### **Validation Checklist**

```markdown
[ ] Model loads successfully without errors
[ ] Tokenizer processes code input correctly
[ ] Generation produces meaningful text output
[ ] Different chunks yield different summaries
[ ] Performance maintains ~200ms per chunk
[ ] Memory usage stays reasonable
[ ] Error handling works gracefully
[ ] Parallel processing functions correctly
[ ] All 30 iggy chunks process successfully
[ ] Output quality meets user expectations
```

---

## 📚 **References & Resources**

### **Primary Sources**
- **HuggingFace Model**: https://huggingface.co/HuggingFaceTB/SmolLM2-135M-Instruct
- **ONNX Runtime**: https://github.com/microsoft/onnxruntime
- **Rust ONNX Bindings**: https://github.com/pykeio/ort
- **Tokenizers Library**: https://github.com/huggingface/tokenizers

### **Documentation**
- **HuggingFace Hub**: https://huggingface.co/docs/hub/en/models-downloading
- **ONNX Export**: https://huggingface.co/docs/optimum/en/onnxruntime/getting_started
- **Generation Strategies**: https://huggingface.co/docs/transformers/main/en/generation_strategies
- **Rust ML Frameworks**: https://github.com/huggingface/candle

### **Community & Support**
- **HuggingFace Forums**: https://discuss.huggingface.co/
- **ONNX Runtime Issues**: https://github.com/microsoft/onnxruntime/issues
- **Rust ML Community**: https://github.com/rust-ml/coordination

---

## 🚀 **Next Steps After Implementation**

### **Enhancement Opportunities**
1. **Fine-tuning**: Custom train SmolLM2 on code summary datasets
2. **Prompt Optimization**: A/B test different prompt templates
3. **Quantization**: Reduce model size and improve speed
4. **Alternative Models**: Test other models like FastThink-0.5B or custom options

### **Scaling Considerations**
1. **GPU Acceleration**: Implement CUDA support for faster inference
2. **Model Serving**: Deploy as API endpoint for multiple clients
3. **Batch Optimization**: Process multiple chunks simultaneously
4. **Caching Layer**: Implement Redis or in-memory caching for repeated requests

### **Monitoring & Maintenance**
1. **Performance Metrics**: Track inference time, memory usage, success rates
2. **Quality Metrics**: Monitor summary quality and user satisfaction
3. **Model Updates**: Regularly update to newer model versions
4. **Security**: Implement input validation and output sanitization

---

*Document Version: 1.0*
*Last Updated: 2025-10-24*
*Author: AI Assistant*
*License: Internal Use Only*