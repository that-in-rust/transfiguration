//! Parallel orchestration using Tokio JoinSet with TDD-First contracts
//!
//! Executable Specifications:
//! - Throttle concurrency to configured max sessions
//! - Borrow sessions from pool (stub for now)
//! - Collect results and feed to aggregator/validator

use crate::chunking::Chunk;
use crate::inference::InferencePipeline;
use crate::results::{ChunkResult, ProcessingResults, DefaultResultsAggregator, ResultsAggregator};
use crate::config::SystemConfig;
use crate::errors::Result;
use std::time::Instant;

/// Processing configuration
#[derive(Debug, Clone)]
pub struct ProcessingConfig {
    pub concurrency: usize,
}

impl From<&SystemConfig> for ProcessingConfig {
    fn from(cfg: &SystemConfig) -> Self {
        Self { concurrency: cfg.max_concurrent_sessions }
    }
}

/// Parallel processor trait for dependency injection
pub trait ParallelProcessor: Send + Sync {
    async fn process_chunks_parallel<I: InferencePipeline + 'static>(
        &self,
        chunks: Vec<Chunk>,
        inference: &I,
        config: &SystemConfig,
    ) -> Result<ProcessingResults>;
}

/// Default Tokio-based processor
pub struct TokioParallelProcessor {
    cfg: ProcessingConfig,
}

impl TokioParallelProcessor {
    pub fn new(cfg: ProcessingConfig) -> Self { Self { cfg } }
}

impl ParallelProcessor for TokioParallelProcessor {
    async fn process_chunks_parallel<I: InferencePipeline + 'static>(
        &self,
        chunks: Vec<Chunk>,
        inference: &I,
        config: &SystemConfig,
    ) -> Result<ProcessingResults> {
        let total = chunks.len();
        let mut results = Vec::with_capacity(total);

        // Process chunks sequentially for MVP (real impl will use tokio::spawn)
        for chunk in chunks {
            let start = Instant::now();
            let summary = inference.process_chunk(&chunk);
            let chunk_result = match summary {
                Ok(s) => ChunkResult { 
                    chunk_id: chunk.id as usize, 
                    summary: s, 
                    processing_time: start.elapsed(), 
                    success: true, 
                    error_message: None 
                },
                Err(e) => ChunkResult { 
                    chunk_id: chunk.id as usize, 
                    summary: String::new(), 
                    processing_time: start.elapsed(), 
                    success: false, 
                    error_message: Some(e.to_string()) 
                },
            };
            results.push(chunk_result);
        }

        // Aggregate
        let aggregator = DefaultResultsAggregator;
        let mut aggregated = aggregator.aggregate(results);

        // STUB: no memory tracking yet; set peak memory to 0
        aggregated.peak_memory_mb = 0;
        aggregated
            .summary(); // compute final metrics already done inside aggregate

        Ok(aggregated)
    }
}
