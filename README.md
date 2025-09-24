# Rust File Unpacker

A secure Rust command-line tool for safely extracting .deb files with path traversal protection.

## Features

- Safe extraction of .deb files with security protections
- Path traversal attack prevention
- Support for common compression formats (gzip, tar)
- Comprehensive error handling with structured error types

## Project Structure

- `src/lib.rs` - Core error types and library functions
- `src/main.rs` - CLI application entry point
- `Cargo.toml` - Project dependencies and configuration

## Dependencies

- `clap` - Command-line argument parsing
- `ar` - AR archive format parsing (.deb outer format)
- `tar` - TAR archive extraction (.deb inner archives)
- `flate2` - Gzip compression support
- `thiserror` - Structured error handling

## Test File

The project is configured to work with the test file located at:
`/home/amuldotexe/Desktop/test_Kiro202509172055-distro-linux-x64.deb`

## Development

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run clippy for linting
cargo clippy

# Run the application
cargo run
```

## Error Types

The project defines two main error types:

- `ExtractionError` - Errors during .deb file extraction
- `PathError` - Errors during path validation (traversal protection)

Both error types use the `thiserror` crate for structured error handling.