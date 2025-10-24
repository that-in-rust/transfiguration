//! SmolLM2 Tokenizer Implementation
//!
//! TDD-First: Tokenizer with RAII resource management and dependency injection

use crate::errors::{ProcessingError, Result};
use std::path::Path;
use tokenizers::Tokenizer;

/// SmolLM2 Tokenizer with proper resource management (Principle 4: RAII)
pub struct SmolLM2Tokenizer {
    tokenizer: Tokenizer,
    eos_token_id: u32,
    pad_token_id: u32,
}

impl SmolLM2Tokenizer {
    /// Create SmolLM2 tokenizer from file
    ///
    /// # Preconditions
    /// - tokenizer.json file exists at specified path
    /// - File contains valid SmolLM2 tokenizer configuration
    ///
    /// # Postconditions
    /// - Returns Ok(SmolLM2Tokenizer) if tokenizer loads successfully
    /// - Tokenizer ready for encoding/decoding operations
    ///
    /// # Error Conditions
    /// - ProcessingError::TokenizerLoadFailed if file doesn't exist
    /// - ProcessingError::TokenizerLoadFailed if file is invalid JSON
    pub fn from_file(tokenizer_path: &str) -> Result<Self> {
        let path = Path::new(tokenizer_path);

        if !path.exists() {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: format!("Tokenizer file not found: {}", tokenizer_path).into(),
            });
        }

        // Load tokenizer with proper error handling
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| ProcessingError::TokenizerLoadFailed {
                source: format!("Failed to load tokenizer from {}: {}", tokenizer_path, e).into(),
            })?;

        // Configure tokenizer for generation (no padding)
        let tokenizer = tokenizer.with_padding(None);

        // Extract special token IDs
        let eos_token_id = tokenizer.get_vocab().get("</s>")
            .copied()
            .unwrap_or(2) as u32; // Default EOS token ID

        let pad_token_id = tokenizer.get_vocab().get("<pad>")
            .copied()
            .unwrap_or(0) as u32; // Default PAD token ID

        Ok(Self {
            tokenizer,
            eos_token_id,
            pad_token_id,
        })
    }

    /// Create SmolLM2 tokenizer from default model path
    pub fn from_default_model() -> Result<Self> {
        Self::from_file("models/smolLM2-onnx/tokenizer.json")
    }

    /// Encode text with prompt prefix for summarization
    ///
    /// # Preconditions
    /// - Text is valid UTF-8 string
    /// - Combined prompt + text length < model's maximum sequence length
    ///
    /// # Postconditions
    /// - Returns Ok(Vec<u64>) with token IDs
    /// - Tokens include prompt prefix "Summarize this code in one line: "
    ///
    /// # Error Conditions
    /// - ProcessingError::TokenizationFailed if encoding fails
    pub fn encode_for_summarization(&self, text: &str) -> Result<Vec<u64>> {
        let prompt = format!("Summarize this code in one line: {}", text);

        let encoding = self.tokenizer.encode(&prompt, false)
            .map_err(|e| ProcessingError::TokenizationFailed {
                text: prompt.clone(),
                source: format!("Failed to encode text: {}", e).into(),
            })?;

        Ok(encoding.get_ids().to_vec())
    }

    /// Decode tokens back to text
    ///
    /// # Preconditions
    /// - Token IDs are valid for this tokenizer's vocabulary
    ///
    /// # Postconditions
    /// - Returns Ok(String) with decoded text
    /// - Removes prompt prefix and cleans up output
    ///
    /// # Error Conditions
    /// - ProcessingError::DetokenizationFailed if decoding fails
    pub fn decode_summary(&self, token_ids: &[u32]) -> Result<String> {
        // Convert u32 to u64 for tokenizer
        let ids: Vec<u64> = token_ids.iter().map(|&id| id as u64).collect();

        let decoded = self.tokenizer.decode(&ids)?
            .map_err(|e| ProcessingError::DetokenizationFailed {
                token_ids: token_ids.to_vec(),
                source: format!("Failed to decode tokens: {}", e).into(),
            })?;

        // Clean up the decoded text
        let cleaned = decoded
            .trim()
            .strip_prefix("Summarize this code in one line:")
            .unwrap_or(&decoded)
            .trim()
            .lines()
            .next() // Take only first line for 1-line summary
            .unwrap_or("")
            .trim()
            .to_string();

        Ok(cleaned)
    }

    /// Get EOS token ID for generation stopping
    pub fn eos_token_id(&self) -> u32 {
        self.eos_token_id
    }

    /// Get PAD token ID for padding
    pub fn pad_token_id(&self) -> u32 {
        self.pad_token_id
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.tokenizer.get_vocab_size()
    }
}

/// Trait-based dependency injection for testability (Principle 3)
pub trait SmolTokenizerProvider: Send + Sync {
    fn encode_for_summarization(&self, text: &str) -> Result<Vec<u64>>;
    fn decode_summary(&self, token_ids: &[u32]) -> Result<String>;
    fn eos_token_id(&self) -> u32;
    fn pad_token_id(&self) -> u32;
}

impl SmolTokenizerProvider for SmolLM2Tokenizer {
    fn encode_for_summarization(&self, text: &str) -> Result<Vec<u64>> {
        self.encode_for_summarization(text)
    }

    fn decode_summary(&self, token_ids: &[u32]) -> Result<String> {
        self.decode_summary(token_ids)
    }

    fn eos_token_id(&self) -> u32 {
        self.eos_token_id()
    }

    fn pad_token_id(&self) -> u32 {
        self.pad_token_id()
    }
}

/// Mock tokenizer for testing
#[cfg(test)]
pub struct MockSmolTokenizer {
    pub mock_responses: std::sync::Mutex<Vec<String>>,
}

#[cfg(test)]
impl MockSmolTokenizer {
    pub fn new() -> Self {
        Self {
            mock_responses: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn add_mock_response(&mut self, response: String) {
        self.mock_responses.lock().unwrap().push(response);
    }
}

#[cfg(test)]
impl SmolTokenizerProvider for MockSmolTokenizer {
    fn encode_for_summarization(&self, _text: &str) -> Result<Vec<u64>> {
        // Return simple token IDs for testing
        Ok(vec![1, 2, 3, 4, 5])
    }

    fn decode_summary(&self, _token_ids: &[u32]) -> Result<String> {
        let mut responses = self.mock_responses.lock().unwrap();
        if !responses.is_empty() {
            Ok(responses.remove(0))
        } else {
            Ok("Mock summary for testing".to_string())
        }
    }

    fn eos_token_id(&self) -> u32 {
        2
    }

    fn pad_token_id(&self) -> u32 {
        0
    }
}

/// Factory function for creating tokenizers (dependency injection)
pub fn create_smollm_tokenizer() -> Result<impl SmolTokenizerProvider> {
    SmolLM2Tokenizer::from_default_model()
}

/// Factory function for creating mock tokenizer (for testing)
#[cfg(test)]
pub fn create_mock_tokenizer() -> impl SmolTokenizerProvider {
    MockSmolTokenizer::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_tokenizer() {
        let mut mock = MockSmolTokenizer::new();
        mock.add_mock_response("Test summary 1".to_string());
        mock.add_mock_response("Test summary 2".to_string());

        let result1 = mock.decode_summary(&[1, 2, 3]);
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), "Test summary 1");

        let result2 = mock.decode_summary(&[4, 5, 6]);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), "Test summary 2");
    }

    #[test]
    fn test_mock_tokenizer_encoding() {
        let mock = MockSmolTokenizer::new();
        let result = mock.encode_for_summarization("fn test() {}");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_tokenizer_special_tokens() {
        let mock = MockSmolTokenizer::new();
        assert_eq!(mock.eos_token_id(), 2);
        assert_eq!(mock.pad_token_id(), 0);
    }

    #[test]
    fn test_real_tokenizer_error_handling() {
        // Test with non-existent file
        let result = SmolLM2Tokenizer::from_file("non_existent_file.json");
        assert!(result.is_err());
        match result.unwrap_err() {
            ProcessingError::TokenizerLoadFailed { .. } => {
                // Expected error type
            }
            _ => panic!("Expected TokenizerLoadFailed error"),
        }
    }
}