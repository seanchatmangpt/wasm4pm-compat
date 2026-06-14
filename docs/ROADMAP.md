# Roadmap

## Current: `PAPERLAW_CROWN_ALIVE_005` (v26.6.13)

Sealed/tagged. 217 compile-fail + 408 compile-pass receipts; 271-paper corpus
(436 unique witness keys, compile-time uniqueness proof); MIRI clean (no UB);
exactly three public features; zero runtime dependencies.

Shipped in this milestone:

- **271-paper witness corpus** with a self-validating `witness_corpus` (compile-time KEY-uniqueness).
- **Concrete admission** — `LinkedOcel`, the first `Admit` impl in `src/`, enforcing the named OCEL object-centricity laws through the typed one-way door.
- **Teaching diagnostics** (`on_unimplemented`) on the family-authority traits.
- **Nightly innovations** — `ConstParamTy` on `WitnessFamily`, `const trait Witness`, `witness_law` (family gating, co-citation string law, SIMD batch check, `gcd`/`NormedBetween01`).
- **TS sidecar extraction** (`wasm4pm-compat-ts`) restoring the three-feature / zero-runtime-dep invariants.
- **Durability pin** + documented `generic_const_exprs`/mGCA finding.
- **Restored** the `SeparableWfNet` non-forgeability seal lost in a prior refactor.

## Next follow-ups

- **Literal-100% item-level rustdoc** — module docs complete; ~690 public-item docs (compiler-measured via `missing_docs`, largely serde struct fields) remain.
- **Kani harness on `LinkedOcel::admit`** — prove the admit fn is total and the refusal branch fires for the malformed class (now unblocked by the concrete impl).
- **Verus on one witness end-to-end** (e.g. WF-net soundness) — behavioral proof against the source paper.
- **Wire `cargo make miri` into `ci`** once a baseline of findings is confirmed.
- **mGCA watch** — re-evaluate the const-law-kernel migration when mGCA's non-min expansion supports computed const arguments.
- **Streaming event window law** — `EventWindow<T, SIZE>` with TRYBUILD receipts.
- **Cross-log correlation / process-cube / temporal-profile** negative fixtures.

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
