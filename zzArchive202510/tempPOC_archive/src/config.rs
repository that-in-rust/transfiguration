//! System configuration with TDD-First contracts
//!
//! Executable Specifications:
//! - max_concurrent_sessions: 20
//! - max_memory_mb: 4096 (4GB)
//! - chunk_size: 300 lines
//! - max_chunk_processing_time_s: 2

use std::path::PathBuf;

/// System configuration with executable contracts
#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub tokio_source_path: PathBuf,
    pub model_path: PathBuf,
    pub max_concurrent_sessions: usize,
    pub max_memory_mb: usize,
    pub chunk_size: usize,
    pub max_chunk_processing_time_s: u64,
    pub min_parallel_efficiency: f32,
}

impl SystemConfig {
    /// Production configuration with validated contracts
    pub fn production() -> Self {
        Self {
            tokio_source_path: PathBuf::from("/Users/amuldotexe/Projects/transfiguration/tokio-rs-tokio-8a5edab282632443.txt"),
            model_path: PathBuf::from("models/torch_model.onnx"),
            max_concurrent_sessions: 20,
            max_memory_mb: 4096,
            chunk_size: 300,
            max_chunk_processing_time_s: 2,
            min_parallel_efficiency: 0.8,
        }
    }

    /// Test configuration for unit tests
    pub fn test_config() -> Self {
        Self {
            tokio_source_path: PathBuf::from("test_data/sample.txt"),
            model_path: PathBuf::from("models/torch_model.onnx"),
            max_concurrent_sessions: 4,
            max_memory_mb: 1024,
            chunk_size: 50,
            max_chunk_processing_time_s: 5,
            min_parallel_efficiency: 0.7,
        }
    }

    /// Performance test configuration
    pub fn performance_test_config() -> Self {
        Self {
            tokio_source_path: PathBuf::from("test_data/large.txt"),
            model_path: PathBuf::from("models/torch_model.onnx"),
            max_concurrent_sessions: 20,
            max_memory_mb: 4096,
            chunk_size: 300,
            max_chunk_processing_time_s: 2,
            min_parallel_efficiency: 0.8,
        }
    }

    /// Validate configuration contracts
    pub fn validate(&self) -> Result<(), String> {
        if self.max_concurrent_sessions == 0 || self.max_concurrent_sessions > 100 {
            return Err(format!(
                "Invalid max_concurrent_sessions: {} (must be 1-100)",
                self.max_concurrent_sessions
            ));
        }

        if self.chunk_size == 0 || self.chunk_size > 1000 {
            return Err(format!(
                "Invalid chunk_size: {} (must be 1-1000)",
                self.chunk_size
            ));
        }

        if self.min_parallel_efficiency < 0.0 || self.min_parallel_efficiency > 1.0 {
            return Err(format!(
                "Invalid min_parallel_efficiency: {} (must be 0.0-1.0)",
                self.min_parallel_efficiency
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation_contracts() {
        // TDD-First: GREEN phase - valid configs
        let config = SystemConfig::production();
        assert!(config.validate().is_ok());

        let test_config = SystemConfig::test_config();
        assert!(test_config.validate().is_ok());
    }

    #[test]
    fn test_config_contract_violations() {
        // TDD-First: RED phase - invalid configs
        let mut config = SystemConfig::production();
        
        config.max_concurrent_sessions = 0;
        assert!(config.validate().is_err());
        
        config.max_concurrent_sessions = 150;
        assert!(config.validate().is_err());
    }
}
