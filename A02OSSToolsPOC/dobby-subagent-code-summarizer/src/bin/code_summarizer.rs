use clap::Parser;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use tokio::join;
use log::{info, error};

use dobby_subagent_code_summarizer::inference::RealInferencePipeline;

#[derive(Parser)]
#[command(name = "code_summarizer")]
#[command(about = "Real Neural Code Summarizer with Qwen ONNX", long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
    #[arg(short, long, default_value = "./models/qwen2.5-0.5b-int4")]
    model_dir: PathBuf,
    #[arg(short, long, default_value = "./tokenizer_dir")]
    tokenizer_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    info!("Loading real inference pipeline");
    let pipeline = RealInferencePipeline::new(args.model_dir.clone(), args.tokenizer_dir.clone())?;

    let code = fs::read_to_string(&args.file)?;
    let chunks = chunk_code(&code, 500);

    info!("Processing {} chunks in parallel", chunks.len());
    let mut handles = vec![];
    for chunk in chunks {
        let pipeline_clone = pipeline.clone();
        let handle = tokio::spawn(async move {
            let summary = pipeline_clone.summarize_chunk(&chunk)
                .unwrap_or_else(|e| {
                    error!("Inference failed for chunk: {}", e);
                    String::new()  // Empty on failure
                });
            (chunk, summary)
        });
        handles.push(handle);
    }

    let results: Vec<(String, String)> = futures::future::join_all(handles).await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    let full_summary = results.iter().map(|(_, s)| s.as_str()).collect::<Vec<_>>().join("\n");
    println!("Full Summary:\n{}", full_summary);

    let md_path = format!("{}_Summary.md", args.file.replace(".rs", ""));
    fs::write(&md_path, full_summary)?;
    info!("Saved to {}", md_path);

    Ok(())
}

fn chunk_code(code: &str, max_chars: usize) -> Vec<String> {
    code.split('\n').collect::<Vec<_>>()
        .chunks((max_chars / 50).max(1))
        .map(|lines| lines.join("\n"))
        .collect()
}