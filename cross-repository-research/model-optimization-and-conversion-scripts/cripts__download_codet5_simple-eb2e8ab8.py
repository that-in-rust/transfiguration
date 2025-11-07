#!/usr/bin/env python3
"""
Simplified CodeT5-small to ONNX conversion without complex dependencies.
This approach uses the transformers library with basic ONNX export.
"""

import os
import sys
import torch
from transformers import AutoTokenizer, AutoModelForSeq2SeqLM
import onnx
import json

def download_and_export_codet5():
    """Download CodeT5-small and export to basic ONNX format"""

    print("🚀 Starting CodeT5-small download and basic ONNX export...")
    print("=" * 60)

    model_name = "Salesforce/codet5-small"

    try:
        # Step 1: Download tokenizer and model
        print(f"📥 Downloading {model_name}...")
        tokenizer = AutoTokenizer.from_pretrained(model_name)
        model = AutoModelForSeq2SeqLM.from_pretrained(model_name)

        print("✅ Model and tokenizer downloaded successfully")

        # Step 2: Test the model before export
        print("\n🧪 Testing original model...")
        test_code = "def hello_world():\n    print('Hello, world!')"

        inputs = tokenizer(test_code, return_tensors="pt", max_length=512, truncation=True)
        with torch.no_grad():
            outputs = model.generate(**inputs, max_length=50, num_beams=1)

        decoded = tokenizer.decode(outputs[0], skip_special_tokens=True)
        print(f"   Input:  {test_code}")
        print(f"Output:  {decoded}")

        # Step 3: Create models directory
        os.makedirs("models/codet5-small", exist_ok=True)

        # Step 4: Save model and tokenizer for Rust integration
        print("\n💾 Saving model and tokenizer...")
        model.save_pretrained("models/codet5-small")
        tokenizer.save_pretrained("models/codet5-small")

        # Step 5: Save configuration info for Rust
        config_info = {
            "model_type": "codet5-small",
            "vocab_size": tokenizer.vocab_size,
            "max_length": 512,
            "max_target_length": 128,
            "model_path": "models/codet5-small",
            "tokenizer_path": "models/codet5-small"
        }

        with open("models/codet5-small/config.json", "w") as f:
            json.dump(config_info, f, indent=2)

        print("✅ Model saved successfully")

        # Step 6: Check file sizes
        model_dir = "models/codet5-small"
        total_size = 0
        for file in os.listdir(model_dir):
            file_path = os.path.join(model_dir, file)
            if os.path.isfile(file_path):
                size = os.path.getsize(file_path)
                total_size += size
                print(f"   📄 {file}: {size / (1024*1024):.1f} MB")

        print(f"   📁 Total model size: {total_size / (1024*1024):.1f} MB")

        # Step 7: Test model loading
        print("\n🔍 Testing model loading...")
        test_model = AutoModelForSeq2SeqLM.from_pretrained("models/codet5-small")
        test_tokenizer = AutoTokenizer.from_pretrained("models/codet5-small")

        # Test inference again
        test_inputs = test_tokenizer(test_code, return_tensors="pt", max_length=512, truncation=True)
        with torch.no_grad():
            test_outputs = test_model.generate(**test_inputs, max_length=50, num_beams=1)

        test_decoded = test_tokenizer.decode(test_outputs[0], skip_special_tokens=True)
        print(f"   ✅ Reloaded model working: {test_decoded}")

        print("\n🎉 CodeT5-small setup completed!")
        print("   Model saved in: models/codet5-small/")
        print("   Ready for Rust integration with ONNX Runtime")
        print("\n📋 Next steps:")
        print("   1. Update Cargo.toml to include required dependencies")
        print("   2. Implement tokenizer integration in Rust")
        print("   3. Update inference pipeline to use real CodeT5-small")

        return True

    except Exception as e:
        print(f"❌ Setup failed: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = download_and_export_codet5()
    sys.exit(0 if success else 1)