#!/usr/bin/env python3
"""
Download T5-small model and convert to ONNX format for parallel code summarization.
This script will:
1. Download T5-small from HuggingFace
2. Export it to ONNX format
3. Save tokenizer assets
4. Optimize for parallel inference
"""

import os
import sys
from pathlib import Path

def main():
    print("Starting T5-small model download and conversion...")

    # Create models directory
    models_dir = Path("models")
    models_dir.mkdir(exist_ok=True)

    try:
        # Import required libraries
        from optimum.onnxruntime import ORTModelForSeq2SeqLM
        from transformers import AutoTokenizer
        import torch

        model_name = "t5-small"
        print(f"Downloading {model_name} model and tokenizer...")

        # Download tokenizer and save assets
        tokenizer = AutoTokenizer.from_pretrained(model_name)
        tokenizer.save_pretrained(models_dir / "t5-small-tokenizer")
        print(f"Tokenizer saved to {models_dir / 't5-small-tokenizer'}")

        # Download and convert model to ONNX
        print("Converting model to ONNX format...")
        model = ORTModelForSeq2SeqLM.from_pretrained(
            model_name,
            export=True,
            provider="CPUExecutionProvider"
        )

        # Save ONNX model
        onnx_path = models_dir / "t5-small.onnx"
        model.save_pretrained(models_dir / "t5-small-onnx")
        print(f"ONNX model saved to {models_dir / 't5-small-onnx'}")

        # Test the model
        print("Testing model conversion...")
        test_text = "summarize: def hello_world(): print('Hello, World!')"
        inputs = tokenizer(test_text, return_tensors="pt", padding=True, truncation=True, max_length=512)

        # Generate output
        outputs = model.generate(**inputs, max_length=50, num_beams=4, early_stopping=True)
        summary = tokenizer.decode(outputs[0], skip_special_tokens=True)

        print(f"Test successful!")
        print(f"Input: {test_text}")
        print(f"Output: {summary}")

        # Check file sizes
        onnx_files = list((models_dir / "t5-small-onnx").glob("**/*.onnx"))
        total_size = sum(f.stat().st_size for f in onnx_files) / (1024 * 1024)  # MB

        print(f"\nModel files created:")
        for f in onnx_files:
            size_mb = f.stat().st_size / (1024 * 1024)
            print(f"  {f.name}: {size_mb:.1f} MB")

        print(f"Total ONNX model size: {total_size:.1f} MB")

        # Estimate memory usage for 20 parallel instances
        estimated_memory_per_instance = total_size * 3  # Rough estimate: model + activations + overhead
        total_estimated_memory = estimated_memory_per_instance * 20

        print(f"\nMemory estimates for 20 parallel instances:")
        print(f"  Per instance: ~{estimated_memory_per_instance:.1f} MB")
        print(f"  Total: ~{total_estimated_memory:.1f} MB ({total_estimated_memory/1024:.2f} GB)")

        if total_estimated_memory < 9 * 1024:  # 9GB limit
            print("✅ Memory usage is within the 9GB limit!")
        else:
            print("⚠️  Memory usage may exceed the 9GB limit")

        return True

    except ImportError as e:
        print(f"Missing required library: {e}")
        print("Please install: pip install optimum[onnxruntime] transformers")
        return False
    except Exception as e:
        print(f"Error during model conversion: {e}")
        return False

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)