use rust_file_unpacker::{Result, validate_path};
use std::path::Path;

/// Test file location for development and testing
const TEST_DEB_FILE: &str = "/home/amuldotexe/Desktop/test_Kiro202509172055-distro-linux-x64.deb";

fn main() -> Result<()> {
    println!("Rust File Unpacker - .deb extraction tool");
    
    // Verify test file exists
    if Path::new(TEST_DEB_FILE).exists() {
        println!("Test file found: {}", TEST_DEB_FILE);
    } else {
        eprintln!("Warning: Test file not found at {}", TEST_DEB_FILE);
    }
    
    // Test the path validation function
    let base_dir = Path::new("/tmp/safe");
    match validate_path("test/file.txt", base_dir) {
        Ok(validated_path) => println!("Path validation working: {:?}", validated_path),
        Err(e) => eprintln!("Path validation error: {}", e),
    }
    
    println!("Project structure initialized successfully!");
    Ok(())
}
