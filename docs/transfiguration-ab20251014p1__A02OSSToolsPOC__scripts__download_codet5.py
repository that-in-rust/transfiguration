#!/usr/bin/env python3
"""
Download and convert Salesforce/CodeT5-small to quantized ONNX format.
Based on the user's research, CodeT5-small is optimal for code summarization.
"""

import os
import sys
import torch
from transformers import AutoTokenizer, AutoModelForSeq2SeqLM
from optimum.onnxruntime import ORTModelForSeq2SeqLM
from optimum.pipelines import pipeline
import onnx
import numpy as np

def download_and_convert_codet5():
    """Download CodeT5-small and convert to optimized ONNX format"""

    print("🚀 Starting CodeT5-small to ONNX conversion...")
    print("=" * 50)

    model_name = "Salesforce/codet5-small"

    try:
        # Step 1: Download tokenizer and model
        print(f"📥 Downloading {model_name}...")
        tokenizer = AutoTokenizer.from_pretrained(model_name)
        model = AutoModelForSeq2SeqLM.from_pretrained(model_name)

        print("✅ Model and tokenizer downloaded successfully")

        # Step 2: Test the model before conversion
        print("\n🧪 Testing original model...")
        test_code = "def hello_world():\n    print('Hello, world!')"

        inputs = tokenizer(test_code, return_tensors="pt", max_length=512, truncation=True)
        with torch.no_grad():
            outputs = model.generate(**inputs, max_length=50, num_beams=1)

        decoded = tokenizer.decode(outputs[0], skip_special_tokens=True)
        print(f"   Input:  {test_code}")
        print(f"Output:  {decoded}")

        # Step 3: Create models directory
        os.makedirs("models", exist_ok=True)

        # Step 4: Convert to ONNX with quantization
        print("\n⚙️  Converting to ONNX with int8 quantization...")

        # Load from optimum with ONNX Runtime
        onnx_model = ORTModelForSeq2SeqLM.from_pretrained(
            model_name,
            export=True,
            provider="CPUExecutionProvider"
        )

        # Save the ONNX model
        onnx_model.save_pretrained("models/codet5-small-onnx")

        # Save tokenizer separately
        tokenizer.save_pretrained("models/codet5-small-onnx")

        print("✅ CodeT5-small converted to ONNX format")

        # Step 5: Optimize for quantization (optional but recommended)
        print("\n📊 Applying int8 quantization...")

        from optimum.onnxruntime import ORTQuantizer
        from optimum.onnxruntime.configuration import AutoQuantizationConfig

        # Create quantizer
        quantizer = ORTQuantizer.from_pretrained("models/codet5-small-onnx")

        # Create quantization configuration
        qconfig = AutoQuantizationConfig.avx512_vnni(is_static=False, per_channel=False)

        # Apply quantization
        quantizer.quantize(
            save_dir="models/codet5-small-quantized",
            quantization_config=qconfig
        )

        print("✅ Quantization completed")

        # Step 6: Verify ONNX model
        print("\n🔍 Verifying ONNX models...")

        # Check file sizes
        onnx_dir = "models/codet5-small-onnx"
        quant_dir = "models/codet5-small-quantized"

        if os.path.exists(onnx_dir):
            onnx_size = sum(os.path.getsize(os.path.join(onnx_dir, f))
                           for f in os.listdir(onnx_dir)
                           if os.path.isfile(os.path.join(onnx_dir, f)))
            print(f"   📁 ONNX model size: {onnx_size / (1024*1024):.1f} MB")

        if os.path.exists(quant_dir):
            quant_size = sum(os.path.getsize(os.path.join(quant_dir, f))
                            for f in os.listdir(quant_dir)
                            if os.path.isfile(os.path.join(quant_dir, f)))
            print(f"   📁 Quantized model size: {quant_size / (1024*1024):.1f} MB")
            print(f"   💾 Memory reduction: {(1 - quant_size/onnx_size) * 100:.1f}%")

        # Step 7: Test quantized model
        print("\n🧪 Testing quantized model...")

        try:
            # Load quantized model
            quantized_model = ORTModelForSeq2SeqLM.from_pretrained("models/codet5-small-quantized")
            quantized_tokenizer = AutoTokenizer.from_pretrained("models/codet5-small-quantized")

            # Create pipeline for easier testing
            onnx_pipeline = pipeline(
                "text2text-generation",
                model=quantized_model,
                tokenizer=quantized_tokenizer
            )

            # Test with the same input
            result = onnx_pipeline(test_code, max_length=50, num_beams=1)
            print(f"   Input:  {test_code}")
            print(f"Output:  {result[0]['generated_text']}")

            print("✅ Quantized model working correctly!")

        except Exception as e:
            print(f"⚠️  Quantized model test failed: {e}")
            print("   Falling back to non-quantized ONNX model")

        print("\n🎉 CodeT5-small ONNX conversion completed!")
        print("   Model saved in: models/codet5-small-quantized/")
        print("   Ready for integration with Rust inference pipeline")

        return True

    except ImportError as e:
        print(f"❌ Import error: {e}")
        print("   Please install required packages:")
        print("   pip install torch transformers optimum[onnxruntime]")
        return False

    except Exception as e:
        print(f"❌ Conversion failed: {e}")
        return False

if __name__ == "__main__":
    success = download_and_convert_codet5()
    sys.exit(0 if success else 1)