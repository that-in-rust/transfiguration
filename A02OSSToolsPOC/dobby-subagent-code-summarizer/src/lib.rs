//! ONNX-based Parallel Code Summarization Library
//!
//! TDD-First Architecture following design101-tdd-architecture-principles.md
//! Pure ONNX model integration for parallel code summarization - no fallbacks

pub mod chunking;
pub mod model;
pub mod session_pool;
pub mod orchestrator;
pub mod inference;
pub mod results;
pub mod validation;
pub mod config;
pub mod errors;
pub mod tokenizer;
pub mod tokenizer_simple;
pub mod tokenizer_codet5;
pub mod tokenizer_smollm;
pub mod t5_inference_contract;
pub mod smol_inference_contract;

// Import traits for use within the library
use orchestrator::ParallelProcessor;
use validation::ContractValidator;

// Re-export main components
pub use chunking::{TextChunker, Chunk};
pub use model::ModelManager;
pub use session_pool::SessionPool;
pub use orchestrator::{TokioParallelProcessor, ProcessingConfig};
pub use inference::{OnnxInferencePipeline, InferenceConfig};
pub use results::{DefaultResultsAggregator, ProcessingResults, ChunkResult};
pub use validation::{DefaultContractValidator, ValidationReport};
pub use config::SystemConfig;
pub use errors::{ProcessingError, Result};

// Re-export SmolLM2 components
pub use tokenizer_smollm::{SmolLM2Tokenizer, SmolTokenizerProvider, create_smollm_tokenizer};
pub use smol_inference_contract::{
    SmolLM2Inference, SmolModelConfiguration, SmolInferenceError,
    SmolOutputValidator, SmolPerformanceMetrics
};

/// Main library interface
pub struct OnnxSummarizer {
    config: SystemConfig,
}

impl OnnxSummarizer {
    pub fn new(config: SystemConfig) -> Result<Self> {
        // Validate config
        config.validate()
            .map_err(|e| ProcessingError::ContractViolation {
                contract_name: "SystemConfig".to_string(),
                violation: e,
            })?;

        Ok(Self { config })
    }

    /// Main processing function - TDD contracts enforced
    pub async fn process_tokio_source(&self) -> Result<ProcessingResults> {
        println!("🚀 Starting Pure ONNX Processing - No Pattern Fallbacks...");

        // Phase 1: Create chunks with measurable contracts
        let chunker = TextChunker::new();
        let chunks = chunker
            .create_chunks_from_file(
                self.config.tokio_source_path.to_str().unwrap()
            )
            .await
            .map_err(|e| ProcessingError::ChunkingFailed {
                path: self.config.tokio_source_path.display().to_string(),
                reason: e.to_string(),
            })?;

        println!("✅ Created {} chunks", chunks.len());

        // Phase 2: Create pure ONNX inference pipeline
        let inference = OnnxInferencePipeline::new(InferenceConfig::default())?;

        // Phase 3: Process chunks with parallel orchestration
        let processor = TokioParallelProcessor::new(ProcessingConfig::from(&self.config));
        let results = processor
            .process_chunks_parallel(chunks, &inference, &self.config)
            .await?;

        // Phase 4: Validate TDD contracts
        let validator = DefaultContractValidator;
        let validation_report = validator
            .validate_processing_results(&results, &self.config)?;

        println!("✅ Pure ONNX Processing Complete");
        println!("📊 Results: {} chunks processed", results.processed_count());
        println!("🎯 Contracts: {}",
            if validation_report.all_contracts_satisfied { "PASSED ✅" } else { "FAILED ❌" });
        println!("\n{}", validation_report.summary());

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarizer_creation() {
        // TDD-First: STUB phase - basic creation
        let config = SystemConfig::test_config();
        let summarizer = OnnxSummarizer::new(config);
        assert!(summarizer.is_ok(), "Should create summarizer");
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
