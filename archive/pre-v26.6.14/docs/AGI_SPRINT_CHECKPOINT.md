# AGI Quality Sprint Checkpoint

**Date:** 2026-05-31
**Branch:** main

## Counts

| Metric | Value |
|---|---|
| May commits (since 2026-05-01) | 553 |
| Source modules (`src/*.rs`) | 37 |
| Compile-fail fixtures (`tests/ui/compile_fail/`) | 189 |
| Compile-pass fixtures (`tests/ui/compile_pass/`) | 406 |
| Benchmarks (`benches/`) | 4 |

## Build Status

| Check | Result |
|---|---|
| `cargo build --all-features` | PASS |
| `cargo clippy --all-features -- -D warnings` | PASS |
| `cargo test --all-features --tests` | PASS — 33 passed, 0 failed |

## New Modules Added This Sprint

Modules introduced during the May AGI quality sprint:

| Module | Type Law Surface |
|---|---|
| `src/process_cube.rs` | ProcessCube dimensional structure — van der Aalst 2013 |
| `src/temporal.rs` | Temporal ordering and profile law surfaces |
| `src/object_lifecycle.rs` | ObjectLifecycle typed phase law — const-generic phase transitions |
| `src/streaming.rs` | Streaming evidence context law — online vs offline markers |
| `src/causality.rs` | Causal consistency law — CausalChain, CausalLink, CausalConsistency |
| `src/correlation.rs` | Cross-log correlation law — CorrelationKey, CorrelatedLog, CorrelationSchema |
| `src/multiperspective.rs` | Multi-perspective evidence law — ControlFlow/Data/Resource/Time |
| `src/graduation.rs` | Graduation bridge traits toward the wasm4pm execution engine |
| `src/prelude.rs` | DX prelude — re-exports for law-compliant constructions |
| `src/causal_net.rs` | CausalNet structural shapes — Weijters and Ribeiro 2011 Heuristics Miner output |

## Invariants Verified

- `#![forbid(unsafe_code)]` — no exceptions.
- Exactly three public Cargo features (`formats`, `strict`, `wasm4pm`).
- Every refusal carries a specific named law as the reason type.
- Lossy projections go through `Project` with a `LossPolicy` and emit a `LossReport`.
- Type law lives in public modules — not hidden behind cfg gates.
- 189 compile-fail receipts each tied to a named law violation.
- 406 compile-pass receipts proving all lawful paths remain open.
