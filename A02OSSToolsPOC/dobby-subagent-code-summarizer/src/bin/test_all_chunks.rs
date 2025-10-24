//! Test program to run ONNX inference on all iggy chunks
use dobby_subagent_code_summarizer::{
    OnnxInferencePipeline, InferenceConfig, Chunk, inference::InferencePipeline
};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting ONNX inference on all iggy chunks...");

    // Create inference pipeline
    let config = InferenceConfig::default();
    let pipeline = OnnxInferencePipeline::new(config)?;

    let chunks_dir = std::fs::read_dir("chunks/")?;
    let mut chunks = Vec::new();

    // Collect all chunk files
    for entry in chunks_dir {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("txt") || path.file_name().is_some() {
            chunks.push(path);
        }
    }

    // Sort chunks alphabetically
    chunks.sort();

    println!("📁 Found {} chunk files", chunks.len());

    let start_time = Instant::now();
    let mut successful = 0;
    let mut failed = 0;

    // Process each chunk
    for (i, chunk_path) in chunks.iter().enumerate() {
        let chunk_name = chunk_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        println!("\n📄 Processing chunk {} ({})", i+1, chunk_name);

        // Read chunk content
        let content = std::fs::read_to_string(chunk_path)?;
        let line_count = content.lines().count();

        let chunk = Chunk {
            id: i as u64,
            line_start: 1,
            line_end: line_count,
            line_count,
            content,
        };

        // Run inference
        let chunk_start = Instant::now();
        match pipeline.process_chunk(&chunk) {
            Ok(summary) => {
                let duration = chunk_start.elapsed();
                println!("✅ Chunk {} ({} lines): {}ms", chunk_name, line_count, duration.as_millis());
                println!("   📝 Summary: \"{}\"", summary);
                successful += 1;
            }
            Err(e) => {
                println!("❌ Chunk {} failed: {:?}", chunk_name, e);
                failed += 1;
            }
        }
    }

    let total_time = start_time.elapsed();

    println!("\n🎯 === FINAL RESULTS ===");
    println!("✅ Successful: {}", successful);
    println!("❌ Failed: {}", failed);
    println!("⏱️  Total time: {}ms", total_time.as_millis());
    println!("📊 Average per chunk: {}ms", if successful > 0 { total_time.as_millis() / successful as u128 } else { 0 });

    Ok(())
}