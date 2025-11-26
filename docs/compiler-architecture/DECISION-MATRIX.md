# Transfiguration Compiler: Executive Decision Matrix

**Last Updated**: November 26, 2025
**Status**: Strategic Roadmap Complete

---

## Quick Decision Framework

### If Your Timeline Is...

| Timeline | Choose | Risk | Expected Outcome | Cost |
|----------|--------|------|------------------|------|
| **1-3 months** | Analysis-Only (Parseltongue) | Very Low | Offline analysis tools | $10K-$30K |
| **3-6 months** | Salsa + Redb Hybrid | Low | 10-50× incremental speedup | $50K-$150K |
| **6-12 months** | Hybrid rustc Integration | Medium | 50-100× incremental speedup | $300K-$500K |
| **12-18 months** | Full CompGraph | Medium | 100-250× incremental speedup | $500K-$750K |

---

## Technology Comparison (1-Page Summary)

### Database Options

| Technology | Speed | Maturity | Risk | Best For |
|------------|-------|----------|------|----------|
| **Salsa + Redb** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | 🟢 LOW | **Production (RECOMMENDED)** |
| CozoDB | ⭐⭐⭐⭐ | ⭐⭐ | 🔴 HIGH | Research only |
| Kuzu | ⭐⭐⭐⭐ | ⭐⭐⭐ | 🟡 MEDIUM | Graph analytics |
| Differential Dataflow | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | 🟡 MEDIUM | Distributed systems |
| SQLite | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | 🟢 LOW | Small projects (<100K LOC) |
| **CompGraph (Custom)** | ⭐⭐⭐⭐⭐ | N/A | 🟡 MEDIUM | **Production (Ultimate Goal)** |

---

## The Winning Strategy

```
┌─────────────────────────────────────────────────┐
│ PHASE 1: Analysis-Only (1-2 months)            │
│ ✓ COMPLETE: Parseltongue v0.9                  │
│ Validate: Graph queries useful?                 │
└──────────────┬──────────────────────────────────┘
               │
               ↓
┌─────────────────────────────────────────────────┐
│ PHASE 2: Salsa + Redb (3-6 months)             │
│ Goal: Prove 10× incremental speedup            │
│ Team: 1-2 engineers                             │
│ Cost: $50K-$150K                                │
│ Gate: Is it ≥10× faster? → If NO, stop here    │
└──────────────┬──────────────────────────────────┘
               │
               ↓
┌─────────────────────────────────────────────────┐
│ PHASE 3: Hybrid rustc Integration (6-9 months) │
│ Goal: Function-level dependencies              │
│ Team: 3-5 engineers                             │
│ Cost: $300K-$500K                               │
│ Gate: Passes rustc tests? → If NO, freeze      │
└──────────────┬──────────────────────────────────┘
               │
               ↓
┌─────────────────────────────────────────────────┐
│ PHASE 4: CompGraph (12-18 months)              │
│ Goal: 100-250× incremental speedup             │
│ Team: 2-3 engineers                             │
│ Cost: $500K-$750K                               │
│ Result: Production compiler                     │
└─────────────────────────────────────────────────┘
```

---

## Critical Risks (Top 5)

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| **1. Write performance bottleneck** | MEDIUM-HIGH | VERY HIGH | Benchmark Week 2, fallback to Salsa |
| **2. CozoDB bus factor** | MEDIUM | HIGH | Use for prototype only, build CompGraph |
| **3. Algorithms don't fit Datalog** | MEDIUM | HIGH | Hybrid approach (Datalog + Rust) |
| **4. <10× speedup** | MEDIUM | CRITICAL | Gate at Week 8, pivot if fails |
| **5. Debugging Datalog queries** | HIGH | HIGH | 30% time on test infrastructure |

---

## Go/No-Go Gates

### Gate 1: POC (Week 4)
- ✅ Parse 10K LOC → CozoDB/Salsa
- ✅ 5 queries work correctly
- ✅ Generate working binary
- ✅ Write perf <100ms per 10K batch

**Decision**: GO if all pass, NO-GO if write perf >1000ms

---

### Gate 2: Performance (Week 8)
- ✅ Incremental <2sec on 100K LOC
- ✅ Memory <1GB
- ✅ Queries <10ms
- ✅ ≥10× faster than rustc incremental

**Decision**: GO if ≥10× faster, NO-GO if <10× → Pivot to analysis-only

---

### Gate 3: Correctness (Month 4)
- ✅ Passes rustc test suite
- ✅ Error messages correct
- ✅ No silent miscompilation
- ✅ 1000+ codebases match rustc output

**Decision**: GO if correct, NO-GO if unfixable bugs → Freeze at current phase

---

### Gate 4: Production (Month 12-18)
- ✅ Compiles Chromium-scale
- ✅ 100× faster incremental
- ✅ Memory <50% of rustc
- ✅ Full ecosystem integration

**Decision**: GO → Ship, NO-GO → Extend timeline

---

## Recommended Immediate Actions (Week 1-2)

| Week | Action | Owner | Output |
|------|--------|-------|--------|
| **1** | Benchmark CozoDB write perf | Eng1 | Accept/reject CozoDB |
| **1** | Design CompGraph schema | Eng2 | DB schema document |
| **2-4** | Build Salsa + Redb POC | Eng1+2 | Working incremental compiler |
| **3-4** | Set up differential testing | Eng1 | Test harness (100+ codebases) |

---

## Key Numbers to Remember

### Performance Targets

| Metric | Current (rustc) | Target (Transfiguration) | Multiplier |
|--------|----------------|-------------------------|------------|
| **Incremental build (1% change)** | 20-60 seconds | <2 seconds | **10-30×** |
| **Memory peak** | 5-10 GB | <1 GB | **5-10×** |
| **Query latency** | N/A (in-memory) | <10ms | N/A |
| **Cold start (cached)** | 90 seconds | <5 seconds | **18×** |

### Cost Estimates

| Phase | Duration | Team | Cost |
|-------|----------|------|------|
| Phase 1 (Analysis) | 1-2 months | 1-2 | $10K-$30K |
| Phase 2 (Salsa+Redb) | 3-6 months | 1-2 | $50K-$150K |
| Phase 3 (Hybrid rustc) | 6-9 months | 3-5 | $300K-$500K |
| Phase 4 (CompGraph) | 12-18 months | 2-3 | $500K-$750K |

---

## What to Build First (Priority Order)

| Priority | Component | Why | Timeline |
|----------|-----------|-----|----------|
| **P0** | Salsa query system | Foundation for incrementality | 2-4 weeks |
| **P0** | Redb persistence | Cross-session caching | 2-3 weeks |
| **P0** | Fingerprinting | Detects what changed | 1-2 weeks |
| **P1** | Red-green algorithm | Minimize recompilation | 3-4 weeks |
| **P1** | Name resolution | First phase to migrate | 4-6 weeks |
| **P2** | Dependency tracking | Function-level granularity | 4-6 weeks |
| **P2** | Provenance tracking | "Why recompile?" explanation | 3-4 weeks |
| **P3** | Time-travel queries | Build comparison | 3-4 weeks |

---

## Industry Precedent

**What production compilers actually use:**

| Compiler | Storage | Why Not CozoDB/Neo4j? |
|----------|---------|----------------------|
| rustc | Salsa (in-memory) + fingerprints | Too slow for hot path |
| Bazel | Custom content-addressed cache | Domain-specific = 10-100× faster |
| Buck2 | Custom graph + SQLite | Control over performance |
| TypeScript | In-memory incremental | Sub-100ms latency required |
| GCC/Clang | No persistence | Fast enough without |

**Conclusion**: Build domain-specific (CompGraph), don't use general-purpose.

---

## Red Flags (Abandon If...)

| Condition | Action |
|-----------|--------|
| ❌ CozoDB write >100ms per 10K batch | Use Salsa + Redb instead |
| ❌ Incremental <5× faster than rustc | Stop, not worth effort |
| ❌ Memory >80% of rustc | Minimal benefit, abandon |
| ❌ Correctness issues unfixable | Freeze at current phase |
| ❌ Performance cliffs at scale | Re-architect or stop |

---

## Success Criteria

**The project succeeds if:**
- ✅ Incremental builds ≥10× faster (with benchmarks)
- ✅ Memory ≤50% of rustc
- ✅ Correctness = rustc (differential testing on 1000+ codebases)
- ✅ Error messages as good as rustc
- ✅ Ecosystem integration (Cargo, IDEs work)

**Timeline**: 18 months to full success
**Investment**: $500K-$750K
**Risk**: MEDIUM (manageable)

---

## Final Recommendation

**START WITH SALSA + REDB** (Option B)

**Why:**
- ✅ Lowest risk (proven in rust-analyzer)
- ✅ Fastest path to value (3-6 months)
- ✅ 10-50× incremental speedup (validated)
- ✅ Can migrate to CompGraph later

**Then:**
- Measure at Week 8
- If ≥10× faster → Continue to Phase 3
- If <10× faster → Pivot to analysis-only

**Don't:**
- ❌ Bet everything on CozoDB (bus factor)
- ❌ Skip benchmarks (no claims without data)
- ❌ Force everything into Datalog (hybrid is OK)

---

**VERDICT: PROCEED with phased approach.**

The technology is sound. The risks are manageable. The benefits are transformative.

**Next Action**: Benchmark CozoDB write performance (Week 1).
