#!/usr/bin/env python3
"""
Convert CodeT5-small to ONNX format with int8 quantization
Target: ~80MB total size for CPU-optimized inference
"""

import os
from transformers import AutoTokenizer, AutoModelForSeq2SeqLM
from optimum.onnxruntime import ORTModelForSeq2SeqLM, ORTQuantizer
from optimum.onnxruntime.configuration import AutoQuantizationConfig

def main():
    print("🔍 Converting CodeT5-small to ONNX format...")

    model_id = "Salesforce/codet5-small"
    output_dir = "../models/codet5-small-onnx"

    # Ensure output directory exists
    os.makedirs(output_dir, exist_ok=True)

    print(f"📦 Loading model: {model_id}")

    # Load tokenizer and model
    tokenizer = AutoTokenizer.from_pretrained(model_id)
    model = AutoModelForSeq2SeqLM.from_pretrained(model_id)

    print("🔄 Converting to ONNX format...")

    # Convert to ONNX using Optimum
    ort_model = ORTModelForSeq2SeqLM.from_pretrained(
        model_id,
        export=True,
        feature="seq2seq-lm-with-past"
    )

    print("💾 Saving ONNX model...")
    ort_model.save_pretrained(output_dir)
    tokenizer.save_pretrained(output_dir)

    print("⚖️  Applying int8 quantization...")

    # Quantize to int8 for size reduction
    quantizer = ORTQuantizer.from_pretrained(model_id)
    dqconfig = AutoQuantizationConfig.avx512_vnni(
        is_static=False,
        per_channel=False,
        reduce_range=True
    )

    quantized_dir = f"{output_dir}-quantized"
    os.makedirs(quantized_dir, exist_ok=True)

    quantizer.quantize(
        save_dir=quantized_dir,
        quantization_config=dqconfig
    )

    # Copy tokenizer to quantized directory
    tokenizer.save_pretrained(quantized_dir)

    print(f"✅ Conversion complete!")
    print(f"📁 Models saved to: {output_dir}")
    print(f"📁 Quantized models saved to: {quantized_dir}")

    # Check file sizes
    onnx_files = []
    for root, dirs, files in os.walk(output_dir):
        for file in files:
            if file.endswith('.onnx'):
                filepath = os.path.join(root, file)
                size_mb = os.path.getsize(filepath) / (1024 * 1024)
                onnx_files.append((file, size_mb))
                print(f"📊 {file}: {size_mb:.1f} MB")

    total_size = sum(size for _, size in onnx_files)
    print(f"📦 Total ONNX size: {total_size:.1f} MB")

    # Check quantized sizes
    quant_files = []
    for root, dirs, files in os.walk(quantized_dir):
        for file in files:
            if file.endswith('.onnx'):
                filepath = os.path.join(root, file)
                size_mb = os.path.getsize(filepath) / (1024 * 1024)
                quant_files.append((file, size_mb))
                print(f"📊 Quantized {file}: {size_mb:.1f} MB")

    quant_total_size = sum(size for _, size in quant_files)
    print(f"📦 Total quantized size: {quant_total_size:.1f} MB")

    if quant_total_size < 100:
        print("✅ Quantized model meets <100MB target!")
    else:
        print("⚠️  Quantized model exceeds <100MB target")

    return quantized_dir

if __name__ == "__main__":
    main()