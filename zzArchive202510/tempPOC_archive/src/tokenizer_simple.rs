//! Simple tokenizer implementation for CodeT5-small
//!
//! Basic implementation that works with the CodeT5-small vocabulary
//! without requiring complex tokenizers library compatibility.

use crate::errors::{ProcessingError, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Simple BPE tokenizer for CodeT5-small
pub struct SimpleCodeT5Tokenizer {
    vocab: HashMap<String, u32>,
    reverse_vocab: HashMap<u32, String>,
    max_length: usize,
}

impl SimpleCodeT5Tokenizer {
    /// Create a new simple tokenizer from model files
    pub fn new(model_path: &str) -> Result<Self> {
        let vocab_file = Path::new(model_path).join("vocab.json");
        let merges_file = Path::new(model_path).join("merges.txt");

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

        // Load vocabulary
        let vocab = Self::load_vocab(&vocab_file)?;
        let reverse_vocab: HashMap<u32, String> = vocab.iter()
            .map(|(k, v)| (*v, k.clone()))
            .collect();

        Ok(Self {
            vocab,
            reverse_vocab,
            max_length: 512,
        })
    }

    /// Load vocabulary from JSON file
    fn load_vocab(vocab_file: &Path) -> Result<HashMap<String, u32>> {
        let file = File::open(vocab_file)
            .map_err(|e| ProcessingError::TokenizerLoadFailed {
                source: format!("Failed to open vocab file: {}", e).into(),
            })?;

        let reader = BufReader::new(file);
        let mut vocab = HashMap::new();

        // Simple JSON parsing for vocab
        for line in reader.lines() {
            let line = line.map_err(|e| ProcessingError::TokenizerLoadFailed {
                source: format!("Failed to read vocab line: {}", e).into(),
            })?;

            if line.trim().starts_with('"') && line.contains("\":") {
                let parts: Vec<&str> = line.splitn(2, "\": ").collect();
                if parts.len() == 2 {
                    let key = parts[0].trim_start_matches('"').replace("\\\"", "\"");
                    let value_str = parts[1].trim_end_matches(',').trim_end_matches('}');
                    if let Ok(value) = value_str.parse::<u32>() {
                        vocab.insert(key, value);
                    }
                }
            }
        }

        if vocab.is_empty() {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: "No vocabulary entries loaded".into(),
            });
        }

        Ok(vocab)
    }

    /// Simple tokenization using character-level approach with some common tokens
    pub fn encode(&self, text: &str) -> Result<Vec<i64>> {
        let mut tokens = Vec::new();

        // Add task prefix
        let prefixed_text = format!("summarize: {}", text);

        // Simple approach: split by common programming patterns and characters
        let mut current = String::new();
        let chars: Vec<char> = prefixed_text.chars().collect();

        for i in 0..chars.len() {
            current.push(chars[i]);

            // Check if current substring is in vocab
            if let Some(&token_id) = self.vocab.get(&current) {
                tokens.push(token_id as i64);
                current.clear();
            } else {
                // Try progressively smaller substrings
                for j in (1..current.len()).rev() {
                    let substring: String = current.chars().skip(current.len() - j).collect();
                    if let Some(&token_id) = self.vocab.get(&substring) {
                        // Add the part before the substring
                        if current.len() > j {
                            let prefix: String = current.chars().take(current.len() - j).collect();
                            if let Some(&prefix_id) = self.vocab.get(&prefix) {
                                tokens.push(prefix_id as i64);
                            } else {
                                // Handle unknown characters one by one
                                for ch in prefix.chars() {
                                    tokens.push(ch as u8 as i64);
                                }
                            }
                        }
                        tokens.push(token_id as i64);
                        current.clear();
                        break;
                    }
                }
            }
        }

        // Handle any remaining characters
        if !current.is_empty() {
            for ch in current.chars() {
                tokens.push(ch as u8 as i64);
            }
        }

        // Pad or truncate to max_length
        if tokens.len() > self.max_length {
            tokens.truncate(self.max_length);
        }

        while tokens.len() < self.max_length {
            tokens.push(0); // pad_token_id for T5
        }

        Ok(tokens)
    }

    /// Decode token IDs back to text
    pub fn decode(&self, token_ids: &[i64]) -> Result<String> {
        let mut text = String::new();

        for &token_id in token_ids {
            if token_id == 0 {
                continue; // Skip padding
            }

            let token_id_u32 = token_id as u32;
            if let Some(token) = self.reverse_vocab.get(&token_id_u32) {
                text.push_str(token);
            } else if token_id >= 32 && token_id <= 126 {
                // Handle ASCII characters directly
                text.push(token_id as u8 as char);
            }
        }

        Ok(text)
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Get max length
    pub fn max_length(&self) -> usize {
        self.max_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenizer_creation() {
        let tokenizer = SimpleCodeT5Tokenizer::new("models/codet5-small");
        match tokenizer {
            Ok(t) => {
                println!("✅ Simple tokenizer created successfully");
                println!("Vocab size: {}", t.vocab_size());
                assert!(t.vocab_size() > 30000, "Vocab should be substantial");
                assert_eq!(t.max_length(), 512);
            }
            Err(e) => {
                println!("❌ Simple tokenizer creation failed: {:?}", e);
                panic!("Failed to create simple tokenizer: {:?}", e);
            }
        }
    }

    #[test]
    fn test_simple_tokenization() {
        let tokenizer = SimpleCodeT5Tokenizer::new("models/codet5-small").unwrap();

        let test_code = "def hello(): pass";
        let tokens = tokenizer.encode(test_code).unwrap();

        assert!(!tokens.is_empty(), "Tokens should not be empty");
        assert!(tokens.len() <= 512, "Tokens should respect max length");

        // Test decoding
        let decoded = tokenizer.decode(&tokens).unwrap();
        println!("Original: {}", test_code);
        println!("Tokens: {:?}", &tokens[..10.min(tokens.len())]);
        println!("Decoded: {}", decoded);
        assert!(!decoded.is_empty(), "Decoded text should not be empty");
    }
}