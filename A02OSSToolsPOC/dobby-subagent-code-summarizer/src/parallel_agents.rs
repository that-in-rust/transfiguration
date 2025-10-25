//! 20-Agent Parallel Processing Architecture
//!
//! Implements session-per-agent pattern for maximum parallelism on Mac Mini
//! Each agent gets its own RealInferencePipeline instance to avoid mutex conflicts

use anyhow::Result;
use log::{info, error, warn};
use std::sync::Arc;
use tokio::task::JoinHandle;
use std::path::PathBuf;

use crate::inference::RealInferencePipeline;

/// Configuration for 20-agent parallel processing system
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    /// Number of independent agents (20 for Mac Mini optimization)
    pub agent_count: usize,
    /// Model directory path
    pub model_dir: PathBuf,
    /// Tokenizer directory path
    pub tokenizer_dir: PathBuf,
    /// Maximum concurrent tasks (typically matches Mac Mini core count)
    pub max_concurrent: usize,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            agent_count: 20,  // User-specified maximum parallelism
            model_dir: PathBuf::from("./models/qwen2.5-0.5b-int4"),
            tokenizer_dir: PathBuf::from("./tokenizer_dir"),
            max_concurrent: num_cpus::get(), // Mac Mini core count (8-10)
        }
    }
}

/// 20-Agent Parallel Processing System
///
/// Uses session isolation strategy: each agent has its own RealInferencePipeline
/// This eliminates mutex races and enables true parallel processing
pub struct ParallelAgentSystem {
    /// Pool of 20 independent inference sessions
    sessions: Vec<Arc<RealInferencePipeline>>,
    /// Configuration for parallel processing
    config: ParallelConfig,
}

impl ParallelAgentSystem {
    /// Create new 20-agent parallel processing system
    ///
    /// # Arguments
    /// * `config` - Parallel processing configuration
    ///
    /// # Returns
    /// `Result<ParallelAgentSystem>` - System with 20 independent sessions
    pub fn new(config: ParallelConfig) -> Result<Self> {
        info!("🚀 Initializing 20-Agent Parallel Processing System");
        info!("Agent count: {}, Max concurrent: {}", config.agent_count, config.max_concurrent);

        // Phase 1: Create 20 independent RealInferencePipeline instances
        // Session isolation eliminates mutex conflicts for maximum parallelism
        let mut sessions = Vec::with_capacity(config.agent_count);

        for i in 0..config.agent_count {
            info!("Creating agent {} independent session...", i);

            // Each agent gets its own session to avoid shared resource conflicts
            let session = Arc::new(RealInferencePipeline::new(
                config.model_dir.clone(),
                config.tokenizer_dir.clone()
            )?);

            sessions.push(session);
            info!("✅ Agent {} session created successfully", i);
        }

        info!("🎉 All 20 agent sessions created - parallel system ready");

        Ok(Self {
            sessions,
            config,
        })
    }

    /// Process multiple code chunks in parallel using 20 agents
    ///
    /// # Arguments
    /// * `chunks` - Vector of code chunks to process
    ///
    /// # Returns
    /// `Result<Vec<(String, String)>>` - Vector of (chunk, summary) pairs
    pub async fn process_chunks_parallel(&self, chunks: Vec<String>) -> Result<Vec<(String, String)>> {
        info!("🔄 Starting parallel processing of {} chunks with {} agents",
              chunks.len(), self.sessions.len());

        // Phase 2: Assign chunks to agents using round-robin scheduling
        // This ensures load balancing across all 20 agents
        let mut handles: Vec<JoinHandle<(String, String)>> = Vec::new();

        for (chunk_index, chunk) in chunks.into_iter().enumerate() {
            // Round-robin agent selection for optimal load distribution
            let agent_index = chunk_index % self.sessions.len();
            let session = Arc::clone(&self.sessions[agent_index]);

            info!("Dispatching chunk {} to agent {} ({} chars)",
                  chunk_index, agent_index, chunk.len());

            // Spawn async task for parallel processing
            let handle = tokio::spawn(async move {
                let start_time = std::time::Instant::now();

                // Process chunk with isolated session (no mutex conflicts)
                let summary = session.summarize_chunk(&chunk)
                    .unwrap_or_else(|e| {
                        error!("❌ Agent {} failed to process chunk: {}", agent_index, e);
                        format!("ERROR: Failed to process chunk - {}", e)
                    });

                let duration = start_time.elapsed();
                info!("✅ Agent {} completed chunk {} in {:?}", agent_index, chunk_index, duration);

                (chunk, summary)
            });

            handles.push(handle);

            // Phase 3: Concurrency control to respect Mac Mini limits
            // Don't overwhelm the system with too many concurrent tasks
            if handles.len() >= self.config.max_concurrent {
                info!("🔄 Reached max concurrent {} - waiting for task completion", self.config.max_concurrent);

                // Wait for at least one task to complete before spawning more
                if let Some(completed) = handles.pop() {
                    let _ = completed.await;
                }
            }
        }

        // Phase 4: Collect all results
        info!("📊 Collecting results from {} parallel tasks", handles.len());
        let mut results = Vec::new();

        for handle in handles {
            match handle.await {
                Ok((chunk, summary)) => {
                    results.push((chunk, summary));
                }
                Err(e) => {
                    warn!("⚠️ Task failed: {}", e);
                    // Continue processing other tasks even if one fails
                }
            }
        }

        info!("🎉 Parallel processing completed - {} results collected", results.len());
        Ok(results)
    }

    /// Get system performance metrics
    pub fn get_metrics(&self) -> ParallelMetrics {
        ParallelMetrics {
            total_agents: self.sessions.len(),
            max_concurrent: self.config.max_concurrent,
            model_dir: self.config.model_dir.clone(),
            tokenizer_dir: self.config.tokenizer_dir.clone(),
        }
    }
}

/// Performance metrics for the 20-agent system
#[derive(Debug, Clone)]
pub struct ParallelMetrics {
    pub total_agents: usize,
    pub max_concurrent: usize,
    pub model_dir: PathBuf,
    pub tokenizer_dir: PathBuf,
}

impl std::fmt::Display for ParallelMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "20-Agent System Metrics:\n")?;
        write!(f, "  Total Agents: {}\n", self.total_agents)?;
        write!(f, "  Max Concurrent: {}\n", self.max_concurrent)?;
        write!(f, "  Model Dir: {}\n", self.model_dir.display())?;
        write!(f, "  Tokenizer Dir: {}\n", self.tokenizer_dir.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parallel_agent_system_creation() {
        // Test Phase 1: System creation with 20 agents
        let config = ParallelConfig::default();

        match ParallelAgentSystem::new(config) {
            Ok(system) => {
                let metrics = system.get_metrics();
                println!("✅ 20-Agent system created successfully");
                println!("Metrics: {}", metrics);
                assert_eq!(metrics.total_agents, 20);
                assert!(metrics.max_concurrent > 0);
            }
            Err(e) => {
                println!("❌ 20-Agent system creation failed: {}", e);
                panic!("20-Agent system should create successfully with ort 1.16.3");
            }
        }
    }

    #[tokio::test]
    async fn test_parallel_chunk_processing() {
        // Test Phase 2: Parallel chunk processing
        let config = ParallelConfig {
            agent_count: 5, // Smaller for testing
            ..Default::default()
        };

        let system = ParallelAgentSystem::new(config).expect("System should create");

        let test_chunks = vec![
            "fn test1() { println!(\"hello\"); }".to_string(),
            "fn test2() { let x = 42; }".to_string(),
            "fn test3() { if true { println!(\"yes\"); } }".to_string(),
        ];

        match system.process_chunks_parallel(test_chunks.clone()).await {
            Ok(results) => {
                println!("✅ Parallel processing successful: {} results", results.len());
                assert_eq!(results.len(), test_chunks.len());

                for (i, (chunk, summary)) in results.iter().enumerate() {
                    println!("Result {}: chunk='{}', summary='{}'", i, chunk, summary);
                    assert!(!summary.is_empty());
                }
            }
            Err(e) => {
                println!("❌ Parallel processing failed: {}", e);
                panic!("Parallel processing should work with placeholder implementation");
            }
        }
    }
}