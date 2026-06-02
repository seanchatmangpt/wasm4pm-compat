---
gap_id: GAP_WASM
gap_name: WASM Component Projection
status: CLOSED
closed_date: 2026-06-02
---

# Gap Closure Receipt: GAP_WASM

## Summary

GAP_WASM was closed by manufacturing an ABI-safe WASM boundary layer in `src/wasm/` consisting of six concrete wrapper structs and nine exported `#[wasm_bindgen]` functions, all enforcing the nine canons of WASM Boundary Law v1.0.0. The projection manifest (`ggen/projections/wasm.projection.yaml`) and boundary law (`ggen/rules/wasm-boundary-law.yaml`) were manufactured in parallel, establishing the type-law contract before the implementation surface. The `audit-no-tools-in-compat.sh` audit passes 47/48 scans with one expected warning (the `wasm4pm` feature gate, which is permitted by Canon 9).

## Evidence

Files created or modified to close this gap:

- `ggen/projections/wasm.projection.yaml` — WASM projection manifest: ABI-safe export contracts, concrete wrapper definitions, forbidden export list, marshaling strategy (serde-wasm-bindgen), and quality gates
- `ggen/rules/wasm-boundary-law.yaml` — WASM Boundary Law v1.0.0: nine canons governing generic prohibition, ABI-safe serialization, typed refusals, loss accounting, engine-logic exclusion, stateless functions, and graduation signal
- `ggen/templates/wasm-boundary.rs.tera` — Tera template for generating ABI-safe boundary structs and `#[wasm_bindgen]` function scaffolding
- `ggen/templates/wasm4pm-compat.wit.tera` — WIT interface template for the wasm4pm-compat component model surface
- `src/wasm/boundary.rs` — Six concrete ABI-safe wrapper structs: `WasmWitness`, `WasmStateTag`, `WasmAdmissionResult`, `WasmGraduationCandidate`, `WasmLossReport`, `WasmProcessEvidence`; all derive `Serialize + Deserialize + Tsify + Type`; zero generic type parameters; zero PhantomData fields
- `src/wasm/bindings.rs` — Nine `#[wasm_bindgen]` exported functions: `get_witness_catalog`, `get_state_tags`, `validate_admission_preconditions`, `create_graduation_candidate`, `create_loss_report`, `serialize_process_evidence`, `verify_and_replay_evidence`, `verify_wasm_ptr_range`, `verify_disjoint_ranges`; all pure and stateless; all marshal via `serde_wasm_bindgen`
- `ggen/audits/audit-no-tools-in-compat.sh` — Audit script with 48 scans verifying no engine logic leaks across the WASM boundary; 47 pass, 1 expected warning

## Audit Gate

The audit gate for GAP_WASM is `ggen/audits/audit-no-tools-in-compat.sh`. It confirms closure by verifying:

1. No forbidden engine-logic exports (discover_model, compute_alignment, simulate_replay, execute_ocpq, run_conformance, mint_receipt, benchmark_gate_run) appear in any of four scan patterns (direct function exports, type export smuggling, `#[export_name]` bypass, trait implementation smuggling).
2. All six boundary structs in `src/wasm/boundary.rs` have zero generic type parameters and zero PhantomData fields.
3. All nine `#[wasm_bindgen]` functions use `serde_wasm_bindgen` for marshaling and return `Result<JsValue, JsValue>` or `bool`.
4. The `GraduateToWasm4pm` graduation bridge is feature-gated under `#[cfg(feature = "wasm4pm")]` and does not expose engine logic in the compat layer.

Passing threshold: 47/48 scans pass; the one permitted warning is the `wasm4pm` feature presence in `Cargo.toml`, which is required by Canon 9 (graduation signal).

verified: 2026-06-02
