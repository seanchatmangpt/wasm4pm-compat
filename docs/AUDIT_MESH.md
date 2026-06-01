# AUDIT_MESH — Audit Script Index

All audit scripts live in `scripts/`. The crown audit runner (`crown_audit_runner.sh`)
executes every script and exits 1 if any hard-fail script fails.

**Current count:** 20 scripts (Gates 6 and 7 require >= 20 and all-pass respectively)

---

## Script Index

| Script | Exit behavior | What it checks |
|--------|---------------|----------------|
| `audit_crosswalk_links.sh` | exit 0 (soft warn) | NIGHTLY_TYPE_LAW.md table rows have type + pass fixture + fail fixture columns; warns on incomplete rows |
| `audit_doctest_disabled.sh` | exit 1 (hard fail) | Cargo.toml [lib] section has `doctest = false`; fails if missing — prevents doctest storm from nightly features |
| `audit_features.sh` | exit 1 (hard fail) | Cargo.toml has exactly `[formats, strict, wasm4pm]` features; fails if `nightly` feature exists or any expected feature is missing |
| `audit_graduation_boundaries.sh` | exit 1 (hard fail) | GRADUATION_BOUNDARIES.md exists and src/lib.rs exports no execution engine APIs (Miner/Checker/Replayer/Aligner); fails if engine API found |
| `audit_id_newtypes.sh` | exit 0 (soft warn) | src/ids.rs uses struct newtypes (e.g., `struct EventId`) rather than type aliases; warns if type aliases found |
| `audit_metric_bounds.sh` | exit 1 (hard fail) | src/law.rs has `Between01` with `NUM<=DEN` and `DEN>0` bounds; fails if either bound is missing |
| `audit_need9_law.sh` | exit 1 (hard fail) | src/law.rs has `ConditionCell` with `BITS<=8` Require bound; fails if the law is missing or was removed |
| `audit_no_algorithm_exports.sh` | exit 1 (hard fail) | src/lib.rs has no `pub use` of Miner/Checker/Replayer/Aligner; fails if any engine algorithm name is publicly re-exported |
| `audit_no_engine_creep.sh` | exit 1 (hard fail) | src/ contains no function or struct names that imply engine execution (discover, replay, align, etc.); fails if engine names found |
| `audit_no_stable_language.sh` | exit 1 (hard fail) | src/ and docs/ contain no affirmative stable-language claims (MSRV not negated, "stable build", "stable Rust supported"); fails if found |
| `audit_no_unsealed_refusal.sh` | exit 1 (hard fail) | src/admission.rs has no `InvalidInput` or `GenericError` variant definitions; fails if catch-all refusal reason found |
| `audit_paper_law_ledger.sh` | exit 1 (hard fail) | docs/PAPER_COVERAGE_LEDGER.md has no outstanding `MISSING_TYPE_LAW` entries; fails if any remain (outside the allowed header context) |
| `audit_pass_fail_pairs.sh` | exit 0 (soft warn) | For each compile-fail fixture, checks if a corresponding compile-pass fixture exists; reports unmatched pairs but does not fail |
| `audit_projection_loss.sh` | exit 1 (hard fail) | src/loss.rs has `LossPolicy`, `ProjectionName`, and `LossReport`; fails if any of the three are missing |
| `audit_receipt_chain.sh` | exit 1 (hard fail) | src/receipt.rs has `ReceiptEnvelope` with `witness`, `digest`, and `replay_hint` fields; fails if any field is missing |
| `audit_stderr_quality.sh` | exit 0 (soft warn) | For each `.stderr` file, checks it contains `"error[E"` (a real Rust error code); warns if empty or missing error code |
| `audit_trybuild_receipts.sh` | exit 1 (hard fail) | Every compile-fail `.rs` fixture has a matching `.stderr` receipt file; fails if any `.rs` is missing its `.stderr` |
| `audit_witness_markers.sh` | exit 1 (hard fail) | src/witness.rs has at least 5 witness structs; fails if count < 5 |
| `crown_audit_runner.sh` | exit 1 if any hard fail | Runs ALL audit scripts; outputs a summary table; exits 1 if any hard-fail script exits 1 |

---

## Hard fail vs. soft warn

**Hard fail (exit 1):** The audit identifies a structural defect. Crown Gate 7 cannot pass while
any hard-fail audit fails. These audits enforce invariants that must hold for the crate to be
considered well-formed.

**Soft warn (exit 0):** The audit reports a gap or inconsistency but does not block the crown.
These are advisory — they surface incompleteness without blocking the gate.

---

## Running all audits

```bash
bash scripts/crown_audit_runner.sh
```

Expected output (all pass):
```
PASS audit_doctest_disabled
PASS audit_features
PASS audit_graduation_boundaries
PASS audit_metric_bounds
PASS audit_need9_law
PASS audit_no_algorithm_exports
PASS audit_no_engine_creep
PASS audit_no_stable_language
PASS audit_no_unsealed_refusal
PASS audit_paper_law_ledger
PASS audit_projection_loss
PASS audit_receipt_chain
PASS audit_trybuild_receipts
PASS audit_witness_markers
WARN audit_crosswalk_links (soft)
WARN audit_id_newtypes (soft)
WARN audit_pass_fail_pairs (soft)
WARN audit_stderr_quality (soft)
ALL HARD AUDITS PASS
```

Exit 0 from `crown_audit_runner.sh` satisfies Crown Gate 7.

---

## Running a single audit

```bash
bash scripts/audit_trybuild_receipts.sh
bash scripts/audit_no_engine_creep.sh
```

---

## Adding new audits

New audits follow the naming pattern: `audit_<what_it_checks>.sh`.

- Hard-fail audits must exit 1 when the invariant is violated.
- Soft-warn audits must exit 0 always and prefix any warning lines with `WARN:`.
- All audits must be idempotent (safe to re-run on an unchanged codebase).
- All audits must print a `PASS: audit_<name>: <summary>` line on success.
