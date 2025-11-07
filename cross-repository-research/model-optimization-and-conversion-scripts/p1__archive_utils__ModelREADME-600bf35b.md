# Model Setup and Conversion Guide

This document provides reproducible instructions for setting up and converting the CodeT5 model to ONNX format for use with the dobby-subagent-code-summarizer.

## Prerequisites

```bash
pip install torch transformers optimum onnx onnxruntime
```

## Model Conversion Steps

### 1. Download CodeT5-small Model
```bash
cd scripts
python download_codet5_final.py
```

### 2. Convert to ONNX Format
```bash
python convert_codet5_to_onnx.py
```

### 3. Verify ONNX Model
```bash
python -c "import onnxruntime as ort; sess = ort.InferenceSession('models/codet5-small-onnx/model.onnx'); print('Model loaded successfully')"
```

## Model Files Structure

After conversion, the model files should be organized as:
```
models/
├── codet5-small-onnx/
│   ├── config.json
│   ├── model.onnx
│   ├── tokenizer.json
│   ├── tokenizer_config.json
│   ├── vocab.json
│   └── merges.txt
└── codet5-onnx/
    ├── config.json
    ├── simple_model.onnx
    ├── tokenizer.json
    ├── tokenizer_config.json
    ├── vocab.json
    └── merges.txt
```

## Usage in Rust

The converted ONNX models are used by the `dobby-subagent-code-summarizer` crate through the `ort` (ONNX Runtime) binding.

## Troubleshooting

- Ensure ONNX version compatibility between conversion and runtime
- Check that tokenizer files match the model version
- Verify model input/output shapes match the Rust implementation