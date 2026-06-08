# Formal Audit Report: `wasm4pm-compat/src/admission.rs`

## Structural Invariants and Boundary Law

The `admission.rs` module establishes the fundamental boundary law for process-mining evidence within the `wasm4pm-compat` ontology. Its design is structurally sound and formally rigorous.

### Observations:
1. **Typestate Monotonicity**: The transition from `Raw` to `Admitted` is strictly monotonic and governed by the `Admit` trait. The use of zero-cost phantom types (`PhantomData<W>`) ensures that admission is coupled to a specific witness (authority/standard) at compile time. This is a robust mechanism preventing the circumvention of admissibility checks.
2. **First-Class Refusals**: The `Refusal` structure mandates a named law variant (e.g., `DanglingEventObjectLink`, `MissingFinalMarking`) rather than a generic string or dynamic error. This aligns with process theory requirements for deterministic, formal rejection of structurally unsound logs or models.
3. **Engine Decoupling**: The module correctly restricts itself to "structure-only" validation. It defines *that* a law was satisfied without attempting to run conformance or discovery solvers. This preserves the static analysis properties of the compatibility layer prior to graduation to the `wasm4pm` execution engine.

### Conclusion:
The admissibility boundary is formally correct and implements zero-cost structural checks. The type-system constraints correctly prevent "data laundering" of raw evidence into the execution runtime.
