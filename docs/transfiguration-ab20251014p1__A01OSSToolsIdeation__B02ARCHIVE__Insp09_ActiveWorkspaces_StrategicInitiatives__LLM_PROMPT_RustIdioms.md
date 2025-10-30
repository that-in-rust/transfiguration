# LLM Operating Prompt: Rust Idioms Curation Workflow

You are an LLM operating in this repository to curate Rust idioms from large source notes into a structured master document with progress tracking. Follow this procedure precisely for each 2,000-line chunk.

Objectives
- Extract 5 high-signal idioms per 2,000-line chunk from Insp06RustNotes/RustIdioms_trun_rust_idiom_large.txt
- Append idioms to Insp09_ActiveWorkspaces_StrategicInitiatives/RustIdiomsLong20251021.md (section “A. Curated Idioms”) as A.N entries, continuing numbering
- Update the checklist and Progress Log in the same markdown file
- Keep edits additive and non-destructive

Primary files
- Source: Insp09_ActiveWorkspaces_StrategicInitiatives/Insp06RustNotes/RustIdioms_trun_rust_idiom_large.txt
- Target: Insp09_ActiveWorkspaces_StrategicInitiatives/RustIdiomsLong20251021.md

Chunking
- Process the source in strict 2,000-line ranges (e.g., 46001–48000)
- For each chunk: read, extract, write, and log before moving to the next

Idiom entry template
- Title: A.<number> <concise idiom name>
- Bullets:
  - Use when: one line
  - Context: 1–3 lines with concrete, practical guidance
  - Avoid/Anti-pattern: 1–3 lines of common pitfalls
- Optional micro-example code block(s) using this exact fence format:
  ```rust path=null start=null
  // minimal illustrative snippet
  ```
- Prefer concrete naming, real crate APIs, and short code; avoid long prose

Formatting & style
- CommonMark markdown only
- Code blocks must include language + `path` and `start` metadata (use `path=null start=null` for illustrative snippets)
- Use relative file paths when referencing files
- If you need diagrams, use Mermaid only
- Keep entries concise and practical; avoid repetition

Editing rules
- Never rewrite or reorder existing idioms; only append new A.N items
- Maintain strictly increasing A.N numbering
- Do not modify prior Progress Log rows; only append a new row per chunk
- Keep changes minimal and localized (chunk checklist tick, idioms, progress row)

Checklist update
- In RustIdiomsLong20251021.md, in “Tasks (2000-line mode)”, find the line for the processed chunk (e.g., “Process chunk 46001–48000”) and change `[ ]` to `[x]`

Progress Log update
- Append a new row to the Progress Log table with:
  - Chunk (e.g., 46001–48000)
  - Source lines: always the chunk range
  - Idioms added: 5
  - Target total LOC: current total lines of RustIdiomsLong20251021.md after your edits (compute with `wc -l`)
  - Delta LOC vs prev: difference vs previous total LOC in the table
  - Notes: brief topics covered (comma-separated)
  - Timestamp: YYYY-MM-DD (UTC)

Example Progress Log row
```
|||||||||| 27 | 46001–48000 | 5 | 2520 | 70 | shared-state vs message-passing; Send/Sync rules; atomics ordering; crossbeam-channel; scoped threads | 2025-10-24 |
```
(Indentation level should match surrounding rows; do not alter previous rows.)

Idioms topic selection (examples)
- Concurrency: Send/Sync rules; OnceLock; Mutex poisoning; data races vs race conditions; atomics (Acquire/Release/AcqRel/SeqCst); scoped threads; loom modeling
- Channels: std::sync::mpsc, crossbeam-channel, flume
- Work-stealing: crossbeam-deque injector/worker/stealer patterns
- Lock-free memory: crossbeam-epoch pin/retire patterns
- Embedded: defmt + RTT logging; probe-rs/cargo-flash cycles; linker scripts; target triples
- Data engineering/tooling (if encountered): Polars/DataFusion/Arrow, rdkafka, NATS, Pulsar, serde/postcard

Quality bar
- Each idiom must deliver: actionable guidance (Use when/Context/Avoid) and, when appropriate, a tiny snippet
- Avoid generic advice; prefer concrete API names and patterns
- Keep micro-examples compilable in principle (though illustrative), minimal, and self-contained

Safety & constraints
- Do not delete or move existing content
- Respect the established idiom structure and numbering
- Keep per-chunk additions to exactly 5 idioms unless instructed otherwise

Commit guidance
- By default, prepare changes without committing; only commit/push when explicitly asked
- Commit message format when requested:
  - `chore(rust-idioms): process chunk <START>–<END>, add A.<N>–A.<N+4>`

Operating loop per chunk
1) Read the 2,000-line source range
2) Extract 5 high-signal idioms and draft entries
3) Append to “A. Curated Idioms” as A.N … A.N+4
4) Tick the checklist for the chunk
5) Compute new LOC: `wc -l Insp09_ActiveWorkspaces_StrategicInitiatives/RustIdiomsLong20251021.md`
6) Append a Progress Log row with computed values and brief Notes
7) Stop (await further instruction before proceeding or committing)

Minimal idiom example
```markdown
A.999 Example Idiom Name
- Use when: clear one-liner
- Context: concrete details and trade-offs
- Avoid/Anti-pattern: common pitfalls

```rust path=null start=null
// tiny snippet showing the pattern in context
```
```
