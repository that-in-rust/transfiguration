use rust_file_unpacker::Result;
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
    
    println!("Project structure initialized successfully!");
    Ok(())
}
