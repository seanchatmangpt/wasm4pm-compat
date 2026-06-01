# DX/UX/QoL/Examples/Docs Sprint Checkpoint

**Date:** 2026-05-31
**Sprint type:** DX / UX / QoL / Examples / Docs — no type-law changes, no engine additions

## Metrics

| Metric | Value |
|---|---|
| May 2026 commits | 512 |
| Examples | 15 |
| Docs files | 37 |
| Tests (all-features) | 33 passed, 0 failed |
| Build status | pass |
| Clippy status | pass (0 warnings) |

## New implementations added this sprint

### DX (developer experience)
- `IntoIterator` impls for collection-holding process evidence types
- Structural bridge conversions between related process evidence types
- Developer-facing comments on const assertion error messages
- Typed ID family integration test
- Metric bounds integration test
- Witness authority metadata integration test
- Loss chain integration test
- Evidence lifecycle integration test
- Test helper builders for common law-compliant constructions

### UX (user experience)
- Deprecation markers and stability annotations on public API
- Feature flag documentation in `Cargo.toml`
- Flattened commonly-used types in crate root re-exports
- `#[doc(alias)]` for common API search terms

### QoL (quality of life)
- `into_inner` / `as_inner` accessor methods on newtype wrappers
- `::new()` convenience constructors for typed ID families
- `#[must_use]` annotations on `Result`/`Option`-returning functions
- Type aliases for common `Evidence<T, State, W>` combinations

### Examples (15 total in `examples/`)
- `conformance_metrics.rs` — compile-time [0,1] metric type construction
- `receipt_chain.rs` — `ReceiptEnvelope`, `ReceiptChain`, `ReceiptChainConst<2>`, `GraduationReceipt`, `ReceiptVerdict`
- `witness_authority.rs` — zero-cost type-level authority markers
- `causal_net_shape.rs` — `CausalNet` output shape, bindings, and `DependencyMeasure`
- `powl_process_tree.rs` — POWL/process-tree shapes and arity enforcement
- (10 pre-existing examples retained)

### Fixture-fail additions (type-law receipts — ALIVE gate)
- Final batch to 165 compile-fail fixtures
- Receipt without lawful witness marker rejected
- `replay_hint` wrong type rejected
- `GraduationReceipt` not substitutable for `ReceiptEnvelope`
- `ReceiptVerdict` on non-receipt type rejected
- `ReceiptChain` wrong length `N` rejected
- Receipt wrong witness marker rejected

## Build evidence

```
cargo build --all-features     → Finished dev profile in 0.03s
cargo clippy --all-features    → Finished dev profile in 0.06s (0 warnings)
cargo test  --all-features --tests → 33 passed; 0 failed in 0.212s total
```

## Invariants preserved

- No type-law changes (no modifications to `src/law.rs`, `src/petri.rs`, `src/conformance.rs`, etc.)
- No engine logic added
- `#![forbid(unsafe_code)]` intact
- Three public Cargo features only (`formats`, `strict`, `wasm4pm`)
- Every refusal still carries a specific named law reason type
- Nightly-only covenant upheld
