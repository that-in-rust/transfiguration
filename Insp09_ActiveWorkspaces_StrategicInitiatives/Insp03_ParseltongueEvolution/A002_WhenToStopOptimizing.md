# When to Stop Optimizing: Parseltongue vs. Zed vs. RustRover

*A pragmatic comparison through the "Shreyas Doshi 1000 IQ" lens*

## The Core Question

**If Parseltongue already hits its < 12 ms update / < 1 ms query goals, what—if anything—do Zed or RustRover still do more effectively, and when is it worth copying them?**

## Capability-by-Capability Analysis

| Capability | Parseltongue now (syn + petgraph) | Zed (tree-sitter, Rust) | RustRover (IntelliJ platform) | Who's "more effective"? | Trade-off to adopt |
|---|---|---|---|---|---|
| Incremental parsing | Full-file `syn` reparse (cheap until files > 5k LOC) | True subtree deltas; edit → InputEdit → 10–50× less work | PSI stubs + incremental re-index | Zed, **only for live typing on big files** | +1 lib, +400 LoC; gains taper if files are small |
| Text storage | `String` snapshots | Rope (O(log n) splice) | Gap buffer in IDE | Zed | –30% RSS but extra translation layer |
| FS scanning | `notify` + naïve glob | `.gitignore` filtering | VFS with δ caching | Zed / RustRover | One-day refactor, instant cooler fans |
| Job coalescing | One task per FS event | Bounded worker queue, collapse bursts | Global "dumb mode" throttle | Zed (**much lower jitter**) | Simple; prevents rare 40ms spikes |
| Project model | single-crate / dump | `cargo metadata` graph | full Cargo & workspace model | RustRover (richest) | Needed only for multi-crate queries |
| Cross-lang | Rust + Python PoC | Any tree-sitter grammar (light) | 70+ languages (heavy) | RustRover | Not free; adds 200MiB index per lang |
| Persistent index | JSON snapshot | none (re-parse) | on-disk stub index (cold start ≈ 0s) | RustRover, **if** you open same repo daily | Adds SQLite/LMDB layer; slower writes |
| Telemetry / self-metrics | none | Prom-like counters, debug UI | IDE status bar, usage stats | Zed / RustRover | Near-zero risk, helps CI regressions |
| UI plumbing | CLI & HTML viz | stdio JSON-RPC → editor commands | deep IDE integration | RustRover (rich), Zed (lean) | Thin RPC layer gives 80% of value |

## Net Evaluation

### 1. Zed's parsing & scheduling tricks still win
**When**: You care about keystroke-level latency in *very* large files (>10k LOC) or high-frequency changes (code-gen, format-on-save).

### 2. RustRover's persistent stub index beats everyone
**When**: Cold-start performance matters (no reparse at open), but the price is huge disk/RAM overhead and a JVM—overkill if Parseltongue is a background micro-service.

### 3. Everything else is low-hanging fruit
Smart scanner, metrics, cargo graph have clear upside—even an "already efficient" system benefits.

## Decision Grid: Keep / Consider / Skip

### Keep Status-Quo If:
- Targets are single-crate
- Dev machines ≥ 16GB
- Cold start isn't a pain point

### Consider Zed-Style Increments When:
- Users edit megabyte-scale files
- You plan real-time in-editor hints
- You want to advertise "updates < 3ms" as a differentiator

### Skip RustRover-Grade Index Unless:
- Multi-language, cross-repo analysis is a roadmap item
- You are ready to ship and maintain an embedded key-value store

## Bottom Line

**Parseltongue is already "efficient enough" for today's SLA.**

**Borrow the cheap, low-risk parts:**
- Ignore-aware scanning
- Debounced job queuing
- Cargo metadata integration
- Basic telemetry/metrics

**Only reach for Zed's full incremental pipeline if future UX demands sub-5ms live feedback on huge files.**

## Strategic Insight

This analysis exemplifies a key principle: **knowing when to stop optimizing is as valuable as knowing how to optimize.** 

The temptation to chase theoretical performance improvements can distract from:
- User experience improvements that matter
- Business metrics that drive adoption
- Features that differentiate from competitors

Sometimes "good enough" + shipping is better than "perfect" + never.

---

*Analysis contributed by a friend who understands the difference between engineering for engineering's sake vs. engineering for user value.*