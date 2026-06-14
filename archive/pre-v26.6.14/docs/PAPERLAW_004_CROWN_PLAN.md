# PAPERLAW_004_CROWN_PLAN

**Target:** PAPERLAW_CROWN_ALIVE_004
**Commit target:** 500 receipt-bearing commits
**Status:** PLANNING

---

## What is PAPERLAW_CROWN_ALIVE_004?

PAPERLAW_CROWN_ALIVE_004 is the crown certification milestone for the wasm4pm-compat crate.
It is achieved when all gate criteria in `docs/CROWN_GATE_CRITERIA.md` are satisfied simultaneously
and sealed by a tag `PAPERLAW_CROWN_ALIVE_004`.

Crown means: every process-evidence shape is backed by a named paper family, sealed by a
compile-time type law, witnessed by a trybuild fixture, and auditable by a process-mining script.

---

## Phase Breakdown

| Phase | Name                        | Commit classes                    | Target commits |
|-------|-----------------------------|-----------------------------------|---------------|
| 1     | Ledger & Planning           | ledger, docs-law                  | ~20            |
| 2     | Paper Coverage Expansion    | paper-ledger, paper-law           | ~80            |
| 3     | Type-Law Surfaces           | type-law                          | ~80            |
| 4     | Compile-Pass Fixtures       | fixture-pass                      | ~120           |
| 5     | Compile-Fail Fixtures       | fixture-fail, stderr              | ~100           |
| 6     | Witness & State Surfaces    | type-law, fixture-pass            | ~40            |
| 7     | Audit Scripts               | audit                             | ~20            |
| 8     | Checkpoint & Gate Sealing   | checkpoint, tag                   | ~10            |
| 9     | Residual / Overflow         | any class                         | ~30            |
| —     | **Total**                   |                                   | **~500**       |

---

## Metrics Dashboard (targets)

| Metric                    | Entry state | Crown target |
|---------------------------|-------------|--------------|
| Paper families covered    | (see entry) | >= 80        |
| MISSING_TYPE_LAW count    | TBD         | 0            |
| compile-pass fixtures     | 83          | >= 200       |
| compile-fail fixtures     | 45          | >= 160       |
| .stderr files matching    | (see entry) | == fail count|
| Audit scripts             | 0           | >= 20        |
| All proof gates pass      | TBD         | yes          |

---

## Key Documents

| Document                               | Purpose                                      |
|----------------------------------------|----------------------------------------------|
| `docs/CROWN_STANDARD.md`               | Formal crown definition and entry state      |
| `docs/CROWN_GATE_CRITERIA.md`          | Exact gate criteria for crown certification  |
| `docs/CROWN_COMMIT_LAW.md`             | Commit class system                          |
| `docs/PAPERLAW_004_COMMIT_ACCOUNTING.md` | Running tally by phase/class               |
| `docs/PAPERLAW_004_PAPER_TARGETS.md`   | Paper family targets                         |
| `docs/PAPERLAW_004_FIXTURE_TARGETS.md` | Fixture count targets by law family          |
| `docs/PAPERLAW_004_AUDIT_TARGETS.md`   | 20 audit scripts to add in Phase 7           |
| `docs/NEGATIVE_RECEIPTS.md`            | Index of all compile-fail fixtures           |

---

## Sequence

1. Seal crown entry state (checkpoint).
2. Expand paper coverage to 80+ families (paper-ledger + paper-law commits).
3. Add type-law surfaces for uncovered shapes.
4. Grow compile-pass fixtures to 200+.
5. Grow compile-fail fixtures to 160+; ensure every fail has a matching .stderr.
6. Add 20 audit scripts.
7. Run all proof gates; fix any failures.
8. Tag `PAPERLAW_CROWN_ALIVE_004`.
