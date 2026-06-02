---
gap_id: GAP_TS
gap_name: TypeScript Projection Template
status: CLOSED
closed_date: 2026-06-02
---

# Gap Closure Receipt: GAP_TS

## Summary

GAP_TS is closed by the manufacture of a TypeScript projection template surface within the ggen ecosystem. The surface delivers a Tera template (`ts-projection.rs.tera`) driven by a projection manifest (`ts.projection.yaml`) and governed by a law file (`ts-projection-law.yaml`), together establishing a typed, receipt-bearing path from wasm4pm-compat process-evidence types to TypeScript binding artifacts. An audit script (`audit-ts-projection.sh`) and its Tera source confirm the audit gate is machine-executable.

## Evidence

Files created or modified to close GAP_TS:

- `/Users/sac/wasm4pm-compat/ggen/projections/ts.projection.yaml` — projection manifest declaring TypeScript as a target surface with input type bindings and emission rules
- `/Users/sac/wasm4pm-compat/ggen/templates/ts-projection.rs.tera` — Tera template that generates Rust-side TypeScript projection glue code from the manifest
- `/Users/sac/wasm4pm-compat/ggen/templates/audit-ts-projection.sh.tera` — Tera template for generating the TypeScript projection audit script
- `/Users/sac/wasm4pm-compat/ggen/audits/audit-ts-projection.sh` — emitted audit script; verifies TypeScript projection output conforms to the declared law
- `/Users/sac/wasm4pm-compat/ggen/rules/ts-projection-law.yaml` — law constraints governing what TypeScript projection is and is not permitted to emit
- `/Users/sac/wasm4pm-compat/ggen/intel/specta-ts-projection-candidates.yaml` — capability map of specta-derived TypeScript projection candidates
- `/Users/sac/wasm4pm-compat/ggen/intel/tsify-capability-map.md` — tsify capability analysis supporting TypeScript binding surface decisions

## Audit Gate

The audit gate is `ggen/audits/audit-ts-projection.sh`. A passing audit confirms:

1. `ts.projection.yaml` is present and structurally valid.
2. `ts-projection.rs.tera` renders without error against the projection manifest.
3. Emitted TypeScript binding artifacts carry projection receipts naming the `GAP_TS` law surface.
4. No silent loss occurs: every type mapping either succeeds or emits a named `LossReport`.
5. The rendered output compiles under `cargo test --all-features --tests` without new failures.
