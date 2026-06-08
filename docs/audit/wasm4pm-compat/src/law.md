# Formal Audit Report: `wasm4pm-compat/src/law.rs`

## Compile-Time Law Kernel and Ontological Constants

The `law.rs` module represents the core formal verification machinery of the `wasm4pm-compat` boundary. It leverages bleeding-edge compiler features (`generic_const_exprs`, `adt_const_params`) to elevate domain-specific process mining rules into compile-time invariants.

### Observations:
1. **Compile-Time Bounds (`Assert` / `Require`)**: The implementation of `Assert<const OK: bool>` combined with the `IsTrue` trait provides a mechanism to force compiler errors for invariant violations. This replaces runtime `panic!` checks with un-compilable code, guaranteeing that deployed binaries are structurally sound by definition.
2. **Domain Invariants**: The `Between01<NUM, DEN>` struct forces quality metrics (Fitness, Precision, F1, Generalization) to mathematically prove they lie within the $[0, 1]$ interval at compile time. Similarly, `ConditionCell<BITS>` enforces an 8-bit bound limit (the "Need9 means split" rule), preventing architectural bloat and guaranteeing fixed-width state encodings for downstream ML orchestration.
3. **Ontological Classifications**: The module defines exhaustive `ConstParamTy` enums for every formal concept in process mining theory. Examples include `EndpointKind` (ensuring Petri net bipartite arc validity), `SoundnessState` (distinguishing between `Claimed` and formally `Witnessed` soundness), `ObjectCentricity`, and `WorkflowPattern` (cataloging the canonical 20 Russell/van der Aalst workflow patterns).
4. **Zero-Cost Verification**: Because these constructs exist entirely as `PhantomData` or const generics, they impose zero runtime overhead, satisfying the project's strict nanosecond-scale execution mandate.

### Conclusion:
The `law.rs` module is an exemplary realization of "parse, don't validate." It proves that the framework does not merely *check* process mining formalisms; it *requires* them for compilation. This is the highest standard of structural verification.
