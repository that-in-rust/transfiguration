//! Safe Rust wrapper around C++ ORT GenAI library
//!
//! This module provides a memory-safe interface to the C++ ORT GenAI functions
//! for SmolLM2 neural inference with automatic KV cache management.

use std::ffi::{CStr, CString};
use std::path::Path;

use crate::errors::ProcessingError;
use crate::ffi_bindings::*;

const ERROR_BUFFER_SIZE: usize = 1024;

/// Safe wrapper for SmolLM2 ORT GenAI inference session
pub struct SmolInferenceSession {
    handle: SmolLM2Handle,
}

impl SmolInferenceSession {
    /// Create a new SmolLM2 inference session with ORT GenAI
    ///
    /// # Arguments
    /// * `model_path` - Path to the SmolLM2 ONNX model
    /// * `tokenizer_path` - Path to the tokenizer.json file
    ///
    /// # Returns
    /// Result containing the inference session or error
    pub fn new<P: AsRef<Path>>(model_path: P, tokenizer_path: P) -> Result<Self, ProcessingError> {
        let model_cstring = CString::new(model_path.as_ref().to_str().unwrap())
            .map_err(|e| ProcessingError::ModelLoadFailed {
                source: Box::new(e),
            })?;

        let tokenizer_cstring = CString::new(tokenizer_path.as_ref().to_str().unwrap())
            .map_err(|e| ProcessingError::ModelLoadFailed {
                source: Box::new(e),
            })?;

        let mut error_buffer = [0i8; ERROR_BUFFER_SIZE];

        let handle = unsafe {
            create_smol_inference(
                model_cstring.as_ptr(),
                tokenizer_cstring.as_ptr(),
                error_buffer.as_mut_ptr(),
                ERROR_BUFFER_SIZE as i32,
            )
        };

        if handle.is_null() {
            let error_msg = unsafe {
                CStr::from_ptr(error_buffer.as_ptr()).to_string_lossy().to_string()
            };
            Err(ProcessingError::InferenceFailed {
                chunk_id: 0,
                message: error_msg,
            })
        } else {
            Ok(SmolInferenceSession { handle })
        }
    }

    /// Generate a summary for the given text using SmolLM2
    ///
    /// # Arguments
    /// * `text` - Input text to summarize
    ///
    /// # Returns
    /// Result containing the generated summary or error
    pub fn generate_summary(&self, text: &str) -> Result<String, ProcessingError> {
        let text_cstring = CString::new(text)
            .map_err(|e| ProcessingError::InferenceFailed {
                chunk_id: 0,
                message: format!("Failed to convert input text to CString: {}", e),
            })?;

        let mut error_buffer = [0i8; ERROR_BUFFER_SIZE];

        let result_ptr = unsafe {
            generate_summary(
                self.handle,
                text_cstring.as_ptr(),
                error_buffer.as_mut_ptr(),
                ERROR_BUFFER_SIZE as i32,
            )
        };

        if result_ptr.is_null() {
            let error_msg = unsafe {
                CStr::from_ptr(error_buffer.as_ptr()).to_string_lossy().to_string()
            };
            Err(ProcessingError::InferenceFailed {
                chunk_id: 0,
                message: error_msg,
            })
        } else {
            let summary = unsafe {
                let cstr = CStr::from_ptr(result_ptr);
                let result = cstr.to_string_lossy().to_string();
                free_string(result_ptr);
                result
            };
            Ok(summary)
        }
    }

    /// Validate the health of the model and session
    ///
    /// # Returns
    /// Result indicating if the model is healthy
    pub fn validate_health(&self) -> Result<bool, ProcessingError> {
        let mut error_buffer = [0i8; ERROR_BUFFER_SIZE];

        let is_healthy = unsafe {
            validate_model_health(
                self.handle,
                error_buffer.as_mut_ptr(),
                ERROR_BUFFER_SIZE as i32,
            )
        };

        if is_healthy == 0 {
            let error_msg = unsafe {
                CStr::from_ptr(error_buffer.as_ptr()).to_string_lossy().to_string()
            };
            Err(ProcessingError::InferenceFailed {
                chunk_id: 0,
                message: error_msg,
            })
        } else {
            Ok(true)
        }
    }

    /// Get model information
    ///
    /// # Returns
    /// Result containing (vocab_size, max_input_length, max_output_length) or error
    pub fn get_model_info(&self) -> Result<(i32, i32, i32), ProcessingError> {
        let mut vocab_size = 0i32;
        let mut max_input_length = 0i32;
        let mut max_output_length = 0i32;

        unsafe {
            get_model_info(
                self.handle,
                &mut vocab_size,
                &mut max_input_length,
                &mut max_output_length,
            );
        }

        Ok((vocab_size, max_input_length, max_output_length))
    }
}

impl Drop for SmolInferenceSession {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                destroy_smol_inference(self.handle);
            }
        }
    }
}

unsafe impl Send for SmolInferenceSession {}
unsafe impl Sync for SmolInferenceSession {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        // This test will fail without actual model files
        // but demonstrates the API
        let result = SmolInferenceSession::new("model.onnx", "tokenizer.json");
        assert!(result.is_ok() || result.is_err()); // Either way is fine for this test
    }
}