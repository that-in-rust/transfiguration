use thiserror::Error;
use std::path::PathBuf;

/// Enhanced error handling with context chains and actionable messages
#[derive(Error, Debug)]
pub enum EnhancedExtractionError {
    #[error("IO error during {operation}: {source}")]
    Io {
        operation: String,
        source: std::io::Error,
    },
    
    #[error("Path traversal attack detected: '{path}' contains dangerous components")]
    PathTraversal { path: String },
    
    #[error("Archive format error in {file}: {message}")]
    Format { file: String, message: String },
    
    #[error("Path validation failed for '{path}': {source}")]
    Path { path: String, source: PathError },
    
    #[error("Compression error in {archive}: {message}")]
    Compression { archive: String, message: String },
    
    #[error("Resource limit exceeded: {limit_type} ({current}/{max})")]
    ResourceLimit {
        limit_type: String,
        current: u64,
        max: u64,
    },
    
    #[error("Multiple extraction failures: {failed_count} of {total_count} files failed")]
    PartialFailure {
        failed_count: usize,
        total_count: usize,
        failures: Vec<String>,
    },
}

/// Enhanced path validation errors with specific context
#[derive(Error, Debug)]
pub enum PathError {
    #[error("Path contains '..' directory traversal")]
    Traversal,
    
    #[error("Absolute path not allowed in archive")]
    Absolute,
    
    #[error("Path too long: {length} characters (max: {max})")]
    TooLong { length: usize, max: usize },
    
    #[error("Invalid characters in path: {invalid_chars:?}")]
    InvalidCharacters { invalid_chars: Vec<char> },
}

/// Progress reporting for verbose output
#[derive(Debug, Clone)]
pub struct ExtractionProgress {
    pub current_file: String,
    pub files_processed: usize,
    pub total_files: usize,
    pub bytes_processed: u64,
    pub operation: String,
}

/// Verbose logger for detailed extraction progress
pub struct VerboseLogger {
    enabled: bool,
}

impl VerboseLogger {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    
    pub fn log_progress(&self, progress: &ExtractionProgress) {
        if self.enabled {
            println!(
                "[{}/{}] {}: {} ({} bytes processed)",
                progress.files_processed,
                progress.total_files,
                progress.operation,
                progress.current_file,
                progress.bytes_processed
            );
        }
    }
    
    pub fn log_operation(&self, operation: &str, details: &str) {
        if self.enabled {
            println!("VERBOSE: {}: {}", operation, details);
        }
    }
    
    pub fn log_warning(&self, warning: &str) {
        if self.enabled {
            println!("WARNING: {}", warning);
        }
    }
    
    pub fn log_error(&self, error: &str) {
        if self.enabled {
            println!("ERROR: {}", error);
        }
    }
    
    pub fn log_success(&self, message: &str) {
        if self.enabled {
            println!("SUCCESS: {}", message);
        }
    }
}

/// Enhanced extraction context with error collection
pub struct ExtractionContext {
    pub verbose_logger: VerboseLogger,
    pub errors: Vec<EnhancedExtractionError>,
    pub warnings: Vec<String>,
    pub files_processed: usize,
    pub files_skipped: usize,
    pub bytes_processed: u64,
}

impl ExtractionContext {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose_logger: VerboseLogger::new(verbose),
            errors: Vec::new(),
            warnings: Vec::new(),
            files_processed: 0,
            files_skipped: 0,
            bytes_processed: 0,
        }
    }
    
    pub fn add_error(&mut self, error: EnhancedExtractionError) {
        self.verbose_logger.log_error(&error.to_string());
        self.errors.push(error);
    }
    
    pub fn add_warning(&mut self, warning: String) {
        self.verbose_logger.log_warning(&warning);
        self.warnings.push(warning);
    }
    
    pub fn log_progress(&self, current_file: &str, operation: &str) {
        let progress = ExtractionProgress {
            current_file: current_file.to_string(),
            files_processed: self.files_processed,
            total_files: self.files_processed + self.files_skipped, // Approximate
            bytes_processed: self.bytes_processed,
            operation: operation.to_string(),
        };
        self.verbose_logger.log_progress(&progress);
    }
    
    pub fn increment_processed(&mut self, bytes: u64) {
        self.files_processed += 1;
        self.bytes_processed += bytes;
    }
    
    pub fn increment_skipped(&mut self) {
        self.files_skipped += 1;
    }
    
    pub fn has_critical_errors(&self) -> bool {
        self.errors.iter().any(|e| matches!(e, 
            EnhancedExtractionError::PathTraversal { .. } |
            EnhancedExtractionError::ResourceLimit { .. }
        ))
    }
    
    pub fn print_summary(&self) {
        println!("\n=== Extraction Summary ===");
        println!("Files processed: {}", self.files_processed);
        println!("Files skipped: {}", self.files_skipped);
        println!("Bytes processed: {}", self.bytes_processed);
        println!("Warnings: {}", self.warnings.len());
        println!("Errors: {}", self.errors.len());
        
        if !self.warnings.is_empty() {
            println!("\nWarnings:");
            for warning in &self.warnings {
                println!("  - {}", warning);
            }
        }
        
        if !self.errors.is_empty() {
            println!("\nErrors:");
            for error in &self.errors {
                println!("  - {}", error);
            }
        }
        
        if self.has_critical_errors() {
            println!("\n⚠️  Critical security issues detected!");
        } else if self.errors.is_empty() {
            println!("\n✅ Extraction completed successfully!");
        } else {
            println!("\n⚠️  Extraction completed with {} non-critical errors", self.errors.len());
        }
    }
}

/// Enhanced path validation with detailed error context
pub fn validate_path_enhanced(path: &str, base_dir: &std::path::Path) -> Result<PathBuf, PathError> {
    const MAX_PATH_LENGTH: usize = 4096;
    
    // Check path length
    if path.len() > MAX_PATH_LENGTH {
        return Err(PathError::TooLong {
            length: path.len(),
            max: MAX_PATH_LENGTH,
        });
    }
    
    // Check for invalid characters
    let invalid_chars: Vec<char> = path.chars()
        .filter(|&c| c == '\0' || c < ' ' && c != '\t' && c != '\n')
        .collect();
    
    if !invalid_chars.is_empty() {
        return Err(PathError::InvalidCharacters { invalid_chars });
    }
    
    // Handle empty path
    if path.is_empty() {
        return Ok(base_dir.to_path_buf());
    }
    
    // Check for absolute paths
    if path.starts_with('/') || 
       (path.len() >= 2 && path.chars().nth(1) == Some(':') && 
        path.chars().next().map_or(false, |c| c.is_ascii_alphabetic())) ||
       path.starts_with("\\\\") {
        return Err(PathError::Absolute);
    }
    
    // Check for path traversal
    if path.contains("..") {
        return Err(PathError::Traversal);
    }
    
    // Build safe path
    let path_buf = std::path::Path::new(path);
    let mut result = base_dir.to_path_buf();
    
    for component in path_buf.components() {
        match component {
            std::path::Component::Normal(name) => {
                result.push(name);
            }
            std::path::Component::CurDir => {
                // Skip current directory references
                continue;
            }
            std::path::Component::ParentDir => {
                return Err(PathError::Traversal);
            }
            std::path::Component::Prefix(_) | std::path::Component::RootDir => {
                return Err(PathError::Absolute);
            }
        }
    }
    
    Ok(result)
}

/// Convert standard errors to enhanced errors with context
impl From<std::io::Error> for EnhancedExtractionError {
    fn from(error: std::io::Error) -> Self {
        EnhancedExtractionError::Io {
            operation: "unknown operation".to_string(),
            source: error,
        }
    }
}

impl From<PathError> for EnhancedExtractionError {
    fn from(error: PathError) -> Self {
        EnhancedExtractionError::Path {
            path: "unknown path".to_string(),
            source: error,
        }
    }
}

/// Helper to create IO errors with context
pub fn io_error_with_context(operation: &str, source: std::io::Error) -> EnhancedExtractionError {
    EnhancedExtractionError::Io {
        operation: operation.to_string(),
        source,
    }
}

/// Helper to create path errors with context
pub fn path_error_with_context(path: &str, source: PathError) -> EnhancedExtractionError {
    EnhancedExtractionError::Path {
        path: path.to_string(),
        source,
    }
}

/// Helper to create format errors with context
pub fn format_error_with_context(file: &str, message: &str) -> EnhancedExtractionError {
    EnhancedExtractionError::Format {
        file: file.to_string(),
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_enhanced_path_validation() {
        let base = Path::new("/tmp/safe");
        
        // Test valid path
        let result = validate_path_enhanced("file.txt", base);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Path::new("/tmp/safe/file.txt"));
        
        // Test path traversal
        let result = validate_path_enhanced("../etc/passwd", base);
        assert!(matches!(result.unwrap_err(), PathError::Traversal));
        
        // Test absolute path
        let result = validate_path_enhanced("/etc/passwd", base);
        assert!(matches!(result.unwrap_err(), PathError::Absolute));
        
        // Test path too long
        let long_path = "a".repeat(5000);
        let result = validate_path_enhanced(&long_path, base);
        assert!(matches!(result.unwrap_err(), PathError::TooLong { .. }));
        
        // Test invalid characters
        let result = validate_path_enhanced("file\0.txt", base);
        assert!(matches!(result.unwrap_err(), PathError::InvalidCharacters { .. }));
    }
    
    #[test]
    fn test_extraction_context() {
        let mut context = ExtractionContext::new(false);
        
        // Test adding errors and warnings
        context.add_error(EnhancedExtractionError::PathTraversal {
            path: "../etc/passwd".to_string(),
        });
        context.add_warning("Skipping unsupported file".to_string());
        
        assert_eq!(context.errors.len(), 1);
        assert_eq!(context.warnings.len(), 1);
        assert!(context.has_critical_errors());
        
        // Test progress tracking
        context.increment_processed(1024);
        assert_eq!(context.files_processed, 1);
        assert_eq!(context.bytes_processed, 1024);
    }
    
    #[test]
    fn test_verbose_logger() {
        let logger = VerboseLogger::new(true);
        
        // These should print when verbose is enabled
        logger.log_operation("test", "testing verbose output");
        logger.log_warning("test warning");
        logger.log_error("test error");
        logger.log_success("test success");
        
        let progress = ExtractionProgress {
            current_file: "test.txt".to_string(),
            files_processed: 1,
            total_files: 10,
            bytes_processed: 1024,
            operation: "extracting".to_string(),
        };
        logger.log_progress(&progress);
    }
    
    #[test]
    fn test_error_context_helpers() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let enhanced_err = io_error_with_context("reading file", io_err);
        
        match enhanced_err {
            EnhancedExtractionError::Io { operation, .. } => {
                assert_eq!(operation, "reading file");
            }
            _ => panic!("Expected IO error"),
        }
        
        let path_err = path_error_with_context("../etc/passwd", PathError::Traversal);
        match path_err {
            EnhancedExtractionError::Path { path, source } => {
                assert_eq!(path, "../etc/passwd");
                assert!(matches!(source, PathError::Traversal));
            }
            _ => panic!("Expected Path error"),
        }
        
        let format_err = format_error_with_context("test.tar", "invalid header");
        match format_err {
            EnhancedExtractionError::Format { file, message } => {
                assert_eq!(file, "test.tar");
                assert_eq!(message, "invalid header");
            }
            _ => panic!("Expected Format error"),
        }
    }
}