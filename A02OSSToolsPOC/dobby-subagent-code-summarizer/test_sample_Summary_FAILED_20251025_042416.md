# Code Summary: test_sample

**Generated:** 2025-10-25 04:24:16 UTC  
**Source File:** `test_sample.rs`  
**Total Lines:** 41  
**File Size:** 0.0 MB  

## 📊 Processing Statistics

- **Chunks Processed:** 1 / 1
- **Success Rate:** 100.0%
- **Processing Time:** 0.000082417s
- **Throughput:** 497470.2 lines/sec
- **Chunk Processing Rate:** 12133.4 chunks/sec

## ⚙️ Configuration

- **Model:** Qwen2.5-0.5B-Instruct INT4 (BROKEN)
- **Tokenizer:** HuggingFace (READY BUT UNUSED)
- **Context Window:** 4096 tokens
- **Integration:** ORT GenAI (FAILED TO INITIALIZE)
- **Status:** HONESTLY BROKEN - No mock workarounds allowed

## 📝 Code Summaries

### ✅ Successfully Processed Chunks

### Chunk 1 (Lines 1-41)

**Summary:** Function that performs operations

**Code Preview:**
```rust
use std::collections::HashMap;

pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: u64, name: String, email: String) -> Self {
... (truncated)
```

---

## 🔍 Technical Details

This summary reflects **BROKEN ORT GenAI integration** - no actual inference was performed. The project follows global CLAUDE.md rules: no mock workarounds, face consequences honestly.

- **Status:** ❌ BROKEN - ORT GenAI pipeline failed to initialize
- **Error:** JSON parsing in genai_config.json at line 4, index 20
- **Root Cause:** Unknown value "quantization" (vs expected "quantization")
- **Real Model Files:** ✅ Qwen2.5-0.5B-int4 ONNX (474MB) exist but unused
- **Tokenizer:** HuggingFace Production (151,936 vocabulary)
- **Architecture:** Design101 TDD-First Principles
- **Compliance:** Global CLAUDE.md - No mocks allowed, suffer broken functionality
- **Total Chunks Attempted:** 1 (all failed)
- **Generated:** 2025-10-25 04:24:16 UTC
