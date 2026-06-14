# PAPERLAW_ALIVE_004_CRITERIA — Exact Gate Criteria for PAPERLAW_CROWN_ALIVE_004

This document states the exact, machine-verifiable gate criteria for PAPERLAW_CROWN_ALIVE_004.
All 10 gates must pass simultaneously on the same commit to award the crown.

The canonical source of truth for the gate table is `docs/CROWN_STANDARD.md` and
`docs/CROWN_GATE_CRITERIA.md`. This document adds measurement commands, current
status, and remediation guidance.

---

## Gate Summary

| Gate | Criterion | Required | Measurement | Status (2026-05-31) |
|------|-----------|----------|-------------|---------------------|
| 1 | Paper families with type-law surfaces | >= 80 | `grep -c '^|' docs/PAPER_COVERAGE_LEDGER.md` | See ledger |
| 2 | MISSING_TYPE_LAW entries | == 0 | `bash scripts/audit/audit_missing_type_law.sh` | See audit |
| 3 | compile-pass fixtures | >= 200 | `ls tests/ui/compile_pass/*.rs \| wc -l` | 406 (PASS) |
| 4 | compile-fail fixtures | >= 160 | `ls tests/ui/compile_fail/*.rs \| wc -l` | 196 (PASS) |
| 5 | .stderr parity | == compile-fail count | `bash scripts/audit/audit_stderr_fixture_parity.sh` | 196 = 196 (PASS) |
| 6 | Audit scripts present | >= 20 | `ls scripts/*.sh \| wc -l` | 20 (PASS) |
| 7 | Master audit gate | exit 0 | `bash scripts/crown_audit_runner.sh` | See runner |
| 8 | cargo test passes | exit 0 | `cargo test --all-features --tests` | See CI |
| 9 | cargo clippy passes | exit 0 | `cargo clippy --all-features -- -D warnings` | See CI |
| 10 | cargo fmt check passes | exit 0 | `cargo fmt --check` | See CI |

---

## Gate 1 — Paper Coverage

**Criterion:** >= 80 paper families with type-law surfaces are registered.

**Measurement:**
```bash
grep -c '^|' docs/PAPER_COVERAGE_LEDGER.md
# subtract header rows (2 per table); count must be >= 80
```

**Pass condition:** registered paper count >= 80

**Remediation:** Add `paper-ledger` + `paper-law` commits for missing families.
See `docs/PAPERLAW_004_PAPER_TARGETS.md` for the gap analysis.

---

## Gate 2 — No Missing Type Law

**Criterion:** Zero modules have a paper registration without a corresponding type-law surface.

**Measurement:**
```bash
bash scripts/audit/audit_missing_type_law.sh
```

**Pass condition:** exit 0, prints `PASS: audit_missing_type_law: MISSING_TYPE_LAW=0`

**Fail condition:** exit 1, lists modules with missing type-law surfaces.

**Remediation:** Add `type-law` commits for each listed module.

---

## Gate 3 — compile-pass Fixtures

**Criterion:** >= 200 compile-pass trybuild fixtures.

**Measurement:**
```bash
ls tests/ui/compile_pass/*.rs | wc -l
```

**Pass condition:** count >= 200

**Current:** 406 — Gate 3 is satisfied.

**Remediation (if ever falls below 200):** Add `fixture-pass` commits.

---

## Gate 4 — compile-fail Fixtures

**Criterion:** >= 160 compile-fail trybuild fixtures.

**Measurement:**
```bash
ls tests/ui/compile_fail/*.rs | wc -l
```

**Pass condition:** count >= 160

**Current:** 196 — Gate 4 is satisfied.

**Remediation (if ever falls below 160):** Add `fixture-fail` + `stderr` commits.
See `docs/NEGATIVE_RECEIPTS.md` for the complete fixture index.

---

## Gate 5 — .stderr Parity

**Criterion:** Every compile-fail fixture has a corresponding .stderr file.

**Measurement:**
```bash
bash scripts/audit/audit_stderr_fixture_parity.sh
# or manual: diff <(ls tests/ui/compile_fail/*.rs | sed 's/.rs$//') <(ls tests/ui/compile_fail/*.stderr | sed 's/.stderr$//')
```

**Pass condition:** exit 0, `.rs count == .stderr count`

**Current:** 196 .rs and 196 .stderr — Gate 5 is satisfied.

**Remediation:** For each `.rs` without a `.stderr`, add a `stderr` commit with the
exact expected compiler diagnostic. The diagnostic must contain `error[E` — a real
Rust error code, not a generic message.

---

## Gate 6 — Audit Scripts

**Criterion:** >= 20 audit scripts present in `scripts/`.

**Measurement:**
```bash
ls scripts/*.sh | wc -l
```

**Pass condition:** count >= 20

**Current:** 20 — Gate 6 is satisfied.

**Remediation:** Add new `audit` commits. See `docs/AUDIT_MESH.md` for the
full index of existing scripts and their purposes.

---

## Gate 7 — Master Audit Gate

**Criterion:** All hard-fail audit scripts pass.

**Measurement:**
```bash
bash scripts/crown_audit_runner.sh
```

**Pass condition:** exit 0

**Fail condition:** exit 1, prints names of failing audits.

**Remediation:** Fix the structural issue identified by the failing audit.
All 14 hard-fail audits must pass (see `docs/AUDIT_MESH.md`).

---

## Gate 8 — Cargo Tests

**Criterion:** `cargo test --all-features --tests` passes.

**Measurement:**
```bash
cargo test --all-features --tests
```

**Pass condition:** exit 0

**Remediation:** Fix failing tests. The test suite includes:
- Unit and integration tests in `src/` and `tests/`
- Trybuild tests are a separate surface (run with `--ignored`)

---

## Gate 9 — Clippy

**Criterion:** `cargo clippy --all-features -- -D warnings` passes with no warnings.

**Measurement:**
```bash
cargo clippy --all-features -- -D warnings
```

**Pass condition:** exit 0

**Remediation:** Fix all clippy lints. `-D warnings` treats every warning as an error.

---

## Gate 10 — Format

**Criterion:** `cargo fmt --check` passes (no formatting deviations).

**Measurement:**
```bash
cargo fmt --check
```

**Pass condition:** exit 0

**Remediation:** Run `cargo fmt` and commit the result.

---

## Combined Gate Check

To verify all gates before tagging:

```bash
set -e
bash scripts/crown_audit_runner.sh
cargo test --all-features --tests
cargo clippy --all-features -- -D warnings
cargo fmt --check
echo "ALL CROWN GATES PASS"
```

If this exits 0, tag:

```bash
git tag PAPERLAW_CROWN_ALIVE_004
```

---

## Gate Semantics

### Gate failure is a defect

Following the Van der Aalst Constitution (see `docs/ANTI_REGRESSION_LAWS.md`):
if any gate fails, that is a **defect**, not a discrepancy. The crown cannot be
awarded with exceptions, waivers, or partial passes.

### Gates are additive, not substitutable

All 10 gates are independently required. Passing 9 of 10 is not a partial crown;
it is no crown. There is no partial crown.

### Gates measure structure, not behavior

Gates 3-6 measure counts and structural invariants. They do not measure runtime
performance, benchmark results, or downstream consumer behavior. The crown belongs
to the crate's structural completeness.

---

## Relationship to Prior ALIVE Gates

| Gate | compile-pass | compile-fail | papers | audits |
|------|--------------|--------------|--------|--------|
| PAPERLAW_ALIVE_002 | ~30 | ~15 | ~20 | 0 |
| PAPERLAW_ALIVE_003 | 83 | 45 | ~40 | 0 |
| PAPERLAW_CROWN_ALIVE_004 | >= 200 | >= 160 | >= 80 | >= 20 |
| **Current (2026-05-31)** | **406** | **196** | TBD | **20** |

Crown is a strict superset of ALIVE_003. ALIVE_003 passing is necessary but not
sufficient for the crown.
