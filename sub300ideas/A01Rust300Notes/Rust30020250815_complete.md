# Rust 300 - Small, High-Impact Rust Libraries (300 LOC or less)
The research reveals a vast landscape of opportunities for creating small (<300 LOC), high-impact, CPU-focused Rust libraries. These opportunities span numerous domains, including mathematical special functions, 3D math, bit-twiddling, data structures, parsing, cryptography, and concurrent programming. A recurring theme is the demand for libraries that are `no_std` compatible, deterministic, SIMD-accelerated, and offer minimal dependencies as alternatives to larger, more complex crates. Key gaps exist for ports of well-established C/C++ micro-libraries (like `libdivide` or the `stb` suite), implementations of classic algorithms from academic literature (like Median of Medians or the Goertzel algorithm), and specialized primitives for modern applications like machine learning (stable softmax) and data engineering (integer compression). The potential for these libraries to be compiled to WebAssembly (WASM) further increases their product-market fit, enabling high-performance computation in web environments.

## Key Opportunity Areas

Mathematical Special Functions: Tiny, `no_std`-friendly libraries for functions like `erfcx`, incomplete gamma/beta, and `sinpi`/`cospi` that are missing or have room for more optimized implementations than `libm` or `statrs`.

Linear Algebra & 3D Math Kernels: Compact, deterministic, fixed-size kernels for 3x3/4x4 matrix multiplication, quaternion operations, and normalization, providing minimal alternatives to large libraries like `nalgebra` or `glam` for games and graphics.

Bit-Twiddling and Succinct Data Structures: Micro-utilities for Morton/Z-order encoding, broadword rank/select, and other bit manipulation tricks from sources like 'Hacker's Delight', crucial for spatial databases and succinct data structures.

Minimal Perfect Hashing & Static Hashing: Implementations of classic MPHF algorithms like BDZ, CHD, and CHM, separating the build-time generation from a tiny runtime lookup function, ideal for static configuration tables and embedded use.

RNG & Sampling Primitives: Lightweight, `no_std` implementations of fast PRNGs (Xoshiro, PCG32), seeders (SplitMix64), and sampling algorithms (Walker's Alias Method, Vitter's Reservoir Sampling) for simulations and data analysis.

Streaming Statistics: Numerically stable, single-pass algorithms like Welford's for variance, Kahan summation, and online covariance/regression, essential for telemetry, real-time analytics, and embedded monitoring.

Checksums & Non-Cryptographic Hashes: Hardware-accelerated CRC32C/CRC64 (using PCLMULQDQ/PMULL), optimized Adler-32, and adapters for modern hashes like HighwayHash, critical for storage and networking.

SIMD-Accelerated Byte/ASCII Primitives: High-performance kernels for common string operations like case conversion, hex encoding/decoding, and multi-needle search, leveraging SIMD for massive speedups in parsers and servers.

Integer Compression & Bitpacking: Micro-kernels for ZigZag/VarInt encoding, Frame-of-Reference, Delta-of-Delta, and SIMD bitpacking, foundational for columnar databases and time-series storage.

Minimal Graph & Data-Structure Algorithms: `no_std`, heap-free implementations of Union-Find, topological sort, and BFS/DFS iterators for fixed-capacity graphs, targeting embedded systems and resource-constrained environments.

Computational Geometry Kernels: Robust, `no_std` primitives for 2D segment intersection, point-in-polygon tests, convex hulls, and AABB operations, essential for GIS, games, and robotics.

Parsing & Formatting Micro-Libraries: High-performance, `no_std` utilities for fast integer parsing (atoi), integer-to-string formatting (itoa), and URL-safe Base64, replacing slower or more complex standard library functions.

Cryptographic & Constant-Time Micro-Primitives: Small, auditable, constant-time building blocks like `memeq`, conditional select, and one-shot HMAC adapters, crucial for secure library development.

Lock-Free & Wait-Free Primitives: Minimalist concurrency primitives like SPSC/MPSC ring buffers, ticket spinlocks, and sequence locks, designed for low-latency, high-throughput CPU-bound pipelines.

Time-Series/DSP Kernels: Compact implementations of classic signal processing algorithms like the Goertzel detector, Haar wavelet transform, and biquad filters, for audio analysis, anomaly detection, and embedded DSP.

Data Layout Transformations: Micro-kernels for efficient AoS-to-SoA conversion, byte deinterleaving, and Morton rearrangement to optimize data for cache locality and SIMD processing.

Numerical Robustness Utilities: Libraries providing stable algorithms like compensated summation (Kahan/Neumaier), log-sum-exp, and stable polynomial evaluation to prevent floating-point overflow, underflow, and precision loss in ML and scientific computing.

## Mathematical Special Functions

### erfcx (Scaled Complementary Error Function)

A highly optimized, standalone `erfcx` function for `f64` and `f32`. This function is essential in probability and statistics for accurately calculating the tail of the normal distribution, as well as in diffusion processes, heat transfer physics, and financial modeling (e.g., Black-Scholes). It avoids the precision loss that occurs when calculating `exp(x*x) * erfc(x)` for large `x`.

**PMF Probability:** 90%

**Success Testing:** Testing involves generating vectors from established libraries like Python's `mpmath` or Boost.Math and measuring ULP (Units in the Last Place) error to ensure near machine precision. The implementation must correctly handle `NaN`, `Inf`, subnormal numbers, and negative inputs using the identity `erfcx(-x) = 2*exp(x^2) - erfcx(x)`.

**References:** Algorithm based on Steven G. Johnson's Faddeeva Package (piecewise Chebyshev polynomials and continued-fraction expansion). Test vectors from Python `mpmath`, Julia `SpecialFunctions.jl`, Boost.Math. Existing Rust alternative: `errorfunctions` crate.

### Incomplete Gamma Function (P(a,x) or Q(a,x))

A `no_std`, standalone implementation of the regularized incomplete gamma function. This function is fundamental in statistics for the chi-squared and Poisson distributions, and is also used in queuing theory, reliability engineering, and physics. A standalone kernel would be valuable for embedded systems and other contexts where dependencies must be minimized.

**PMF Probability:** 85%

**Success Testing:** High. Test vectors are available from the reference implementations of Algorithm AS 239 (Fortran, C++), the `Rmpfr` package in R, Boost.Math, and SciPy. Must handle edge cases like `a=0`, `x=0`, large/small values of `a` and `x`, and `NaN`/`Inf` inputs.

**References:** Implementation can be based on the compact Algorithm AS 239 by B. L. Shea, which uses a series expansion or a continued fraction based on input values. Existing Rust alternative: `statrs::gamma::gamma_inc`.

### Incomplete Beta Function (I_x(a,b))

A `no_std`, standalone implementation of the regularized incomplete beta function. It is crucial for statistical analysis involving the beta distribution, F-distribution, and binomial distribution, and is widely used in Bayesian inference and quality control. A minimal, `no_std` version is a clear gap for resource-constrained environments.

**PMF Probability:** 85%

**Success Testing:** High. Test values can be sourced from the reference ASA63 implementation, `Rmpfr`, Boost.Math, and SciPy. Requires careful handling of input parameters `a` and `b`, ensuring `x` is within `[0, 1]`, and managing `NaN`/`Inf` inputs.

**References:** Standard algorithms like ASA63 and DiDonato & Morris's Algorithm 708 use continued fraction or series expansions. Existing Rust alternative: `statrs::beta::beta_inc`.

### Owen's T Function (T(h,a))

A highly optimized, minimal version of Owen's T function. This function is essential for calculating bivariate normal distribution probabilities, which has applications in multivariate statistics and financial modeling (e.g., pricing options on correlated assets).

**PMF Probability:** 90%

**Success Testing:** High. The existing `owens-t` crate provides accuracy benchmarks (16 decimal places). Test vectors can be generated from Boost.Math or MATLAB. Must handle extreme values of `h` and `a`, as well as `NaN`/`Inf` inputs.

**References:** The state-of-the-art Patefield-Tandy algorithm provides a compact implementation path. The existing `owens-t` crate is an excellent reference and highlights opportunities for `f32` and SIMD optimizations.

### sinpi / cospi (sin(πx), cos(πx))

A `no_std` library for `sinpi` and `cospi` that offers higher precision than `sin(x * PI)`. These functions are standard in other high-performance math libraries (e.g., Julia, SLEEF) because they prevent catastrophic cancellation and precision loss for large `x` or `x` near an integer. Use cases include digital signal processing (DSP), Fourier analysis, and graphics.

**PMF Probability:** 80%

**Success Testing:** High. Test vectors can be generated from `mpmath` or Julia. The primary success metric is achieving low ULP (Unit in the Last Place) error, especially near integer and half-integer values of `x` where `sin(x * PI)` fails. Must correctly handle large `x` values, `NaN`, and `Inf`.

**References:** Implementation involves a clever range reduction of `x` to a small interval like `[-0.5, 0.5]` followed by a low-degree polynomial approximation. See implementations in `musl` libc or FDLIBM. No dedicated `sinpi`/`cospi` functions exist in Rust's `libm`.

### Lambert W Function

A minimal `f32`-only or specialized-branch version of the Lambert W function. This function is used for solving transcendental equations in physics, engineering, combinatorics, and the analysis of algorithms.

**PMF Probability:** 80%

**Success Testing:** High. Test against `mpmath`, Wolfram Alpha, or the existing `lambert_w` crate. Must handle the valid input ranges for real branches (e.g., `x >= -1/e`), `NaN`, and `Inf`.

**References:** The `lambert_w` crate already provides a `no_std` implementation using Fukushima's method of piecewise minimax rational approximation. A new library would need to offer a clear advantage, such as significantly smaller code size for `f32`-heavy applications (e.g., graphics, game development).

### Stable hypot

A `hypot(x, y)` implementation with demonstrably superior stability guarantees for extreme edge cases compared to the standard `libm` version. This function calculates `sqrt(x*x + y*y)` without intermediate overflow or underflow. It is fundamental in geometry, vector mathematics, and complex number arithmetic.

**PMF Probability:** 75%

**Success Testing:** High. Success is measured by correctness with extreme inputs: very large/small values, subnormals, zero, `NaN`, and `Inf`. A new crate would need to target and solve specific, well-documented edge cases where the `libm` version is suboptimal.

**References:** The standard algorithm uses scaling to prevent overflow/underflow (e.g., `x * sqrt(1 + (y/x)^2)`). A more stable version would involve more meticulous scaling and handling of exponents. Existing Rust alternative: `libm::hypot`.

### expm1 / log1p

A version of `expm1(x)` (calculates `exp(x) - 1`) and `log1p(x)` (calculates `log(1 + x)`) with a different performance/accuracy profile than `libm`'s. These functions are crucial for maintaining precision when `x` is close to zero. They are used in financial calculations (e.g., compound interest) and numerical analysis.

**PMF Probability:** 70%

**Success Testing:** High. Test against `mpmath`, Julia, and `libm`, focusing on ULP accuracy for inputs near zero. Must handle `x` near zero correctly, as well as `NaN` and `Inf` inputs.

**References:** The core logic is a simple switch: for small `x`, use a Taylor series polynomial to avoid catastrophic cancellation; for larger `x`, call the standard function directly. Existing Rust alternatives: `libm::expm1` and `libm::log1p`. A new library would only succeed if it could demonstrate a significant performance or accuracy advantage for a specific use case.
