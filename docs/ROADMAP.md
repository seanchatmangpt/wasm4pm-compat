# Roadmap

## Current: `PAPERLAW_CROWN_ALIVE_004`

Sealed. 37 modules, 196 compile-fail receipts, 406 compile-pass receipts, 98 papers, 4 benchmarks.

## Next milestone: `PAPERLAW_CROWN_ALIVE_005`

Target surfaces:

- **E0391 full resolution** — all object_lifecycle const-param impl blocks refactored to type aliases
- **Streaming event window law** — `EventWindow<T, SIZE>` with TRYBUILD receipts
- **Cross-log correlation receipts** — negative fixtures for schema mismatch
- **Process cube fixtures** — fail fixtures for dimension count mismatch
- **Temporal profile conformance** — `TemporalProfile` receipts

## Graduation roadmap (wasm4pm engine)

Surfaces that graduate from compat structure to wasm4pm execution:

| Surface | Graduates when |
|---|---|
| Alpha Miner | wasm4pm event log → Petri net discovery pipeline |
| Inductive Miner | wasm4pm process tree discovery pipeline |
| Token replay | wasm4pm conformance checking pipeline |
| Alignment A* | wasm4pm alignment-based conformance |
| OCPQ evaluation | wasm4pm object-centric query engine |
| Prediction engine | wasm4pm predictive monitoring runtime |

## Invariants that never change

- `#![forbid(unsafe_code)]`
- Exactly 3 public features: `formats`, `strict`, `wasm4pm`
- No engine logic in `src/`
- Nightly-only — no stable fallback
- Every compile-fail fixture has a matching `.stderr`
