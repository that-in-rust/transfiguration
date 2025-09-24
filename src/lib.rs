use thiserror::Error;

/// Errors that can occur during .deb file extraction
#[derive(Error, Debug)]
pub enum ExtractionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Path traversal detected: {path}")]
    PathTraversal { path: String },
    
    #[error("Archive format error: {0}")]
    Format(String),
    
    #[error("Path validation error: {0}")]
    Path(#[from] PathError),
}

/// Errors that can occur during path validation
#[derive(Error, Debug)]
pub enum PathError {
    #[error("Path contains '..'")]
    Traversal,
    
    #[error("Absolute path not allowed")]
    Absolute,
}

/// Result type alias for extraction operations
pub type Result<T> = std::result::Result<T, ExtractionError>;

/// Result type alias for path validation operations
pub type PathResult<T> = std::result::Result<T, PathError>;

use std::path::{Path, PathBuf};

/// Validate extraction path is safe
/// 
/// # Contract
/// - Precondition: path is valid UTF-8
/// - Postcondition: Returns sanitized path within base_dir
/// - Error: Returns PathError for traversal attempts
pub fn validate_path(path: &str, base_dir: &Path) -> PathResult<PathBuf> {
    // Handle empty path - return base directory
    if path.is_empty() {
        return Ok(base_dir.to_path_buf());
    }
    
    // Check for absolute paths (Unix style)
    if path.starts_with('/') {
        return Err(PathError::Absolute);
    }
    
    // Check for Windows absolute paths (drive letters and UNC paths)
    if path.len() >= 2 {
        // Windows drive letter (C:, D:, etc.)
        if path.chars().nth(1) == Some(':') && path.chars().next().map_or(false, |c| c.is_ascii_alphabetic()) {
            return Err(PathError::Absolute);
        }
    }
    
    // Check for UNC paths (\\server\share)
    if path.starts_with("\\\\") {
        return Err(PathError::Absolute);
    }
    
    // Check for path traversal attempts
    if path.contains("..") {
        return Err(PathError::Traversal);
    }
    
    // Create path from components
    let path_buf = Path::new(path);
    
    // Normalize the path by removing current directory references
    let mut components = Vec::new();
    for component in path_buf.components() {
        match component {
            std::path::Component::Normal(name) => {
                components.push(name);
            }
            std::path::Component::CurDir => {
                // Skip current directory references
                continue;
            }
            std::path::Component::ParentDir => {
                // This should have been caught by the ".." check above
                return Err(PathError::Traversal);
            }
            std::path::Component::Prefix(_) | std::path::Component::RootDir => {
                // These indicate absolute paths
                return Err(PathError::Absolute);
            }
        }
    }
    
    // Build the final path by joining with base directory
    let mut result = base_dir.to_path_buf();
    for component in components {
        result.push(component);
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_path_error_display() {
        let traversal_error = PathError::Traversal;
        assert_eq!(traversal_error.to_string(), "Path contains '..'");
        
        let absolute_error = PathError::Absolute;
        assert_eq!(absolute_error.to_string(), "Absolute path not allowed");
    }
    
    #[test]
    fn test_extraction_error_display() {
        let path_error = ExtractionError::PathTraversal { 
            path: "../etc/passwd".to_string() 
        };
        assert_eq!(path_error.to_string(), "Path traversal detected: ../etc/passwd");
        
        let format_error = ExtractionError::Format("Invalid archive".to_string());
        assert_eq!(format_error.to_string(), "Archive format error: Invalid archive");
    }
    
    #[test]
    fn test_path_error_conversion() {
        let path_error = PathError::Traversal;
        let extraction_error: ExtractionError = path_error.into();
        
        match extraction_error {
            ExtractionError::Path(PathError::Traversal) => (),
            _ => panic!("Expected PathError::Traversal conversion"),
        }
    }

    /// Test Contract: validate_path function
    /// 
    /// WHEN path is "file.txt" and base is "/tmp/safe"
    /// THEN system SHALL return Ok("/tmp/safe/file.txt")
    /// 
    /// WHEN path contains ".." 
    /// THEN system SHALL return Err(PathError::Traversal)
    /// 
    /// WHEN path starts with "/"
    /// THEN system SHALL return Err(PathError::Absolute)
    mod path_validation_contracts {
        use super::*;
        use std::path::Path;

        #[test]
        fn test_safe_relative_path_contract() {
            // Test safe relative path returns correct joined path
            let result = validate_path("file.txt", Path::new("/tmp/safe"));
            assert!(result.is_ok(), "Safe relative path should be valid");
            assert_eq!(result.unwrap(), Path::new("/tmp/safe/file.txt"));
        }

        #[test]
        fn test_safe_nested_relative_path_contract() {
            // Test safe nested relative path
            let result = validate_path("subdir/file.txt", Path::new("/tmp/safe"));
            assert!(result.is_ok(), "Safe nested relative path should be valid");
            assert_eq!(result.unwrap(), Path::new("/tmp/safe/subdir/file.txt"));
        }

        #[test]
        fn test_traversal_attack_single_dotdot_contract() {
            // Test single "../" path returns PathError::Traversal
            let result = validate_path("../etc/passwd", Path::new("/tmp/safe"));
            assert!(result.is_err(), "Path with '../' should be rejected");
            assert!(matches!(result.unwrap_err(), PathError::Traversal));
        }

        #[test]
        fn test_traversal_attack_multiple_dotdot_contract() {
            // Test multiple "../" paths return PathError::Traversal
            let result = validate_path("../../etc/passwd", Path::new("/tmp/safe"));
            assert!(result.is_err(), "Path with multiple '../' should be rejected");
            assert!(matches!(result.unwrap_err(), PathError::Traversal));
        }

        #[test]
        fn test_traversal_attack_embedded_dotdot_contract() {
            // Test embedded "../" in path returns PathError::Traversal
            let result = validate_path("safe/../etc/passwd", Path::new("/tmp/safe"));
            assert!(result.is_err(), "Path with embedded '../' should be rejected");
            assert!(matches!(result.unwrap_err(), PathError::Traversal));
        }

        #[test]
        fn test_absolute_path_unix_contract() {
            // Test absolute Unix path returns PathError::Absolute
            let result = validate_path("/etc/passwd", Path::new("/tmp/safe"));
            assert!(result.is_err(), "Absolute Unix path should be rejected");
            assert!(matches!(result.unwrap_err(), PathError::Absolute));
        }

        #[test]
        fn test_absolute_path_windows_contract() {
            // Test absolute Windows path returns PathError::Absolute
            let result = validate_path("C:\\Windows\\System32", Path::new("/tmp/safe"));
            assert!(result.is_err(), "Absolute Windows path should be rejected");
            assert!(matches!(result.unwrap_err(), PathError::Absolute));
        }

        #[test]
        fn test_empty_path_contract() {
            // Test empty path handling
            let result = validate_path("", Path::new("/tmp/safe"));
            assert!(result.is_ok(), "Empty path should be handled gracefully");
            assert_eq!(result.unwrap(), Path::new("/tmp/safe"));
        }

        #[test]
        fn test_current_directory_reference_contract() {
            // Test "./" path handling
            let result = validate_path("./file.txt", Path::new("/tmp/safe"));
            assert!(result.is_ok(), "Current directory reference should be valid");
            assert_eq!(result.unwrap(), Path::new("/tmp/safe/file.txt"));
        }

        #[test]
        fn test_path_with_spaces_contract() {
            // Test path with spaces
            let result = validate_path("my file.txt", Path::new("/tmp/safe"));
            assert!(result.is_ok(), "Path with spaces should be valid");
            assert_eq!(result.unwrap(), Path::new("/tmp/safe/my file.txt"));
        }

        #[test]
        fn test_path_with_special_chars_contract() {
            // Test path with special characters (but safe)
            let result = validate_path("file-name_123.txt", Path::new("/tmp/safe"));
            assert!(result.is_ok(), "Path with safe special chars should be valid");
            assert_eq!(result.unwrap(), Path::new("/tmp/safe/file-name_123.txt"));
        }

        #[test]
        fn test_windows_drive_letter_contract() {
            // Test Windows drive letter (should be rejected as absolute)
            let result = validate_path("D:\\data\\file.txt", Path::new("/tmp/safe"));
            assert!(result.is_err(), "Windows drive letter path should be rejected");
            assert!(matches!(result.unwrap_err(), PathError::Absolute));
        }

        #[test]
        fn test_unc_path_contract() {
            // Test UNC path (should be rejected as absolute)
            let result = validate_path("\\\\server\\share\\file.txt", Path::new("/tmp/safe"));
            assert!(result.is_err(), "UNC path should be rejected");
            assert!(matches!(result.unwrap_err(), PathError::Absolute));
        }
    }
}