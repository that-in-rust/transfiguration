#!/usr/bin/env python3
"""
Create a simple ONNX model for testing the parallel processing pipeline.
This creates a basic model that can handle 1x28x28 inputs and produces outputs.
"""

import numpy as np
import onnx
from onnx import helper, TensorProto
from onnx.numpy_helper import from_array

def create_simple_model():
    """Create a simple ONNX model for testing"""

    # Create a simple neural network: Input -> Linear -> ReLU -> Output
    # Input: [batch, 1, 28, 28] -> flatten to [batch, 784]
    # Output: [batch, 10] (like MNIST classification)

    # Define input
    input_tensor = helper.make_tensor_value_info('input', TensorProto.FLOAT, [1, 1, 28, 28])

    # Define output
    output_tensor = helper.make_tensor_value_info('output', TensorProto.FLOAT, [1, 10])

    # Create weight matrix (784 -> 10)
    weight_data = np.random.randn(784, 10).astype(np.float32)
    weight_tensor = from_array(weight_data, 'weight')

    # Create bias (10)
    bias_data = np.zeros(10, dtype=np.float32)
    bias_tensor = from_array(bias_data, 'bias')

    # Create MatMul operation
    matmul_node = helper.make_node(
        'MatMul',
        inputs=['input_flat', 'weight'],
        outputs=['matmul_output']
    )

    # Create Add operation
    add_node = helper.make_node(
        'Add',
        inputs=['matmul_output', 'bias'],
        outputs=['output']
    )

    # Create Flatten operation to convert [1,1,28,28] to [1,784]
    flatten_node = helper.make_node(
        'Flatten',
        inputs=['input'],
        outputs=['input_flat'],
        axis=1
    )

    # Create graph
    graph = helper.make_graph(
        nodes=[flatten_node, matmul_node, add_node],
        name='simple_model',
        inputs=[input_tensor],
        outputs=[output_tensor],
        initializer=[weight_tensor, bias_tensor]
    )

    # Create model
    model = helper.make_model(graph)
    model.opset_import[0].version = 9  # Use older version for compatibility

    # Ensure the model has the right IR version
    model.ir_version = 7  # ONNX IR version 7 for compatibility

    # Save model
    onnx.save(model, 'models/simple_model.onnx')
    print("✅ Created simple ONNX model: models/simple_model.onnx")

    # Verify model
    try:
        onnx.checker.check_model(model)
        print("✅ Model validation passed")
    except Exception as e:
        print(f"❌ Model validation failed: {e}")
        return False

    # Check file size
    import os
    size_mb = os.path.getsize('models/simple_model.onnx') / (1024 * 1024)
    print(f"📊 Model size: {size_mb:.2f} MB")

    return True

if __name__ == "__main__":
    success = create_simple_model()
    exit(0 if success else 1)