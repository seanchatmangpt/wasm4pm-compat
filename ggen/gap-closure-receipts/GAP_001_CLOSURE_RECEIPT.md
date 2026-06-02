---
gap_id: GAP_001
gap_name: wasm4pm-compat Integration Bridge
status: CLOSED
closed_date: 2026-06-02
---

# Gap Closure Receipt: GAP_001

## Summary

GAP_001 covers the wasm4pm integration bridge — the typed graduation path from the compat crate's admitted process evidence into the wasm4pm execution engine. The gap was closed by manufacturing the `GraduateToWasm4pm` trait and its companion `GraduationCandidate` type in `src/engine_bridge.rs`, the full wasm ABI surface in `src/wasm/` (boundary, bindings, ABI), and the typed transition-system projection layer in `src/ts/`. Together these surfaces provide the only lawful path out of `wasm4pm-compat` and into the engine, enforced by the `wasm4pm` Cargo feature gate.

## Evidence

### wasm4pm integration bridge
- `src/engine_bridge.rs` — `GraduateToWasm4pm` trait, `GraduationCandidate`, `GraduationReason`
- `src/interop.rs` — cross-boundary interop surface referencing `GraduationCandidate`
- `src/receipt.rs` — receipt types tied to graduation witness

### wasm ABI and boundary
- `src/wasm/mod.rs` — wasm feature module root
- `src/wasm/boundary.rs` — `GraduateToWasm4pm` boundary implementation
- `src/wasm/bindings.rs` — generated/hand-written ABI bindings
- `src/wasm/abi.rs` — low-level ABI surface

### Transition-system projection layer
- `src/ts/mod.rs` — ts feature module root
- `src/ts/law_projection.rs` — law-bearing projection for TS export
- `src/ts/export.rs` — typed TS export surface
- `src/ts/bpmn_ts.rs`, `petri_ts.rs`, `powl_ts.rs`, `process_tree_ts.rs` — per-formalism TS projections
- `src/ts/declare_ts.rs`, `causality_ts.rs`, `multiperspective_ts.rs`, `prediction_ts.rs`, `streaming_ts.rs`, `workflow_ts.rs` — extended TS surfaces

### Loss covenant (prerequisite for graduation)
- `src/loss.rs` — `LossPolicy`, `LossReport`, `ProjectionName` types
- `src/process_tree.rs` — `TypedLoopNode<ARITY>`, `Require<{ ARITY == 2 }>: IsTrue`

### ggen projection manifests and templates
- `ggen/projections/wasm.projection.yaml` — wasm projection manifest
- `ggen/projections/ts.projection.yaml` — ts projection manifest
- `ggen/projections/component.projection.yaml` — component-model projection manifest
- `ggen/templates/wasm4pm-compat.wit.tera` — WIT interface template
- `ggen/templates/wasm-boundary.rs.tera` — wasm boundary codegen template
- `ggen/templates/ts-projection.rs.tera` — ts projection codegen template
- `ggen/templates/component-model.tera` — component model codegen template

## Audit Gate

The audit gate confirms GAP_001 is closed when all of the following hold:

1. `cargo build --no-default-features --features wasm4pm` succeeds — the graduation bridge compiles under the `wasm4pm` feature gate without requiring `formats` or `strict`.
2. `src/engine_bridge.rs` exports `GraduateToWasm4pm` as a public trait with at least one `GraduationReason` variant marked `is_hard_signal() == true`.
3. `src/wasm/boundary.rs` implements `GraduateToWasm4pm` for at least one admitted evidence type.
4. `ggen/projections/wasm.projection.yaml` and `ggen/projections/ts.projection.yaml` are present and parseable by the ggen toolchain.
5. No path exists from `Evidence<T, Raw, W>` directly to the wasm ABI — the only lawful route passes through `Admit::admit()` first (enforced by `pub(crate)` on the `Admitted` constructor).
