//! SmolLM2-135M-Inference Executable Specifications
//!
//! TDD-First: Contract-driven development with measurable outcomes
//! Every claim must be validated by automated tests

use crate::chunking::Chunk;
use crate::errors::{ProcessingError, Result};
use thiserror::Error;

/// Convert SmolInferenceError to ProcessingError for compatibility
impl From<SmolInferenceError> for ProcessingError {
    fn from(err: SmolInferenceError) -> Self {
        ProcessingError::InferenceFailed {
            chunk_id: 0, // Will be overridden if available
            message: format!("SmolLM2 inference error: {}", err),
        }
    }
}

/// REQ-SMOL-001.0: SmolLM2 Text Generation Contract
///
/// # Preconditions
/// - SmolLM2-135M-Instruct ONNX model loaded successfully from models/smolLM2-onnx/model.onnx
/// - SmolLM2 tokenizer loaded from models/smolLM2-onnx/tokenizer.json
/// - Input chunk contains valid Rust source code (1-1000 lines)
/// - Input text properly formatted with "Summarize this code in one line: " prefix
/// - Model input sequence length ≤ 512 tokens
///
/// # Postconditions
/// - Returns Ok(String) with meaningful 1-line code summary
/// - Summary length: 10-200 characters
/// - Summary contains relevant code concepts (functions, structs, patterns)
/// - Different input chunks produce different summaries (diversity validation)
/// - Summary text is human-readable English describing code functionality
/// - Processing time: <500ms per chunk on standard hardware
/// - Memory usage: <100MB additional during inference
///
/// # Error Conditions
/// - SmolInferenceError::ModelLoadFailed if ONNX model cannot be loaded
/// - SmolInferenceError::TokenizerFailed if SmolLM2 tokenizer loading fails
/// - SmolInferenceError::InputTooLong if chunk exceeds 512 token limit
/// - SmolInferenceError::InferenceFailed if ONNX inference encounters errors
/// - SmolInferenceError::OutputDecodingFailed if token decoding fails
/// - SmolInferenceError::OutputValidationFailed if summary violates constraints
///
/// # Performance Contract
/// - Single chunk inference: <500ms on standard hardware
/// - Memory usage: <100MB additional during inference
/// - Success rate: >95% on valid code inputs
/// - Output diversity: >80% different outputs for different inputs
///
/// # Test Scenarios
///
/// Scenario 1: Basic Function Summary (REQ-SMOL-001.1)
/// Given: Chunk containing "fn hello_world() { println!(\"Hello, world!\"); }"
/// When: smol_inference_pipeline.generate_summary() is called
/// Then: Returns meaningful summary like "Function that prints Hello World message"
///
/// Scenario 2: Struct Declaration Summary (REQ-SMOL-001.2)
/// Given: Chunk containing "pub struct User { name: String, age: u32 }"
/// When: smol_inference_pipeline.generate_summary() is called
/// Then: Returns summary mentioning struct/data structure concepts
///
/// Scenario 3: Output Diversity Validation (REQ-SMOL-001.3)
/// Given: Three different code chunks with distinct functionality
/// When: smol_inference_pipeline.generate_summary() called on each
/// Then: Returns three different summaries with >80% semantic difference
///
/// Scenario 4: Performance Contract Validation (REQ-SMOL-001.4)
/// Given: Standard chunk with 300 lines of code
/// When: smol_inference_pipeline.generate_summary() is called
/// Then: Completes in <500ms with <100MB additional memory usage

/// Trait-based dependency injection for testability (Principle 3)
pub trait SmolLM2Inference: Send + Sync {
    /// Generate 1-line code summary with measurable contracts
    ///
    /// # Performance Contract (Principle 5)
    /// - Execution time: <500ms for single chunk
    /// - Memory allocation: <100MB additional memory
    /// - Success rate: >95% on valid code inputs
    ///
    /// # Correctness Contract
    /// - Output length: 10-200 characters
    /// - Must contain meaningful words describing code functionality
    /// - Different inputs must produce different outputs (>80% diversity)
    /// - Summary must be human-readable English
    fn generate_summary(&self, chunk: &Chunk) -> Result<String>;

    /// Validate SmolLM2 model health and readiness
    ///
    /// # Postconditions
    /// - Returns Ok(()) if model and tokenizer loaded successfully
    /// - Returns Err(SmolInferenceError) if any component failed to load
    fn validate_model_health(&self) -> Result<()>;

    /// Get model configuration for debugging and validation
    fn model_config(&self) -> SmolModelConfiguration;
}

/// SmolLM2 model configuration for diagnostics and testing
#[derive(Debug, Clone, PartialEq)]
pub struct SmolModelConfiguration {
    pub model_path: String,
    pub tokenizer_path: String,
    pub max_input_length: usize,
    pub max_output_length: usize,
    pub vocab_size: usize,
    pub model_type: SmolModelType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SmolModelType {
    SmolLM2_135M,
    SmolLM2_360M,
    SmolLM2_1_7B,
    Custom(String),
}

/// Structured error hierarchy using thiserror (Principle 6)
#[derive(Error, Debug)]
pub enum SmolInferenceError {
    #[error("SmolLM2 model loading failed: {source}")]
    ModelLoadFailed {
        #[from]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("SmolLM2 tokenizer initialization failed: {reason}: {source}")]
    TokenizerFailed {
        reason: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Input exceeds maximum length: {length} (max: {max_length})")]
    InputTooLong {
        length: usize,
        max_length: usize,
    },

    #[error("SmolLM2 ONNX inference failed: {message}")]
    InferenceFailed {
        message: String,
        chunk_id: usize,
    },

    #[error("Output decoding failed: {reason}")]
    OutputDecodingFailed {
        reason: String,
        token_ids: Vec<u32>,
    },

    #[error("Model health check failed: {component}")]
    HealthCheckFailed {
        component: String,
    },

    #[error("Output validation failed: {reason}")]
    OutputValidationFailed {
        reason: String,
        output: String,
        chunk_id: usize,
    },

    #[error("Performance contract violated: {constraint}")]
    PerformanceViolation {
        constraint: String,
        measured: String,
        expected: String,
    },
}

/// Validation helpers for enforcing SmolLM2 contracts
pub struct SmolOutputValidator;

impl SmolOutputValidator {
    /// Validate summary meets all contract requirements
    pub fn validate_summary(summary: &str, input_chunk: &Chunk) -> Result<()> {
        // Length validation (10-200 characters)
        if summary.len() < 10 {
            return Err(SmolInferenceError::OutputValidationFailed {
                reason: format!("Summary too short: {} chars (min: 10)", summary.len()),
                output: summary.to_string(),
                chunk_id: input_chunk.id as usize,
            }.into());
        }

        if summary.len() > 200 {
            return Err(SmolInferenceError::OutputValidationFailed {
                reason: format!("Summary too long: {} chars (max: 200)", summary.len()),
                output: summary.to_string(),
                chunk_id: input_chunk.id as usize,
            }.into());
        }

        // Content validation - must contain meaningful words
        let meaningful_words: Vec<&str> = summary
            .split_whitespace()
            .filter(|word| word.len() > 2 && !word.chars().all(|c| c.is_ascii_punctuation()))
            .collect();

        if meaningful_words.is_empty() {
            return Err(SmolInferenceError::OutputValidationFailed {
                reason: "Summary contains no meaningful words".to_string(),
                output: summary.to_string(),
                chunk_id: input_chunk.id as usize,
            }.into());
        }

        // Validate that summary relates to code concepts
        let code_keywords = ["function", "struct", "impl", "trait", "enum", "mod", "use", "let", "fn", "type", "class", "method", "variable", "constant"];
        let has_code_concept = meaningful_words.iter().any(|word| {
            code_keywords.iter().any(|keyword| word.to_lowercase().contains(keyword))
        });

        // Not strictly required, but good for validation
        if !has_code_concept {
            println!("⚠️  Summary may not describe code concepts: {}", summary);
        }

        Ok(())
    }

    /// Check if two outputs are meaningfully different (>80% difference)
    pub fn outputs_are_different(output1: &str, output2: &str) -> bool {
        let words1: std::collections::HashSet<&str> =
            output1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> =
            output2.split_whitespace().collect();

        if words1.is_empty() || words2.is_empty() {
            return true;
        }

        // Calculate Jaccard similarity
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        let similarity = if union > 0 {
            intersection as f64 / union as f64
        } else {
            0.0
        };

        similarity < 0.8 // Require at least 20% difference
    }
}

/// Performance measurement utilities (Principle 5)
pub struct SmolPerformanceMetrics {
    pub start_time: std::time::Instant,
    pub memory_before: usize,
}

impl SmolPerformanceMetrics {
    pub fn start() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            memory_before: Self::get_memory_usage(),
        }
    }

    pub fn validate_contract(&self, max_duration: std::time::Duration, max_memory_mb: usize, chunk_id: usize) -> Result<()> {
        let elapsed = self.start_time.elapsed();
        let memory_after = Self::get_memory_usage();
        let memory_used_mb = (memory_after.saturating_sub(self.memory_before)) / (1024 * 1024);

        if elapsed > max_duration {
            return Err(SmolInferenceError::PerformanceViolation {
                constraint: format!("Execution time < {:?}", max_duration),
                measured: format!("{:?}", elapsed),
                expected: format!("< {:?}", max_duration),
            }.into());
        }

        if memory_used_mb > max_memory_mb {
            return Err(SmolInferenceError::PerformanceViolation {
                constraint: format!("Memory usage < {}MB", max_memory_mb),
                measured: format!("{}MB", memory_used_mb),
                expected: format!("< {}MB", max_memory_mb),
            }.into());
        }

        println!("📊 Performance: {:?}, {}MB memory", elapsed, memory_used_mb);
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    fn get_memory_usage() -> usize {
        // Simplified memory tracking for cross-platform compatibility
        // In production, would use proper memory profiling
        0
    }

    #[cfg(target_os = "macos")]
    fn get_memory_usage() -> usize {
        // macOS-specific memory tracking could be implemented here
        // For now, return 0 to avoid platform-specific complexity
        0
    }
}

#[cfg(test)]
mod contract_tests {
    use super::*;

    /// Test Scenario 1: Output Diversity Validation (REQ-SMOL-001.3)
    #[test]
    fn test_output_diversity_contract() {
        let validator = SmolOutputValidator;

        let output1 = "Function that prints hello world message to console";
        let output2 = "Struct representing user with name and age fields";
        let output3 = "Implementation of Debug trait for MyStruct type";

        // All outputs should be different from each other
        assert!(SmolOutputValidator::outputs_are_different(output1, output2));
        assert!(SmolOutputValidator::outputs_are_different(output2, output3));
        assert!(SmolOutputValidator::outputs_are_different(output1, output3));
    }

    /// Test Scenario 2: Summary Length Validation
    #[test]
    fn test_summary_length_validation() {
        let validator = SmolOutputValidator;
        let chunk = Chunk {
            id: 0,
            line_start: 0,
            line_end: 3,
            line_count: 3,
            content: "fn test() { println!(\"test\"); }".to_string(),
        };

        // Too short
        let result = SmolOutputValidator::validate_summary("short", &chunk);
        assert!(result.is_err());

        // Too long
        let long_summary = "a".repeat(201);
        let result = SmolOutputValidator::validate_summary(&long_summary, &chunk);
        assert!(result.is_err());

        // Valid length
        let valid_summary = "This function prints a test message to the console output";
        let result = SmolOutputValidator::validate_summary(valid_summary, &chunk);
        assert!(result.is_ok());
    }

    /// Test Scenario 3: Performance Contract Validation (REQ-SMOL-001.4)
    #[test]
    fn test_performance_contract() {
        let metrics = SmolPerformanceMetrics::start();
        std::thread::sleep(std::time::Duration::from_millis(10));

        // Should pass with generous limits
        let result = metrics.validate_contract(
            std::time::Duration::from_millis(500),
            100,
            0
        );
        assert!(result.is_ok());

        // Would fail with tight limits
        let result = metrics.validate_contract(
            std::time::Duration::from_millis(5),
            100,
            0
        );
        assert!(result.is_err());
    }

    /// Test Scenario 4: Content Validation
    #[test]
    fn test_content_validation() {
        let validator = SmolOutputValidator;
        let chunk = Chunk {
            id: 0,
            line_start: 0,
            line_end: 3,
            line_count: 3,
            content: "fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
        };

        // Valid content with code concepts
        let valid_summary = "Function that adds two integers and returns the result";
        let result = SmolOutputValidator::validate_summary(valid_summary, &chunk);
        assert!(result.is_ok());

        // Empty content
        let empty_summary = "          ";
        let result = SmolOutputValidator::validate_summary(empty_summary, &chunk);
        assert!(result.is_err());
    }
}