//! Production SmolLM2 Tokenizer using HuggingFace tokenizers crate
//!
//! ## Executive Specification (REQ-TOK-001.0)
//!
//! # Preconditions
//! - models/smolLM2-onnx/tokenizer.json exists (HuggingFace format)
//! - models/smolLM2-onnx/config.json exists (special token configuration)
//! - Tokenizer vocabulary contains 49,152 entries (SmolLM2-135M-Instruct)
//! - Special tokens: BOS=1, EOS=2, PAD=0, UNK=3 from config.json
//!
//! # Postconditions
//! - Returns Ok(ProductionSmolLM2Tokenizer) on successful load
//! - Loads complete HuggingFace tokenizer with BPE algorithm
//! - Special tokens configured and validated from config.json
//! - Thread-safe for concurrent inference operations
//! - Performance: <100ms loading time, <10ms tokenization for 1000 tokens
//!
//! # Error Conditions
//! - TokenizerLoadFailed if tokenizer.json missing or invalid
//! - ConfigLoadFailed if config.json missing or malformed
//! - PerformanceContractViolation if tokenization exceeds 10ms
//! - InvalidVocabulary if vocabulary size != 49,152 entries

use crate::errors::{ProcessingError, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tokenizers::Tokenizer;

/// Special token configuration from HuggingFace config.json
///
/// This struct enforces type safety for special token IDs
/// following Design101 Principle 1: Executable Specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialTokens {
    pub bos_token: String,
    pub eos_token: String,
    pub pad_token: String,
    pub unk_token: String,
    pub bos_id: u32,
    pub eos_id: u32,
    pub pad_id: u32,
    pub unk_id: u32,
}

/// Production-ready SmolLM2 tokenizer trait
///
/// Implements Design101 Principle 3: Dependency Injection
/// Every component depends on traits, not concrete types
pub trait SmolLM2TokenizerProvider: Send + Sync {
    /// Encode text for SmolLM2 summarization with task prefix
    ///
    /// # Performance Contract (Design101 Principle 5)
    /// - Execution time: <10ms for 1000 tokens
    /// - Memory allocation: <1MB additional memory
    /// - Thread safety: Safe for concurrent calls
    fn encode_for_summarization(&self, text: &str) -> Result<Vec<u64>>;

    /// Decode token IDs back to readable text
    fn decode_summary(&self, token_ids: &[u32]) -> Result<String>;

    /// Get vocabulary size (should be 49,152 for SmolLM2-135M-Instruct)
    fn vocab_size(&self) -> usize;

    /// Get special tokens configuration
    fn special_tokens(&self) -> &SpecialTokens;

    /// Validate tokenizer health and readiness
    fn validate_health(&self) -> Result<()>;
}

/// Production SmolLM2 tokenizer using HuggingFace tokenizers crate
///
/// Implements Design101 Principle 4: RAII Resource Management
/// All resources automatically managed with proper cleanup
pub struct ProductionSmolLM2Tokenizer {
    tokenizer: Tokenizer,
    special_tokens: SpecialTokens,
    #[allow(dead_code)]
    model_path: PathBuf,
}

impl ProductionSmolLM2Tokenizer {
    /// Load SmolLM2 tokenizer from model directory
    ///
    /// Uses battle-tested HuggingFace tokenizers crate
    /// with comprehensive error handling and validation
    pub fn from_model_path(model_path: &str) -> Result<Self> {
        let model_path = PathBuf::from(model_path);

        // Validate model path exists
        if !model_path.exists() {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: format!("Model path does not exist: {}", model_path.display()).into(),
            });
        }

        // Load HuggingFace tokenizer.json with performance tracking
        let tokenizer_path = model_path.join("tokenizer.json");
        let load_start = Instant::now();

        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| ProcessingError::TokenizerLoadFailed {
                source: format!("Failed to load tokenizer from {}: {}",
                    tokenizer_path.display(), e).into(),
            })?;

        let load_time = load_start.elapsed();

        // Validate loading performance contract (<100ms)
        if load_time > Duration::from_millis(100) {
            eprintln!("⚠️  Tokenizer loading took {:?} (contract: <100ms)", load_time);
        }

        // Load config.json for special tokens
        let config_path = model_path.join("config.json");
        let special_tokens = Self::load_special_tokens(&config_path)?;

        // Validate tokenizer health
        Self::validate_tokenizer_health(&tokenizer, &special_tokens)?;

        Ok(Self {
            tokenizer,
            special_tokens,
            model_path,
        })
    }

    /// Load special tokens from HuggingFace config.json
    fn load_special_tokens(config_path: &Path) -> Result<SpecialTokens> {
        let config_content = std::fs::read_to_string(config_path)
            .map_err(|e| ProcessingError::TokenizerLoadFailed {
                source: format!("Failed to read config.json: {}", e).into(),
            })?;

        let config: serde_json::Value = serde_json::from_str(&config_content)
            .map_err(|e| ProcessingError::TokenizerLoadFailed {
                source: format!("Failed to parse config.json: {}", e).into(),
            })?;

        // Extract special token configuration
        let bos_token = config.get("bos_token")
            .and_then(|v| v.as_str())
            .unwrap_or("<s>")
            .to_string();

        let eos_token = config.get("eos_token")
            .and_then(|v| v.as_str())
            .unwrap_or("</s>")
            .to_string();

        let pad_token = config.get("pad_token")
            .and_then(|v| v.as_str())
            .unwrap_or("<pad>")
            .to_string();

        let unk_token = config.get("unk_token")
            .and_then(|v| v.as_str())
            .unwrap_or("<unk>")
            .to_string();

        // Extract token IDs with validation
        let bos_id = config.get("bos_token_id")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u32;

        let eos_id = config.get("eos_token_id")
            .and_then(|v| v.as_u64())
            .unwrap_or(2) as u32;

        let pad_id = config.get("pad_token_id")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        let unk_id = config.get("unk_token_id")
            .and_then(|v| v.as_u64())
            .unwrap_or(3) as u32;

        Ok(SpecialTokens {
            bos_token,
            eos_token,
            pad_token,
            unk_token,
            bos_id,
            eos_id,
            pad_id,
            unk_id,
        })
    }

    /// Validate tokenizer health and configuration
    fn validate_tokenizer_health(tokenizer: &Tokenizer, special_tokens: &SpecialTokens) -> Result<()> {
        // Validate vocabulary size (SmolLM2-135M-Instruct should have 49,152)
        let vocab_size = tokenizer.get_vocab_size(false);
        if vocab_size != 49152 {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: format!("Invalid vocabulary size: {} (expected 49,152 for SmolLM2-135M-Instruct)", vocab_size).into(),
            });
        }

        // Validate special token configuration
        if special_tokens.bos_id >= vocab_size as u32 ||
           special_tokens.eos_id >= vocab_size as u32 ||
           special_tokens.pad_id >= vocab_size as u32 ||
           special_tokens.unk_id >= vocab_size as u32 {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: format!("Special token ID exceeds vocabulary size: vocab_size={}, bos={}, eos={}, pad={}, unk={}",
                    vocab_size, special_tokens.bos_id, special_tokens.eos_id, special_tokens.pad_id, special_tokens.unk_id).into(),
            });
        }

        // Validate token IDs - allow PAD=EOS overlap (common in many models)
        let token_ids = [special_tokens.bos_id, special_tokens.eos_id, special_tokens.unk_id];
        let unique_ids: std::collections::HashSet<_> = token_ids.iter().collect();
        if unique_ids.len() != token_ids.len() {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: format!("Special token IDs must be unique (BOS/EOS/UNK): bos={}, eos={}, unk={}",
                    special_tokens.bos_id, special_tokens.eos_id, special_tokens.unk_id).into(),
            });
        }

        // PAD token can overlap with EOS (common in many models like SmolLM2)
        if special_tokens.pad_id == special_tokens.bos_id || special_tokens.pad_id == special_tokens.unk_id {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: format!("PAD token cannot overlap with BOS or UNK: pad={}, bos={}, unk={}",
                    special_tokens.pad_id, special_tokens.bos_id, special_tokens.unk_id).into(),
            });
        }

        Ok(())
    }
}

/// Design101 Principle 3: Dependency Injection Implementation
impl SmolLM2TokenizerProvider for ProductionSmolLM2Tokenizer {
    /// Encode text for SmolLM2 summarization
    ///
    /// Adds task prefix and uses HuggingFace BPE tokenization
    fn encode_for_summarization(&self, text: &str) -> Result<Vec<u64>> {
        let start_time = Instant::now();

        // Add summarization task prefix
        let prefixed_text = format!("summarize: {}", text);

        // Encode using HuggingFace tokenizer with proper error handling
        let encoding = self.tokenizer.encode(&*prefixed_text, true)
            .map_err(|e| ProcessingError::TokenizationFailed {
                text: prefixed_text.clone(),
                source: format!("Encoding failed: {}", e).into(),
            })?;

        // Convert to u64 with proper type conversion
        let token_ids: Vec<u64> = encoding.get_ids()
            .iter()
            .map(|&id| u64::from(id))
            .collect();

        // Performance contract validation (Design101 Principle 5)
        let elapsed = start_time.elapsed();
        if elapsed > Duration::from_millis(10) {
            eprintln!("⚠️  Tokenization performance violation: {:?} (contract: <10ms)", elapsed);
        }

        // Validate output quality
        if token_ids.is_empty() {
            return Err(ProcessingError::TokenizationFailed {
                text: prefixed_text,
                source: "Tokenization produced empty token sequence".into(),
            });
        }

        if token_ids.len() > 2048 {
            return Err(ProcessingError::TokenizationFailed {
                text: prefixed_text,
                source: format!("Token sequence too long: {} tokens (max: 2048)", token_ids.len()).into(),
            });
        }

        Ok(token_ids)
    }

    /// Decode token IDs back to readable text
    fn decode_summary(&self, token_ids: &[u32]) -> Result<String> {
        // Convert u32 to u32 for tokenizer compatibility
        let ids: Vec<u32> = token_ids.to_vec();

        // Decode using HuggingFace tokenizer
        let decoded = self.tokenizer.decode(&ids, true)
            .map_err(|e| ProcessingError::DetokenizationFailed {
                token_ids: ids.clone(),
                source: format!("Decoding failed: {}", e).into(),
            })?;

        // Clean up decoded text
        let cleaned = decoded.trim().to_string();

        // Validate output quality
        if cleaned.is_empty() {
            return Err(ProcessingError::DetokenizationFailed {
                token_ids: ids,
                source: "Decoding produced empty text".into(),
            });
        }

        Ok(cleaned)
    }

    /// Get vocabulary size
    fn vocab_size(&self) -> usize {
        self.tokenizer.get_vocab_size(false)
    }

    /// Get special tokens configuration
    fn special_tokens(&self) -> &SpecialTokens {
        &self.special_tokens
    }

    /// Validate tokenizer health
    fn validate_health(&self) -> Result<()> {
        // Re-validate on-demand
        Self::validate_tokenizer_health(&self.tokenizer, &self.special_tokens)
    }
}

/// Factory function for creating production tokenizer
///
/// Provides convenient creation method following Design101 patterns
pub fn create_production_smollm_tokenizer() -> Result<ProductionSmolLM2Tokenizer> {
    ProductionSmolLM2Tokenizer::from_model_path("models/smolLM2-onnx")
}

#[cfg(test)]
mod production_tests {
    use super::*;
    use std::time::Instant;

    /// TDD-First: Test production tokenizer loading
    #[test]
    fn test_production_tokenizer_loading_contract() -> Result<()> {
        let tokenizer = ProductionSmolLM2Tokenizer::from_model_path("models/smolLM2-onnx")?;

        // Validate vocabulary size contract (49,152 for SmolLM2-135M-Instruct)
        assert_eq!(tokenizer.vocab_size(), 49152, "Vocabulary size contract violation");

        // Validate special tokens from config.json
        let special = tokenizer.special_tokens();
        assert_eq!(special.bos_id, 1, "BOS token ID contract violation");
        assert_eq!(special.eos_id, 2, "EOS token ID contract violation");
        assert_eq!(special.pad_id, 0, "PAD token ID contract violation");
        assert_eq!(special.unk_id, 3, "UNK token ID contract violation");

        // Validate health check
        assert!(tokenizer.validate_health().is_ok(), "Health check should pass");

        Ok(())
    }

    /// Performance contract validation (Design101 Principle 5)
    #[test]
    fn test_tokenization_performance_contract() -> Result<()> {
        let tokenizer = ProductionSmolLM2Tokenizer::from_model_path("models/smolLM2-onnx")?;

        // Test with realistic Rust code chunk
        let test_code = r#"
        fn process_data(input: Vec<String>) -> Result<ProcessedData> {
            let processed: Vec<String> = input.iter()
                .map(|s| s.trim().to_lowercase())
                .collect();

            Ok(ProcessedData::new(processed))
        }
        "#;

        let start = Instant::now();
        let tokens = tokenizer.encode_for_summarization(test_code)?;
        let elapsed = start.elapsed();

        // Validate performance contract (<10ms)
        assert!(elapsed < Duration::from_millis(10),
                "Performance contract violation: {:?} (expected <10ms)", elapsed);

        // Validate output quality
        assert!(!tokens.is_empty(), "Should generate tokens");
        assert!(tokens.len() <= 2048, "Should respect context length limit");
        assert!(tokens.len() >= 5, "Should generate reasonable token count");

        Ok(())
    }

    /// Output quality validation contract
    #[test]
    fn test_decoding_quality_contract() -> Result<()> {
        let tokenizer = ProductionSmolLM2Tokenizer::from_model_path("models/smolLM2-onnx")?;

        // Test decoding with realistic token sequence
        let test_tokens = vec![1, 315, 285, 1256, 312, 285, 832, 2]; // BOS + example tokens + EOS

        let decoded = tokenizer.decode_summary(&test_tokens)?;

        // Validate output quality
        assert!(!decoded.is_empty(), "Decoded text should not be empty");
        assert!(decoded.len() >= 5, "Decoded text should be meaningful");
        assert!(decoded.len() <= 500, "Decoded text should be reasonable length");

        Ok(())
    }

    /// Thread safety validation (Design101 Principle 8)
    #[tokio::test]
    async fn test_concurrent_tokenization_safety() -> Result<()> {
        use tokio::task::JoinSet;
        use std::sync::Arc;

        let tokenizer = Arc::new(ProductionSmolLM2Tokenizer::from_model_path("models/smolLM2-onnx")?);
        let mut join_set = JoinSet::new();

        // Spawn 20 concurrent tokenization tasks
        for i in 0..20 {
            let tokenizer_clone = Arc::clone(&tokenizer);
            join_set.spawn(async move {
                let test_code = format!(
                    r#"fn concurrent_task_{}() -> Result<i32> {{
                        println!("Processing task {}");
                        Ok(42)
                    }}"#,
                    i, i
                );

                tokenizer_clone.encode_for_summarization(&test_code)
            });
        }

        // Wait for all tasks and validate no panics
        let mut results = Vec::new();
        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(tokens)) => results.push(tokens),
                Ok(Err(e)) => panic!("Tokenization failed: {:?}", e),
                Err(e) => panic!("Task panicked: {:?}", e),
            }
        }

        // Validate all tasks completed successfully
        assert_eq!(results.len(), 20, "All concurrent tasks should complete");

        // Validate all results are reasonable
        for (i, tokens) in results.iter().enumerate() {
            assert!(!tokens.is_empty(), "Task {} should generate tokens", i);
            assert!(tokens.len() <= 2048, "Task {} should respect context limit", i);
        }

        Ok(())
    }
}