# PAPERLAW_ALIVE_003 — Final Baseline

**Sealed:** 2026-05-31
**Tag:** `wasm4pm-compat-paperlaw-alive-003`

## Exact Counts at Seal

| Criterion | Required | Actual | Status |
|---|---|---|---|
| compile_fail fixtures | ≥ 40 | 45 | PASS |
| compile_pass fixtures | ≥ 60 | 83 | PASS |
| .stderr receipts == fail | stderr == fail | 45 == 45 | PASS |
| papers (COVERED_BY/OUT_OF_SCOPE/DUPLICATE/PARTIAL) | ≥ 32 | 47 | PASS |
| fast-loop (cargo test --all-features --tests) | < 1s | 0.081s | PASS |
| audit_features | PASS | PASS | PASS |
| audit_no_stable_language | PASS | PASS | PASS |
| audit_no_engine_creep | PASS | PASS | PASS |
| audit_trybuild_receipts | PASS | PASS | PASS |
| audit_paper_law_ledger | PASS | PASS | PASS |
| trybuild gate (compile_fail + compile_pass) | all PASS | 2 passed; 0 failed | PASS |

## Gate Summary

All ALIVE_003 criteria met. This baseline is the crown baseline for PAPERLAW_CROWN_ALIVE_004.

## Fix Applied at Seal

`tests/ui/compile_pass/graduation_wasm4pm_bridge_trait.rs` was incorrectly placed in the
`compile_pass/` directory. It uses `wasm4pm_compat::graduation` which requires `--features wasm4pm`
and belongs in `compile_pass_wasm4pm/`. Moved to `tests/ui/compile_pass_wasm4pm/` before sealing.

## Crown Baseline Statement

This is the minimum contract that PAPERLAW_CROWN_ALIVE_004 must exceed:
- fail >= 45
- pass >= 83
- stderr == fail (45)
- papers >= 47
- fast-loop < 1s
- all 5 audits PASS
