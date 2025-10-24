//! Debug inference to understand why model always outputs token 1

use std::path::Path;
use tempPOC::{
    inference::{OnnxInferencePipeline, InferenceConfig},
    chunking::Chunk,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Debugging CodeT5 Inference");
    println!("=============================");

    // Create inference pipeline
    let config = InferenceConfig::default();
    let pipeline = OnnxInferencePipeline::new(config)?;

    // Test with simple code
    let test_chunk = Chunk {
        id: 0,
        line_start: 0,
        line_end: 3,
        line_count: 3,
        content: "fn hello() {\n    println!(\"Hello world\");\n}".to_string(),
    };

    println!("Input content:\n{}", test_chunk.content);
    println!("---");

    // Test tokenization
    let tokenizer = tempPOC::tokenizer_codet5::create_codet5_tokenizer()?;
    let tokens = tokenizer.encode(&test_chunk.content)?;
    println!("Tokenized input (first 20 tokens): {:?}", &tokens[..20.min(tokens.len())]);

    // Test decode
    let decoded = tokenizer.decode(&tokens)?;
    println!("Decoded back: {}", decoded);
    println!("---");

    // Run inference
    println!("Running inference...");
    let result = pipeline.process_chunk(&test_chunk)?;
    println!("Result: '{}'", result);

    // Test with different content
    let test_chunk2 = Chunk {
        id: 1,
        line_start: 0,
        line_end: 1,
        line_count: 1,
        content: "let x = 42;".to_string(),
    };

    println!("\nSecond test:");
    println!("Input: {}", test_chunk2.content);
    let result2 = pipeline.process_chunk(&test_chunk2)?;
    println!("Result: '{}'", result2);

    Ok(())
}