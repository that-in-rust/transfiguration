# Implementation Plan

## Overview

This implementation plan follows the TDD-first approach with executable specifications. Each task follows the STUB → RED → GREEN → REFACTOR cycle, building incrementally from core security functions to complete .deb extraction.

## Tasks



- [x] 1. Set up project structure and core error types
  - Create new Rust binary project with minimal dependencies AND SPOT your test file at /home/amuldotexe/Desktop/test_Kiro202509172055-distro-linux-x64.deb

  - Define `ExtractionError` and `PathError` enums using thiserror
  - Add basic Cargo.toml with clap, ar, tar, flate2, thiserror dependencies
  - _Requirements: 1.1, 1.2_

- [x] 2. Implement path validation with security contracts
  - [x] 2.1 Write path validation test contracts (RED phase)
    - Test safe relative paths return correct joined path
    - Test "../" paths return PathError::Traversal
    - Test absolute paths return PathError::Absolute
    - _Requirements: 2.1, 2.2_
  
  - [x] 2.2 Implement validate_path function (GREEN phase)
    - Write minimal implementation to pass all path validation tests
    - Handle edge cases: empty paths, multiple "..", Windows vs Unix paths
    - _Requirements: 2.1, 2.2_

- [-] 3. Create basic .deb extraction functionality
  - [-] 3.1 Write extraction test contracts (RED phase)
    - Test extract_deb with valid .deb file creates expected output files
    - Test extraction rejects files with path traversal attempts
    - Create test .deb file with known structure for testing
    - _Requirements: 1.1, 1.3, 2.1_
  
  - [ ] 3.2 Implement extract_deb function (GREEN phase)
    - Use ar crate to parse .deb outer archive
    - Extract debian-binary, control.tar.*, data.tar.* members
    - Use tar crate to extract inner tar archives
    - Apply path validation to all extracted file paths
    - _Requirements: 1.1, 1.3, 3.1, 3.2, 3.3_

- [ ] 4. Add CLI interface with clap
  - [ ] 4.1 Write CLI test contracts (RED phase)
    - Test argument parsing for input file, output directory, verbose flag
    - Test default values are applied correctly
    - Test error handling for missing required arguments
    - _Requirements: 4.1, 4.2_
  
  - [ ] 4.2 Implement CLI argument parsing (GREEN phase)
    - Create Args struct with clap derive macros
    - Implement main function that calls extract_deb with parsed arguments
    - Add basic error handling and user-friendly error messages
    - _Requirements: 4.1, 4.2, 4.3, 4.4_

- [ ] 5. Add compression support for tar archives
  - [ ] 5.1 Write compression test contracts (RED phase)
    - Test extraction of .tar.gz files using flate2
    - Test extraction of uncompressed .tar files
    - Test handling of unsupported compression formats
    - _Requirements: 3.1, 3.2, 3.3, 3.4_
  
  - [ ] 5.2 Implement compression detection and handling (GREEN phase)
    - Detect compression by file extension (.tar.gz, .tar.xz, .tar)
    - Use flate2 for gzip decompression
    - Gracefully handle unsupported formats with warnings
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

- [ ] 6. Enhance error handling and user feedback
  - [ ] 6.1 Write error handling test contracts (RED phase)
    - Test graceful handling of corrupted .deb files
    - Test informative error messages for common failure cases
    - Test verbose output shows detailed extraction progress
    - _Requirements: 2.3, 2.4, 4.2, 4.3, 4.4_
  
  - [ ] 6.2 Implement comprehensive error handling (GREEN phase)
    - Add context to errors using error chains
    - Implement verbose logging with println! for progress updates
    - Ensure extraction continues after individual file failures
    - _Requirements: 2.3, 2.4, 4.2, 4.3, 4.4_

- [ ] 7. Create integration tests with real .deb file
  - [ ] 7.1 Write end-to-end test contracts (RED phase)
    - Test complete extraction of Kiro .deb file
    - Verify all expected files are extracted to correct locations
    - Test tool handles the specific structure of Kiro package
    - _Requirements: 1.1, 1.3, 1.4_
  
  - [ ] 7.2 Implement integration test suite (GREEN phase)
    - Create test that downloads or uses provided Kiro .deb file
    - Verify extraction produces expected directory structure
    - Add performance assertion for reasonable extraction time
    - _Requirements: 1.1, 1.3, 1.4_

- [ ] 8. Add basic recursion depth limiting
  - [ ] 8.1 Write recursion limit test contracts (RED phase)
    - Test extraction stops at configured depth limit
    - Test default depth limit prevents excessive nesting
    - Test depth limit can be overridden via CLI flag
    - _Requirements: 2.2, 2.3_
  
  - [ ] 8.2 Implement recursion depth tracking (GREEN phase)
    - Add depth parameter to extraction functions
    - Track current depth during recursive extraction
    - Stop extraction and log warning when limit reached
    - _Requirements: 2.2, 2.3_

## Development Guidelines

### TDD Cycle for Each Task
1. **STUB**: Write function signature with `todo!()` or minimal stub
2. **RED**: Write comprehensive test that fails
3. **GREEN**: Write minimal code to make test pass
4. **REFACTOR**: Clean up code while keeping tests green

### Test-First Requirements
- Every function must have test contracts written before implementation
- Tests must validate the specific acceptance criteria from requirements
- Use descriptive test names that explain the contract being tested
- Prefer multiple small tests over large complex tests

### Code Quality Standards
- Follow Rust idioms: accept `&str`/`&Path`, return owned types
- Use `thiserror` for structured errors, avoid `unwrap()` in production code
- Keep functions small and focused on single responsibility
- Add documentation comments with contracts for public functions

### Security-First Implementation
- Path validation must be tested with malicious inputs
- All file operations must go through path validation
- Fail fast on security violations, don't attempt to "fix" malicious paths
- Log security violations for audit purposes

## Success Criteria

Each task is complete when:
- [ ] All tests pass (unit and integration)
- [ ] Code compiles without warnings
- [ ] Clippy lints pass
- [ ] Function contracts are documented
- [ ] Security requirements are validated with tests

## One-Command Verification

- [ ] final verification

After completing all tasks, the following command should work:
```bash
cargo run -- /home/amuldotexe/Desktop/test_Kiro202509172055-distro-linux-x64.deb --output /home/amuldotexe/Desktop/extracted --verbose
```

This should safely extract the .deb file contents with progress output and complete without errors.