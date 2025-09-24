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
}