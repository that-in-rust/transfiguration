#!/usr/bin/env python3
"""
Convert CodeT5-small from safetensors to ONNX format for Rust inference.
"""

import os
import sys
import torch
from transformers import AutoTokenizer, AutoModelForSeq2SeqLM

def convert_codet5_to_onnx():
    """Convert CodeT5-small to ONNX format"""

    print("🔄 Converting CodeT5-small to ONNX...")
    print("=" * 50)

    model_name = "Salesforce/codet5-small"

    try:
        # Load tokenizer and model
        print("📥 Loading model and tokenizer...")
        tokenizer = AutoTokenizer.from_pretrained("models/codet5-small")
        model = AutoModelForSeq2SeqLM.from_pretrained("models/codet5-small")

        print("✅ Model loaded successfully")

        # Create models directory for ONNX
        os.makedirs("models/codet5-small-onnx", exist_ok=True)

        # Prepare dummy input
        print("🔧 Preparing dummy input...")
        dummy_input_text = "def hello_world():\n    print('Hello, world!')"
        inputs = tokenizer(dummy_input_text, return_tensors="pt", max_length=512, truncation=True)

        print(f"   Input shape: {inputs['input_ids'].shape}")

        # For encoder-decoder model, we need decoder inputs
        decoder_input_ids = torch.tensor([[model.config.decoder_start_token_id]])
        decoder_attention_mask = torch.ones_like(decoder_input_ids)

        print(f"   Decoder input shape: {decoder_input_ids.shape}")

        # Export to ONNX
        print("⚙️  Exporting to ONNX...")

        # Create output path
        onnx_path = "models/codet5-small-onnx/model.onnx"

        # Export the model with encoder-decoder inputs
        torch.onnx.export(
            model,
            (inputs['input_ids'], inputs['attention_mask'], decoder_input_ids, decoder_attention_mask),
            onnx_path,
            input_names=['input_ids', 'attention_mask', 'decoder_input_ids', 'decoder_attention_mask'],
            output_names=['logits'],
            dynamic_axes={
                'input_ids': {0: 'batch_size', 1: 'sequence'},
                'attention_mask': {0: 'batch_size', 1: 'sequence'},
                'decoder_input_ids': {0: 'batch_size', 1: 'decoder_sequence'},
                'decoder_attention_mask': {0: 'batch_size', 1: 'decoder_sequence'},
                'logits': {0: 'batch_size', 1: 'decoder_sequence', 2: 'vocab_size'}
            },
            opset_version=14,  # Use compatible ONNX version
            do_constant_folding=True,
            verbose=False
        )

        print(f"✅ Model exported to {onnx_path}")

        # Verify ONNX model
        print("🔍 Verifying ONNX model...")
        import onnx
        onnx_model = onnx.load(onnx_path)
        onnx.checker.check_model(onnx_model)
        print("✅ ONNX model verification passed")

        # Check file size
        size_mb = os.path.getsize(onnx_path) / (1024 * 1024)
        print(f"📊 ONNX model size: {size_mb:.1f} MB")

        # Test ONNX model with ort
        print("🧪 Testing ONNX model with ORT...")
        try:
            import ort
            import numpy as np

            # Create ONNX Runtime session
            ort_session = ort.InferenceSession(onnx_path, providers=['CPUExecutionProvider'])

            # Prepare input
            ort_inputs = {
                'input_ids': inputs['input_ids'].numpy(),
                'attention_mask': inputs['attention_mask'].numpy(),
                'decoder_input_ids': decoder_input_ids.numpy(),
                'decoder_attention_mask': decoder_attention_mask.numpy()
            }

            # Run inference
            ort_outputs = ort_session.run(None, ort_inputs)
            logits = ort_outputs[0]

            print(f"   Output shape: {logits.shape}")
            print(f"   Output type: {logits.dtype}")

            # Get predicted tokens
            predicted_ids = torch.argmax(torch.tensor(logits), dim=-1)
            predicted_text = tokenizer.decode(predicted_ids[0], skip_special_tokens=True)

            print(f"   Input:  {dummy_input_text}")
            print(f"   Output: {predicted_text}")

            print("✅ ONNX model working correctly!")

        except ImportError:
            print("⚠️  onnxruntime not available for testing")
        except Exception as e:
            print(f"⚠️  ONNX Runtime test failed: {e}")

        # Save tokenizer for Rust
        print("💾 Saving tokenizer for Rust...")
        tokenizer.save_pretrained("models/codet5-small-onnx")

        print("\n🎉 CodeT5-small ONNX conversion completed!")
        print("   ONNX model: models/codet5-small-onnx/model.onnx")
        print("   Tokenizer:  models/codet5-small-onnx/")
        print("   Ready for Rust integration")

        return True

    except Exception as e:
        print(f"❌ Conversion failed: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = convert_codet5_to_onnx()
    sys.exit(0 if success else 1)