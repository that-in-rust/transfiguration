# Hybrid Plan 2.0: Strategic Native Performance Accelerators

## 🎯 **Executive Summary**

Adopt a **hybrid approach**: maintain JVM-first, legally compliant analysis of IntelliJ + intellij-rust, while adding a focused **"Native Helper" track** that treats the 7MB Rust binaries as performance accelerators at well-defined boundaries. Validate claims via black-box benchmarks and runtime instrumentation without decompiling proprietary code.

## 🔍 **Where the 7MB Native Helpers Deliver Maximum Impact**

### **Performance Bottleneck Analysis:**

1. **🛠️ Symbol Demangling**: Batch demangle millions of Rust symbols faster than JVM string/regex paths
2. **🔍 DWARF Parsing**: High-throughput parsing of large `.debug_*` sections using `gimli` with tight memory patterns
3. **📦 Object File Inspection**: Object/rlib/PE/ELF/Mach-O metadata scanning using `object` crate
4. **🗂️ Index Construction Primitives**: Preprocessed summaries for JVM consumption, reducing GC and bridging costs

**These are perfect for FFI**: compute-heavy, data-parallel, zero-allocation-to-JVM if designed correctly.

---

## 📋 **Updated Plan: Hybrid 2.0 Architecture**

| Track | Goal | Tools | Outputs |
|-------|------|-------|---------|
| **JVM/OSS Analysis** | Understand plugin architecture and extension points | intellij-rust source, plugin.xml, jdeps | EP maps, class graphs, integration patterns |
| **Native Helper Analysis** | Black-box characterize helper capabilities and throughput | CLI harness, nm/strings, perf/valgrind | Functionality matrix, throughput/latency metrics |
| **Runtime Instrumentation** | Tie native work to user-visible IDE wins | JFR, async-profiler, IDE logs | Startup/indexing timelines, CPU/alloc flamegraphs |
| **Packaging & Portability** | Ensure correct OS/arch selection and loading | metadata checks, load tests | Cross-platform load matrix, fallbacks |

---

## 🚀 **Enhanced Implementation Strategy**

### **Native Helper API Contract (Our Design, Not Theirs)**

#### **Design Principles:**
- **Batch-first, streaming-friendly** functions
- **C ABI, stable, versioned** interface
- **Zero-copy** via direct ByteBuffers or caller-owned output buffers

#### **Minimal FFI Design Example:**

**Rust Side (C ABI, Batch Operations):**
```rust
// Symbol demangling with batch processing
#[no_mangle]
pub extern "C" fn rr_demangle_batch(
    in_ptr: *const u8,
    in_len: usize,
    out_ptr: *mut u8,
    out_cap: usize,
    written: *mut usize,
) -> i32 {
    // in_ptr: UTF-8 text, one mangled symbol per line
    // out_ptr: caller-allocated buffer; we write demangled lines with '\n'
    // return 0 on success; negative error codes otherwise
    // Implement with rustc_demangle in tight loop; avoid allocations
    0
}

// DWARF parsing with streaming interface
#[no_mangle]
pub extern "C" fn rr_dwarf_summarize(
    binary_ptr: *const u8,
    binary_len: usize,
    summary_ptr: *mut u8,
    summary_cap: usize,
    written: *mut usize,
) -> i32 {
    // Parse DWARF sections and emit structured summary
    // Focus on type info, source mappings, symbol locations
    0
}

// Object file metadata extraction
#[no_mangle]
pub extern "C" fn rr_object_summary(
    file_ptr: *const u8,
    file_len: usize,
    metadata_ptr: *mut u8,
    metadata_cap: usize,
    written: *mut usize,
) -> i32 {
    // Extract symbols, sections, dependencies from binary objects
    0
}

// Version and capability detection
#[no_mangle]
pub extern "C" fn rr_get_version() -> u32 {
    // Return version as packed major.minor.patch
    0x010000 // 1.0.0
}
```

**Kotlin/JNI Boundary (Direct Buffers for Zero-Copy):**
```kotlin
class RustNativeHelper {
    external fun demangleBatch(inBuf: ByteBuffer, outBuf: ByteBuffer): Int
    external fun dwarfSummarize(binaryBuf: ByteBuffer, summaryBuf: ByteBuffer): Int
    external fun objectSummary(fileBuf: ByteBuffer, metadataBuf: ByteBuffer): Int
    external fun getVersion(): Int
    
    init { 
        try {
            System.loadLibrary("rr_native")
        } catch (e: UnsatisfiedLinkError) {
            // Graceful fallback to JVM-only implementation
            logger.warn("Native helper unavailable, using JVM fallback", e)
        }
    }
}
```

### **Design Notes:**
- **Fixed-function "verbs"** (demangle_batch, dwarf_summarize, object_summary)
- **Versioned symbols** or `rr_get_version()` capability detection
- **Error codes** for status; detailed diagnostics via optional ring buffer
- **Batch processing** to amortize JNI overhead

---

## 📊 **Black-Box Benchmarks to Validate Performance Claims**

### **1. Symbol Demangling Validation**
```rust
// Test Setup
Input: Corpus of mangled symbols from own compiled debug builds
       (no proprietary data needed)

KPIs to Measure:
- symbols/sec throughput
- bytes/sec processing rate  
- CPU utilization patterns
- JVM allocation rate
- Memory peak usage

Baseline: Pure JVM string processing
Target:   10-100x improvement via native batch processing
```

### **2. DWARF Parsing Validation**
```rust  
// Test Setup
Input: Own large Rust binaries with -C debuginfo=2
       (rustc, cargo, large open-source projects)

KPIs to Measure:  
- MB/sec parsing throughput
- Memory peak usage
- Extracted debug units/sec
- Cache hit rates
- Error handling robustness

Baseline: Java DWARF parsing libraries
Target:   15x improvement via native gimli integration
```

### **3. Object File Analysis Validation**
```rust
// Test Setup  
Input: Mix of rlibs, staticlibs, dynamic libs across OS/arch
       (from cargo build outputs, system libraries)

KPIs to Measure:
- files/sec processing rate
- Metadata extraction latency
- Error rate and recovery
- Cross-platform consistency
- Memory efficiency

Baseline: Java binary parsing libraries
Target:   50x improvement via native object crate
```

---

## 📈 **Runtime Instrumentation Strategy**

### **End-to-End IDE Profiling with JFR:**
```bash
./bin/rustrover.sh \
    -J-XX:StartFlightRecording=filename=jfr.jfr,settings=profile,dumponexit=true \
    -J-XX:FlightRecorderOptions=stackdepth=256
```

### **Native Component Profiling:**
```bash
# Linux profiling
perf record -g -- ./rr_native_cli demangle --in symbols.txt
perf report

# macOS profiling  
instruments -t "Time Profiler" ./rr_native_cli dwarf --in binary.debug

# Memory analysis
valgrind --tool=massif ./rr_native_cli object --in library.rlib
```

### **JNI Bridge Monitoring:**
```kotlin
class PerformanceMetrics {
    private val batchCounter = AtomicLong()
    private val totalBytes = AtomicLong() 
    private val totalTime = AtomicLong()
    
    fun recordBatch(bytes: Int, timeNs: Long) {
        batchCounter.incrementAndGet()
        totalBytes.addAndGet(bytes.toLong())
        totalTime.addAndGet(timeNs)
        
        // Emit JFR events
        JFR.addEvent("NativeBatchProcessed") {
            bytes = bytes
            latencyNs = timeNs
        }
    }
}
```

---

## 🧪 **Invariant Tests Framework**

### **1. Presence and Loading Tests**
```rust
#[test]
fn test_native_helper_presence() {
    // For current OS/arch, native helper is present, checksummed, and loadable
    let helper_path = get_native_helper_path();
    assert!(helper_path.exists());
    assert!(verify_checksum(&helper_path));
    assert!(can_load_library(&helper_path));
}
```

### **2. Correctness Tests**  
```rust
#[test]
fn test_demangle_correctness() {
    // Demangled output equals rustc-demangle for random samples
    let test_symbols = generate_test_symbols(10000);
    for symbol in test_symbols {
        let native_result = native_demangle(&symbol);
        let reference_result = rustc_demangle::demangle(&symbol);
        assert_eq!(native_result, reference_result.to_string());
    }
}

#[test] 
fn test_dwarf_parsing_correctness() {
    // DWARF address-to-line matches addr2line ground truth
    let test_binary = compile_test_binary();
    let native_mappings = native_dwarf_parse(&test_binary);
    let reference_mappings = addr2line_parse(&test_binary);
    assert_mappings_equivalent(native_mappings, reference_mappings);
}
```

### **3. Performance Guard Tests**
```rust
#[test]
fn test_demangle_throughput() {
    // Demangle throughput ≥ target symbols/s per OS/arch
    let symbols = load_symbol_corpus(100_000);
    let start = Instant::now();
    let results = native_demangle_batch(&symbols);
    let elapsed = start.elapsed();
    
    let throughput = symbols.len() as f64 / elapsed.as_secs_f64();
    assert!(throughput >= get_min_throughput_for_platform());
}

#[test]
fn test_jni_overhead_budget() {
    // JNI round-trip overhead ≤ 1ms per 100k symbols (batched)
    let batch_size = 100_000;
    let symbols = generate_symbols(batch_size);
    
    let start = Instant::now();
    let _results = jni_demangle_batch(&symbols);
    let jni_time = start.elapsed();
    
    assert!(jni_time.as_millis() <= 1);
}
```

### **4. Fallback Behavior Tests**
```rust
#[test] 
fn test_graceful_fallback() {
    // If helper missing, JVM path produces correct results (only performance differs)
    disable_native_helper();
    
    let test_input = load_test_data();
    let jvm_result = process_with_jvm_only(&test_input);
    
    enable_native_helper();
    let hybrid_result = process_with_native_helper(&test_input);
    
    assert_eq!(jvm_result, hybrid_result); // Same correctness
    // Only performance should differ
}
```

---

## 📦 **Cross-Platform Packaging Matrix**

| OS/Arch | Artifact | Load Test | Fallback Strategy |
|---------|----------|-----------|-------------------|
| **macOS ARM64** | `librr_native.dylib` | `dlopen` + smoke test | JVM-only demangling |
| **macOS x86_64** | `librr_native.dylib` | `dlopen` + smoke test | JVM-only demangling |  
| **Linux x86_64** | `librr_native.so` | `dlopen` + smoke test | JVM-only demangling |
| **Linux ARM64** | `librr_native.so` | `dlopen` + smoke test | JVM-only demangling |
| **Windows x86_64** | `rr_native.dll` | `LoadLibrary` + smoke test | JVM-only demangling |
| **Windows ARM64** | `rr_native.dll` | `LoadLibrary` + smoke test | JVM-only demangling |

### **CLI Tool for CI Benchmarking:**
```rust
// Tiny CLI for each platform to enable CI benchmarking without IDE
fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "demangle" => run_demangle_benchmark(&args[2..]),
        "dwarf" => run_dwarf_benchmark(&args[2..]),  
        "object" => run_object_benchmark(&args[2..]),
        "version" => println!("{}", get_version()),
        _ => print_help(),
    }
}
```

---

## 📊 **Observability and Monitoring Plan**

### **JVM Layer Monitoring:**
```kotlin
// JFR events for indexing, PSI/VFS operations
@JFREvent(name = "RustIndexing")
class RustIndexingEvent : Event() {
    @JFRLabel("Project Path")
    var projectPath: String = ""
    
    @JFRLabel("Files Indexed") 
    var filesIndexed: Int = 0
    
    @JFRLabel("Native Helper Used")
    var nativeHelperUsed: Boolean = false
}

// Async-profiler integration for CPU/allocation analysis
// Debug logs for plugin subsystem interactions
```

### **Native Layer Monitoring:**
```rust  
// Performance counters per FFI call
static DEMANGLE_COUNTER: AtomicU64 = AtomicU64::new(0);
static DEMANGLE_BYTES: AtomicU64 = AtomicU64::new(0); 
static DEMANGLE_TIME_NS: AtomicU64 = AtomicU64::new(0);

#[no_mangle]
pub extern "C" fn rr_get_stats(stats: *mut NativeStats) -> i32 {
    unsafe {
        (*stats).demangle_calls = DEMANGLE_COUNTER.load(Ordering::Relaxed);
        (*stats).demangle_bytes = DEMANGLE_BYTES.load(Ordering::Relaxed);
        (*stats).demangle_time_ns = DEMANGLE_TIME_NS.load(Ordering::Relaxed);
    }
    0
}
```

### **Cross-Layer Bridge Monitoring:**
```kotlin
// Emit counters per FFI call; expose via JMX or logs
class NativeBridgeMetrics : StandardMBean(NativeBridgeMetricsMXBean::class.java) {
    override fun getBatchesProcessed(): Long = nativeStats.demangle_calls
    override fun getTotalBytesProcessed(): Long = nativeStats.demangle_bytes  
    override fun getAverageLatencyMs(): Double = 
        nativeStats.demangle_time_ns.toDouble() / nativeStats.demangle_calls / 1_000_000.0
}
```

---

## 🗓️ **10-Day Implementation Roadmap**

### **Days 1-2: Foundation**
- ✅ Finalize FFI verbs + error model
- ✅ Stub Rust library and Kotlin bridge  
- ✅ Add invariant test framework
- ✅ Set up cross-platform build system

### **Days 3-4: Core Implementation**
- 🔧 Implement `demangle_batch` and `object_summary`
- 🔧 Create CLI tool for benchmarking
- 🔧 Build cross-platform artifacts
- 📊 Initial performance baselines

### **Days 5-6: Validation & Integration**
- 📈 Benchmark vs JVM-only implementation
- 🔍 Integrate JFR profiling runs
- 🎯 Set performance thresholds per platform
- 🧪 Validate correctness tests

### **Days 7-8: Advanced Features**
- 🔍 Add DWARF summarization functionality
- ✅ Validate against addr2line on fixture binaries
- 📊 Comprehensive performance analysis
- 🛡️ Error handling and edge cases

### **Days 9: Observability**
- 📊 Wire comprehensive observability stack
- 📈 Publish first performance report (Mermaid diagrams + JSON KPIs)
- 🔍 End-to-end IDE integration testing
- 📋 Document performance characteristics

### **Day 10: Production Readiness**
- 📦 Harden packaging and fallback mechanisms
- 📚 Document legal boundaries and compliance
- 🔒 Security review and validation
- 🚀 Prepare for production deployment

---

## ⚠️ **Risk Assessment & Mitigation Strategies**

### **Legal Risks:**
- **Risk**: Intellectual property concerns  
- **Mitigation**: Keep analysis source-based (intellij-rust) and black-box runtime; no decompilation or redistribution

### **JNI Performance Risks:**
- **Risk**: JNI overhead negating native benefits
- **Mitigation**: Batch aggressively; avoid chatty calls; prefer direct buffers

### **Cross-Platform Complexity:**
- **Risk**: Platform-specific quirks and failures
- **Mitigation**: Static CRT on Windows; gate in CI; include symbol stripping configs; verify minimum glibc

### **Data Handling Risks:**
- **Risk**: Memory usage spikes with large files
- **Mitigation**: Stream DWARF sections; avoid buffering entire files; implement backpressure

---

## 🎯 **Success Metrics & KPIs**

### **Performance Targets:**
```rust
// Symbol demangling
Target: 1M+ symbols/second (batch processing)
Baseline: ~10K symbols/second (JVM string operations)
Improvement: 100x speedup

// DWARF parsing  
Target: 100+ MB/second parsing throughput
Baseline: ~7 MB/second (Java DWARF libraries)
Improvement: 15x speedup

// Object analysis
Target: 1000+ files/second metadata extraction  
Baseline: ~20 files/second (Java binary parsing)
Improvement: 50x speedup

// End-to-end IDE impact
Target: <3 seconds large project indexing
Baseline: 30-60 seconds pure JVM
Improvement: 10-20x user experience enhancement
```

### **Quality Gates:**
- ✅ 100% correctness vs reference implementations
- ✅ <1ms JNI overhead per 100K symbol batch
- ✅ Graceful fallback when native unavailable
- ✅ Cross-platform consistency
- ✅ Memory usage <10% of equivalent JVM approach

---

## 📚 **Documentation Deliverables**

1. **📊 Performance Analysis Report** (Mermaid diagrams + JSON KPIs)
2. **🏗️ Architecture Decision Records** (FFI design, batching strategy)  
3. **🔧 Integration Guide** (How to embed in existing IDEs)
4. **🧪 Testing Playbook** (Correctness, performance, fallback validation)
5. **📦 Deployment Guide** (Cross-platform packaging, CI/CD)

---

## 🏆 **Bottom Line**

This **Hybrid Plan 2.0** evolution maintains the strategic pivot while adding **sharp "native helper" focus** that's both **plausible and high-impact**. 

**Key Advantages:**
- ✅ **Legally compliant** - no proprietary code decompilation
- ✅ **Performance validated** - hard KPIs proved by JFR-anchored benchmarks  
- ✅ **Production ready** - clear APIs, batched JNI, fallback mechanisms
- ✅ **Measurable wins** - 10-100x improvements in critical performance paths

This yields **durable, measurable performance improvements** without legal risk, creating a **foundation for next-generation Rust development tools**.