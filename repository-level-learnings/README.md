# Repository-Level Learnings

This section contains per-project knowledge, experiments, and evolution documentation.

## Projects

### parseltongue-code-to-database
Code analysis tool that extracts structural information from codebases and stores it in CozoDB for temporal reasoning.

**Key learnings:**
- Architectural evolution strategies
- When to stop optimizing decisions
- Interface Structure Graph (ISG) format development
- Minto Pyramid application to technical documentation

### dobby-database-to-summary-pipeline
Database-to-summary pipeline using CozoDB + Candle RS for local LLM inference.

**Key learnings:**
- Cozo + Candle integration research
- Model experimentation and optimization
- Summary generation approaches

### pensieve-local-llm-server
Local LLM server for running Claude Code locally with zero API costs and full privacy.

**Status:** Research and planning phase

### room-of-requirement-claude-code-rust
Rust implementation of Claude Code CLI tool, focusing on performance and security.

**Status:** Initial planning and architecture design

### campfire-rust-chat-app
Rust rewrite of Basecamp's Campfire chat application.

**Status:** Exploration phase

## How to Use This Section

Each project folder contains:
- **Experiments and research**: Failed and successful attempts
- **Architecture decisions**: ADRs and evolution documentation
- **Lessons learned**: What worked, what didn't, what we'd do differently
- **Historical validation data**: Test runs and verification results

Before starting work on any of these projects, review the corresponding folder to understand past context and avoid repeating mistakes.
