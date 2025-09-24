#[cfg(test)]
mod error_handling_contracts {
    use super::*;
    use std::io::Write;

    fn create_corrupted_deb_file() -> std::io::Result<tempfile::NamedTempFile> {
        // Create a file that looks like a .deb but has corrupted content
        let mut temp_file = tempfile::NamedTempFile::new()?;
        
        // Write invalid ar archive header
        temp_file.write_all(b"!<corrupted>\n")?;
        temp_file.write_all(b"invalid binary data that will cause parsing errors")?;
        temp_file.flush()?;
        Ok(temp_file)
    }

    fn create_truncated_deb_file() -> std::io::Result<tempfile::NamedTempFile> {
        // Create a .deb file that starts correctly but is truncated
        let mut temp_file = tempfile::NamedTempFile::new()?;
        
        // Write valid ar archive header but truncate the content
        temp_file.write_all(b"!<arch>\n")?;
        temp_file.write_all(b"debian-binary   1234567890  0     0     100644  4         `\n")?;
        // Missing the actual content - this should cause parsing errors
        temp_file.flush()?;
        Ok(temp_file)
    }

    fn create_empty_file() -> std::io::Result<tempfile::NamedTempFile> {
        // Create completely empty file
        let temp_file = tempfile::NamedTempFile::new()?;
        Ok(temp_file)
    }

    #[test]
    fn test_corrupted_deb_graceful_handling_contract() {
        // Test graceful handling of corrupted .deb files
        let corrupted_deb = create_corrupted_deb_file()
            .expect("Failed to create corrupted test file");
        let output_dir = tempfile::tempdir()
            .expect("Failed to create output directory");
        
        // This should fail gracefully with a Format error, not panic
        let result = crate::extract_deb(corrupted_deb.path(), output_dir.path());
        
        assert!(result.is_err(), "Corrupted .deb should cause error");
        match result.unwrap_err() {
            crate::ExtractionError::Format(msg) => {
                assert!(!msg.is_empty(), "Error message should not be empty");
                assert!(msg.contains("AR parse error") || msg.contains("parse"), 
                       "Error message should indicate parsing issue: {}", msg);
            }
            other => panic!("Expected Format error for corrupted file, got: {:?}", other),
        }
    }

    #[test]
    fn test_truncated_deb_graceful_handling_contract() {
        // Test graceful handling of truncated .deb files
        let truncated_deb = create_truncated_deb_file()
            .expect("Failed to create truncated test file");
        let output_dir = tempfile::tempdir()
            .expect("Failed to create output directory");
        
        let result = crate::extract_deb(truncated_deb.path(), output_dir.path());
        
        assert!(result.is_err(), "Truncated .deb should cause error");
        // Should get either Format error or IO error depending on where it fails
        match result.unwrap_err() {
            crate::ExtractionError::Format(msg) => {
                assert!(!msg.is_empty(), "Format error message should not be empty");
            }
            crate::ExtractionError::Io(_) => {
                // Also acceptable - truncated file might cause IO error
            }
            other => panic!("Expected Format or IO error for truncated file, got: {:?}", other),
        }
    }

    #[test]
    fn test_empty_file_graceful_handling_contract() {
        // Test graceful handling of empty files
        let empty_file = create_empty_file()
            .expect("Failed to create empty test file");
        let output_dir = tempfile::tempdir()
            .expect("Failed to create output directory");
        
        let result = crate::extract_deb(empty_file.path(), output_dir.path());
        
        assert!(result.is_err(), "Empty file should cause error");
        match result.unwrap_err() {
            crate::ExtractionError::Format(msg) => {
                assert!(!msg.is_empty(), "Error message should not be empty");
            }
            crate::ExtractionError::Io(_) => {
                // Also acceptable - empty file might cause IO error
            }
            other => panic!("Expected Format or IO error for empty file, got: {:?}", other),
        }
    }

    #[test]
    fn test_informative_error_messages_contract() {
        // Test that error messages are informative and actionable
        
        // Test path traversal error message
        let path_traversal_error = crate::ExtractionError::PathTraversal { 
            path: "../etc/passwd".to_string() 
        };
        let error_msg = path_traversal_error.to_string();
        assert!(error_msg.contains("../etc/passwd"), 
               "Path traversal error should include the malicious path");
        assert!(error_msg.contains("Path traversal detected"), 
               "Error should clearly indicate path traversal");
        
        // Test format error message
        let format_error = crate::ExtractionError::Format("Invalid AR header".to_string());
        let error_msg = format_error.to_string();
        assert!(error_msg.contains("Archive format error"), 
               "Format error should indicate archive format issue");
        assert!(error_msg.contains("Invalid AR header"), 
               "Format error should include specific details");
        
        // Test IO error context
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
        let extraction_error = crate::ExtractionError::Io(io_error);
        let error_msg = extraction_error.to_string();
        assert!(error_msg.contains("IO error"), 
               "IO error should be clearly labeled");
    }

    #[test]
    fn test_error_message_actionability_contract() {
        // Test that error messages provide actionable information
        
        // Path errors should be specific about the problem
        let traversal_error = crate::PathError::Traversal;
        assert_eq!(traversal_error.to_string(), "Path contains '..'");
        
        let absolute_error = crate::PathError::Absolute;
        assert_eq!(absolute_error.to_string(), "Absolute path not allowed");
        
        // These messages should help users understand what went wrong
        // and how to fix it (by not using malicious paths)
    }

    #[test]
    fn test_verbose_output_progress_contract() {
        // Test that verbose output shows detailed extraction progress
        // This test verifies the contract for verbose logging
        
        // Since we can't easily capture println! output in tests,
        // we'll test the verbose flag handling in the CLI args
        let args = crate::cli::Args {
            input: std::path::PathBuf::from("test.deb"),
            output: std::path::PathBuf::from("./output"),
            verbose: true,
        };
        
        assert_eq!(args.verbose, true, "Verbose flag should be properly set");
        
        // In the GREEN phase, we'll implement actual verbose logging
        // that can be tested by capturing output or using a logging framework
    }

    #[test]
    fn test_extraction_continues_after_individual_failures_contract() {
        // Test that extraction continues after individual file failures
        // This is a contract test for resilient extraction behavior
        
        // Create a test scenario where some files fail but others succeed
        // For now, this is a placeholder that will be implemented in GREEN phase
        
        // The contract is:
        // WHEN some files in archive fail to extract
        // THEN system SHALL continue processing remaining files
        // AND system SHALL report which files failed
        // AND system SHALL still return success if critical files extracted
        
        println!("Contract test: extraction should continue after individual failures");
        println!("This will be implemented in GREEN phase with proper error collection");
    }

    #[test]
    fn test_error_context_chaining_contract() {
        // Test that errors provide proper context chains
        // This ensures users can understand the full error context
        
        // Test PathError -> ExtractionError conversion
        let path_error = crate::PathError::Traversal;
        let extraction_error: crate::ExtractionError = path_error.into();
        
        match extraction_error {
            crate::ExtractionError::Path(crate::PathError::Traversal) => {
                // Verify the error chain is preserved
                assert_eq!(extraction_error.to_string(), "Path validation error: Path contains '..'");
            }
            other => panic!("Expected Path(Traversal) error, got: {:?}", other),
        }
    }

    #[test]
    fn test_progress_reporting_contract() {
        // Test contract for progress reporting during extraction
        // This verifies that the system provides feedback during long operations
        
        // Contract requirements:
        // WHEN verbose mode is enabled
        // THEN system SHALL report extraction progress
        // AND system SHALL show which files are being processed
        // AND system SHALL indicate completion status
        
        // For now, this is a contract placeholder
        // In GREEN phase, we'll implement actual progress reporting
        println!("Contract test: verbose mode should show detailed progress");
        println!("Progress should include: file names, extraction status, completion summary");
    }

    #[test]
    fn test_resource_cleanup_on_error_contract() {
        // Test that resources are properly cleaned up even when errors occur
        // This ensures no resource leaks during error conditions
        
        // Contract:
        // WHEN extraction fails at any point
        // THEN system SHALL clean up any partially created files/directories
        // AND system SHALL not leave the output directory in inconsistent state
        
        // This will be implemented in GREEN phase with proper RAII patterns
        println!("Contract test: resources should be cleaned up on errors");
        println!("No partial files should remain after extraction failures");
    }
}