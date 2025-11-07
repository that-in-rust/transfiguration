#!/usr/bin/env python3
"""
Pre-process CodeT5 vocabulary for easier Rust parsing.
Converts JSON to a simple line-by-line format.
"""

import json

def prepare_vocab():
    print("🔄 Preparing vocabulary for Rust...")

    try:
        # Load JSON vocab
        with open("models/codet5-onnx/vocab.json", "r") as f:
            vocab = json.load(f)

        print(f"   Loaded {len(vocab)} vocabulary entries")

        # Write to simple format: token_id token
        with open("models/codet5-onnx/vocab_simple.txt", "w") as f:
            for token, token_id in vocab.items():
                # Escape newlines and tabs
                escaped_token = token.replace("\n", "\\n").replace("\t", "\\t")
                f.write(f"{token_id} {escaped_token}\n")

        print("   ✅ Vocabulary saved in simple format")

        # Write reverse lookup
        with open("models/codet5-onnx/vocab_reverse.txt", "w") as f:
            for token, token_id in vocab.items():
                escaped_token = token.replace("\n", "\\n").replace("\t", "\\t")
                f.write(f"{escaped_token} {token_id}\n")

        print("   ✅ Reverse vocabulary saved")

        # Write some stats
        print(f"   Vocab size: {len(vocab)}")
        print(f"   Min token ID: {min(vocab.values())}")
        print(f"   Max token ID: {max(vocab.values())}")

        # Check special tokens
        special_tokens = ["<pad>", "<s>", "</s>", "<unk>"]
        print("   Special tokens:")
        for token in special_tokens:
            if token in vocab:
                print(f"     {token}: {vocab[token]}")

        return True

    except Exception as e:
        print(f"❌ Failed: {e}")
        return False

if __name__ == "__main__":
    success = prepare_vocab()
    exit(0 if success else 1)