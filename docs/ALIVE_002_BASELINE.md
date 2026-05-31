# PAPERLAW_ALIVE_002 Baseline Checkpoint

**Date recorded:** 2026-05-31  
**Purpose:** Phase 0 baseline lock before PAPERLAW_003_100_COMMIT_SPRINT.

---

## Exact Command Outputs

```
$ git log --oneline -1
2e85d44 audit: add anti-regression scripts (features, stable-language, engine-creep, receipts, paper-ledger)

$ git tag | grep alive
wasm4pm-compat-paperlaw-alive-002
wasm4pm-compat-typelaw-alive-001

$ ls tests/ui/compile_fail/*.rs | wc -l
      16

$ ls tests/ui/compile_pass/*.rs | wc -l
      30
```

---

## Baseline State

| Metric | Value |
|---|---|
| Baseline commit | `2e85d44` |
| ALIVE tags present | `wasm4pm-compat-paperlaw-alive-002`, `wasm4pm-compat-typelaw-alive-001` |
| compile_fail fixtures | 16 |
| compile_pass fixtures | 30 |
| .stderr receipts | 16 (1:1 with compile_fail) |
| Papers in corpus | 20 |
| Fast dev loop (cargo test --tests) | 0.07 s |
| ALIVE gate (cargo test --test ui_tests) | 10.74 s |

---

## Anti-Regression Scripts (Phase 0)

Five scripts created in `scripts/` and confirmed passing against this baseline:

| Script | Checks |
|---|---|
| `audit_features.sh` | Cargo.toml has exactly [formats, strict, wasm4pm]; no 'nightly' feature |
| `audit_no_stable_language.sh` | No affirmative stable-language claims in src/ or docs/ |
| `audit_no_engine_creep.sh` | No engine function/struct names (discover, replay_log, align_trace, token_replay) in src/ |
| `audit_trybuild_receipts.sh` | Every compile_fail .rs has a matching .stderr receipt |
| `audit_paper_law_ledger.sh` | MISSING_TYPE_LAW count = 0 in PAPER_COVERAGE_LEDGER.md |

All 5 scripts returned PASS on baseline commit `2e85d44`.

---

## Invariant Summary

- Nightly-only crate; `rust-toolchain.toml` pins nightly unconditionally.
- `#![forbid(unsafe_code)]` enforced.
- Exactly 3 public Cargo features: `formats`, `strict`, `wasm4pm`.
- Every refusal carries a specific named law (no `InvalidInput` catch-alls).
- No engine logic (discovery, conformance, replay) in this crate — graduates to `wasm4pm`.
- Type law lives in public modules; never behind a cfg gate.
