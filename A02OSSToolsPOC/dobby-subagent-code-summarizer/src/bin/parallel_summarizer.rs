//! 20-Agent Parallel Code Summarizer CLI
//!
//! Real neural inference using 20 independent agents for maximum parallelism

use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;
use anyhow::Result;
use log::{info, error};
use env_logger::Env;
use chrono::Utc;

use dobby_subagent_code_summarizer::parallel_agents::{ParallelAgentSystem, ParallelConfig};

#[derive(Parser)]
#[command(name = "parallel_summarizer")]
#[command(about = "20-Agent Parallel Neural Code Summarizer with Qwen ONNX")]
struct Args {
    // === REQUIRED INPUTS (NO DEFAULTS) ===
    #[arg(short, long, help = "Input code file to summarize (must exist)")]
    file: String,

    #[arg(long, help = "Absolute path for final summary output")]
    output_file: String,

    #[arg(long, help = "Absolute path for progress/results log")]
    results_file: String,

    // === PROCESSING PARAMETERS (NO DEFAULTS) ===
    #[arg(long, help = "Lines of code per chunk")]
    loc: usize,

    #[arg(long, help = "Custom prompt for summarization")]
    prompt: String,

    #[arg(long, help = "Number of parallel agents")]
    agent_count: usize,

    // === MODEL CONFIGURATION (NO DEFAULTS) ===
    #[arg(long, help = "Model directory path")]
    model_dir: PathBuf,

    #[arg(long, help = "Tokenizer directory path")]
    tokenizer_dir: PathBuf,

    #[arg(long, help = "Maximum concurrent tasks")]
    max_concurrent: Option<usize>,
}

/// Validate all compulsory CLI arguments
fn validate_args(args: &Args) -> Result<()> {
    let mut errors = Vec::new();

    // Validate input file exists
    if !Path::new(&args.file).exists() {
        errors.push(format!("Input file does not exist: {}", args.file));
    }

    // Validate absolute paths for output files
    if !args.output_file.starts_with('/') {
        errors.push(format!("--output-file must be absolute path (start with '/'), got: {}", args.output_file));
    }

    if !args.results_file.starts_with('/') {
        errors.push(format!("--results-file must be absolute path (start with '/'), got: {}", args.results_file));
    }

    // Validate loc is reasonable
    if args.loc == 0 {
        errors.push("--loc must be greater than 0".to_string());
    }
    if args.loc > 50000 {
        errors.push("--loc should be less than 50000 lines for optimal processing".to_string());
    }

    // Validate agent count
    if args.agent_count == 0 {
        errors.push("--agent-count must be greater than 0".to_string());
    }
    if args.agent_count > 100 {
        errors.push("--agent-count should be less than 100 for system stability".to_string());
    }

    // Check model directory exists
    if !Path::new(&args.model_dir).exists() {
        errors.push(format!("Model directory does not exist: {}", args.model_dir.display()));
    }

    // Check tokenizer directory exists
    if !Path::new(&args.tokenizer_dir).exists() {
        errors.push(format!("Tokenizer directory does not exist: {}", args.tokenizer_dir.display()));
    }

    if !errors.is_empty() {
        error!("❌ VALIDATION ERRORS:");
        for err in &errors {
            error!("   {}", err);
        }
        error!("\n📖 Use --help for complete usage information");
        error!("   cargo run --bin parallel_summarizer -- --help");
        return Err(anyhow::anyhow!("Validation failed: {}", errors.join(", ")));
    }

    // Create parent directories if needed
    if let Some(parent) = Path::new(&args.output_file).parent() {
        fs::create_dir_all(parent)?;
        info!("✅ Created output directory: {}", parent.display());
    }

    if let Some(parent) = Path::new(&args.results_file).parent() {
        fs::create_dir_all(parent)?;
        info!("✅ Created results directory: {}", parent.display());
    }

    Ok(())
}

/// Write progress to results file
fn write_progress(results_file: &str, message: &str) -> Result<()> {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let log_entry = format!("[{}] {}\n", timestamp, message);
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(results_file)?
        .write_all(log_entry.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Args::parse();

    // Phase 0: Validate all compulsory arguments
    validate_args(&args)?;

    // Initialize progress file
    write_progress(&args.results_file, "🚀 Starting 20-Agent Parallel Code Summarizer")?;
    write_progress(&args.results_file, &format!("📁 Input file: {}", args.file))?;
    write_progress(&args.results_file, &format!("📄 Output file: {}", args.output_file))?;
    write_progress(&args.results_file, &format!("📊 Results file: {}", args.results_file))?;
    write_progress(&args.results_file, &format!("🔢 Lines per chunk: {}", args.loc))?;
    write_progress(&args.results_file, &format!("🤖 Agent count: {}", args.agent_count))?;
    write_progress(&args.results_file, &format!("💭 Custom prompt: \"{}\"", args.prompt))?;

    info!("🚀 Starting 20-Agent Parallel Code Summarizer");
    info!("File: {}", args.file);
    info!("Output: {}", args.output_file);
    info!("Results: {}", args.results_file);
    info!("Lines per chunk: {}", args.loc);
    info!("Agents: {}", args.agent_count);

    // Phase 1: Read the file
    let code = fs::read_to_string(&args.file)?;
    let file_size = code.len();
    let line_count = code.lines().count();
    info!("📖 Read file: {} ({} bytes, {} lines)", args.file, file_size, line_count);
    write_progress(&args.results_file, &format!("📖 Read file: {} ({} bytes, {} lines)", args.file, file_size, line_count))?;

    // Phase 2: Create chunks for parallel processing using LOC-based chunking
    let chunks = chunk_code_by_loc(&code, args.loc);
    info!("✅ Created {} chunks for parallel processing ({} lines each)", chunks.len(), args.loc);
    write_progress(&args.results_file, &format!("✅ Created {} chunks for parallel processing ({} lines each)", chunks.len(), args.loc))?;

    // Phase 3: Configure parallel system with user-specified parameters
    let max_concurrent = args.max_concurrent.unwrap_or_else(num_cpus::get);
    let config = ParallelConfig {
        agent_count: args.agent_count,
        model_dir: args.model_dir.clone(),
        tokenizer_dir: args.tokenizer_dir.clone(),
        max_concurrent,
    };

    // Phase 4: Initialize parallel system
    info!("🔧 Initializing {}-agent parallel system...", args.agent_count);
    write_progress(&args.results_file, "🔧 Initializing parallel system...")?;
    let system = ParallelAgentSystem::new(config)?;

    let metrics = system.get_metrics();
    info!("✅ Parallel system ready: {}", metrics);
    write_progress(&args.results_file, &format!("✅ Parallel system ready: {}", metrics))?;

    // Phase 5: Process chunks in parallel using custom prompts
    info!("🔄 Starting parallel processing with {} agents...", args.agent_count);
    write_progress(&args.results_file, "🔄 Starting parallel processing...")?;
    let start_time = std::time::Instant::now();

    let results = system.process_chunks_parallel_with_prompts(chunks, &args.prompt).await?;

    let processing_time = start_time.elapsed();
    info!("✅ Parallel processing completed in {:?}", processing_time);
    write_progress(&args.results_file, &format!("✅ Parallel processing completed in {:?}", processing_time))?;

    // Phase 6: Display summary to console
    println!("\n🎯 PROCESSING SUMMARY:");
    println!("=====================");
    println!("Total chunks processed: {}", results.len());
    println!("Total processing time: {:?}", processing_time);
    println!("Average time per chunk: {}ms", processing_time.as_millis() / results.len() as u128);
    println!("Throughput: {:.2} chunks/second", results.len() as f64 / processing_time.as_secs_f64());

    // Phase 7: Save final summary to specified output file
    let full_summary = results.iter()
        .map(|(_, s)| s.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");

    fs::write(&args.output_file, full_summary)?;
    info!("💾 Final summary saved to: {}", args.output_file);
    write_progress(&args.results_file, &format!("💾 Final summary saved to: {}", args.output_file))?;

    // Phase 8: Final progress update
    write_progress(&args.results_file, "🎉 PARALLEL PROCESSING COMPLETE!")?;
    write_progress(&args.results_file, &format!("📊 Final metrics: {} chunks, {:?} total time", results.len(), processing_time))?;

    println!("\n🎉 PARALLEL PROCESSING COMPLETE!");
    println!("📄 Summary saved to: {}", args.output_file);
    println!("📊 Progress logged to: {}", args.results_file);
    Ok(())
}

/// LOC-based chunking function - replaces char-based chunking
fn chunk_code_by_loc(code: &str, loc: usize) -> Vec<String> {
    code.split('\n')
        .collect::<Vec<_>>()
        .chunks(loc)
        .map(|lines| lines.join("\n"))
        .collect()
}