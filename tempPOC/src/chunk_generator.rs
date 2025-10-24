//! Utility to generate 300-line chunks from the Tokio source file
//!
//! This is a standalone utility to pre-process the large Tokio source file
//! into manageable 300-line chunks for parallel processing.

use std::path::Path;
use anyhow::{Context, Result};
use tokio::fs;

/// Generate 300-line chunks from the Tokio source file
pub async fn generate_chunks() -> Result<()> {
    println!("🔧 Generating 300-line chunks from Tokio source file...");

    let source_file = Path::new("/Users/amuldotexe/Projects/transfiguration/tokio-rs-tokio-8a5edab282632443.txt");
    let chunks_dir = Path::new("/Users/amuldotexe/Projects/transfiguration/tempPOC/chunks");

    // Create chunks directory if it doesn't exist
    fs::create_dir_all(&chunks_dir).await
        .context("Failed to create chunks directory")?;

    // Read the entire source file
    println!("📖 Reading Tokio source file...");
    let content = fs::read_to_string(&source_file).await
        .context("Failed to read source file")?;

    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    println!("📊 Total lines in source: {}", total_lines);

    let chunk_size = 300;
    let total_chunks = (total_lines + chunk_size - 1) / chunk_size;

    println!("📦 Creating {} chunks of {} lines each...", total_chunks, chunk_size);

    // Generate chunks
    for chunk_index in 0..total_chunks {
        let start_line = chunk_index * chunk_size;
        let end_line = (start_line + chunk_size).min(total_lines);
        let chunk_lines = &lines[start_line..end_line];
        let chunk_content = chunk_lines.join("\n");

        // Write chunk to file
        let chunk_filename = format!("chunk_{:04}_lines_{:05}-{:05}.rs",
            chunk_index + 1, start_line + 1, end_line);
        let chunk_path = chunks_dir.join(&chunk_filename);

        fs::write(&chunk_path, chunk_content).await
            .context(format!("Failed to write chunk: {}", chunk_filename))?;

        println!("✅ Created {}: lines {}-{} ({} lines)",
            chunk_filename, start_line + 1, end_line, end_line - start_line);
    }

    println!("🎉 Successfully generated {} chunks!", total_chunks);
    println!("📁 Chunks saved to: {:?}", chunks_dir);

    // Generate summary report
    let summary = format!(r#"
# Tokio Source Chunks Summary

## Statistics
- **Total lines**: {}
- **Chunk size**: {} lines
- **Total chunks**: {}
- **Lines per chunk**: 300 (except possibly last chunk)

## Files Generated
All chunks saved to: `chunks/` directory
Naming convention: `chunk_XXXX_lines_YYYYY-ZZZZZ.rs`

- chunk_0001_lines_00001-00300.rs
- chunk_0002_lines_00301-00600.rs
- ...
- chunk_NNNN_lines_YYYYY-ZZZZZ.rs

## Next Steps
Run: `cargo run --bin process_chunks` to process all chunks in parallel using 20 ONNX model instances.
"#, total_lines, chunk_size, total_chunks);

    let summary_path = Path::new("/Users/amuldotexe/Projects/transfiguration/tempPOC/chunks/README.md");
    fs::write(summary_path, summary).await
        .context("Failed to write chunks summary")?;

    println!("📝 Summary report generated: chunks/README.md");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    generate_chunks().await
}