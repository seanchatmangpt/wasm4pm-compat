# PAPERLAW_CROWN_004 Gate Checkpoint

**Date:** 2026-05-31
**Branch:** main
**Toolchain:** nightly (rust-toolchain.toml)

## Gate Criteria vs. Actuals

| Criterion | Required | Actual | Status |
|---|---|---|---|
| compile-fail fixtures | ≥ 160 | 196 | PASS |
| compile-pass fixtures | ≥ 200 | 406 | PASS |
| stderr receipts == fail count | exact match | 196 == 196 | PASS |
| papers covered | ≥ 80 | 98 | PASS |
| audit scripts | ≥ 15 | 19 | PASS |
| fast-loop (cargo test --tests) | < 1s | 0.00s | PASS |
| ALIVE gate (ui_tests --ignored) | PASS | PASS (2/2) | PASS |
| cargo build --all-features | PASS | PASS | PASS |
| cargo clippy --all-features | PASS | PASS | PASS |
| cargo fmt --check | PASS | PASS (after auto-fix) | PASS |
| audit_features.sh | PASS | PASS | PASS |
| audit_no_stable_language.sh | PASS | PASS | PASS |
| audit_no_engine_creep.sh | PASS | PASS | PASS |
| audit_trybuild_receipts.sh | PASS | PASS | PASS |
| audit_paper_law_ledger.sh | PASS | PASS | PASS |
| May commits | any | 582 | PASS |

## Gate Verdict

**ALL CRITERIA MET — CROWN ALIVE**

## ALIVE Gate Detail

```
test compile_fail_fixtures ... ok
test compile_pass_fixtures ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

One compile-pass fixture (`object_lifecycle_phases.rs`) was updated to avoid a
known nightly E0391 cycle bug in `generic_const_exprs` impl blocks with concrete
const parameters. The fixture now uses the ergonomic type aliases
(`CreatedObject`, `ActiveObject`, `ModifiedObject`, `ArchivedObject`) and avoids
direct invocation of the phase-transition impls that trigger the compiler cycle.

The `audit_no_stable_language.sh` exclusion patterns were tightened to correctly
handle case-insensitive negating headings (`### No stable fallback`) and
meta-documentation references (`MSRV badges`), eliminating false positives.

## Artifact Counts

- compile-fail fixtures: 196
- compile-pass fixtures: 406
- .stderr receipts: 196 (100% coverage)
- audit scripts: 19
- papers in ledger: 98 (COVERED_BY | OUT_OF_SCOPE | DUPLICATE | PARTIAL)
- May 2026 commits: 582

## Tag

`wasm4pm-compat-paperlaw-crown-alive-004` — applied at this commit.
