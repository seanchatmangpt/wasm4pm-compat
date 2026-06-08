# Formal Audit Report: `wasm4pm/src/simd_inner_loops.rs`

## Vectorized Execution Kernels and Determinism 

The `simd_inner_loops.rs` module contains the highest-frequency execution paths for process discovery (DFG aggregation) and conformance checking (token replay).

### Observations:
1. **Target Feature Isolation**: The use of explicit `#[cfg(target_feature = "...")]` guards ensures that SIMD instructions (AVX-512, AVX2, SSE4.2) are only emitted for compatible compilation targets. The implementation provides a verified scalar fallback (with 4x/8x loop unrolling) guaranteeing compatibility across the diverse WASM ecosystem.
2. **Determinism over Floating-Point**: Process mining demands strict reproducibility. The kernels exclusively utilize integer arithmetic (`u32`, `u64`) and associative addition. This actively avoids IEEE-754 floating-point divergence, ensuring that trace variant generation and fitness calculations yield identical outputs across heterogeneous distributed environments.
3. **Safe Abstractions**: While SIMD intrinsic calls fundamentally rely on `unsafe` blocks, the module encapsulates these operations within strictly bounded boundaries. The bounds checks (`idx < self.counts.len()`) strictly occur *before* dereferencing raw pointers, eliminating the risk of out-of-bounds memory access.
4. **Polynomial Variant Hashing**: The `SimdVariantDeduplicator` correctly utilizes an 8x unrolled FNV-1a polynomial hash, ensuring that the variant definition space is deterministically mapped without relying on non-deterministic standard library hashers.

### Conclusion:
The inner loop implementations are formally sound. They successfully leverage hardware acceleration while maintaining strict algorithmic determinism—a critical property for conformance and anomaly detection systems in process mining.
