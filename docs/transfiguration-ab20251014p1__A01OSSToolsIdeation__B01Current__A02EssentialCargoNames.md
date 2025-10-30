# Essential Cargo Names for Parseltongue Project

## Research Foundation & Analysis

### Archive Inspiration & Naming Patterns
Based on analysis of `/Users/amuldotexe/Projects/transfiguration/A01OSSToolsIdeation/B02ARCHIVE/Insp09_ActiveWorkspaces_StrategicInitiatives/`, key naming patterns identified:

- **Descriptive Functionality**: Names clearly communicate tool purpose
- **Snake Case Convention**: Consistent with Rust cargo naming standards
- **Modular Structure**: Multiple related tools forming a cohesive system
- **Technical Precision**: Avoids ambiguity while remaining accessible

### Modern Rust Naming Research Findings

**From Rust API Guidelines:**
- Must use alphanumeric characters with `-` or `_` only
- Should follow `snake_case` for binary names
- Avoid reserved names and special Windows names
- Prefer descriptive, purpose-driven names

**From GitHub Topic Analysis:**
- Popular `local-llm` and `rust` combinations show `rust-local-llm` patterns
- Code analysis tools favor `rust-code-` and `tool` suffixes
- MCP servers use consistent `mcp-rust` naming conventions

**From Cargo Documentation:**
- Names must be valid identifiers when imported
- Use hyphens for multi-word names
- Names should be discoverable and searchable

## Essential 4-Word Cargo Names

### 1. local-llm-orchestrator-daemon
**Function**: Main ONNX Runtime orchestrator (Dobby equivalent)
**Rationale**: 
- Clear indication of local LLM management
- "Orchestrator" conveys multi-session coordination
- "Daemon" suggests background service capability
- Follows `local-llm-{purpose}-{mode}` pattern found in archives
- Aligns with popular `rust-local-llm` tooling patterns

### 2. code-graph-builder-stream
**Function**: ISG-code-chunk-streamer + ingest-chunks-to-CodeGraph
**Rationale**:
- Combines two related tools into cohesive name
- "Graph Builder" clearly indicates code graph construction
- "Stream" reflects streaming nature of code processing
- Uses `code-{domain}-{function}-{mode}` pattern
- Maintains consistency with Rust code analysis tool naming

### 3. interface-summary-generator
**Function**: LLM-powered interface summarization tool
**Rationale**:
- Clear focus on interface-level summaries
- "Generator" indicates automation capability
- Consistent with Rust tooling naming patterns
- Follows `interface-{domain}-{function}` structure
- Mirrors archive naming conventions for analysis tools

### 4. code-simulator-sorcerer
**Function**: Advanced code simulation and change orchestration
**Rationale**:
- "Simulator" clearly indicates testing capability
- "Sorcerer" adds magical/advanced connotation fitting for complex reasoning
- Maintains code-focused naming consistency
- Uses `code-{function}-{mode}` pattern
- Evokes the transformation aspect of Parseltongue

### 5. rust-preflight-verifier
**Function**: Rust-specific compilation and test verification
**Rationale**:
- "Preflight" indicates safety/before-flight checks
- "Verifier" emphasizes validation purpose
- Explicitly identifies Rust focus
- Follows `{language}-{stage}-{function}` pattern
- Aligns with verification-focused tool naming conventions

### 6. final-change-applicator
**Function**: Applies final code changes with safety checks
**Rationale**:
- "Final" indicates end-stage processing
- "Applicator" clearly conveys code application
- Maintains action-oriented naming
- Uses `change-{function}-{mode}` pattern
- Emphasizes responsibility of irreversible changes

### 7. code-slate-protocol
**Function**: Clean-slate protocol for system reset and commit
**Rationale**:
- "Slate" evokes clean slate/reset functionality
- "Protocol" indicates standardized process
- Follows `code-{concept}-{function}` pattern
- Abstract naming suggests systematic approach
- Consistent with archive naming for technical protocols

### 8. mcp-server-integration
**Function**: MCP server integration for Claude Code plugin
**Rationale**:
- Explicitly identifies MCP server purpose
- "Integration" indicates connectivity role
- Uses `mcp-{domain}-{function}` standard pattern
- Clear technical specification
- Aligns with MCP ecosystem naming conventions

## Implementation Strategy

### Primary Components
1. **local-llm-orchestrator-daemon** - Core runtime infrastructure
2. **code-graph-builder-stream** - Code ingestion and indexing
3. **interface-summary-generator** - LLM-based analysis
4. **code-simulator-sorcerer** - Advanced simulation engine

### Secondary Components
5. **rust-preflight-verifier** - Safety validation
6. **final-change-applicator** - Change application
7. **code-slate-protocol** - System reset management
8. **mcp-server-integration** - Plugin connectivity

### Naming Convention Alignment

**Pattern 1: `{domain}-{function}-{mode}`**
- `code-graph-builder-stream`
- `code-simulator-sorcerer`
- `rust-preflight-verifier`

**Pattern 2: `local-{concept}-{function}-{mode}`**
- `local-llm-orchestrator-daemon`
- `local-code-analyzer-tool`

**Pattern 3: `{concept}-{function}-{domain}`**
- `interface-summary-generator`
- `code-slate-protocol`
- `mcp-server-integration`

### Research Validation

The selected names demonstrate:
- **Technical Precision**: Each name clearly communicates functionality
- **Rust Compliance**: All follow snake_case and identifier rules
- **Discoverability**: Easy to search and recognize in ecosystem
- **Consistency**: Unified naming approach across all components
- **Modern Patterns**: Align with current Rust tooling naming trends

## Usage Examples

```bash
# Core infrastructure
cargo build local-llm-orchestrator-daemon
cargo run local-llm-orchestrator-daemon -- --config default.toml

# Code processing
cargo run code-graph-builder-stream -- --repo /path/to/rust-project
cargo run interface-summary-generator -- --cozo cozo://isg

# Simulation and verification
cargo run code-simulator-sorcerer -- --micro-prd "add logging"
cargo run rust-preflight-verifier -- --check build-and-test

# Application and management
cargo run final-change-applicator -- --apply --verify
cargo run code-slate-protocol -- --reset --commit
cargo run mcp-server-integration -- --serve --port 8080
```

Each cargo name serves as a distinct, purpose-built component while contributing to the unified Parseltongue ecosystem, designed for reliability, maintainability, and seamless integration within the Rust code analysis landscape.

