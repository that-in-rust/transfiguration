//! Qwen2.5 Production CLI - Real Neural Code Summarization
//!
//! Usage:
//!   cargo run --bin code_summarizer -- --file <path> --model-dir <path> --tokenizer-dir <path>
//!
//! Generates markdown summary file: <inputFileName>_Summary_<DATETIME>.md

use std::fs;
use std::path::{Path, PathBuf};
use clap::Parser;
use log::{info, error};
use anyhow::{Context, Result};
use tokio::task::JoinHandle;

use dobby_subagent_code_summarizer::inference::RealInferencePipeline;

/// CLI arguments for the code summarizer
#[derive(Parser, Debug)]
#[command(name = "code_summarizer")]
#[command(about = "Real Neural Code Summarizer with Qwen ONNX", long_about = None)]
struct Args {
    /// Path to the file to summarize
    #[arg(short, long)]
    file: String,

    /// Directory containing ONNX model files
    #[arg(long = "model-dir", default_value = "./models/qwen2.5-0.5b-int4")]
    model_dir: PathBuf,

    /// Directory containing tokenizer files
    #[arg(long = "tokenizer-dir", default_value = "./tokenizer_dir")]
    tokenizer_dir: PathBuf,

    /// Lines per chunk
    #[arg(short, long, default_value_t = 500)]
    chunk_size: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();  // Initialize logging
    let args = Args::parse();

    info!("🚀 Starting Qwen2.5 Real Neural Code Summarizer");
    info!("📁 Input file: {}", args.file);
    info!("📂 Model dir: {}", args.model_dir.display());
    info!("🗂️  Tokenizer dir: {}", args.tokenizer_dir.display());
    info!("📏 Chunk size: {} lines", args.chunk_size);

    // Validate input file exists
    if !Path::new(&args.file).exists() {
        error!("❌ Error: File not found: {}", args.file);
        std::process::exit(1);
    }

    // Read and analyze file
    let file_content = fs::read_to_string(&args.file)
        .context("Failed to read input file")?;

    let lines: Vec<&str> = file_content.lines().collect();
    let file_size = fs::metadata(&args.file)?.len();

    info!("📊 File stats: {} lines, {:.1} MB", lines.len(), file_size as f64 / 1_048_576.0);

    // Load real inference pipeline
    info!("🔧 Loading real neural inference pipeline...");
    let pipeline = RealInferencePipeline::new(args.model_dir.clone(), args.tokenizer_dir.clone())
        .context("Failed to load model/tokenizer - using empty summaries")?;

    // Split into chunks
    let chunks = chunk_code(&file_content, args.chunk_size);
    info!("✅ Created {} chunks", chunks.len());
    info!("🔄 Processing chunks with REAL ONNX neural inference...");

    // Process chunks in parallel
    let mut handles: Vec<JoinHandle<(String, String)>> = vec![];

    for (i, chunk) in chunks.iter().enumerate() {
        let pipeline_clone = pipeline.clone();  // Clone for parallel processing
        let chunk_content = chunk.clone();

        let handle = tokio::spawn(async move {
            info!("🔥 Processing chunk {} ({} chars)", i + 1, chunk_content.len());

            let summary = pipeline_clone.summarize_chunk(&chunk_content)
                .unwrap_or_else(|e| {
                    error!("❌ Inference failed for chunk {}: {}", i + 1, e);
                    format!("❌ Chunk {} failed: Empty summary due to inference error", i + 1)
                });

            (chunk_content, summary)
        });

        handles.push(handle);
    }

    // Wait for all chunks to complete
    let results: Vec<(String, String)> = futures::future::join_all(handles).await
        .into_iter()
        .map(|r| r.unwrap_or_else(|e| {
            error!("Task failed: {}", e);
            ("Failed chunk".to_string(), "❌ Task failed".to_string())
        }))
        .collect();

    info!("✅ All chunks processed");

    // Generate summary output
    let full_summary = results.iter()
        .map(|(chunk, summary)| format!("### Code Chunk\n\n```\n{}\n```\n\n### Summary\n\n{}\n\n---\n", chunk, summary))
        .collect::<Vec<_>>()
        .join("\n");

    // Generate output filename
    let input_path = Path::new(&args.file);
    let file_stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("summary");

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_filename = format!("{}_Summary_{}.md", file_stem, timestamp);

    // Save summary
    fs::write(&output_filename, &full_summary)
        .context("Failed to write summary file")?;

    info!("📝 Summary saved to: {}", output_filename);
    info!("🎉 Real neural summarization completed!");

    // Print summary to console
    println!("\n=== REAL NEURAL SUMMARY ===\n");
    println!("{}", full_summary);

    Ok(())
}

/// Split code into chunks by line count
fn chunk_code(code: &str, max_lines: usize) -> Vec<String> {
    let lines: Vec<&str> = code.lines().collect();
    let mut chunks = Vec::new();

    for chunk_lines in lines.chunks(max_lines) {
        chunks.push(chunk_lines.join("\n"));
    }

    chunks
}