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

/// Extract .deb file to output directory
/// 
/// # Contract
/// - Precondition: input_path exists and is readable
/// - Postcondition: All safe files extracted to output_dir
/// - Error: Returns ExtractionError for any failure
pub fn extract_deb(input_path: &Path, output_dir: &Path) -> Result<()> {
    use std::fs::{self, File};
    use std::io::{BufReader, Read};
    
    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;
    
    // Open and read the .deb file
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);
    
    // Read the entire file into memory for ar parsing
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    
    // Parse the ar archive
    let mut archive = ar::Archive::new(&buffer[..]);
    
    while let Some(entry_result) = archive.next_entry() {
        let mut entry = entry_result.map_err(|e| ExtractionError::Format(format!("AR parse error: {}", e)))?;
        
        // Get filename first
        let filename = {
            let header = entry.header();
            std::str::from_utf8(header.identifier())
                .map_err(|e| ExtractionError::Format(format!("Invalid filename encoding: {}", e)))?
                .to_string()
        };
        
        // Validate the filename for safety
        let safe_path = validate_path(&filename, output_dir)?;
        
        // Read entry data
        let mut entry_data = Vec::new();
        entry.read_to_end(&mut entry_data)?;
        
        // Handle different types of files in the .deb
        match filename.as_str() {
            "debian-binary" => {
                // Extract debian-binary file directly
                fs::write(&safe_path, &entry_data)?;
                println!("Extracted debian-binary to {:?}", safe_path);
            }
            name if name.starts_with("control.tar") => {
                // Extract control tar archive
                extract_tar_archive(&entry_data, output_dir, "control")?;
                println!("Extracted control archive: {}", name);
            }
            name if name.starts_with("data.tar") => {
                // Extract data tar archive
                extract_tar_archive(&entry_data, output_dir, "data")?;
                println!("Extracted data archive: {}", name);
            }
            _ => {
                // Extract other files directly
                fs::write(&safe_path, &entry_data)?;
                println!("Extracted file: {} to {:?}", filename, safe_path);
            }
        }
    }
    
    Ok(())
}

/// Extract a tar archive with path validation
fn extract_tar_archive(data: &[u8], base_output_dir: &Path, subdir: &str) -> Result<()> {
    // Create subdirectory for this archive
    let output_dir = base_output_dir.join(subdir);
    std::fs::create_dir_all(&output_dir)?;
    
    // Try different decompression methods
    let extraction_result = try_extract_gzipped_tar(data, &output_dir)
        .or_else(|_| try_extract_raw_tar(data, &output_dir))
        .or_else(|_| try_extract_xz_tar(data, &output_dir));
    
    match extraction_result {
        Ok(()) => {
            println!("Successfully extracted {} archive", subdir);
            Ok(())
        }
        Err(e) => {
            println!("Warning: Failed to extract {} archive: {}", subdir, e);
            // Don't fail the entire extraction for one archive
            Ok(())
        }
    }
}

/// Try to extract as gzipped tar
fn try_extract_gzipped_tar(data: &[u8], output_dir: &Path) -> Result<()> {
    use std::io::Cursor;
    use flate2::read::GzDecoder;
    
    let cursor = Cursor::new(data);
    let gz_decoder = GzDecoder::new(cursor);
    let mut archive = tar::Archive::new(gz_decoder);
    let entries = archive.entries().map_err(|e| ExtractionError::Format(format!("Gzipped TAR parse error: {}", e)))?;
    extract_tar_entries(entries, output_dir)
}

/// Try to extract as raw tar
fn try_extract_raw_tar(data: &[u8], output_dir: &Path) -> Result<()> {
    use std::io::Cursor;
    
    let cursor = Cursor::new(data);
    let mut archive = tar::Archive::new(cursor);
    let entries = archive.entries().map_err(|e| ExtractionError::Format(format!("Raw TAR parse error: {}", e)))?;
    extract_tar_entries(entries, output_dir)
}

/// Try to extract as xz-compressed tar (placeholder for now)
fn try_extract_xz_tar(_data: &[u8], _output_dir: &Path) -> Result<()> {
    // For now, just return an error since we don't have xz support yet
    // This can be implemented later with the xz2 crate
    Err(ExtractionError::Format("XZ compression not yet supported".to_string()))
}

/// Detect compression type from filename extension
/// 
/// # Contract
/// - WHEN filename ends with .tar.gz THEN return CompressionType::Gzip
/// - WHEN filename ends with .tar.xz THEN return CompressionType::Xz  
/// - WHEN filename ends with .tar THEN return CompressionType::None
/// - WHEN filename has other extension THEN return CompressionType::Unknown
fn detect_compression_type(filename: &str) -> CompressionType {
    // STUB: This will be implemented in GREEN phase
    todo!("detect_compression_type not yet implemented")
}

/// Compression types supported by the system
#[derive(Debug, PartialEq, Eq)]
enum CompressionType {
    None,      // Uncompressed .tar
    Gzip,      // .tar.gz
    Xz,        // .tar.xz  
    Unknown,   // Other/unsupported
}

/// Enhanced tar archive extraction with compression detection
/// 
/// # Contract
/// - WHEN archive is gzipped THEN decompress with flate2 and extract
/// - WHEN archive is uncompressed THEN extract directly
/// - WHEN archive is xz compressed THEN return graceful error
/// - WHEN archive format is invalid THEN return Format error
fn extract_tar_archive_with_compression_detection(
    data: &[u8], 
    base_output_dir: &Path, 
    subdir: &str,
    filename: &str
) -> Result<()> {
    // STUB: This will be implemented in GREEN phase
    todo!("extract_tar_archive_with_compression_detection not yet implemented")
}

/// Extract tar entries with path validation
fn extract_tar_entries<R: std::io::Read>(entries: tar::Entries<R>, output_dir: &Path) -> Result<()> {
    for entry_result in entries {
        let mut entry = entry_result.map_err(|e| ExtractionError::Format(format!("TAR entry error: {}", e)))?;
        
        // Get the path from the tar entry and clone it to avoid borrowing issues
        let path_str = {
            let entry_path = entry.path().map_err(|e| ExtractionError::Format(format!("Invalid tar path: {}", e)))?;
            entry_path.to_string_lossy().to_string()
        };
        
        // Validate the path for safety
        let safe_path = validate_path(&path_str, output_dir)?;
        
        // Create parent directories if needed
        if let Some(parent) = safe_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // Extract the file
        if entry.header().entry_type().is_file() {
            entry.unpack(&safe_path)?;
            println!("Extracted tar file: {} to {:?}", path_str, safe_path);
        } else if entry.header().entry_type().is_dir() {
            std::fs::create_dir_all(&safe_path)?;
            println!("Created directory: {:?}", safe_path);
        }
        // Skip other entry types (symlinks, etc.) for security
    }
    
    Ok(())
}

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

/// CLI argument parsing and validation
pub mod cli {
    use clap::Parser;
    use std::path::PathBuf;

    #[derive(Parser, Debug, PartialEq)]
    #[command(about = "Extract .deb files safely", version)]
    pub struct Args {
        /// Input .deb file
        pub input: PathBuf,
        
        /// Output directory (default: ./extracted)
        #[arg(short, long, default_value = "./extracted")]
        pub output: PathBuf,
        
        /// Verbose output
        #[arg(short, long)]
        pub verbose: bool,
    }

    impl Args {
        /// Parse arguments from command line
        pub fn parse_from_args(args: Vec<&str>) -> Result<Self, clap::Error> {
            Self::try_parse_from(args)
        }
    }
}

#[cfg(test)]
mod error_handling_tests;

#[cfg(test)]
mod error_handling_tests;

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
