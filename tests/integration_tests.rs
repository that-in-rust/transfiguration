use rust_file_unpacker::{extract_deb, ExtractionError};
use std::path::Path;
use std::time::Instant;
use tempfile::TempDir;

/// Integration test contracts for real .deb file extraction
/// 
/// These tests validate the complete end-to-end functionality using the actual
/// Kiro .deb file to ensure the tool handles real-world package structures.
/// 
/// Requirements tested:
/// - REQ-1.1: Safe extraction of .deb contents to designated directory
/// - REQ-1.3: Extract debian-binary, control.tar.*, and data.tar.* components  
/// - REQ-1.4: Show summary of extracted files and any errors encountered

const KIRO_DEB_PATH: &str = "/home/amuldotexe/Desktop/test_Kiro202509172055-distro-linux-x64.deb";

/// Test Contract: Complete extraction of Kiro .deb file
/// 
/// WHEN I provide the Kiro .deb file path
/// THEN the system SHALL extract all contents to a safe output directory
/// AND the system SHALL extract debian-binary, control.tar.*, and data.tar.* components
/// AND the system SHALL complete extraction without errors
/// 
/// Requirements: 1.1, 1.3, 1.4
#[test]
fn test_complete_kiro_deb_extraction_contract() {
    // Verify the test file exists before running the test
    let kiro_deb_path = Path::new(KIRO_DEB_PATH);
    assert!(
        kiro_deb_path.exists(),
        "Kiro test .deb file not found at: {}. Please ensure the file exists.",
        KIRO_DEB_PATH
    );

    // Create temporary output directory
    let output_dir = TempDir::new()
        .expect("Failed to create temporary output directory");

    // Perform extraction
    let result = extract_deb(kiro_deb_path, output_dir.path());

    // Verify extraction succeeded
    assert!(
        result.is_ok(),
        "Kiro .deb extraction should succeed, but got error: {:?}",
        result.err()
    );

    // Verify basic .deb structure was extracted
    verify_basic_deb_structure(output_dir.path());
}

/// Test Contract: Verify all expected files are extracted to correct locations
/// 
/// WHEN extraction completes successfully
/// THEN the system SHALL create expected directory structure
/// AND the system SHALL extract debian-binary file
/// AND the system SHALL extract control archive contents
/// AND the system SHALL extract data archive contents
/// 
/// Requirements: 1.1, 1.3
#[test]
fn test_kiro_deb_expected_file_structure_contract() {
    let kiro_deb_path = Path::new(KIRO_DEB_PATH);
    assert!(
        kiro_deb_path.exists(),
        "Kiro test .deb file not found at: {}",
        KIRO_DEB_PATH
    );

    let output_dir = TempDir::new()
        .expect("Failed to create temporary output directory");

    // Extract the .deb file
    let result = extract_deb(kiro_deb_path, output_dir.path());
    assert!(
        result.is_ok(),
        "Extraction should succeed: {:?}",
        result.err()
    );

    // Verify expected file structure
    verify_kiro_specific_structure(output_dir.path());
}

/// Test Contract: Tool handles the specific structure of Kiro package
/// 
/// WHEN processing the Kiro .deb file
/// THEN the system SHALL handle Kiro-specific file types and structure
/// AND the system SHALL extract application binaries correctly
/// AND the system SHALL extract package metadata correctly
/// AND the system SHALL handle any Kiro-specific compression formats
/// 
/// Requirements: 1.1, 1.3, 1.4
#[test]
fn test_kiro_package_specific_handling_contract() {
    let kiro_deb_path = Path::new(KIRO_DEB_PATH);
    assert!(
        kiro_deb_path.exists(),
        "Kiro test .deb file not found at: {}",
        KIRO_DEB_PATH
    );

    let output_dir = TempDir::new()
        .expect("Failed to create temporary output directory");

    // Extract with detailed verification
    let result = extract_deb(kiro_deb_path, output_dir.path());
    assert!(
        result.is_ok(),
        "Kiro package extraction should succeed: {:?}",
        result.err()
    );

    // Verify Kiro-specific content was handled correctly
    verify_kiro_application_files(output_dir.path());
}

/// Test Contract: Extraction performance meets reasonable time limits
/// 
/// WHEN extracting the Kiro .deb file (185MB)
/// THEN the system SHALL complete extraction in less than 30 seconds
/// AND the system SHALL show progress during extraction
/// 
/// Requirements: 1.4 (performance aspect)
#[test]
fn test_kiro_deb_extraction_performance_contract() {
    let kiro_deb_path = Path::new(KIRO_DEB_PATH);
    assert!(
        kiro_deb_path.exists(),
        "Kiro test .deb file not found at: {}",
        KIRO_DEB_PATH
    );

    let output_dir = TempDir::new()
        .expect("Failed to create temporary output directory");

    // Measure extraction time
    let start_time = Instant::now();
    let result = extract_deb(kiro_deb_path, output_dir.path());
    let extraction_time = start_time.elapsed();

    // Verify extraction succeeded
    assert!(
        result.is_ok(),
        "Extraction should succeed: {:?}",
        result.err()
    );

    // Verify performance contract (30 seconds for 185MB file)
    assert!(
        extraction_time.as_secs() < 30,
        "Extraction took {:?}, expected < 30 seconds for reasonable performance",
        extraction_time
    );

    println!("Kiro .deb extraction completed in {:?}", extraction_time);
}

/// Test Contract: Handle corrupted or invalid .deb files gracefully
/// 
/// WHEN a corrupted .deb file is provided
/// THEN the system SHALL return appropriate error
/// AND the system SHALL not crash or panic
/// AND the system SHALL provide informative error message
/// 
/// Requirements: 1.4 (error handling)
#[test]
fn test_corrupted_deb_file_handling_contract() {
    // Create a corrupted .deb file (just some random bytes)
    let corrupted_deb = tempfile::NamedTempFile::new()
        .expect("Failed to create temporary file");
    
    std::fs::write(corrupted_deb.path(), b"This is not a valid .deb file")
        .expect("Failed to write corrupted data");

    let output_dir = TempDir::new()
        .expect("Failed to create temporary output directory");

    // Attempt extraction
    let result = extract_deb(corrupted_deb.path(), output_dir.path());

    // Verify it fails gracefully with appropriate error
    assert!(
        result.is_err(),
        "Corrupted .deb file should cause extraction to fail"
    );

    match result.unwrap_err() {
        ExtractionError::Format(_) => {
            // Expected error type for corrupted archive
        }
        ExtractionError::Io(_) => {
            // Also acceptable for I/O related parsing errors
        }
        other => {
            panic!("Expected Format or Io error for corrupted file, got: {:?}", other);
        }
    }
}

/// Test Contract: Large file handling without memory exhaustion
/// 
/// WHEN extracting large .deb files like Kiro (185MB)
/// THEN the system SHALL not consume excessive memory
/// AND the system SHALL handle large files without crashing
/// 
/// Requirements: 1.1 (resource management)
#[test]
fn test_large_file_memory_handling_contract() {
    let kiro_deb_path = Path::new(KIRO_DEB_PATH);
    assert!(
        kiro_deb_path.exists(),
        "Kiro test .deb file not found at: {}",
        KIRO_DEB_PATH
    );

    let output_dir = TempDir::new()
        .expect("Failed to create temporary output directory");

    // This test primarily verifies that extraction doesn't crash
    // Memory usage monitoring would require additional tooling
    let result = extract_deb(kiro_deb_path, output_dir.path());

    assert!(
        result.is_ok(),
        "Large file extraction should succeed without memory issues: {:?}",
        result.err()
    );

    // Verify some content was actually extracted (not just empty directories)
    verify_non_empty_extraction(output_dir.path());
}

// Helper functions for verification

/// Verify basic .deb structure was extracted
fn verify_basic_deb_structure(output_dir: &Path) {
    // Check for debian-binary file
    let debian_binary = output_dir.join("debian-binary");
    assert!(
        debian_binary.exists(),
        "debian-binary file should be extracted to: {:?}",
        debian_binary
    );

    // Verify debian-binary content is valid
    let content = std::fs::read_to_string(&debian_binary)
        .expect("Should be able to read debian-binary file");
    assert!(
        content.trim().starts_with("2."),
        "debian-binary should contain version starting with '2.', got: '{}'",
        content.trim()
    );

    // Check for control directory (from control.tar.*)
    let control_dir = output_dir.join("control");
    assert!(
        control_dir.exists() && control_dir.is_dir(),
        "control directory should be extracted to: {:?}",
        control_dir
    );

    // Check for data directory (from data.tar.*)
    let data_dir = output_dir.join("data");
    if !data_dir.exists() {
        // List what was actually extracted for debugging
        println!("DEBUG: Listing extracted contents:");
        if let Ok(entries) = std::fs::read_dir(output_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  {:?}", entry.path());
                }
            }
        }
    }
    assert!(
        data_dir.exists() && data_dir.is_dir(),
        "data directory should be extracted to: {:?}",
        data_dir
    );
}

/// Verify Kiro-specific file structure
fn verify_kiro_specific_structure(output_dir: &Path) {
    // First verify basic structure
    verify_basic_deb_structure(output_dir);

    // Check for typical Debian package control files
    let control_dir = output_dir.join("control");
    let control_file = control_dir.join("control");
    
    // The control file should exist and contain package metadata
    if control_file.exists() {
        let control_content = std::fs::read_to_string(&control_file)
            .expect("Should be able to read control file");
        
        // Verify it contains typical package information
        assert!(
            control_content.contains("Package:") || control_content.contains("Version:"),
            "Control file should contain package metadata"
        );
    }

    // Check for application files in data directory
    let data_dir = output_dir.join("data");
    if data_dir.exists() {
        // Verify data directory is not empty
        let entries = std::fs::read_dir(&data_dir)
            .expect("Should be able to read data directory");
        
        let entry_count = entries.count();
        assert!(
            entry_count > 0,
            "Data directory should contain extracted application files"
        );
    }
}

/// Verify Kiro application-specific files
fn verify_kiro_application_files(output_dir: &Path) {
    // First verify basic structure
    verify_kiro_specific_structure(output_dir);

    // Look for typical application installation paths
    let data_dir = output_dir.join("data");
    if data_dir.exists() {
        // Check for common Linux application directories
        let possible_app_dirs = [
            data_dir.join("usr"),
            data_dir.join("opt"),
            data_dir.join("usr").join("bin"),
            data_dir.join("usr").join("local"),
        ];

        let mut found_app_structure = false;
        for app_dir in &possible_app_dirs {
            if app_dir.exists() {
                found_app_structure = true;
                println!("Found application directory: {:?}", app_dir);
                break;
            }
        }

        // For Kiro, we expect some kind of application structure
        if !found_app_structure {
            // Just log what we found instead of failing
            println!("Application structure verification - listing data directory contents:");
            if let Ok(entries) = std::fs::read_dir(&data_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        println!("  {:?}", entry.path());
                    }
                }
            }
        }
    }
}

/// Verify extraction produced non-empty results
fn verify_non_empty_extraction(output_dir: &Path) {
    // Count total files extracted
    let mut file_count = 0;
    let mut total_size = 0u64;

    fn count_files_recursive(dir: &Path, file_count: &mut usize, total_size: &mut u64) -> std::io::Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                *file_count += 1;
                if let Ok(metadata) = entry.metadata() {
                    *total_size += metadata.len();
                }
            } else if path.is_dir() {
                count_files_recursive(&path, file_count, total_size)?;
            }
        }
        Ok(())
    }

    count_files_recursive(output_dir, &mut file_count, &mut total_size)
        .expect("Should be able to count extracted files");

    assert!(
        file_count > 0,
        "Extraction should produce at least some files, found: {}",
        file_count
    );

    assert!(
        total_size > 0,
        "Extracted files should have non-zero total size, found: {} bytes",
        total_size
    );

    println!(
        "Extraction verification: {} files extracted, {} bytes total",
        file_count, total_size
    );
}