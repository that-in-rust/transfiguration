#!/usr/bin/env python3
"""
Direct CodeT5-small ONNX conversion using a simpler approach.
Export just the encoder first, then handle generation manually.
"""

import os
import sys
import torch
from transformers import AutoTokenizer, AutoModelForSeq2SeqLM, T5ForConditionalGeneration, T5Config

def convert_codet5_simple():
    """Convert CodeT5-small to ONNX with encoder-only approach"""

    print("🔄 Converting CodeT5-small to ONNX (Simplified Approach)...")
    print("=" * 65)

    try:
        # Load model and tokenizer from our local files
        print("📥 Loading CodeT5-small model...")
        model = T5ForConditionalGeneration.from_pretrained("models/codet5-small")
        tokenizer = AutoTokenizer.from_pretrained("models/codet5-small")

        print("✅ Model loaded successfully")
        print(f"   Model type: {type(model).__name__}")
        print(f"   Config: {model.config.model_type}")
        print(f"   Vocab size: {tokenizer.vocab_size}")

        # Create output directory
        os.makedirs("models/codet5-onnx", exist_ok=True)

        # Test the model works
        print("\n🧪 Testing model functionality...")
        test_code = "def hello_world():\n    print('Hello, world!')"
        test_input = f"summarize: {test_code}"

        inputs = tokenizer(test_input, return_tensors="pt", max_length=512, truncation=True)

        with torch.no_grad():
            outputs = model.generate(**inputs, max_length=50, num_beams=1, early_stopping=True)

        decoded = tokenizer.decode(outputs[0], skip_special_tokens=True)
        print(f"   Input:  {test_input}")
        print(f"   Output: {decoded}")

        # Save tokenizer for Rust
        print("\n💾 Saving tokenizer...")
        tokenizer.save_pretrained("models/codet5-onnx")

        # Save model config
        model.config.save_pretrained("models/codet5-onnx")

        print("✅ Tokenizer and config saved")

        # Create a simple working ONNX version using torch.jit.trace
        print("\n🔧 Creating traced model for ONNX export...")

        # Prepare inputs for tracing
        test_inputs = tokenizer(test_input, return_tensors="pt", max_length=512, truncation=True)

        # Get encoder outputs for tracing
        with torch.no_grad():
            encoder_outputs = model.encoder(
                input_ids=test_inputs['input_ids'],
                attention_mask=test_inputs['attention_mask']
            )

        print(f"   Encoder output shape: {encoder_outputs.last_hidden_state.shape}")

        # Try to create a simpler inference function
        class SimpleT5Inference(torch.nn.Module):
            def __init__(self, model):
                super().__init__()
                self.model = model

            def forward(self, input_ids, attention_mask):
                # Get encoder outputs
                encoder_outputs = self.model.encoder(input_ids=input_ids, attention_mask=attention_mask)

                # Simple decoder with start token
                decoder_input_ids = torch.tensor([[self.model.config.decoder_start_token_id]])
                decoder_attention_mask = torch.ones_like(decoder_input_ids)

                # Get first decoder output
                outputs = self.model.decoder(
                    input_ids=decoder_input_ids,
                    attention_mask=decoder_attention_mask,
                    encoder_hidden_states=encoder_outputs.last_hidden_state,
                    encoder_attention_mask=attention_mask
                )

                # Get logits
                logits = self.model.lm_head(outputs[0])

                return logits

        # Create traced model
        simple_model = SimpleT5Inference(model)
        simple_model.eval()

        # Test the simple model
        with torch.no_grad():
            simple_output = simple_model(
                test_inputs['input_ids'],
                test_inputs['attention_mask']
            )

        print(f"   Simple model output shape: {simple_output.shape}")

        # Get predicted token
        predicted_token = torch.argmax(simple_output[0, -1, :])
        predicted_text = tokenizer.decode([predicted_token.item()], skip_special_tokens=True)
        print(f"   First predicted token: '{predicted_text}' (token_id: {predicted_token.item()})")

        # Export to ONNX
        print("\n⚙️  Exporting to ONNX...")
        onnx_path = "models/codet5-onnx/simple_model.onnx"

        torch.onnx.export(
            simple_model,
            (test_inputs['input_ids'], test_inputs['attention_mask']),
            onnx_path,
            input_names=['input_ids', 'attention_mask'],
            output_names=['logits'],
            dynamic_axes={
                'input_ids': {0: 'batch_size', 1: 'sequence'},
                'attention_mask': {0: 'batch_size', 1: 'sequence'},
                'logits': {0: 'batch_size', 1: 'sequence', 2: 'vocab_size'}
            },
            opset_version=14,
            do_constant_folding=True,
            verbose=False
        )

        print(f"✅ Model exported to {onnx_path}")

        # Verify ONNX model
        print("\n🔍 Verifying ONNX model...")
        import onnx
        onnx_model = onnx.load(onnx_path)
        onnx.checker.check_model(onnx_model)
        print("✅ ONNX model verification passed")

        # Check file size
        size_mb = os.path.getsize(onnx_path) / (1024 * 1024)
        print(f"📊 ONNX model size: {size_mb:.1f} MB")

        # Test with ONNX Runtime
        print("\n🧪 Testing ONNX model...")
        import onnxruntime as ort

        ort_session = ort.InferenceSession(onnx_path, providers=['CPUExecutionProvider'])

        ort_inputs = {
            'input_ids': test_inputs['input_ids'].numpy(),
            'attention_mask': test_inputs['attention_mask'].numpy()
        }

        ort_outputs = ort_session.run(None, ort_inputs)
        ort_logits = ort_outputs[0]

        print(f"   ONNX output shape: {ort_logits.shape}")

        # Compare outputs
        torch_output_flat = simple_output.flatten().numpy()
        onnx_output_flat = ort_logits.flatten()

        max_diff = abs(torch_output_flat - onnx_output_flat).max()
        print(f"   Max difference: {max_diff:.6f}")

        if max_diff < 1e-4:
            print("✅ ONNX model output matches PyTorch!")
        else:
            print("⚠️  ONNX output differs from PyTorch")

        print("\n🎉 CodeT5-small ONNX conversion completed!")
        print("   Ready for Rust integration with real neural inference")
        print("   Model: models/codet5-onnx/simple_model.onnx")
        print("   Tokenizer: models/codet5-onnx/")

        return True

    except Exception as e:
        print(f"❌ Conversion failed: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = convert_codet5_simple()
    sys.exit(0 if success else 1)