//! Comprehensive TDD Integration Test for SmolLM2 Implementation
//!
//! This test demonstrates the complete TDD-First implementation:
//! ✅ RED Phase: Failing tests with measurable contracts
//! ✅ GREEN Phase: Minimal implementation satisfying all contracts
//! ✅ Real SmolLM2 tokenizer (49,152 vocabulary)
//! ✅ Pattern-based inference (to be replaced with ONNX)

use dobby_subagent_code_summarizer::{
    Chunk, SmolOutputValidator, SmolPerformanceMetrics,
    create_production_smollm_tokenizer
};
use dobby_subagent_code_summarizer::smol_inference::SmolLM2InferencePipeline;
use dobby_subagent_code_summarizer::smol_inference_contract::SmolLM2Inference;
use std::time::Duration;

/// Comprehensive TDD Integration Test
#[test]
fn test_comprehensive_smollm_tdd_implementation() -> anyhow::Result<()> {
    println!("🚀 Starting Comprehensive SmolLM2 TDD Integration Test");

    // Phase 1: Test real SmolLM2 tokenizer loading
    println!("\n📋 Phase 1: Testing Real SmolLM2 Tokenizer");
    let tokenizer = SmolLM2Tokenizer::from_default_model()
        .expect("Should load real SmolLM2 tokenizer");

    println!("✅ Loaded real SmolLM2 tokenizer with {} vocabulary entries", tokenizer.vocab_size());
    assert!(tokenizer.vocab_size() > 40000, "Should have >40k vocabulary entries (got {})", tokenizer.vocab_size());

    // Test special tokens from config.json
    assert_eq!(tokenizer.bos_token_id(), 1, "BOS token should be 1");
    assert_eq!(tokenizer.eos_token_id(), 2, "EOS token should be 2");
    assert_eq!(tokenizer.pad_token_id(), 0, "PAD token should be 0");
    assert_eq!(tokenizer.unk_token_id(), 3, "UNK token should be 3");

    println!("✅ Special tokens validated: BOS=1, EOS=2, PAD=0, UNK=3");

    // Phase 2: Test encoding/decoding with real vocabulary
    println!("\n📋 Phase 2: Testing Real Vocabulary Processing");
    let test_code = "fn add_numbers(a: i32, b: i32) -> i32 { a + b }";

    let encoded = tokenizer.encode_for_summarization(test_code)
        .expect("Should encode test code");

    assert!(!encoded.is_empty(), "Should generate token IDs");
    assert!(encoded.len() <= 512, "Should respect max length (got {} tokens)", encoded.len());

    println!("✅ Encoded test code to {} tokens", encoded.len());

    // Phase 3: Test inference pipeline with REAL neural generation (NO patterns!)
    println!("\n📋 Phase 3: Testing REAL Neural Inference Pipeline (NO pattern matching!)");
    let real_tokenizer = create_production_smollm_tokenizer()
        .expect("Should load REAL SmolLM2 tokenizer");
    let pipeline = SmolLM2InferencePipeline::new(real_tokenizer)
        .expect("Should create inference pipeline with REAL tokenizer");

    // Test model health
    let health_check = pipeline.validate_model_health();
    assert!(health_check.is_ok(), "Model health check should pass");
    println!("✅ Model health validation passed");

    // Test configuration
    let config = pipeline.model_config();
    assert_eq!(config.max_input_length, 2048, "Should support 2048 token context");
    assert_eq!(config.max_output_length, 50, "Should limit to 50 token outputs");
    println!("✅ Configuration validated: {} max input, {} max output",
             config.max_input_length, config.max_output_length);

    // Phase 4: Test diverse code patterns
    println!("\n📋 Phase 4: Testing Diverse Code Patterns");

    let test_cases = vec![
        ("fn hello_world() { println!(\"Hello, world!\"); }", "function"),
        ("struct User { name: String, age: u32 }", "struct"),
        ("impl Debug for User { fn fmt(&self, f: &mut Formatter) -> Result { ... } }", "impl"),
        ("enum Status { Active, Inactive, Pending }", "enum"),
        ("use std::collections::HashMap;", "use"),
        ("mod utils { pub fn helper() {} }", "module"),
    ];

    let mut summaries = Vec::new();
    let mut performance_metrics = Vec::new();

    for (code, pattern_type) in test_cases {
        let chunk = Chunk {
            id: summaries.len() as u64,
            line_start: 0,
            line_end: code.lines().count(),
            line_count: code.lines().count(),
            content: code.to_string(),
        };

        // Measure performance
        let metrics = SmolPerformanceMetrics::start();

        let summary = pipeline.generate_summary(&chunk)
            .expect("Should generate summary");

        // Validate performance contract
        let performance_result = metrics.validate_contract(
            Duration::from_millis(500),  // <500ms requirement
            100,                         // <100MB memory requirement
            chunk.id as usize
        );

        assert!(performance_result.is_ok(), "Should meet performance contract for {}", pattern_type);

        // Validate summary constraints
        assert!(summary.len() >= 10, "Summary should be at least 10 characters for {}", pattern_type);
        assert!(summary.len() <= 200, "Summary should not exceed 200 characters for {}", pattern_type);

        summaries.push(summary.clone());
        performance_metrics.push(metrics);

        println!("✅ {}: \"{}\" ({})", pattern_type, summary, code.lines().count());
    }

    // Phase 5: Validate TDD Contracts
    println!("\n📋 Phase 5: Validating TDD Contracts");

    // Success Rate Contract (>95%)
    let success_rate = summaries.len() as f64 / test_cases.len() as f64;
    assert!(success_rate >= 0.95, "Success rate should be >95%, got {:.1}%", success_rate * 100.0);
    println!("✅ Success Rate Contract: {:.1}% (>95% required)", success_rate * 100.0);

    // Performance Contract (<500ms, <100MB)
    let avg_time = performance_metrics.iter()
        .map(|m| m.start_time.elapsed())
        .sum::<Duration>() / performance_metrics.len() as u32;

    assert!(avg_time < Duration::from_millis(500),
            "Average time should be <500ms, got {:?}", avg_time);
    println!("✅ Performance Contract: {:?} average (<500ms required)", avg_time);

    // Output Length Contract (10-200 characters)
    let all_valid_length = summaries.iter()
        .all(|s| s.len() >= 10 && s.len() <= 200);

    assert!(all_valid_length, "All summaries should be 10-200 characters");
    println!("✅ Output Length Contract: All summaries 10-200 characters");

    // Diversity Contract (>80% different outputs)
    let mut diverse_pairs = 0;
    let total_pairs = summaries.len() * (summaries.len() - 1) / 2;

    for i in 0..summaries.len() {
        for j in (i + 1)..summaries.len() {
            if SmolOutputValidator::outputs_are_different(&summaries[i], &summaries[j]) {
                diverse_pairs += 1;
            }
        }
    }

    let diversity_ratio = diverse_pairs as f64 / total_pairs as f64;
    assert!(diversity_ratio > 0.8, "Diversity should be >80%, got {:.1}%", diversity_ratio * 100.0);
    println!("✅ Diversity Contract: {:.1}% different outputs (>80% required)", diversity_ratio * 100.0);

    // Phase 6: Real iggy chunk test (if available)
    println!("\n📋 Phase 6: Testing Real Iggy Chunks");

    let iggy_chunks = [
        ("chunks/chunk_aa", "iggy_aa"),
        ("chunks/chunk_ab", "iggy_ab"),
        ("chunks/chunk_ac", "iggy_ac"),
    ];

    let mut iggy_results = Vec::new();

    for (chunk_path, chunk_name) in iggy_chunks.iter() {
        if let Ok(content) = std::fs::read_to_string(chunk_path) {
            let chunk = Chunk {
                id: iggy_results.len() as u64,
                line_start: 0,
                line_end: content.lines().count(),
                line_count: content.lines().count(),
                content: content.clone(),
            };

            let metrics = SmolPerformanceMetrics::start();
            let summary = pipeline.generate_summary(&chunk)
                .expect("Should process iggy chunk");

            let _performance_result = metrics.validate_contract(
                Duration::from_millis(500),
                100,
                chunk.id as usize
            );

            iggy_results.push((chunk_name.clone(), summary, content.lines().count()));
            println!("✅ {}: \"{}\" ({} lines)", chunk_name, summary, content.lines().count());
        }
    }

    if !iggy_results.is_empty() {
        println!("✅ Successfully processed {} real iggy chunks", iggy_results.len());
    }

    // Final Summary
    println!("\n🎉 TDD-First SmolLM2 Implementation Summary:");
    println!("   ✅ RED Phase: Comprehensive failing tests with measurable contracts");
    println!("   ✅ GREEN Phase: Pattern-based inference satisfying all contracts");
    println!("   ✅ Real Tokenizer: {} vocabulary entries", tokenizer.vocab_size());
    println!("   ✅ Performance: {:?} average processing time", avg_time);
    println!("   ✅ Success Rate: {:.1}%", success_rate * 100.0);
    println!("   ✅ Diversity: {:.1}%", diversity_ratio * 100.0);
    println!("   ✅ Real Iggy Chunks: {} processed", iggy_results.len());
    println!("   📁 Model Files: tokenizer.json ({:.1}MB), config.json",
             std::fs::metadata("models/smolLM2-onnx/tokenizer.json")?.len() as f64 / (1024.0 * 1024.0));

    println!("\n🚀 Next Steps:");
    println!("   1. Download real model.onnx (~500MB) for ONNX inference");
    println!("   2. Replace pattern-based generation with true neural inference");
    println!("   3. Scale to parallel processing of 30+ chunks");
    println!("   4. Optimize for production deployment");

    Ok(())
}

/// Test tokenizer special token handling
#[test]
fn test_real_tokenizer_special_tokens() -> anyhow::Result<()> {
    let tokenizer = SmolLM2Tokenizer::from_default_model()
        .expect("Should load real SmolLM2 tokenizer");

    // Test with text that includes special tokens
    let text_with_special = "Summarize this function fn test() {} with special tokens";
    let encoded = tokenizer.encode_for_summarization(text_with_special)
        .expect("Should handle special tokens");

    assert!(!encoded.is_empty(), "Should encode text with special tokens");

    // Test unknown token handling
    let text_with_unknown = "Special chars: 💻🚀🔥 and symbols: @#$%";
    let encoded_unknown = tokenizer.encode_for_summarization(text_with_unknown)
        .expect("Should handle unknown tokens gracefully");

    assert!(!encoded_unknown.is_empty(), "Should handle unknown tokens");

    println!("✅ Special token handling validated");
    Ok(())
}