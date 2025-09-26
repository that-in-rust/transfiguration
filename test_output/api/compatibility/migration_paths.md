# Extension Migration Paths for Kiro → Rust/WASM

## Overview

This document outlines migration strategies for different types of extensions when transitioning from Electron-based Kiro to Rust/WASM implementation.

## Migration Categories

### 1. Fully Compatible Extensions
- **Criteria**: Use only standard VS Code APIs, no Kiro-specific features
- **Migration**: Direct compatibility, no changes required
- **Examples**: Standard language extensions, themes, snippets

### 2. Kiro-Specific Extensions Requiring Updates
- **Criteria**: Use Kiro-specific APIs or AWS integrations
- **Migration**: Requires API adaptation layer or reimplementation
- **Examples**: Kiro Agent extension, AWS-integrated extensions

### 3. Extensions Requiring Rust/WASM Ports
- **Criteria**: Performance-critical extensions that could benefit from Rust
- **Migration**: Rewrite core logic in Rust, compile to WASM
- **Examples**: Language servers, heavy computation extensions

## Migration Strategies

### Strategy 1: Compatibility Layer
- Implement Kiro-specific APIs in Rust/WASM version
- Maintain backward compatibility for existing extensions
- Gradual migration path for extension developers

### Strategy 2: Extension Modernization
- Encourage extension developers to update to standard APIs
- Provide migration tools and documentation
- Deprecation timeline for Kiro-specific features

### Strategy 3: Native Rust Extensions
- New extension model using Rust + WASM
- Better performance and security
- Gradual adoption alongside traditional extensions

## Implementation Recommendations

1. **Phase 1**: Implement compatibility layer for critical extensions
2. **Phase 2**: Provide migration tools and documentation
3. **Phase 3**: Introduce native Rust extension model
4. **Phase 4**: Deprecate legacy Kiro-specific APIs

