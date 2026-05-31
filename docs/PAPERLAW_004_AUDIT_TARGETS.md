# PAPERLAW_004_AUDIT_TARGETS

20 audit scripts to add in Phase 7 of the PAPERLAW_CROWN_ALIVE_004 manufacturing sprint.

All scripts live in `scripts/audit/`. They are process-mining audit instruments — not test
runners. Each script verifies a structural law by static analysis of the codebase, not
by compiling or executing Rust code.

---

## Audit Script Registry

| # | Script name                            | What it audits                                              | Gate |
|---|----------------------------------------|-------------------------------------------------------------|------|
| 1 | `audit_missing_type_law.sh`            | Finds modules with no const-generic law surface             | MISSING_TYPE_LAW=0 |
| 2 | `audit_refusal_named_law.sh`           | Verifies every Refusal carries a named reason type          | no bare InvalidInput |
| 3 | `audit_loss_policy_coverage.sh`        | Verifies every lossy transform has a LossPolicy             | no silent loss |
| 4 | `audit_loss_report_coverage.sh`        | Verifies every non-refusing lossy path emits a LossReport   | no silent loss |
| 5 | `audit_unsafe_code_absent.sh`          | Confirms forbid(unsafe_code) is present in lib.rs           | unsafe=0 |
| 6 | `audit_feature_count.sh`               | Verifies exactly 3 public Cargo features                    | features==3 |
| 7 | `audit_doctest_coverage.sh`            | Counts public fns with and without doctests                 | coverage report |
| 8 | `audit_stderr_fixture_parity.sh`       | Verifies compile_fail .rs count == .stderr count            | parity |
| 9 | `audit_fixture_law_header.sh`          | Verifies every fixture has a law-name comment in header     | all named |
| 10| `audit_witness_exhaustive.sh`          | Lists all Witness impls; flags any without paper citation   | all cited |
| 11| `audit_paper_ledger_coverage.sh`       | Cross-checks paper ledger against module witness list       | no orphan modules |
| 12| `audit_engine_creep.sh`                | Scans for discovery/conformance/replay logic in crate       | engine_creep=0 |
| 13| `audit_pub_type_rustdoc.sh`            | Verifies every public type has a rustdoc comment            | coverage report |
| 14| `audit_admission_only_path.sh`         | Verifies no admitted evidence created outside Admit::admit  | path=only |
| 15| `audit_state_token_sealed.sh`          | Verifies all state tokens are empty enums (no fields)       | sealed |
| 16| `audit_graduation_boundary.sh`         | Verifies no engine logic in wasm4pm feature                 | boundary clean |
| 17| `audit_between01_const_bounds.sh`      | Verifies all metric types use Between01 const bounds        | all bounded |
| 18| `audit_no_stable_fallback.sh`          | Verifies no #[cfg(not(feature = "nightly"))] fallback       | nightly-only |
| 19| `audit_paper_law_commit_count.sh`      | Counts paper-law commits in git log                         | count report |
| 20| `audit_crown_gate_all.sh`              | Master gate: runs all audits, emits PASS/FAIL for each      | all pass |

---

## Script Interface

Each audit script must:

1. Exit 0 on PASS, exit 1 on FAIL.
2. Print a single summary line: `PASS: <script_name>: <metric>` or `FAIL: <script_name>: <detail>`.
3. Accept `--verbose` for detailed output.
4. Be runnable with `bash scripts/audit/<name>.sh` from the repo root.

---

## Master Gate Script

`scripts/audit/audit_crown_gate_all.sh` is the master gate. It:

1. Runs all 19 individual audit scripts.
2. Collects exit codes.
3. Prints a summary table.
4. Exits 0 only if all 19 audits pass.

The crown certification requires `bash scripts/audit/audit_crown_gate_all.sh` to exit 0.

---

## Audit Commit Class

Each audit script addition uses commit class `audit`:
```
audit: add audit_missing_type_law.sh — MISSING_TYPE_LAW gate
audit: add audit_refusal_named_law.sh — named-law refusal gate
```

---

## Phase 7 Sequence

Scripts are added in dependency order:
1. Foundational (unsafe, features, nightly): scripts 3, 5, 6, 18
2. Structural law (refusal, loss, admission, state): scripts 2, 3, 4, 14, 15
3. Coverage (doctest, rustdoc, witness, paper): scripts 7, 10, 11, 13
4. Fixtures (stderr parity, fixture headers): scripts 8, 9
5. Boundaries (engine creep, graduation, graduation): scripts 12, 16
6. Metrics (Between01, missing type law): scripts 1, 17
7. History (paper-law commit count): script 19
8. Master gate: script 20
