//! Test ORT 2.0 API Compatibility
//!
//! Minimal test to verify correct session creation patterns

use anyhow::Result;
use ort::{
    environment::Environment,
    execution_providers::ExecutionProvider,
    session::builder::{GraphOptimizationLevel, SessionBuilder},
    session::Session,
    value::Value,
};
use std::path::PathBuf;
use ndarray::Array2;

pub fn test_ort_session_creation() -> Result<()> {
    println!("🔍 Testing ORT 2.0 session creation patterns...");

    // Create environment - ort 2.0 API
    let env = Environment::new()?;

    println!("✅ Environment created successfully");

    // Test session builder with CPU provider ( ort 2.0 API)
    let session_builder = SessionBuilder::new()?
        .with_optimization_level(GraphOptimizationLevel::Level1)?
        .with_execution_providers(&[ExecutionProvider::cpu()])?;

    println!("✅ Session builder created successfully");

    // Try to load a model if it exists (optional test)
    let model_path = PathBuf::from("./models/qwen2.5-0.5b-int4/model.onnx");
    if model_path.exists() {
        println!("📂 Testing model loading: {}", model_path.display());
        match session_builder.commit_from_file(&model_path) {
            Ok(session) => {
                println!("✅ Model loaded successfully");

                // Test session inputs
                println!("   Session inputs: {}", session.inputs.len());
                for (i, input) in session.inputs.iter().enumerate() {
                    println!("   Input {}: {} {:?}", i, input.name, input.dimensions);
                }

                // Test basic tensor creation - ort 2.0 API
                let test_data = vec![1.0f32, 2.0, 3.0, 4.0];
                let test_shape = (2, 2);
                let test_value = Value::from_array(test_shape, test_data.into_boxed_slice())?;
                println!("✅ Tensor creation test: PASSED");

                return Ok(());
            }
            Err(e) => {
                println!("❌ Model loading failed: {}", e);
                return Err(e.into());
            }
        }
    } else {
        println!("⚠️  No model found at {}, skipping session test", model_path.display());
    }

    Ok(())
}

fn main() -> Result<()> {
    test_ort_session_creation()?;
    println!("🔍 ORT 2.0 API testing complete");
    Ok(())
}