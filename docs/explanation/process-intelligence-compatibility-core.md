# Explanation: The Process Intelligence Compatibility Core

This document provides a comprehensive overview of how `wasm4pm-compat` version `26.6.23` serves as the foundational process-evidence type standard for the entire process intelligence workspace.

---

## 1. The Role of the Compatibility Core

In full-lifecycle process intelligence (spanning design, operation, conformance checking, and decommissioning), systems must exchange event logs, Petri nets, queries, and conformance reports. Exchanging these shapes across heterogeneous boundaries (microservices, databases, JS runtimes, or smart contracts) presents a high risk of state pollution and model corruption.

`wasm4pm-compat` serves as the **Rust process-evidence court**. It establishes the type laws, structural constraints, and transition invariants that govern all process mining structures.

The core relationship is defined as follows:
- **`wasm4pm-compat`** defines the Rust process-evidence court.
- **`ggen`** projects code and ontology assets into that court.
- **`wasm4pm`** executes active judgment (model discovery, replay, conformance alignments) after graduation.

---

## 2. Structural Integrity via Zero-Cost Abstractions

To enforce constraints without adding runtime memory or CPU overhead, the compatibility core uses three advanced Rust type patterns:

### A. Typestate Lifecycle Carrier (`Evidence<T, State, W>`)
Tracks the lifecycle status of evidence (`Raw -> Parsed -> Admitted -> Projected/Exportable/Receipted`) using zero-sized markers. The compiler statically prevents applications from running projections or exports on unvalidated (`Raw` or `Parsed`) data.

### B. Kind-Typed Identifier Wrappers (`EventId<K>`, `ObjectId<K>`)
Provides namespace separation using `#[repr(transparent)]` and zero-sized markers `K`. This prevents identifier confusion (e.g. passing a case ID where an event ID is expected, or mixing up event streams from different logs) during compilation.

### C. Compile-Time Law Bounds (`ConditionCell` and `Between01`)
Evaluates domain constraints (such as the *Need9 means split* rule or rational probability metric bounds in $[0, 1]$) during compilation using const-generic assertions, returning compiler errors for invalid parameters.

---

## 3. Decoupling and the "Structure-Only" Philosophy

`wasm4pm-compat` contains **zero active engine behavior**. It does not perform process discovery or conformance check execution. This decoupling ensures:
1. **Low Dependency Footprint**: It does not import heavy solvers or process mining execution runtimes, keeping compile times fast.
2. **Embeddability**: The crate compiles cleanly to WASM targets and is safe to execute in restricted runtimes (such as AI agent sandboxes).
3. **Seam Graduation**: When computational execution is required, host structures implement the `GraduateToWasm4pm` trait to package a `GraduationCandidate`, which is handed off to the execution runtime.

---

## 4. The Nightly Covenant

Because these type-level constraints and compile-time evaluations rely on advanced compiler features (such as `generic_const_exprs`, `adt_const_params`, and `min_specialization`), the compatibility core requires a **permanent nightly-only toolchain**. 

---

## 5. Versioning Policy

To respect package and workspace constraints, the physical `Cargo.toml` version field is locked at `26.6.23`. However, the logical target of all reports, specifications, and documentation is the release version `26.6.23`.
