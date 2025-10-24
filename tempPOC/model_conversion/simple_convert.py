#!/usr/bin/env python3
"""
Simple CodeT5-small conversion using basic optimum
"""

import os
from transformers import AutoTokenizer, T5ForConditionalGeneration
import torch

def main():
    print("🔍 Converting CodeT5-small to ONNX...")

    model_id = "Salesforce/codet5-small"
    output_dir = "../models/codet5-simple"
    os.makedirs(output_dir, exist_ok=True)

    print(f"📦 Loading model: {model_id}")

    # Load tokenizer and model
    tokenizer = AutoTokenizer.from_pretrained(model_id)
    model = T5ForConditionalGeneration.from_pretrained(model_id)

    print("🔄 Preparing dummy input for export...")

    # Create dummy input for ONNX export
    dummy_text = "summarize: fn example() { return 42; }"
    inputs = tokenizer(dummy_text, return_tensors="pt", padding=True, truncation=True, max_length=512)

    print("💾 Exporting to ONNX...")

    # Export to ONNX
    torch.onnx.export(
        model,
        (inputs['input_ids'], inputs['attention_mask']),
        os.path.join(output_dir, "codet5-small.onnx"),
        input_names=['input_ids', 'attention_mask'],
        output_names=['logits'],
        dynamic_axes={
            'input_ids': {0: 'batch', 1: 'sequence'},
            'attention_mask': {0: 'batch', 1: 'sequence'},
            'logits': {0: 'batch', 1: 'sequence'}
        },
        opset_version=12,
        do_constant_folding=True,
    )

    # Save tokenizer
    tokenizer.save_pretrained(output_dir)

    print("✅ Conversion complete!")

    # Check file size
    onnx_path = os.path.join(output_dir, "codet5-small.onnx")
    if os.path.exists(onnx_path):
        size_mb = os.path.getsize(onnx_path) / (1024 * 1024)
        print(f"📊 Model size: {size_mb:.1f} MB")

        if size_mb < 100:
            print("✅ Model meets <100MB target!")
        else:
            print("⚠️  Model exceeds <100MB target")

if __name__ == "__main__":
    main()