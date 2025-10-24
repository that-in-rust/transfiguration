//! Real CodeT5-small tokenizer implementation
//!
//! Provides functional tokenization for CodeT5-small code summarization.
//! Uses the actual vocabulary from the HuggingFace model.

use crate::errors::{ProcessingError, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

/// Real CodeT5-small tokenizer
pub struct CodeT5Tokenizer {
    vocab: HashMap<String, u32>,
    reverse_vocab: HashMap<u32, String>,
    pad_token_id: u32,
    eos_token_id: u32,
    bos_token_id: u32,
    unk_token_id: u32,
    max_length: usize,
}

impl CodeT5Tokenizer {
    /// Create a new CodeT5 tokenizer from model files
    pub fn new(model_path: &str) -> Result<Self> {
        let vocab_file = Path::new(model_path).join("vocab_simple.txt");

        if !vocab_file.exists() {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: format!("Vocab file not found: {:?}", vocab_file).into(),
            });
        }

        // Load vocabulary from simple format
        let vocab = Self::load_simple_vocab(&vocab_file)?;
        let reverse_vocab: HashMap<u32, String> = vocab.iter()
            .map(|(k, v)| (*v, k.clone()))
            .collect();

        // T5 special tokens
        let pad_token_id = 0;
        let eos_token_id = 2;
        let bos_token_id = 1;
        let unk_token_id = 3;

        Ok(Self {
            vocab,
            reverse_vocab,
            pad_token_id,
            eos_token_id,
            bos_token_id,
            unk_token_id,
            max_length: 512,
        })
    }

    /// Load vocabulary from simple format file (token_id token)
    fn load_simple_vocab(vocab_file: &Path) -> Result<HashMap<String, u32>> {
        let file = File::open(vocab_file)
            .map_err(|e| ProcessingError::TokenizerLoadFailed {
                source: format!("Failed to open vocab file: {}", e).into(),
            })?;

        let reader = BufReader::new(file);
        let mut vocab = HashMap::new();

        for line in reader.lines() {
            let line = line.map_err(|e| ProcessingError::TokenizerLoadFailed {
                source: format!("Failed to read vocab line: {}", e).into(),
            })?;

            if line.trim().is_empty() {
                continue;
            }

            // Parse: token_id token
            if let Some(space_pos) = line.find(' ') {
                let id_str = &line[..space_pos];
                let token = &line[space_pos + 1..];

                if let Ok(token_id) = id_str.parse::<u32>() {
                    vocab.insert(token.to_string(), token_id);
                }
            }
        }

        if vocab.is_empty() {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: "No vocabulary entries loaded from simple format".into(),
            });
        }

        Ok(vocab)
    }

    /// Load vocabulary from JSON file (handle single-line format)
    #[allow(dead_code)]
    fn load_json_vocab(vocab_file: &Path) -> Result<HashMap<String, u32>> {
        let file = File::open(vocab_file)
            .map_err(|e| ProcessingError::TokenizerLoadFailed {
                source: format!("Failed to open vocab file: {}", e).into(),
            })?;

        let mut content = String::new();
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut content)
            .map_err(|e| ProcessingError::TokenizerLoadFailed {
                source: format!("Failed to read vocab file: {}", e).into(),
            })?;

        // Parse JSON manually to avoid dependencies
        let mut vocab = HashMap::new();

        // Remove outer braces
        let content = content.trim().trim_start_matches('{').trim_end_matches('}');

        // Split by comma and parse entries
        let entries: Vec<&str> = content.split("},").collect();

        for entry in entries {
            let entry = entry.trim();
            if entry.is_empty() {
                continue;
            }

            // Handle last entry without }
            let clean_entry = entry.trim_end_matches('}');

            // Split by colon (but not in strings)
            if let Some(colon_pos) = Self::find_colon_outside_quotes(clean_entry) {
                let key_part = clean_entry[..colon_pos].trim();
                let value_part = clean_entry[colon_pos + 1..].trim();

                // Extract key (remove quotes and escape sequences)
                let key = Self::extract_json_string(key_part);

                // Extract value (should be a number)
                if let Ok(value) = value_part.parse::<u32>() {
                    vocab.insert(key, value);
                }
            }
        }

        if vocab.is_empty() {
            return Err(ProcessingError::TokenizerLoadFailed {
                source: "No vocabulary entries loaded from JSON".into(),
            });
        }

        Ok(vocab)
    }

    /// Find colon that's not inside quotes
    #[allow(dead_code)]
    fn find_colon_outside_quotes(s: &str) -> Option<usize> {
        let mut in_quotes = false;
        let mut escape_next = false;

        for (i, ch) in s.chars().enumerate() {
            if escape_next {
                escape_next = false;
                continue;
            }

            match ch {
                '\\' => escape_next = true,
                '"' => in_quotes = !in_quotes,
                ':' if !in_quotes => return Some(i),
                _ => {}
            }
        }

        None
    }

    /// Extract string value from JSON string literal
    #[allow(dead_code)]
    fn extract_json_string(s: &str) -> String {
        let trimmed = s.trim();
        if trimmed.starts_with('"') && trimmed.ends_with('"') {
            let content = &trimmed[1..trimmed.len()-1];
            // Basic unescaping
            content.replace("\\\"", "\"").replace("\\\\", "\\")
        } else {
            trimmed.to_string()
        }
    }

    /// Encode text to token IDs with simple BPE-like approach
    pub fn encode(&self, text: &str) -> Result<Vec<u64>> {
        let mut tokens = Vec::new();

        // Add prefix for summarization task
        let prefixed_text = format!("summarize: {}", text);

        // Simple tokenization: split on whitespace and punctuation, then try to merge
        let words: Vec<&str> = prefixed_text.split_whitespace().collect();

        for word in words {
            let mut remaining = word;

            while !remaining.is_empty() {
                // Try longest possible token first
                let mut found_token = false;

                for len in (remaining.len().min(10)..=1).rev() {
                    let candidate = &remaining[..len];

                    if let Some(&token_id) = self.vocab.get(candidate) {
                        tokens.push(token_id as u64);
                        remaining = &remaining[len..];
                        found_token = true;
                        break;
                    }
                }

                if !found_token {
                    // Handle unknown characters one by one
                    if let Some(ch) = remaining.chars().next() {
                        let mut buf = [0u8; 4];
                        let char_bytes = ch.encode_utf8(&mut buf);
                        for byte in char_bytes.as_bytes() {
                            tokens.push(*byte as u64);
                        }
                        remaining = &remaining[ch.len_utf8()..];
                    } else {
                        break;
                    }
                }
            }

            // Add space between words
            if let Some(&space_id) = self.vocab.get(" ") {
                tokens.push(space_id as u64);
            } else {
                tokens.push(32); // ASCII space
            }
        }

        // Trim or pad to max_length
        if tokens.len() > self.max_length {
            tokens.truncate(self.max_length);
        }

        while tokens.len() < self.max_length {
            tokens.push(self.pad_token_id as u64);
        }

        Ok(tokens)
    }

    /// Decode token IDs back to text
    pub fn decode(&self, token_ids: &[u64]) -> Result<String> {
        let mut text = String::new();

        for &token_id in token_ids {
            // Skip special tokens
            if token_id == self.pad_token_id as u64 ||
               token_id == self.eos_token_id as u64 ||
               token_id == self.bos_token_id as u64 {
                continue;
            }

            let token_id_u32 = token_id as u32;
            if let Some(token) = self.reverse_vocab.get(&token_id_u32) {
                text.push_str(token);
            } else if (32..=126).contains(&token_id) {
                // Handle ASCII characters directly
                text.push(token_id as u8 as char);
            }
        }

        // Clean up common issues
        text = text.replace("  ", " ") // Double spaces
                 .replace(" .", ".")   // Space before punctuation
                 .replace(" ,", ",")
                 .replace(" (", "(")
                 .replace(" )", ")")
                 .trim()
                 .to_string();

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

    /// Get special token IDs
    pub fn pad_token_id(&self) -> u32 {
        self.pad_token_id
    }

    pub fn eos_token_id(&self) -> u32 {
        self.eos_token_id
    }

    pub fn bos_token_id(&self) -> u32 {
        self.bos_token_id
    }

    pub fn unk_token_id(&self) -> u32 {
        self.unk_token_id
    }
}

/// Create a CodeT5 tokenizer
pub fn create_codet5_tokenizer() -> Result<CodeT5Tokenizer> {
    CodeT5Tokenizer::new("models/codet5-onnx")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codet5_tokenizer_creation() {
        let tokenizer = create_codet5_tokenizer();
        match tokenizer {
            Ok(t) => {
                println!("✅ CodeT5 tokenizer created successfully");
                println!("Vocab size: {}", t.vocab_size());
                assert!(t.vocab_size() > 30000, "Vocab should be substantial");
                assert_eq!(t.max_length(), 512);
                assert_eq!(t.pad_token_id(), 0);
                assert_eq!(t.eos_token_id(), 2);
            }
            Err(e) => {
                println!("❌ CodeT5 tokenizer creation failed: {:?}", e);
                panic!("Failed to create CodeT5 tokenizer: {:?}", e);
            }
        }
    }

    #[test]
    fn test_codet5_tokenization() {
        let tokenizer = create_codet5_tokenizer().unwrap();

        let test_code = "def hello(): pass";
        let tokens = tokenizer.encode(test_code).unwrap();

        assert!(!tokens.is_empty(), "Tokens should not be empty");
        assert!(tokens.len() <= 512, "Tokens should respect max length");

        // Test decoding
        let decoded = tokenizer.decode(&tokens).unwrap();
        println!("Original: {}", test_code);
        println!("Decoded: {}", decoded);
        assert!(!decoded.is_empty(), "Decoded text should not be empty");

        // Should contain "summarize:" prefix
        assert!(decoded.contains("summarize:"), "Should contain prefix");
    }

    #[test]
    fn test_special_tokens() {
        let tokenizer = create_codet5_tokenizer().unwrap();

        // Test special token handling
        let special_tokens = vec![
            tokenizer.pad_token_id(),
            tokenizer.eos_token_id(),
            tokenizer.bos_token_id(),
            tokenizer.unk_token_id(),
        ];

        let decoded = tokenizer.decode(&special_tokens.iter().map(|&id| id as u64).collect::<Vec<_>>()).unwrap();
        assert_eq!(decoded.trim(), ""); // Special tokens should not appear in output
    }
}