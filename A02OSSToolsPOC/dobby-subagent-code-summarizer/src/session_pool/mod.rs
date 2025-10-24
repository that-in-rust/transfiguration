//! ONNX Session Pool with TDD-First contracts (STUB)
//!
//! NOTE: This is a STUB implementation for MVP. Real implementation will:
//! - Use ort::Session with session pooling
//! - Implement RAII with Drop trait
//! - Support 20 concurrent sessions
//! - Track memory usage per session


/// Session pool configuration
#[derive(Debug, Clone)]
pub struct SessionPoolConfig {
    pub max_concurrent_sessions: usize,
    pub max_memory_per_session_mb: u32,
}

impl Default for SessionPoolConfig {
    fn default() -> Self {
        Self {
            max_concurrent_sessions: 20,
            max_memory_per_session_mb: 200,
        }
    }
}

/// Simplified session pool for MVP (STUB)
pub struct SessionPool {
    #[allow(dead_code)]
    config: SessionPoolConfig,
}

impl SessionPool {
    pub fn new(config: SessionPoolConfig) -> Self {
        println!("🔧 Creating ONNX session pool (STUB)");
        Self { config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_pool_creation() {
        let config = SessionPoolConfig::default();
        let _pool = SessionPool::new(config);
        // STUB: Just verify creation works
    }
}
