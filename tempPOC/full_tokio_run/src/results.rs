//! Results aggregation with TDD-First contracts
//!
//! Executable Specifications:
//! - Track successful and failed chunk processing
//! - Calculate parallel efficiency (active_time / total_time)
//! - Monitor memory usage during processing

use std::time::{Duration, Instant};
use crate::errors::Result;

/// Individual chunk processing result
#[derive(Debug, Clone)]
pub struct ChunkResult {
    pub chunk_id: usize,
    pub summary: String,
    pub processing_time: Duration,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Aggregated processing results with TDD contracts
#[derive(Debug)]
pub struct ProcessingResults {
    pub chunk_results: Vec<ChunkResult>,
    pub total_chunks: usize,
    pub successful_chunks: usize,
    pub failed_chunks: usize,
    pub total_processing_time: Duration,
    pub parallel_efficiency: f32,
    pub peak_memory_mb: usize,
    pub start_time: Instant,
}

impl ProcessingResults {
    pub fn new(total_chunks: usize) -> Self {
        Self {
            chunk_results: Vec::with_capacity(total_chunks),
            total_chunks,
            successful_chunks: 0,
            failed_chunks: 0,
            total_processing_time: Duration::from_secs(0),
            parallel_efficiency: 0.0,
            peak_memory_mb: 0,
            start_time: Instant::now(),
        }
    }

    /// Add chunk result and update metrics
    pub fn add_result(&mut self, result: ChunkResult) {
        if result.success {
            self.successful_chunks += 1;
        } else {
            self.failed_chunks += 1;
        }
        self.chunk_results.push(result);
    }

    /// Finalize results and calculate metrics
    pub fn finalize(&mut self) {
        self.total_processing_time = self.start_time.elapsed();
        
        // Calculate parallel efficiency
        // Formula: (sum of individual processing times) / (total wall clock time * num_sessions)
        let sum_processing_time: Duration = self.chunk_results
            .iter()
            .map(|r| r.processing_time)
            .sum();
        
        if !self.total_processing_time.is_zero() {
            self.parallel_efficiency = sum_processing_time.as_secs_f32() 
                / self.total_processing_time.as_secs_f32();
        }
    }

    /// Get processing count
    pub fn processed_count(&self) -> usize {
        self.chunk_results.len()
    }

    /// Check if all contracts are satisfied
    pub fn contracts_satisfied(&self, min_efficiency: f32, max_time_per_chunk_s: u64) -> bool {
        // Contract 1: Success rate ≥95%
        let success_rate = self.successful_chunks as f32 / self.total_chunks as f32;
        if success_rate < 0.95 {
            return false;
        }

        // Contract 2: Parallel efficiency ≥ threshold
        if self.parallel_efficiency < min_efficiency {
            return false;
        }

        // Contract 3: Max processing time per chunk
        let max_chunk_time = self.chunk_results
            .iter()
            .map(|r| r.processing_time.as_secs())
            .max()
            .unwrap_or(0);
        
        if max_chunk_time > max_time_per_chunk_s {
            return false;
        }

        true
    }

    /// Generate summary report
    pub fn summary(&self) -> String {
        format!(
            "Processing Results:\n\
             - Total chunks: {}\n\
             - Successful: {} ({:.1}%)\n\
             - Failed: {} ({:.1}%)\n\
             - Total time: {:?}\n\
             - Parallel efficiency: {:.1}%\n\
             - Peak memory: {}MB",
            self.total_chunks,
            self.successful_chunks,
            (self.successful_chunks as f32 / self.total_chunks as f32) * 100.0,
            self.failed_chunks,
            (self.failed_chunks as f32 / self.total_chunks as f32) * 100.0,
            self.total_processing_time,
            self.parallel_efficiency * 100.0,
            self.peak_memory_mb
        )
    }
}

/// Results aggregator trait for dependency injection
pub trait ResultsAggregator: Send + Sync {
    fn aggregate(&self, results: Vec<ChunkResult>) -> ProcessingResults;
}

/// Default results aggregator implementation
pub struct DefaultResultsAggregator;

impl ResultsAggregator for DefaultResultsAggregator {
    fn aggregate(&self, results: Vec<ChunkResult>) -> ProcessingResults {
        let total_chunks = results.len();
        let mut processing_results = ProcessingResults::new(total_chunks);
        
        for result in results {
            processing_results.add_result(result);
        }
        
        processing_results.finalize();
        processing_results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results_aggregation_contracts() {
        // TDD-First: GREEN phase - successful aggregation
        let mut results = ProcessingResults::new(10);
        
        for i in 0..10 {
            results.add_result(ChunkResult {
                chunk_id: i,
                summary: format!("Summary {}", i),
                processing_time: Duration::from_millis(100),
                success: true,
                error_message: None,
            });
        }
        
        results.finalize();
        
        assert_eq!(results.successful_chunks, 10);
        assert_eq!(results.failed_chunks, 0);
        assert!(results.parallel_efficiency > 0.0);
    }

    #[test]
    fn test_contract_satisfaction() {
        // TDD-First: Validate contracts
        let mut results = ProcessingResults::new(100);
        
        // Add 96 successful results (96% success rate)
        for i in 0..96 {
            results.add_result(ChunkResult {
                chunk_id: i,
                summary: format!("Summary {}", i),
                processing_time: Duration::from_millis(500),
                success: true,
                error_message: None,
            });
        }
        
        // Add 4 failed results
        for i in 96..100 {
            results.add_result(ChunkResult {
                chunk_id: i,
                summary: String::new(),
                processing_time: Duration::from_millis(0),
                success: false,
                error_message: Some("Failed".to_string()),
            });
        }
        
        results.finalize();
        
        // Should satisfy contracts with 96% success rate
        assert!(results.contracts_satisfied(0.8, 2));
    }
}
