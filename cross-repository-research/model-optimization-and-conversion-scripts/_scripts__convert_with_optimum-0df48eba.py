#!/usr/bin/env python3
"""
Convert CodeT5-small to ONNX using the optimum library.
This should handle the complex encoder-decoder architecture properly.
"""

import os
import sys

def convert_with_optimum():
    """Convert using optimum library"""

    print("🔄 Converting CodeT5-small to ONNX with Optimum...")
    print("=" * 55)

    try:
        from optimum.onnxruntime import ORTModelForSeq2SeqLM
        from optimum.onnxruntime.configuration import AutoQuantizationConfig
        from transformers import AutoTokenizer

        model_name = "Salesforce/codet5-small"
        onnx_path = "models/codet5-small-onnx"

        print(f"📥 Loading {model_name}...")

        # Load model and tokenizer from our local files
        model = ORTModelForSeq2SeqLM.from_pretrained(
            "models/codet5-small",
            export=True,
            provider="CPUExecutionProvider"
        )

        tokenizer = AutoTokenizer.from_pretrained("models/codet5-small")

        print("✅ Model loaded and exported to ONNX")

        # Save the ONNX model
        model.save_pretrained(onnx_path)
        tokenizer.save_pretrained(onnx_path)

        print(f"✅ Model saved to {onnx_path}")

        # Check file sizes
        onnx_files = os.listdir(onnx_path)
        total_size = 0
        for file in onnx_files:
            if os.path.isfile(os.path.join(onnx_path, file)):
                size = os.path.getsize(os.path.join(onnx_path, file))
                total_size += size
                print(f"   📄 {file}: {size / (1024*1024):.1f} MB")

        print(f"   📁 Total size: {total_size / (1024*1024):.1f} MB")

        # Test the ONNX model
        print("\n🧪 Testing ONNX model...")
        test_code = "def hello_world():\n    print('Hello, world!')"

        # Tokenize input
        inputs = tokenizer(test_code, return_tensors="pt", max_length=512, truncation=True)

        # Generate output
        with torch.no_grad():
            outputs = model.generate(**inputs, max_length=50, num_beams=1)

        decoded = tokenizer.decode(outputs[0], skip_special_tokens=True)
        print(f"   Input:  {test_code}")
        print(f"   Output: {decoded}")

        print("\n🎉 CodeT5-small ONNX conversion completed!")
        print("   Ready for Rust integration")

        return True

    except ImportError as e:
        print(f"❌ Import error: {e}")
        print("   Please install: pip install optimum[onnxruntime]")
        return False

    except Exception as e:
        print(f"❌ Conversion failed: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = convert_with_optimum()
    sys.exit(0 if success else 1)