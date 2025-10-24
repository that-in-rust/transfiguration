#!/usr/bin/env python3
"""
Working CodeT5-small ONNX conversion with proper input handling
"""

import os
import torch
from transformers import AutoTokenizer, T5ForConditionalGeneration

def main():
    print("🔍 Converting CodeT5-small to ONNX...")

    model_id = "Salesforce/codet5-small"
    output_dir = "../models/codet5-working"
    os.makedirs(output_dir, exist_ok=True)

    print(f"📦 Loading model: {model_id}")

    # Load tokenizer and model
    tokenizer = AutoTokenizer.from_pretrained(model_id)
    model = T5ForConditionalGeneration.from_pretrained(model_id)
    model.eval()

    print("🔄 Preparing dummy input for export...")

    # Create proper input for T5 model (both encoder and decoder inputs)
    dummy_text = "summarize: fn test() { return 42; }"
    inputs = tokenizer(dummy_text, return_tensors="pt", padding=True, truncation=True, max_length=512)

    print("📊 Input shapes:")
    print(f"  input_ids: {inputs['input_ids'].shape}")
    print(f"  attention_mask: {inputs['attention_mask'].shape}")

    print("💾 Exporting encoder to ONNX...")

    # Export encoder part first (simpler and more useful)
    with torch.no_grad():
        encoder_outputs = model.encoder(
            input_ids=inputs['input_ids'],
            attention_mask=inputs['attention_mask'],
            return_dict=False
        )

    print(f"📊 Encoder output shape: {encoder_outputs[0].shape}")

    # Export encoder to ONNX
    torch.onnx.export(
        lambda x, a: model.encoder(input_ids=x, attention_mask=a),
        (inputs['input_ids'], inputs['attention_mask']),
        os.path.join(output_dir, "codet5-encoder.onnx"),
        input_names=['input_ids', 'attention_mask'],
        output_names=['encoder_hidden_states'],
        dynamic_axes={
            'input_ids': {0: 'batch', 1: 'sequence'},
            'attention_mask': {0: 'batch', 1: 'sequence'},
            'encoder_hidden_states': {0: 'batch', 1: 'sequence', 2: 'hidden'}
        },
        opset_version=12,
        do_constant_folding=True,
    )

    # Save tokenizer
    tokenizer.save_pretrained(output_dir)

    print("✅ Encoder conversion complete!")

    # Check file size
    onnx_path = os.path.join(output_dir, "codet5-encoder.onnx")
    if os.path.exists(onnx_path):
        size_mb = os.path.getsize(onnx_path) / (1024 * 1024)
        print(f"📊 Encoder size: {size_mb:.1f} MB")

        if size_mb < 100:
            print("✅ Model meets <100MB target!")
        else:
            print("⚠️  Model exceeds <100MB target")

        # Get total size including tokenizer files
        total_size = 0
        for root, dirs, files in os.walk(output_dir):
            for file in files:
                total_size += os.path.getsize(os.path.join(root, file))

        total_mb = total_size / (1024 * 1024)
        print(f"📦 Total size including tokenizer: {total_mb:.1f} MB")

if __name__ == "__main__":
    main()