//! tempPOC: Parallel ONNX LLM inference for Tokio code summarization
//!
//! Follows TDD-First architecture with executable specifications and measurable contracts.
//!
//! ## Architecture Overview
//! ```mermaid
//! graph TD
//!     A[Tokio Source File] --> B[TextChunker]
//!     B --> C[ParallelOrchestrator]
//!     C --> D[ONNX Model Provider]
//!     C --> E[20 Parallel Instances]
//!     D --> F[Processing Results]
//!     E --> F
//!     F --> G[Final Output Files]
//! ```
//!
//! ## Performance Contracts
//! - Chunking: 1MB file in <100ms
//! - Processing: 300-line chunks in <5s each
//! - Parallelism: 20 concurrent model instances
//! - Memory: <512MB per instance
//! - Summaries: ≤120 characters, ≥60% confidence

pub mod chunker;
pub mod types;
pub mod model_provider;
pub mod orchestrator;
pub mod complete_onnx_provider;

pub use chunker::{TextChunker, CodeChunk, ChunkError};
pub use types::{
    ProcessingResult, ProcessingStats, ProcessingError,
    ModelPerformanceContract, BatchConfig, OrchestratorState
};
pub use model_provider::OnnxModelProvider;
pub use orchestrator::ParallelOrchestrator;
pub use complete_onnx_provider::CompleteOnnxProvider;