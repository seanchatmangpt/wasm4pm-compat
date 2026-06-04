# Feature Model

The `wasm4pm-compat` crate uses a precise, capability-based Cargo feature model. The crate's design minimizes feature flags to prevent feature-combination explosion and maintain type-system soundness.

---

## The Three-Feature Invariant

The public feature surface of `wasm4pm-compat` is capped at **exactly three** optional features. There are no other features, such as per-format flags (`ocel`, `xes`) or target-specific flags (`ts`, `wasm`). 

| Feature | Default | Module | Capability Added |
|---|:---:|---|---|
| `formats` | **Yes** | `src/formats.rs` | Enables external serialization import/export traits, format covenants, and loss-policy interfaces. |
| `strict` | No | `src/strict.rs` | Enables strict boundary checks, `ProcessBoundary`, and `StrictViolation` diagnostic markers. |
| `wasm4pm` | No | `src/engine_bridge.rs` | Enables the graduation bridge, `GraduateToWasm4pm`, and `GraduationCandidate` types for transition to the execution engine. |

---

## Always-On Canon (Base Profile)

A key architectural mandate is that the **entire process-evidence ontology** and all underlying structures are compiled under the base profile (with `--no-default-features`). 

Because `wasm4pm-compat` is a structure-only standard, all core structures (events, traces, event logs, Petri nets, WF-nets, OC-Petri-nets, BPMN, OCEL, XES, POWL, process trees, Declare, OC-Declare, OCPQ, DFGs, and conformance verdicts) are **always available**. Feature flags do not gate type definitions; they only gate additional capabilities acting on those types.

---

## Feature Descriptions

### 1. `formats`
* **Purpose:** Provides the interface layer for admitting external data into typed compat structures and exporting them back.
* **Key Types & Traits:**
  * `ImportFormat` / `ExportFormat` traits.
  * `FormatEnvelope` and `FormatExport`.
  * `LossPolicy`, `LossReport`, and `Project` trait for governing lossy projections.
* **Default Status:** Enabled by default because most callers need to parse or serialize process evidence.

### 2. `strict`
* **Purpose:** Restricts the admission boundaries to enforce stricter invariants.
* **Key Types & Traits:**
  * `ProcessBoundary` validation surface.
  * `StrictViolation` diagnostics.
* **Default Status:** Disabled by default; opt-in for callers requiring hard boundary verification.

### 3. `wasm4pm`
* **Purpose:** Bridges the gap between static evidence structures and the `wasm4pm` execution engine.
* **Key Types & Traits:**
  * `GraduateToWasm4pm` trait.
  * `GraduationCandidate` wrapper.
  * `GraduationReason` enums.
* **Default Status:** Disabled by default; enabled by the host environment or compiler when preparing evidence for active calculation.

---

## Unconditional Nightly Requirement

Nightly Rust is **not** a Cargo feature. The compatibility core relies on advanced nightly compiler capabilities (`generic_const_exprs`, `adt_const_params`, etc.) to enforce process-evidence invariants at compile time without runtime overhead. 

The nightly requirement is unconditional and locked via `rust-toolchain.toml`. There are no feature gates (such as `#[cfg(feature = "nightly")]`) or stable fallbacks.

---

## Related Documentation

- Back to [README](../../README.md)
- [Public API for ggen](public-api-for-ggen.md)
- [Module Map & Layout](module-map.md)
- [Evidence Lifecycle States](lifecycle-states.md)
