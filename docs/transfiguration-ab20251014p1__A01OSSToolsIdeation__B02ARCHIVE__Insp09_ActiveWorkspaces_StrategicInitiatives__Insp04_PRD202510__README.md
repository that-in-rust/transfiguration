# Parseltongue Component Builder

A minimalistic, JSON-driven build system for creating modular Parseltongue components.

## Quick Start

```bash
# Build all components
python build.py build

# Build specific component
python build.py build system-detective

# See all available variants
python build.py list-variants

# See iteration strategies for different approaches
python build.py show-strategies

# Run quality checks
python build.py quality-check
```

## Architecture

The system uses `build-manifest.json` to define:

- **Components**: 7 modular crates (system-detective, universal-isg, etc.)
- **Dependencies**: Clear build order and dependencies between components
- **Variants**: Multiple implementation approaches per component
- **Iteration Strategies**: Easy ways to swap implementations (ISG approaches, storage backends, etc.)

## Key Features

### 🔧 Modularity
Each component is a separate Rust crate with clear interfaces.

### 🔄 Easy Iteration
Change one line in JSON to try different approaches:

```json
{
  "iterationStrategies": {
    "isg-approaches": [
      {
        "name": "tree-sitter-enhanced",
        "modifications": {
          "universal-isg": {
            "features": ["tree-sitter", "hybrid"]
          }
        }
      }
    ]
  }
}
```

### 🚀 Fast Development
- JSON-driven configuration
- Automated dependency management
- Built-in testing and quality checks

## Component Overview

| Component | Purpose | Key Variants |
|-----------|---------|--------------|
| `system-detective` | Hardware/service detection | basic, full, minimal |
| `universal-isg` | Semantic understanding | rust-only, tree-sitter, hybrid |
| `graph-atlas` | Graph + KV storage | cozodb-only, with-caching |
| `intent-parser` | Natural language parsing | anthropic-only, multi-provider |
| `delta-reasoner` | Confident change management | conservative, aggressive |
| `validation-loop` | AI self-validation | single-iteration, multi-iteration |
| `universal-tui-chat` | Conversational UI | basic, rich, intelligent |

## File Structure

```
Insp04_PRD202510/
├── build-manifest.json    # Component definitions and build steps
├── build.py              # Build automation script
└── src/                  # Generated Rust crates
    ├── system-detective/
    ├── universal-isg/
    ├── graph-atlas/
    ├── intent-parser/
    ├── delta-reasoner/
    ├── validation-loop/
    └── universal-tui-chat/
```

## Example: Trying Different ISG Approaches

```bash
# See available ISG approaches
python build.py show-strategies

# The JSON shows 4 ISG variants:
# - rust-analyzer-only: Pure Rust AST/HIR
# - tree-sitter-enhanced: Better parsing with tree-sitter
# - hybrid: Combined approach
# - incremental: Only compute differences

# Modify build-manifest.json to use tree-sitter-enhanced:
# "universal-isg": { "features": ["tree-sitter", "hybrid"] }

# Rebuild with new approach
python build.py build universal-isg
```

This approach enables **rapid iteration** on different architectural decisions while maintaining clean modularity.
