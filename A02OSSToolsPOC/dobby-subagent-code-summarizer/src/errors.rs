//! Error types for ONNX parallel processing
//!
//! TDD-First: Structured error handling with thiserror for library

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ProcessingError>;

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("Model loading failed: {source}")]
    ModelLoadFailed {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Chunking failed for file {path}: {reason}")]
    ChunkingFailed {
        path: String,
        reason: String,
    },

    #[error("Resource exhaustion: {resource} (limit: {limit})")]
    ResourceExhaustion {
        resource: String,
        limit: usize,
    },

    #[error("Inference failed for chunk {chunk_id}: {message}")]
    InferenceFailed {
        chunk_id: usize,
        message: String,
    },

    #[error("Contract violation: {contract_name} - {violation}")]
    ContractViolation {
        contract_name: String,
        violation: String,
    },

    #[error("Session pool error: {message}")]
    SessionPoolError {
        message: String,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ONNX Runtime error: {0}")]
    OrtError(String),

    #[error("Tokenizer loading failed: {source}")]
    TokenizerLoadFailed {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Tokenization failed for text '{text}': {source}")]
    TokenizationFailed {
        text: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Detokenization failed for tokens {:?}: {source}")]
    DetokenizationFailed {
        token_ids: Vec<u32>,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

// Convert ort::Error to ProcessingError
impl From<ort::Error> for ProcessingError {
    fn from(err: ort::Error) -> Self {
        ProcessingError::OrtError(err.to_string())
    }
}

/// Convert from our new InferenceError to existing ProcessingError
impl From<crate::t5_inference_contract::InferenceError> for ProcessingError {
    fn from(err: crate::t5_inference_contract::InferenceError) -> Self {
        ProcessingError::InferenceFailed {
            chunk_id: 0, // Will be overridden if provided
            message: format!("Inference error: {}", err),
        }
    }
}
