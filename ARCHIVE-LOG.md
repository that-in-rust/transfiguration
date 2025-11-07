# Archive Activity Log

This file tracks what content was archived, when, and why.

---

## 2025-11-07: Major Reorganization & File Shortening

### Actions Taken

**1. File Renaming (386 files)**
- Shortened 353 files with names longer than 30 characters
- Format: `last30chars-8charhash.ext`
- Created `FILE-MAPPING.json` (100KB) to track all original names
- Script: `00-archive-management-scripts/shorten_filename.py`

**2. Directory Restructure**
- Created `repository-level-learnings/` parent folder to group all per-repo archives
- Organized 6 top-level categories instead of 10+ scattered folders
- Created domain-specific subdirectories with meaningful 3+ word names

**3. Content Organization (386 files classified)**

From: `T99unclassifiedDocs/`

To organized structure:

**Repository-Level Learnings:**
- `parseltongue-code-to-database/architectural-evolution-strategy/` - Parseltongue architecture evolution docs, ISG format, Minto Pyramid strategy
- `dobby-database-to-summary-pipeline/research-and-experimentation/` - Cozo+Candle research, model experimentation
- `pensieve-local-llm-server/` - Created for future content
- `room-of-requirement-claude-code-rust/` - Created for future content
- `campfire-rust-chat-app/` - Created for future content

**Cross-Repository Research:**
- `ide-tooling-ecosystem-analysis/kiro-behavioral-analysis/` - Kiro IDE analysis, VSCode fork catalog, behavioral patterns, configuration analysis, UI structure analysis
- `rust-performance-optimization-patterns/` - RustRover analysis, Rust idioms, performance research
- `tree-sitter-and-semantic-chunking/` - Tree-sitter research and semantic chunking strategies
- `llm-and-ai-integration-patterns/` - AI UX patterns, service integration, subagent patterns
- `model-optimization-and-conversion-scripts/` - Model conversion scripts, ONNX export, quantization
- `product-requirement-documents-archive/` - PRD documents from October 2024 ideation phase

**Cross-Repository Learnings:**
- `tdd-and-architecture-principles/` - TDD practices, architecture decision patterns

**Archived Projects:**
- `oss-tools-poc-october-2024/` - Complete OSS tools proof of concept from October 2024, git artifacts, old transfiguration branch files

### Reason for Archiving

**Primary Goal:** Prevent information loss and create searchable institutional memory

**Problems Solved:**
1. Long, unwieldy filenames (150+ characters) reduced to readable format
2. Scattered research documents organized by domain and project
3. No clear structure → Clear hierarchy with meaningful folder names
4. Historical context preserved for future reference

### Statistics
- **Files processed:** 386
- **Files renamed:** 353
- **Folders created:** 15+
- **Old structure removed:** T99unclassifiedDocs (empty)
- **Mapping entries:** 353 in FILE-MAPPING.json

---

## Future Additions

When adding new content, update this log with:
- Date
- What was moved/archived
- Where it came from
- Why it was archived
- Any relevant context for future reference
