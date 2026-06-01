# Testing Strategy

Three surfaces, three purposes. Each is opt-in beyond the fast loop.

## Surface 1 — Fast loop

```bash
cargo test --all-features --tests
```

Sub-second warm run. Unit tests and integration tests only. No trybuild, no doctests.
This is the daily development gate — run it before every commit.

## Surface 2 — ALIVE gate (type-law receipts)

```bash
cargo test --test ui_tests -- --ignored
```

Explicit opt-in. Runs all `compile_fail` and `compile_pass` trybuild fixtures.
Each `compile_fail` fixture must fail for the **named law** (type/const/trait error),
not for scaffolding (import not found). Every `compile_fail/*.rs` has a matching
`.stderr` — the audit script `scripts/audit/audit_trybuild_receipts.sh` enforces this.

## Surface 3 — Documentation audit

```bash
cargo test --doc --all-features
```

Explicit opt-in. Each doctest is a separate nightly `rustc` invocation — 200+ doctests
means 4+ minutes. `doctest = false` under `[lib]` keeps the fast loop fast.
The rule: **doctests teach usage, trybuild proves law.**

## Crown audit mesh

```bash
./scripts/audit/audit_crown_gate_all.sh
```

Runs all 21 audit scripts. Hard failures (engine creep, unsealed refusals, missing
features) exit 1. Soft warnings (annotation gaps, unmatched pass/fail pairs) exit 0.

## What is NOT tested here

- Process discovery correctness (graduates to wasm4pm)
- Conformance checking accuracy (graduates to wasm4pm)
- Replay trace alignment (graduates to wasm4pm)
- Runtime performance (covered by `benches/`)
