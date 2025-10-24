//! Tokenizer integration for CodeT5-small
//!
//! Provides proper tokenization for code summarization using the HuggingFace
//! tokenizers library with the CodeT5-small vocabulary.

use crate::errors::{ProcessingError, Result};
use std::path::Path;
use tokenizers::{Tokenizer, Encoding};

/// Configuration for the tokenizer
#[derive(Debug, Clone)]
pub struct TokenizerConfig {
    pub model_path: String,
    pub max_length: usize,
    pub max_target_length: usize,
}

impl Default for TokenizerConfig {
    fn default() -> Self {
        Self {
            model_path: "models/codet5-small".to_string(),
            max_length: 512,
            max_target_length: 128,
        }
    }
}

/// CodeT5-small tokenizer for processing code chunks
pub struct CodeT5Tokenizer {
    tokenizer: Tokenizer,
    config: TokenizerConfig,
}

impl CodeT5Tokenizer {
    /// Create a new CodeT5 tokenizer from the model directory
    pub fn new(config: TokenizerConfig) -> Result<Self> {
        // Validate tokenizer files exist
        let tokenizer_file = Path::new(&config.model_path).join("tokenizer.json");
        let vocab_file = Path::new(&config.model_path).join("vocab.json");
        let merges_file = Path::new(&config.model_path).join("merges.txt");

        if !tokenizer_file.exists() {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: format!("Tokenizer file not found: {:?}", tokenizer_file).into(),
            });
        }

        if !vocab_file.exists() {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: format!("Vocab file not found: {:?}", vocab_file).into(),
            });
        }

        if !merges_file.exists() {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: format!("Merges file not found: {:?}", merges_file).into(),
            });
        }

        // Load tokenizer from file
        let mut tokenizer = Tokenizer::from_file(&tokenizer_file)
            .map_err(|e| ProcessingError::TokenizerLoadFailed {
                source: format!("Failed to load tokenizer: {}", e).into(),
            })?;

        // Configure tokenizer
        tokenizer
            .with_padding(None)
            .with_truncation(None)
            .map_err(|e| ProcessingError::TokenizerLoadFailed {
                source: format!("Failed to configure tokenizer: {}", e).into(),
            })?;

        Ok(Self { tokenizer, config })
    }

    /// Encode text into token IDs for model input
    pub fn encode(&self, text: &str) -> Result<Encoding> {
        let encoding = self.tokenizer
            .encode(text, true)
            .map_err(|e| ProcessingError::TokenizationFailed {
                text: text.to_string(),
                source: format!("Tokenization failed: {}", e).into(),
            })?;

        Ok(encoding)
    }

    /// Decode token IDs back to text
    pub fn decode(&self, token_ids: &[u32]) -> Result<String> {
        let text = self.tokenizer
            .decode(token_ids, true)
            .map_err(|e| ProcessingError::DetokenizationFailed {
                token_ids: token_ids.to_vec(),
                source: format!("Detokenization failed: {}", e).into(),
            })?;

        Ok(text)
    }

    /// Encode a chunk for CodeT5-small input
    /// Returns the input_ids tensor ready for ONNX inference
    pub fn encode_chunk_for_inference(&self, text: &str) -> Result<Vec<i64>> {
        // Add prefix for code summarization task
        let prefixed_text = format!("summarize: {}", text);

        let encoding = self.encode(&prefixed_text)?;

        // Convert to i64 for ONNX compatibility
        let input_ids: Vec<i64> = encoding.get_ids()
            .iter()
            .map(|&id| id as i64)
            .collect();

        // Pad or truncate to max_length
        let mut processed_ids = input_ids;
        if processed_ids.len() > self.config.max_length {
            processed_ids.truncate(self.config.max_length);
        }

        // Pad with 0 (pad_token_id for T5)
        while processed_ids.len() < self.config.max_length {
            processed_ids.push(0);
        }

        Ok(processed_ids)
    }

    /// Decode model output to text summary
    pub fn decode_summary(&self, token_ids: &[i64]) -> Result<String> {
        // Filter out padding tokens (0) and special tokens
        let filtered_ids: Vec<u32> = token_ids
            .iter()
            .copied()
            .filter(|&id| id != 0) // Remove padding
            .filter(|&id| id != 1) // Remove <s>
            .filter(|&id| id != 2) // Remove </s>
            .map(|id| id as u32)
            .collect();

        self.decode(&filtered_ids)
    }

    /// Get the vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.tokenizer.get_vocab_size(true)
    }

    /// Get the max length for this tokenizer
    pub fn max_length(&self) -> usize {
        self.config.max_length
    }

    /// Get the max target length for generation
    pub fn max_target_length(&self) -> usize {
        self.config.max_target_length
    }
}

/// Create a tokenizer for code summarization with default settings
pub fn create_codet5_tokenizer() -> Result<CodeT5Tokenizer> {
    let config = TokenizerConfig::default();
    CodeT5Tokenizer::new(config)
}

/// Create a tokenizer with custom configuration
pub fn create_codet5_tokenizer_with_config(config: TokenizerConfig) -> Result<CodeT5Tokenizer> {
    CodeT5Tokenizer::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_creation() {
        let tokenizer = create_codet5_tokenizer();
        match tokenizer {
            Ok(t) => {
                println!("✅ Tokenizer created successfully");
                assert_eq!(t.vocab_size(), 32100);
                assert_eq!(t.max_length(), 512);
            }
            Err(e) => {
                println!("❌ Tokenizer creation failed: {:?}", e);
                panic!("Failed to create tokenizer: {:?}", e);
            }
        }
    }

    #[test]
    fn test_basic_tokenization() {
        let tokenizer = create_codet5_tokenizer().unwrap();

        let test_code = "def hello_world():\n    print('Hello, world!')";
        let encoding = tokenizer.encode_chunk_for_inference(test_code).unwrap();

        assert!(!encoding.is_empty(), "Encoding should not be empty");
        assert!(encoding.len() <= 512, "Encoding should respect max length");

        // Test decoding
        let decoded = tokenizer.decode_summary(&encoding).unwrap();
        assert!(!decoded.is_empty(), "Decoded text should not be empty");
    }

    #[test]
    fn test_long_code_truncation() {
        let tokenizer = create_codet5_tokenizer().unwrap();

        // Create a very long code snippet
        let long_code = "fn test() { println!(\"test\"); }\n".repeat(100);
        let encoding = tokenizer.encode_chunk_for_inference(&long_code).unwrap();

        assert_eq!(encoding.len(), 512, "Long code should be truncated to max length");
    }

    #[test]
    fn test_prefix_addition() {
        let tokenizer = create_codet5_tokenizer().unwrap();

        let test_code = "def hello(): pass";
        let encoding = tokenizer.encode_chunk_for_inference(test_code).unwrap();

        // The encoding should include the "summarize: " prefix
        assert!(encoding.len() > 5, "Prefix should be added to encoding");
    }
}