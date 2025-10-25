//! Qwen2.5 Production Code Summarization Library
//!
//! TDD-First Architecture following design101-tdd-architecture-principles.md
//! Production Qwen2.5-0.5B-Instruct with INT4 quantization and ONNX Runtime optimization

pub mod chunking;
pub mod session_pool;
pub mod orchestrator;
pub mod results;
pub mod validation;
pub mod config;
pub mod errors;
pub mod smol_inference_contract;
// pub mod smol_inference; // Temporarily disabled - focusing on QwenSummarizer
// pub mod smol_inference_ffi; // Temporarily disabled
// pub mod smol_inference_ffi_pipeline; // Temporarily disabled
// pub mod smol_inference_mock; // REMOVED - mocks not allowed per global CLAUDE.md
// pub mod ffi_bindings; // REMOVED - no longer needed
// pub mod ort_genai_bindings; // REMOVED - no longer needed
pub mod smol_inference_ort_genai;
pub mod inference;  // New ort 2.0 real inference implementation

// Import traits for use within the library
use orchestrator::ParallelProcessor;
use validation::ContractValidator;

// Re-export main components
pub use chunking::{TextChunker, Chunk};
pub use session_pool::SessionPool;
pub use orchestrator::{TokioParallelProcessor, ProcessingConfig};
pub use results::{DefaultResultsAggregator, ProcessingResults, ChunkResult};
pub use validation::{DefaultContractValidator, ValidationReport};
pub use config::SystemConfig;
pub use errors::{ProcessingError, Result};

// Re-export Qwen2.5 components (Production First - Design101 Principle 9)
pub use smol_inference_contract::{
    SmolLM3Inference, SmolModelConfiguration, SmolInferenceError,
    SmolOutputValidator, SmolPerformanceMetrics
};
// pub use smol_inference::SmolLM3InferencePipeline;  // Temporarily disabled
// pub use smol_inference_ffi_pipeline::SmolLM3InferenceFFIPipeline;  // Temporarily disabled
// pub use smol_inference_mock::SmolLM2MockPipeline; // REMOVED - mocks not allowed
pub use smol_inference_ort_genai::OrtGenAiInferencePipeline;
// pub use real_inference::RealInferencePipeline; // Temporarily disabled

// Create type aliases for consistency
// Use real ORT-based pipeline with neural inference
// pub type QwenInferencePipeline = crate::real_inference::RealInferencePipeline; // Temporarily disabled

/// Main library interface
pub struct QwenSummarizer {
    config: SystemConfig,
}

impl QwenSummarizer {
    pub fn new(config: SystemConfig) -> Result<Self> {
        // Validate config
        config.validate()
            .map_err(|e| ProcessingError::ContractViolation {
                contract_name: "SystemConfig".to_string(),
                violation: e,
            })?;

        Ok(Self { config })
    }

    /// Main processing function - ORT GenAI Integration (Currently Broken)
    pub async fn process_file(&self, file_path: &str) -> Result<ProcessingResults> {
        println!("🚀 Starting ORT GenAI Processing - Real Neural Inference...");

        // Phase 1: Create chunks with measurable contracts
        let chunker = TextChunker::new();
        let chunks = chunker
            .create_chunks_from_file(file_path)
            .await
            .map_err(|e| ProcessingError::ChunkingFailed {
                path: file_path.to_string(),
                reason: e.to_string(),
            })?;

        println!("✅ Created {} chunks", chunks.len());

        // Phase 2: Face the broken reality - NO MOCKS ALLOWED per global CLAUDE.md
        println!("🔧 GLOBAL CLAUDE.md Rule: SUFFER WITH BROKEN FUNCTIONALITY - NO WORKAROUNDS");
        println!("❌ ORT GenAI Integration: BROKEN - Facing consequences honestly");

        // Try to create real ORT GenAI pipeline (this will fail)
        let model_path = format!("{}/models/qwen2.5-0.5b-int4", std::env::current_dir().unwrap().display());
        println!("📂 Attempting ORT GenAI pipeline: {}", model_path);

        // Temporarily return error since real_inference is disabled
        return Err(crate::errors::ProcessingError::InferenceFailed {
            chunk_id: 0,
            message: "Real inference pipeline temporarily disabled for API testing".to_string(),
        });

        /*
        let mut ort_genai_pipeline = match QwenInferencePipeline::new(&model_path) {
            Ok(pipeline) => {
                println!("✅ ORT GenAI pipeline created successfully");
                pipeline
            }
            Err(e) => {
                println!("❌ ORT GenAI pipeline creation FAILED: {}", e);
                println!("📋 ERROR DETAILS: {}", e);
                println!("⚠️  Project remains NON-FUNCTIONAL until this is fixed");
                return Err(e);
            }
        };

        // Phase 3: Process chunks with broken pipeline (This will also fail)
        println!("🔄 Processing chunks with BROKEN ORT GenAI pipeline...");
        let processor = TokioParallelProcessor::new(ProcessingConfig::from(&self.config));
        let results = processor
            .process_chunks_parallel(chunks, &mut ort_genai_pipeline, &self.config)
            .await?;

        // Phase 4: Validate TDD contracts
        let validator = DefaultContractValidator;
        let validation_report = validator
            .validate_processing_results(&results, &self.config)?;

        println!("❌ Processing Complete - BUT PROJECT IS BROKEN");
        println!("📊 Results: {} chunks processed", results.processed_count());
        println!("🎯 Contracts: {}",
            if validation_report.all_contracts_satisfied { "PASSED ✅" } else { "FAILED ❌" });
        println!("⚠️  Status: HONESTLY BROKEN - No mocks allowed per global CLAUDE.md");
        println!("\n{}", validation_report.summary());

        Ok(results)
        */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarizer_creation() {
        // TDD-First: STUB phase - basic creation
        let config = SystemConfig::test_config();
        let summarizer = QwenSummarizer::new(config);
        assert!(summarizer.is_ok(), "Should create Qwen summarizer");
    }

    #[tokio::test]
    async fn test_chunking_integration() {
        // TDD-First: GREEN phase - chunking works
        let chunker = TextChunker::new();

        // Test with small content
        let test_content = "line 1\nline 2\nline 3\n";
        let test_file = "test_chunking.txt";
        std::fs::write(test_file, test_content).unwrap();

        let result = chunker.create_chunks_from_file(test_file).await;
        std::fs::remove_file(test_file).ok();

        assert!(result.is_ok());
    }
}
