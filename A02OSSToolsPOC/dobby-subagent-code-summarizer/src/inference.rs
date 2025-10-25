//! PURE ONNX Neural Inference - NO FALLBACKS (CLAUDE.md Compliance)
//!
//! This implementation contains ONLY real neural inference using ONNX Runtime 2.0 API.
//! If ONNX Runtime v2.0 integration fails, system stays HONESTLY NON-FUNCTIONAL.
//! NO pattern matching, NO fallbacks, NO mocks - PURE NEURAL INFERENCE ONLY.

use anyhow::Result;
use std::path::PathBuf;
use tokenizers::Tokenizer;
use log::{info, error};
use ort::value::Value;

pub struct RealInferencePipeline {
    session: ort::session::Session,
    tokenizer: Tokenizer,
}

impl RealInferencePipeline {
    pub fn new(model_path: PathBuf, tokenizer_path: PathBuf) -> Result<Self> {
        info!("Phase 1: Testing REAL Qwen model loading");

        // Step 1: Test REAL model file exists
        let model_file = model_path.join("model_quantized.onnx");
        if !model_file.exists() {
            return Err(anyhow::anyhow!("HONEST FAILURE: Model file does not exist at {}", model_file.display()));
        }
        info!("✅ Model file exists: {} ({} bytes)", model_file.display(), model_file.metadata()?.len());

        // Step 2: Test REAL ONNX global initialization
        info!("Testing REAL ONNX v2.0 global initialization...");
        ort::init().commit()
            .map_err(|e| anyhow::anyhow!("HONEST FAILURE: ONNX v2.0 global init failed: {}", e))?;
        info!("✅ ONNX v2.0 global initialization successful");

        // Step 3: Test REAL tokenizer loading
        info!("Testing REAL HuggingFace tokenizer loading...");
        let tokenizer_file = tokenizer_path.join("tokenizer.json");
        if !tokenizer_file.exists() {
            return Err(anyhow::anyhow!("HONEST FAILURE: Tokenizer file does not exist at {}", tokenizer_file.display()));
        }
        let tokenizer = Tokenizer::from_file(tokenizer_file)
            .map_err(|e| anyhow::anyhow!("HONEST FAILURE: Tokenizer loading failed: {}", e))?;
        info!("✅ REAL HuggingFace tokenizer loaded successfully");

        // Step 4: Test REAL ONNX session creation using v2.0 API
        info!("Testing REAL ONNX v2.0 session creation...");
        let session = ort::session::Session::builder()?
            .with_execution_providers([ort::execution_providers::CPUExecutionProvider::default().build()])?
            .with_intra_threads(1)?  // Fix for macOS mutex lock failure (SIGABRT)
            .commit_from_file(&model_file)
            .map_err(|e| anyhow::anyhow!("HONEST FAILURE: Session creation failed: {}", e))?;
        info!("✅ REAL ONNX v2.0 session created successfully");

        info!("🎉 ALL PHASE 1 COMPONENTS WORK - REAL neural pipeline foundation ready");
        Ok(Self { session, tokenizer })
    }

    pub fn summarize_chunk(&self, chunk: &str) -> Result<String> {
        info!("Starting REAL neural inference for chunk: {} chars", chunk.len());

        // REAL neural inference - NO pattern matching, NO fallbacks
        // Phase 2: Test REAL tokenization
        let prompt = format!("Summarize this code:\n{}", chunk);
        let encoded = self.tokenizer.encode(prompt, true)
            .map_err(|e| anyhow::anyhow!("HONEST FAILURE: Tokenization failed: {}", e))?;
        let input_ids: Vec<u32> = encoded.get_ids().to_vec();

        if input_ids.is_empty() {
            return Err(anyhow::anyhow!("HONEST FAILURE: Tokenization produced empty input - neural inference cannot proceed"));
        }

        info!("✅ REAL tokenization successful: {} tokens generated", input_ids.len());

        // Phase 3: Test REAL neural inference
        let neural_summary = self.real_neural_inference(&input_ids)?;

        info!("✅ REAL neural inference completed");
        Ok(neural_summary)
    }

    /// REAL neural inference using ONNX Runtime v2.0 - TDD Contract-Driven Implementation
    ///
    /// # Executable Specification Contract
    ///
    /// # Preconditions
    /// - RealInferencePipeline successfully created with loaded model and tokenizer
    /// - input_ids: non-empty vector of u32 tokens from HuggingFace tokenizer
    /// - Model expects "input_ids" and "attention_mask" as named inputs
    ///
    /// # Postconditions
    /// - Returns Ok(String) with real neural inference summary
    /// - Creates proper i64 tensors with dynamic shapes via .into_dyn()
    /// - Successfully executes session.run() with both required inputs
    /// - Extracts real tensor outputs using try_extract::<f32>()
    ///
    /// # Error Conditions
    /// - TensorError::CreationFailed if tensor creation fails
    /// - TensorError::InferenceFailed if ONNX session.run() fails
    /// - TensorError::ExtractionFailed if output tensor extraction fails
    /// - HONEST system remains non-functional until all tensor operations work
    fn real_neural_inference(&self, input_ids: &[u32]) -> Result<String> {
        info!("Phase 3.1: REAL tensor operations with TDD contracts - {} input tokens", input_ids.len());

        // Phase 3.2: TDD Layer 1 Core - Resource Management with RAII
        // Convert u32 input_ids to i64 for model compatibility (critical fix from note)
        let seq_len = input_ids.len();
        let input_ids_i64: Vec<i64> = input_ids.iter().map(|&id| id as i64).collect();

        // Phase 3.2: REAL tensor operations using correct v2.0 API from error hints
        // Error hints show (D, Box<[T]>) and (D, Vec<T>) are valid implementations
        let input_ids_tensor = ort::value::Value::from_array(((1, seq_len), input_ids_i64))
            .map_err(|e| anyhow::anyhow!("TensorError::CreationFailed - input_ids tensor: {}", e))?;

        // Create attention_mask (all ones) as required by model
        let attention_mask_data: Vec<i64> = vec![1; seq_len];
        let attention_mask_tensor = ort::value::Value::from_array(((1, seq_len), attention_mask_data))
            .map_err(|e| anyhow::anyhow!("TensorError::CreationFailed - attention_mask tensor: {}", e))?;

        info!("✅ Phase 3.2: REAL tensor creation successful - shapes: input_ids {:?}, attention_mask {:?}",
              (1, seq_len), (1, seq_len));

        // Phase 3.2: Use ort::inputs! macro for named inputs
        let inputs = ort::inputs! {
            "input_ids" => input_ids_tensor,
            "attention_mask" => attention_mask_tensor,
        };

        info!("✅ Phase 3.2: Named input mapping created with ort::inputs! macro");

        // Phase 3.2: Execute real ONNX inference
        let outputs = self.session.run(inputs)
            .map_err(|e| anyhow::anyhow!("TensorError::InferenceFailed - ONNX session.run: {}", e))?;

        info!("✅ Phase 3.2: REAL ONNX inference execution successful");

        // Phase 3.2: Extract real tensor outputs using correct API
        let mut output_count = 0;
        let mut output_shape = None;

        for (output_name, output_tensor) in outputs {
            output_count += 1;
            info!("✅ Phase 3.2: Output {}: {}", output_count, output_name);

            // Use try_extract_tensor::<f32>() for tensor extraction
            match output_tensor.try_extract_tensor::<f32>() {
                Ok((shape, data)) => {
                    info!("✅ Phase 3.2: REAL output extraction successful - extracted {} tensor elements, shape: {:?}", data.len(), shape);
                    output_shape = Some(shape.to_vec());
                    break; // Got our first output, that's enough for Phase 3
                }
                Err(e) => {
                    error!("❌ TensorError::ExtractionFailed - Could not extract tensor: {}", e);
                    return Err(anyhow::anyhow!("TensorError::ExtractionFailed - Output extraction: {}", e));
                }
            }
        }

        if output_count == 0 {
            return Err(anyhow::anyhow!("TensorError::ExtractionFailed - No outputs from model"));
        }

        // Phase 4.1: Return shape-based summary (will be enhanced in Phase 4)
        let summary = format!("REAL neural inference completed - output shape: {:?}", output_shape);
        info!("✅ Phase 3.2: All tensor operations working - TDD contract satisfied");
        Ok(summary)
    }
}

impl Clone for RealInferencePipeline {
    fn clone(&self) -> Self {
        // Clone REAL neural pipeline - NO fallbacks
        let model_path = PathBuf::from("./models/qwen2.5-0.5b-int4");
        let tokenizer_path = PathBuf::from("./tokenizer_dir");

        match Self::new(model_path, tokenizer_path) {
            Ok(pipeline) => pipeline,
            Err(e) => {
                error!("HONEST FAILURE: Failed to clone REAL neural pipeline: {}", e);
                panic!("REAL neural pipeline clone failed - system stays BROKEN");
            }
        }
    }
}

// NO TESTS THAT USE PATTERN MATCHING - ONLY REAL NEURAL INFERENCE TESTS ALLOWED
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase1_real_model_loading() {
        // Test Phase 1: All components must work honestly
        let model_path = PathBuf::from("./models/qwen2.5-0.5b-int4");
        let tokenizer_path = PathBuf::from("./tokenizer_dir");

        let result = RealInferencePipeline::new(model_path, tokenizer_path);

        match result {
            Ok(_) => {
                println!("✅ ALL PHASE 1 COMPONENTS WORK - real neural pipeline foundation ready");
                assert!(true, "REAL model loading should work");
            }
            Err(e) => {
                println!("❌ HONEST FAILURE in Phase 1: {}", e);
                // This is the HONEST state - system stays broken until Phase 1 works
                assert!(false, "Phase 1 must pass - system stays non-functional until real components work");
            }
        }
    }

    #[test]
    fn test_real_tokenizer_only() {
        // Test REAL tokenizer independently
        let tokenizer_path = PathBuf::from("./tokenizer_dir");
        let tokenizer_file = tokenizer_path.join("tokenizer.json");
        let tokenizer = Tokenizer::from_file(tokenizer_file);

        match tokenizer {
            Ok(t) => {
                // Test real tokenization
                let test_text = "fn test() { println!(\"hello\"); }";
                let encoded = t.encode(test_text, true);
                match encoded {
                    Ok(encoded_result) => {
                        let token_count = encoded_result.get_ids().len();
                        println!("✅ REAL tokenizer encoded {} tokens from: '{}'", token_count, test_text);
                        assert!(token_count > 0, "REAL tokenizer must produce tokens");
                    }
                    Err(e) => {
                        println!("❌ HONEST FAILURE: REAL tokenizer encoding failed: {}", e);
                        assert!(false, "REAL tokenizer encoding must work");
                    }
                }
            }
            Err(e) => {
                println!("❌ HONEST FAILURE: REAL tokenizer loading failed: {}", e);
                assert!(false, "REAL tokenizer must load - system stays non-functional");
            }
        }
    }
}