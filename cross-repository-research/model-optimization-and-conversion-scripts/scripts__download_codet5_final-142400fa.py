#!/usr/bin/env python3
"""
Final approach to download CodeT5-small with proper configuration.
"""

import os
import sys
import torch
from transformers import AutoTokenizer, AutoModelForSeq2SeqLM

def download_codet5():
    """Download CodeT5-small with proper configuration"""

    print("🚀 Downloading CodeT5-small...")
    print("=" * 40)

    model_name = "Salesforce/codet5-small"

    try:
        # Create models directory
        os.makedirs("models/codet5-small", exist_ok=True)

        print(f"📥 Downloading {model_name}...")

        # Download model and tokenizer
        tokenizer = AutoTokenizer.from_pretrained(model_name)
        model = AutoModelForSeq2SeqLM.from_pretrained(model_name)

        print("✅ Model and tokenizer downloaded")

        # Test model before saving
        test_code = "def hello_world():\n    print('Hello, world!')"
        inputs = tokenizer(test_code, return_tensors="pt", max_length=512, truncation=True)

        with torch.no_grad():
            outputs = model.generate(**inputs, max_length=50, num_beams=1)

        decoded = tokenizer.decode(outputs[0], skip_special_tokens=True)
        print(f"   Test - Input:  {test_code}")
        print(f"   Test - Output: {decoded}")

        # Save model and tokenizer properly
        print("\n💾 Saving model and tokenizer...")
        model.save_pretrained("models/codet5-small")
        tokenizer.save_pretrained("models/codet5-small")

        # Verify the save worked
        print("🔍 Verifying saved model...")
        saved_model = AutoModelForSeq2SeqLM.from_pretrained("models/codet5-small")
        saved_tokenizer = AutoTokenizer.from_pretrained("models/codet5-small")

        # Test with saved model
        test_inputs = saved_tokenizer(test_code, return_tensors="pt", max_length=512, truncation=True)
        with torch.no_grad():
            test_outputs = saved_model.generate(**test_inputs, max_length=50, num_beams=1)

        test_decoded = saved_tokenizer.decode(test_outputs[0], skip_special_tokens=True)
        print(f"   Verify - Input:  {test_code}")
        print(f"   Verify - Output: {test_decoded}")

        # Show model info
        print(f"\n📊 Model Info:")
        print(f"   Model type: {type(saved_model).__name__}")
        print(f"   Vocab size: {saved_tokenizer.vocab_size}")
        print(f"   Max length: {saved_tokenizer.model_max_length}")

        # Check file sizes
        model_dir = "models/codet5-small"
        total_size = 0
        for file in sorted(os.listdir(model_dir)):
            if os.path.isfile(os.path.join(model_dir, file)):
                size = os.path.getsize(os.path.join(model_dir, file))
                total_size += size
                print(f"   📄 {file}: {size / (1024*1024):.1f} MB")

        print(f"   📁 Total size: {total_size / (1024*1024):.1f} MB")

        print("\n🎉 CodeT5-small download and verification completed!")
        print("   Ready for Rust integration")

        return True

    except Exception as e:
        print(f"❌ Failed: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = download_codet5()
    sys.exit(0 if success else 1)