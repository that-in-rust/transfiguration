//! Parallel processing orchestrator for ONNX model instances
//!
//! Follows TDD-First architecture with executable contracts and performance validation.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tracing::{info, debug, warn, error};
use anyhow::{Context, Result};

use crate::types::{
    ProcessingResult, ProcessingStats, ProcessingError,
    BatchConfig, OrchestratorState, ProcessingState, ContractValidation
};
use crate::chunker::{TextChunker, CodeChunk};
use crate::model_provider::OnnxModelProvider;
use crate::types::ModelProvider;

/// Main orchestrator for parallel ONNX processing
pub struct ParallelOrchestrator {
    /// Text chunker for breaking large files
    chunker: TextChunker,
    /// ONNX model provider for inference
    model_provider: Arc<dyn ModelProvider>,
    /// Batch processing configuration
    config: BatchConfig,
    /// Current orchestrator state
    state: Arc<tokio::sync::Mutex<OrchestratorState>>,
    /// Performance metrics collection
    metrics: Arc<tokio::sync::Mutex<Vec<ContractValidation>>>,
}

impl ParallelOrchestrator {
    /// Create new orchestrator with executable validation
    ///
    /// # Preconditions
    /// - chunk_size > 0
    /// - model_provider is properly initialized
    /// - config has valid parameters
    ///
    /// # Postconditions
    /// - Returns Ok(ParallelOrchestrator) ready for processing
    /// - All components initialized and validated
    ///
    /// # Error Conditions
    /// - ProcessingError::ParallelCoordinationFailed if initialization fails
    pub async fn new(
        chunk_size: usize,
        model_provider: Arc<dyn ModelProvider>,
        config: BatchConfig,
    ) -> Result<Arc<Self>> {
        info!("Initializing ParallelOrchestrator with chunk_size: {}", chunk_size);

        // Validate preconditions
        if chunk_size == 0 {
            return Err(ProcessingError::ParallelCoordinationFailed {
                message: "chunk_size must be > 0".to_string(),
            }.into());
        }

        if config.max_parallel_chunks == 0 {
            return Err(ProcessingError::ParallelCoordinationFailed {
                message: "max_parallel_chunks must be > 0".to_string(),
            }.into());
        }

        let chunker = TextChunker::new(chunk_size)?;
        let state = Arc::new(tokio::sync::Mutex::new(OrchestratorState::new(0)));

        let orchestrator = Arc::new(Self {
            chunker,
            model_provider,
            config,
            state,
            metrics: Arc::new(tokio::sync::Mutex::new(Vec::new())),
        });

        info!("ParallelOrchestrator initialized successfully");
        Ok(orchestrator)
    }

    /// Process file with parallel ONNX model instances
    ///
    /// # Preconditions
    /// - file_path points to readable Rust source file
    /// - Model provider is initialized
    ///
    /// # Postconditions
    /// - Returns ProcessingStats with comprehensive metrics
    /// - All chunks processed in parallel up to max_parallel_chunks
    /// - Performance contracts validated and recorded
    ///
    /// # Error Conditions
    /// - ProcessingError::ParallelCoordinationFailed if orchestration fails
    /// - ProcessingError::ResourceExhaustion if limits exceeded
    pub async fn process_file(
        &self,
        file_path: &std::path::Path,
    ) -> Result<ProcessingStats> {
        let overall_start = Instant::now();

        // Update state
        {
            let mut state = self.state.lock().await;
            state.current_state = ProcessingState::LoadingChunks;
        }

        info!("Starting parallel processing of file: {:?}", file_path);

        // Step 1: Break file into chunks
        let chunks = self.chunker.chunk_file(file_path).await
            .context("Failed to chunk input file")?;

        info!("File broken into {} chunks", chunks.len());

        // Update state with total chunks
        {
            let mut state = self.state.lock().await;
            state.total_chunks = chunks.len();
            state.current_state = ProcessingState::ProcessingParallel;
        }

        // Step 2: Initialize parallel model instances
        let model_instances_needed = self.config.max_parallel_chunks.min(chunks.len());
        info!("Initializing {} model instances for parallel processing", model_instances_needed);

        let initialization_semaphore = Arc::new(Semaphore::new(model_instances_needed));
        let mut init_tasks = JoinSet::new();

        for instance_id in 0..model_instances_needed {
            let provider = Arc::clone(&self.model_provider);
            let semaphore = Arc::clone(&initialization_semaphore);

            init_tasks.spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                provider.initialize(instance_id).await
            });
        }

        // Wait for all instances to initialize
        let mut initialized_instances = 0;
        while let Some(init_result) = init_tasks.join_next().await {
            match init_result {
                Ok(Ok(_)) => initialized_instances += 1,
                Ok(Err(e)) => {
                    warn!("Failed to initialize model instance: {:?}", e);
                },
                Err(e) => {
                    error!("Task failed during model initialization: {:?}", e);
                }
            }
        }

        info!("Successfully initialized {} model instances", initialized_instances);

        // Step 3: Process chunks in parallel batches
        let processing_start = Instant::now();
        let mut all_results = Vec::new();

        // Process chunks in batches to respect parallel limits
        for chunk_batch in chunks.chunks(self.config.max_parallel_chunks) {
            let batch_results = self.process_chunk_batch(chunk_batch).await?;
            all_results.extend(batch_results);
        }

        let processing_time = processing_start.elapsed();

        // Step 4: Cleanup model instances
        self.cleanup_model_instances(initialized_instances).await?;

        // Step 5: Generate final statistics
        let stats = self.generate_processing_stats(
            &chunks,
            &all_results,
            overall_start.elapsed(),
            processing_time,
        ).await?;

        // Update final state
        {
            let mut state = self.state.lock().await;
            state.current_state = ProcessingState::Completed;
        }

        info!("Parallel processing completed successfully: {:?}", stats);

        Ok(stats)
    }

    /// Process a batch of chunks in parallel
    async fn process_chunk_batch(
        &self,
        chunk_batch: &[CodeChunk],
    ) -> Result<Vec<ProcessingResult>> {
        debug!("Processing batch of {} chunks", chunk_batch.len());

        let batch_semaphore = Arc::new(Semaphore::new(chunk_batch.len()));
        let mut tasks = JoinSet::new();

        for (i, chunk) in chunk_batch.iter().enumerate() {
            let chunk = chunk.clone();
            let provider = Arc::clone(&self.model_provider);
            let semaphore = Arc::clone(&batch_semaphore);
            let timeout = self.config.chunk_timeout;

            tasks.spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();

                // Process chunk with timeout
                let process_future = provider.process_chunk(&chunk.content, i as u32);

                match tokio::time::timeout(timeout, process_future).await {
                    Ok(result) => result,
                    Err(_) => Err(ProcessingError::InferenceFailed {
                        instance_id: i as u32,
                        message: format!("Processing timeout after {:?}", timeout),
                    }),
                }
            });
        }

        let mut batch_results = Vec::new();
        while let Some(result) = tasks.join_next().await {
            match result {
                Ok(Ok(processing_result)) => {
                    debug!("Chunk processed successfully");
                    batch_results.push(processing_result);
                },
                Ok(Err(e)) => {
                    warn!("Chunk processing failed: {:?}", e);
                    if !self.config.continue_on_failure {
                        return Err(ProcessingError::ParallelCoordinationFailed {
                            message: format!("Chunk processing failed: {:?}", e),
                        });
                    }
                },
                Err(e) => {
                    error!("Task failed: {:?}", e);
                    return Err(ProcessingError::ParallelCoordinationFailed {
                        message: format!("Task failed: {:?}", e),
                    });
                }
            }
        }

        // Update state
        {
            let mut state = self.state.lock().await;
            state.chunks_processed += chunk_batch.len();
            state.results_collected += batch_results.len();
        }

        Ok(batch_results)
    }

    /// Cleanup initialized model instances
    async fn cleanup_model_instances(&self, instance_count: usize) -> Result<()> {
        info!("Cleaning up {} model instances", instance_count);

        let mut cleanup_tasks = JoinSet::new();
        let cleanup_semaphore = Arc::new(Semaphore::new(5)); // Limit concurrent cleanups

        for instance_id in 0..instance_count {
            let provider = Arc::clone(&self.model_provider);
            let semaphore = Arc::clone(&cleanup_semaphore);

            cleanup_tasks.spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                provider.cleanup(instance_id).await
            });
        }

        // Wait for all cleanup tasks to complete
        while let Some(cleanup_result) = cleanup_tasks.join_next().await {
            match cleanup_result {
                Ok(Ok(_)) => debug!("Instance cleanup successful"),
                Ok(Err(e)) => warn!("Instance cleanup failed: {:?}", e),
                Err(e) => error!("Cleanup task failed: {:?}", e),
            }
        }

        info!("Model instance cleanup completed");
        Ok(())
    }

    /// Generate comprehensive processing statistics
    async fn generate_processing_stats(
        &self,
        chunks: &[CodeChunk],
        results: &[ProcessingResult],
        total_time: Duration,
        processing_time: Duration,
    ) -> Result<ProcessingStats> {
        let successful_chunks = results.len();
        let failed_chunks = chunks.len().saturating_sub(successful_chunks);

        // Calculate average processing time
        let avg_processing_time = if results.is_empty() {
            Duration::ZERO
        } else {
            let total: Duration = results.iter().map(|r| r.processing_time).sum();
            total / results.len() as u32
        };

        // Calculate parallel efficiency
        let sequential_time = processing_time * chunks.len() as u32;
        let parallel_efficiency = if processing_time.as_secs_f32() > 0.0 {
            (sequential_time.as_secs_f32() / processing_time.as_secs_f32()) / chunks.len() as f32 * 100.0
        } else {
            100.0
        };

        let stats = ProcessingStats {
            total_chunks: chunks.len(),
            successful_chunks,
            failed_chunks,
            avg_processing_time,
            total_processing_time: processing_time,
            parallel_efficiency,
            instances_utilized: self.config.max_parallel_chunks.min(chunks.len()),
        };

        // Log detailed performance metrics
        info!("=== Processing Statistics ===");
        info!("Total chunks: {}", stats.total_chunks);
        info!("Successful: {}, Failed: {}", stats.successful_chunks, stats.failed_chunks);
        info!("Average processing time: {:?}", stats.avg_processing_time);
        info!("Total processing time: {:?}", stats.total_processing_time);
        info!("Parallel efficiency: {:.1}%", stats.parallel_efficiency);
        info!("Instances utilized: {}", stats.instances_utilized);
        info!("Overall time: {:?}", total_time);

        // Store performance validations
        {
            let mut metrics = self.metrics.lock().await;
            for result in results {
                let validations = self.validate_processing_result(result);
                metrics.extend(validations);
            }
        }

        Ok(stats)
    }

    /// Validate individual processing results
    fn validate_processing_result(&self, result: &ProcessingResult) -> Vec<ContractValidation> {
        let mut validations = Vec::new();

        // Validate processing time (should be under 5 seconds)
        let processing_time_s = result.processing_time.as_secs_f32();
        let time_validation = if processing_time_s <= 5.0 {
            ContractValidation::success(
                "chunk_processing_time",
                &format!("{:.2}s", processing_time_s),
                "≤5.0s",
            )
        } else {
            ContractValidation::failure(
                "chunk_processing_time",
                &format!("{:.2}s", processing_time_s),
                "≤5.0s",
            )
        };
        validations.push(time_validation);

        // Validate summary is not empty
        let summary_validation = if !result.summary.trim().is_empty() {
            ContractValidation::success(
                "summary_non_empty",
                &format!("{} chars", result.summary.len()),
                ">0 chars",
            )
        } else {
            ContractValidation::failure(
                "summary_non_empty",
                "0 chars",
                ">0 chars",
            )
        };
        validations.push(summary_validation);

        // Validate confidence is reasonable
        let confidence_validation = if (0.0..=1.0).contains(&result.confidence) {
            ContractValidation::success(
                "confidence_range",
                &format!("{:.3}", result.confidence),
                "0.0-1.0",
            )
        } else {
            ContractValidation::failure(
                "confidence_range",
                &format!("{:.3}", result.confidence),
                "0.0-1.0",
            )
        };
        validations.push(confidence_validation);

        validations
    }

    /// Get current orchestrator state for monitoring
    pub async fn get_state(&self) -> OrchestratorState {
        self.state.lock().await.clone()
    }

    /// Get collected performance metrics
    pub async fn get_metrics(&self) -> Vec<ContractValidation> {
        self.metrics.lock().await.clone()
    }

    /// Reset orchestrator state for new processing session
    pub async fn reset(&self) -> Result<()> {
        info!("Resetting orchestrator state");

        {
            let mut state = self.state.lock().await;
            *state = OrchestratorState::new(0);
        }

        {
            let mut metrics = self.metrics.lock().await;
            metrics.clear();
        }

        info!("Orchestrator state reset completed");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ModelInfo;
    use tokio_test;

    // Mock model provider for testing
    struct MockModelProvider;

    #[async_trait::async_trait]
    impl ModelProvider for MockModelProvider {
        async fn initialize(&self, instance_id: u32) -> Result<(), ProcessingError> {
            Ok(())
        }

        async fn process_chunk(
            &self,
            chunk_content: &str,
            instance_id: u32,
        ) -> Result<ProcessingResult, ProcessingError> {
            Ok(ProcessingResult {
                id: uuid::Uuid::new_v4(),
                chunk_id: uuid::Uuid::new_v4(),
                summary: format!("Mock summary for instance {}", instance_id),
                processing_time: Duration::from_millis(100),
                model_instance_id: instance_id,
                confidence: 0.8,
            })
        }

        async fn cleanup(&self, _instance_id: u32) -> Result<(), ProcessingError> {
            Ok(())
        }

        fn model_info(&self) -> ModelInfo {
            ModelInfo {
                name: "MockModel".to_string(),
                version: "1.0".to_string(),
                max_sequence_length: 1000,
                supports_parallel: true,
                memory_requirement_mb: 100,
            }
        }
    }

    #[tokio_test]
    async fn test_orchestrator_initialization() {
        let provider = Arc::new(MockModelProvider);
        let config = BatchConfig::default();

        let result = ParallelOrchestrator::new(300, provider, config).await;
        assert!(result.is_ok());
    }

    #[tokio_test]
    async fn test_orchestrator_state_management() {
        let provider = Arc::new(MockModelProvider);
        let config = BatchConfig::default();
        let orchestrator = ParallelOrchestrator::new(100, provider, config).await.unwrap();

        let state = orchestrator.get_state().await;
        assert_eq!(state.total_chunks, 0);
        assert_eq!(state.progress(), 0.0);
        assert!(!state.is_complete());
    }

    #[tokio_test]
    async fn test_processing_result_validation() {
        let provider = Arc::new(MockModelProvider);
        let config = BatchConfig::default();
        let orchestrator = ParallelOrchestrator::new(300, provider, config).await.unwrap();

        let good_result = ProcessingResult {
            id: uuid::Uuid::new_v4(),
            chunk_id: uuid::Uuid::new_v4(),
            summary: "Valid summary".to_string(),
            processing_time: Duration::from_secs(1),
            model_instance_id: 1,
            confidence: 0.8,
        };

        let validations = orchestrator.validate_processing_result(&good_result);
        assert!(validations.iter().all(|v| v.satisfied));

        let bad_result = ProcessingResult {
            id: uuid::Uuid::new_v4(),
            chunk_id: uuid::Uuid::new_v4(),
            summary: "".to_string(), // Empty summary
            processing_time: Duration::from_secs(10), // Too slow
            model_instance_id: 1,
            confidence: 1.5, // Invalid confidence
        };

        let validations = orchestrator.validate_processing_result(&bad_result);
        assert!(!validations.iter().all(|v| v.satisfied));
    }
}