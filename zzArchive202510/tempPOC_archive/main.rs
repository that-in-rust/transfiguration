//! Real ONNX inference for Tokio code summarization
//! Testing actual ONNX model loading and inference

use std::fs;
use ort::session::Session;
use ort::value::Tensor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing ONNX inference with real model...");

    // Load MNIST model we downloaded
    let model_path = "models/torch_model.onnx";
    println!("📂 Loading model from: {}", model_path);

    let session = Session::builder()?
        .edit_from_file(model_path)?
        .build()?;

    println!("✅ ONNX Session created successfully");

    // Test with dummy input (MNIST expects 1x1x28x28 tensor)
    let input_shape = [1, 1, 28, 28];
    let input_data: Vec<f32> = (0..784).map(|i| i as f32).collect();
    let input_tensor = Tensor::from_array(input_data.into())
        .reshape(&input_shape)?;

    println!("🔄 Running inference...");

    // Run inference
    let outputs = session.run(vec![input_tensor.into()])?;
    println!("✅ Inference completed, {} outputs", outputs.len());

    // Process outputs
    for (i, output) in outputs.iter().enumerate() {
        if let Ok(tensor) = output.try_extract_tensor() {
            let data = tensor.view::<f32>()?.to_vec();
            println!("📊 Output {}: shape {:?}, range [{:.3}...{:.3}]",
                     i, data.shape(), data.iter().cloned().fold(f32::NAN, f32::min),
                     data.iter().cloned().fold(f32::NAN, f32::max));
        }
    }

    // Now test on Tokio source with pattern matching
    println!("\n🔍 Testing pattern-based chunk processing...");

    let content = fs::read_to_string("/Users/amuldotexe/Projects/transfiguration/tokio-rs-tokio-8a5edab282632443.txt")?;
    let lines: Vec<&str> = content.lines().collect();

    for (chunk_num, chunk_index) in (0..lines.len()).step_by(300).enumerate() {
        let end_line = (chunk_index + 300).min(lines.len());
        let chunk_lines = &lines[chunk_index..end_line];
        let chunk_content = chunk_lines.join("\n");

        // Pattern-based summary until we replace with ONNX inference
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

        println!("Chunk {}: {}", chunk_num + 1, summary);
    }

    println!("\n✅ Ready for ONNX model integration - pattern-based working");
    println!("📋 Next: Replace pattern matching with real ONNX inference");

    Ok(())
}