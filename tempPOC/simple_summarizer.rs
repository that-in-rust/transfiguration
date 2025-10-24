//! Simple 1-line summarizer for 300-line code chunks
//! Each chunk is processed independently with clean context

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("/Users/amuldotexe/Projects/transfiguration/tokio-rs-tokio-8a5edab282632443.txt")?;
    let lines: Vec<&str> = content.lines().collect();

    for chunk_index in (0..lines.len()).step_by(300) {
        let end_line = (chunk_index + 300).min(lines.len());
        let chunk_lines = &lines[chunk_index..end_line];
        let chunk_content = chunk_lines.join("\n");

        let summary = if chunk_content.contains("async") {
            "Async Rust code with tokio runtime"
        } else if chunk_content.contains("impl") {
            "Rust implementation for trait or struct"
        } else if chunk_content.contains("struct") {
            "Rust struct definition with fields"
        } else if chunk_content.contains("fn") {
            "Rust function definition"
        } else {
            "Rust code snippet"
        };

        println!("Lines {}-{}: {}", chunk_index + 1, end_line, summary);
    }

    Ok(())
}