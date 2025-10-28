//! Layer 1 Core Traits
//!
//! This module defines the foundational abstractions for the dobby code summarizer.
//! All traits are designed with executable specifications and performance contracts.
//!
//! ## Architecture Principles:
//! - TDD-First: All traits have comprehensive failing tests
//! - RAII: Automatic resource management with lifetimes
//! - Async-First: All operations are async by default
//! - Performance Contracts: Measurable performance guarantees
//! - Error Propagation: Detailed error types with context

// pub mod database;  // Temporarily disabled for REFACTOR demonstration
// pub mod error;     // Temporarily disabled for REFACTOR demonstration
// pub mod inference; // Temporarily disabled for REFACTOR demonstration
// pub mod pipeline;  // Temporarily disabled for REFACTOR demonstration

// GREEN PHASE: Minimal implementations
pub mod implementations;

// Include test modules
#[cfg(test)]
pub mod tests;

// Re-export core types for convenience
// pub use database::*;  // Temporarily disabled for REFACTOR demonstration
// pub use error::*;     // Temporarily disabled for REFACTOR demonstration
// pub use inference::*; // Temporarily disabled for REFACTOR demonstration
// pub use pipeline::*;  // Temporarily disabled for REFACTOR demonstration