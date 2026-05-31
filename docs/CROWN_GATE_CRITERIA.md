# CROWN_GATE_CRITERIA — Exact Gate Criteria for PAPERLAW_CROWN_ALIVE_004

This document states the exact, machine-verifiable gate criteria for PAPERLAW_CROWN_ALIVE_004.
All 10 gates must pass simultaneously on the same commit to award the crown.

---

## Gate 1 — Paper Coverage

**Criterion:** >= 80 paper families with type-law surfaces are registered.

**Measurement command:**
```bash
grep -c '^|' docs/PAPER_COVERAGE_LEDGER.md
# subtract header rows; count must be >= 80
```

**Pass condition:** registered paper count >= 80

**Fail condition:** registered paper count < 80

**Remediation:** Add `paper-ledger` + `paper-law` commits for missing families.

---

## Gate 2 — No Missing Type Law

**Criterion:** Zero modules have a paper registration without a corresponding type-law surface.

**Measurement command:**
```bash
bash scripts/audit/audit_missing_type_law.sh
```

**Pass condition:** exit 0, prints `PASS: audit_missing_type_law: MISSING_TYPE_LAW=0`

**Fail condition:** exit 1, lists modules with missing type-law surfaces.

**Remediation:** Add `type-law` commits for each listed module.

---

## Gate 3 — compile-pass Fixtures

**Criterion:** >= 200 compile-pass trybuild fixtures.

**Measurement command:**
```bash
ls tests/ui/compile_pass/*.rs | wc -l
```

**Pass condition:** count >= 200

**Fail condition:** count < 200

**Remediation:** Add `fixture-pass` commits.

---

## Gate 4 — compile-fail Fixtures

**Criterion:** >= 160 compile-fail trybuild fixtures.

**Measurement command:**
```bash
ls tests/ui/compile_fail/*.rs | wc -l
```

**Pass condition:** count >= 160

**Fail condition:** count < 160

**Remediation:** Add `fixture-fail` + `stderr` commits.

---

## Gate 5 — .stderr Parity

**Criterion:** Every compile-fail fixture has a corresponding .stderr file with the expected diagnostic.

**Measurement command:**
```bash
bash scripts/audit/audit_stderr_fixture_parity.sh
```

**Pass condition:** exit 0, prints `PASS: audit_stderr_fixture_parity: .rs count == .stderr count`

**Fail condition:** exit 1, lists fixtures missing .stderr files.

**Remediation:** Add `stderr` commits for each listed fixture.

---

## Gate 6 — Audit Scripts

**Criterion:** >= 20 audit scripts present in `scripts/audit/`.

**Measurement command:**
```bash
ls scripts/audit/*.sh | wc -l
```

**Pass condition:** count >= 20

**Fail condition:** count < 20

**Remediation:** Add `audit` commits.

---

## Gate 7 — Master Audit Gate

**Criterion:** All audit scripts pass.

**Measurement command:**
```bash
bash scripts/audit/audit_crown_gate_all.sh
```

**Pass condition:** exit 0

**Fail condition:** exit 1, prints failing audit names.

**Remediation:** Fix the structural issues identified by failing audits.

---

## Gate 8 — Cargo Tests

**Criterion:** `cargo test --all-features --tests` passes.

**Measurement command:**
```bash
cargo test --all-features --tests
```

**Pass condition:** exit 0

**Fail condition:** exit non-zero

**Remediation:** Fix failing tests.

---

## Gate 9 — Clippy

**Criterion:** `cargo clippy --all-features -- -D warnings` passes.

**Measurement command:**
```bash
cargo clippy --all-features -- -D warnings
```

**Pass condition:** exit 0

**Fail condition:** exit non-zero, prints clippy warnings treated as errors.

**Remediation:** Fix all clippy warnings.

---

## Gate 10 — Format

**Criterion:** `cargo fmt --check` passes (no formatting deviations).

**Measurement command:**
```bash
cargo fmt --check
```

**Pass condition:** exit 0

**Fail condition:** exit non-zero, lists files with formatting issues.

**Remediation:** Run `cargo fmt` and commit the result.

---

## Combined Gate Check

To check all gates at once (before tagging):

```bash
set -e
bash scripts/audit/audit_crown_gate_all.sh
cargo test --all-features --tests
cargo clippy --all-features -- -D warnings
cargo fmt --check
echo "ALL CROWN GATES PASS"
```

If this script exits 0, all 10 criteria are satisfied and the crown tag may be applied:

```bash
git tag PAPERLAW_CROWN_ALIVE_004
```

---

## Gate Failure Is Not a Discrepancy

Following the Van der Aalst Constitution: if any gate fails, that is a defect, not a discrepancy.
The crown cannot be awarded with exceptions, waivers, or partial passes.
