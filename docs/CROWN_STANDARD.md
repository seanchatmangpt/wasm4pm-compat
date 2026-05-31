# CROWN_STANDARD — PAPERLAW_CROWN_ALIVE_004

This document defines what PAPERLAW_CROWN_ALIVE_004 means.

A release is PAPERLAW_CROWN_ALIVE_004 if and only if ALL of the following are true simultaneously.

---

## Crown Criteria

| # | Criterion                           | Required value      | Measurement                                |
|---|-------------------------------------|---------------------|--------------------------------------------|
| 1 | Paper families with type-law        | >= 80               | `docs/PAPER_COVERAGE_LEDGER.md` row count  |
| 2 | MISSING_TYPE_LAW count              | == 0                | `scripts/audit/audit_missing_type_law.sh`  |
| 3 | compile-pass fixtures               | >= 200              | `ls tests/ui/compile_pass/*.rs \| wc -l`  |
| 4 | compile-fail fixtures               | >= 160              | `ls tests/ui/compile_fail/*.rs \| wc -l`  |
| 5 | .stderr files == compile-fail count | == (criterion 4)    | `scripts/audit/audit_stderr_fixture_parity.sh` |
| 6 | Audit scripts present               | >= 20               | `ls scripts/audit/*.sh \| wc -l`          |
| 7 | Master audit gate passes            | exit 0              | `bash scripts/audit/audit_crown_gate_all.sh` |
| 8 | `cargo test --tests` passes         | exit 0              | CI / local run                             |
| 9 | `cargo clippy --all-features`       | exit 0              | CI / local run                             |
| 10| `cargo fmt --check` passes          | exit 0              | CI / local run                             |

All 10 criteria must be satisfied. There is no partial crown.

---

## Crown Tag

When all criteria pass, the release is sealed with:

```
git tag PAPERLAW_CROWN_ALIVE_004
```

The tag is immutable. It references the exact commit at which all 10 criteria first pass.

---

## What Crown Is Not

- Crown is NOT a performance milestone. No benchmark numbers are required.
- Crown is NOT about runtime behavior. This crate has no runtime — it is structure-only.
- Crown is NOT about downstream consumers. The crown belongs to the crate itself.
- Crown is NOT ALIVE_003 + features. Crown requires 500 receipt-bearing commits manufactured
  under the commit class system defined in `docs/CROWN_COMMIT_LAW.md`.

---

## Relationship to ALIVE Gates

| Gate                  | compile-pass | compile-fail | papers | audits | commits |
|-----------------------|--------------|--------------|--------|--------|---------|
| PAPERLAW_ALIVE_002    | ~30          | ~15          | ~20    | 0      | 2000+   |
| PAPERLAW_ALIVE_003    | 83           | 45           | ~40    | 0      | 100+    |
| PAPERLAW_CROWN_ALIVE_004 | >= 200   | >= 160       | >= 80  | >= 20  | 500     |

Crown is a strict superset of ALIVE_003.

---

## Entry State

*(Sealed at crown sprint entry — see checkpoint commit)*
