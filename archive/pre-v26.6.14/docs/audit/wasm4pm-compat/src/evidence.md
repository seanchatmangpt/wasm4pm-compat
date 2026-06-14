# Formal Audit Report: `wasm4pm-compat/src/evidence.rs`

## Typestate Constraints and the Universal Carrier

The `evidence.rs` module implements the universal evidence carrier `Evidence<T, State, W>`, which fundamentally guarantees that process artifacts cannot bypass structural admission boundaries.

### Observations:
1. **Affine Typestate Enforcement**: The `Evidence` struct utilizes zero-sized `PhantomData` type parameters to encode both the lifecycle `State` (e.g., `Raw`, `Admitted`) and the authority `Witness`. Because these are encoded in the type signature, functions demanding `Admitted` evidence will yield a compiler error if supplied with `Raw` evidence. This is a zero-cost abstraction that formally guarantees boundary adherence.
2. **The One-Way Door**: The implementation strictly limits the constructors. While `Evidence::raw` is public, the `sealed` constructor (which produces the `Admitted` state) is `pub(crate)` and accessible exclusively through the `Admit` trait implementations. This creates an enforced one-way door: untrusted data cannot be "laundered" into an admitted state without passing through the named law verification.
3. **Structured Refusal Fast-Paths**: The module provides `refuse` and `into_refused` methods, which correctly consume the evidence carrier while preserving the payload for diagnostic transparency. This allows for fast-rejection of fundamentally invalid shapes without complex dynamic error propagation.
4. **Graduation Handoff**: Methods like `into_exportable` and `into_receipted` provide monotonic advancement toward cryptographic sealing and external export, enforcing that only structurally verified and `Admitted` evidence can ever be exported or graduated to the execution engine.

### Conclusion:
The `Evidence` carrier is implemented with mathematical rigor. By leveraging Rust's affine types and ownership semantics, it provides an infallible, compile-time guarantee of process-evidence lifecycle integrity.
