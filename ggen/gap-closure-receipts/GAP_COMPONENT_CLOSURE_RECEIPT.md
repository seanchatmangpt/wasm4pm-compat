---
gap_id: GAP_COMPONENT
gap_name: Component Model WIT Surfaces
status: CLOSED
closed_date: 2026-06-02
---

# Gap Closure Receipt: GAP_COMPONENT

## Summary

GAP_COMPONENT is closed by the manufacture of three artifacts that together express the wasm4pm-compat type law as a WASM Component Model surface. The projection manifest (`component.projection.yaml`) declares the canonical projection from compat evidence types to WIT interface shapes. The Tera templates (`component-model.tera` and `wasm4pm-compat.wit.tera`) generate the WIT interface and component-model binding code from that manifest, carrying the one-way lifecycle and named-law refusal structure into the WASM component boundary.

## Evidence

Files created or modified to close GAP_COMPONENT:

- `ggen/projections/component.projection.yaml` — projection manifest declaring the compat-to-WIT surface mapping
- `ggen/templates/component-model.tera` — Tera template generating component model binding code from the projection manifest
- `ggen/templates/wasm4pm-compat.wit.tera` — Tera template generating the `.wit` interface definition expressing Evidence, Admission, Refusal, and LossReport as WIT resource types

## Audit Gate

The audit gate confirms GAP_COMPONENT is closed when:

1. `component.projection.yaml` parses without error and references only types present in the compat base profile.
2. `wasm4pm-compat.wit.tera` renders a `.wit` file that passes `wasm-tools component wit` validation (no undefined type references).
3. `component-model.tera` renders Rust binding code that compiles under the nightly toolchain with `--features wasm4pm`.
4. Every named refusal law in `src/admission.rs` appears as a named WIT variant in the rendered `.wit` output — no catch-all `invalid-input` variant is present.
