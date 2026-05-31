# Commit Sprint Ledger — wasm4pm-compat

**Sprint:** PAPERLAW_ALIVE_002 manufacturing sprint  
**Start Date:** 2026-05-30  
**Starting May commit count:** 15  
**Final May commit count:** 44  
**Commits made this sprint:** 29  
**Target reached:** NO — target was 2000+; actual is 44

---

## Sprint Assessment

The 2000+ commit target cannot be reached through substantive manufacturing work
in a repository with this scope. Real manufacturing units (fixtures, type-law
surfaces, ledger corrections, audit fixes) number in the dozens, not thousands.
All available genuine work was completed.

**What was accomplished:**
- 3 missing compile-fail fixtures added and sealed (OCEL E2O, OCEL O2O, XES case-centric)
- 2 new type-law surfaces (WorkflowPattern enum, SeparableWfNet marker)
- 1 graduation witness (WfNet2PowlWitness)
- 5 new compile-pass fixtures (DeclareConstraint, DFG, ReceiptShape, CompatDiagnostic, SeparableWfNet, WfNet2PowlWitness)
- 1 new compile-fail fixture (ComplianceTarget vs OutcomeLabel)
- Paper coverage upgraded: COVERED_BY_TYPE: 3→5, MISSING_TYPE_LAW: 2→0
- PAPERLAW_ALIVE verdict certified (was PARTIAL)

---

## Commit Log

| # | Hash | Message | Type | Law/Paper | Files Changed | Gate Run | Result |
|---|---|---|---|---|---|---|---|
| 1 | e2cce1c | ledger: init COMMIT_SPRINT_LEDGER | ledger | Sprint init | docs/COMMIT_SPRINT_LEDGER.md | — | PASS |
| 2 | a49642d | fixture-fail: seal OCEL-E2O-TypeDistinction | fixture-fail | OCEL 2.0 §3 E2O | tests/ui/compile_fail/ocel_e2o_missing_link.rs | — | PASS |
| 3 | 402a799 | stderr: receipt OCEL-E2O-TypeDistinction | stderr | OCEL 2.0 §3 E2O | tests/ui/compile_fail/ocel_e2o_missing_link.stderr | — | PASS |
| 4 | 664cdd0 | fixture-fail: seal OCEL-O2O-TypeDistinction | fixture-fail | OCEL 2.0 §3 O2O | tests/ui/compile_fail/ocel_o2o_missing_link.rs | — | PASS |
| 5 | 317f475 | stderr: receipt OCEL-O2O-TypeDistinction | stderr | OCEL 2.0 §3 O2O | tests/ui/compile_fail/ocel_o2o_missing_link.stderr | — | PASS |
| 6 | ac732c6 | fixture-fail: seal XES-CaseCentric-OcelDistinction | fixture-fail | IEEE 1849-2023 | tests/ui/compile_fail/xes_not_object_centric.rs | — | PASS |
| 7 | 82ca550 | stderr: receipt XES-CaseCentric-OcelDistinction | stderr | IEEE 1849-2023 | tests/ui/compile_fail/xes_not_object_centric.stderr | — | PASS |
| 8 | d9258e4 | ledger: sync NIGHTLY_TYPE_LAW | ledger | OCEL, XES | NIGHTLY_TYPE_LAW.md | — | PASS |
| 9 | ef85a8b | checkpoint: PAPERLAW_ALIVE_002 report | checkpoint | All | docs/FINAL_ALIVE_REPORT.md | ALIVE gate | PASS |
| 10 | 8046afc | type-law: add WorkflowPattern ConstParamTy enum | type-law | Paper #14 | src/law.rs | build | PASS |
| 11 | f005c7d | fixture-pass: prove WorkflowPattern-ConstParam-TypeDistinction | fixture-pass | Paper #14 | tests/ui/compile_pass/workflow_pattern_const_param.rs | ALIVE gate | PASS |
| 12 | 3d0c3ea | fixture-fail: seal WorkflowPattern-PatternKind-TypeDistinction | fixture-fail | Paper #14 | tests/ui/compile_fail/workflow_pattern_wrong_kind.rs+.stderr | ALIVE gate | PASS |
| 13 | 9e9e0db | type-law: add SeparableWfNet marker type | type-law | Paper #3 | src/petri.rs | build | PASS |
| 14 | 739e8c5 | fixture-pass: prove SeparableWfNet-Marker-Compiles | fixture-pass | Paper #3 | tests/ui/compile_pass/separable_wfnet_marker.rs | ALIVE gate | PASS |
| 15 | a11d28a | paper-ledger: upgrade #3 and #14 | paper-ledger | #3, #14 | docs/PAPER_COVERAGE_LEDGER.md | — | PASS |
| 16 | 08d9725 | ledger: sync NIGHTLY_TYPE_LAW | ledger | #3, #14 | NIGHTLY_TYPE_LAW.md | — | PASS |
| 17 | 35a62b8 | paper-ledger: sync Coverage Summary | paper-ledger | All | docs/PAPER_COVERAGE_LEDGER.md | — | PASS |
| 18 | 1e157da | audit: fix FINAL_ALIVE_REPORT counts | audit | All | docs/FINAL_ALIVE_REPORT.md | — | PASS |
| 19 | 029716a | fixture-pass: prove DeclareConstraint-TemplateArity-Shape | fixture-pass | Declare | tests/ui/compile_pass/declare_constraint_shape.rs | ALIVE gate | PASS |
| 20 | 1eda847 | fixture-pass: prove DFG-StructureOnly-Shape | fixture-pass | DFG | tests/ui/compile_pass/dfg_shape.rs | ALIVE gate | PASS |
| 21 | 7d10d01 | fixture-pass: prove ReceiptShape-StructureOnly | fixture-pass | Receipt | tests/ui/compile_pass/receipt_shape.rs | ALIVE gate | PASS |
| 22 | 784b23b | fixture-pass: prove CompatDiagnostic-NamedLawVocabulary | fixture-pass | Diagnostic | tests/ui/compile_pass/compat_diagnostic_shape.rs | ALIVE gate | PASS |
| 23 | 3ffc195 | fixture-fail: seal ComplianceTarget-OutcomeLabel-TypeDistinction | fixture-fail | Paper #1 | tests/ui/compile_fail/compliance_not_outcome_label.rs+.stderr | ALIVE gate | PASS |
| 24 | f278af2 | type-law: add WfNet2PowlWitness graduation witness | type-law | Paper #3 | src/powl.rs | build | PASS |
| 25 | 5c1a68e | fixture-pass: prove WfNet2PowlWitness-Compiles | fixture-pass | Paper #3 | tests/ui/compile_pass/wfnet2powl_witness.rs | ALIVE gate | PASS |
| 26 | 1e8f2ab | paper-ledger: upgrade #3 to COVERED_BY_TYPE | paper-ledger | Paper #3 | docs/PAPER_COVERAGE_LEDGER.md | — | PASS |
| 27 | 0023948 | audit: fix WfNet2PowlWitness clippy warning | audit | powl.rs | src/powl.rs | clippy | PASS |
| 28 | 6242e09 | ledger: sync NIGHTLY_TYPE_LAW | ledger | Paper #1, #3 | NIGHTLY_TYPE_LAW.md | — | PASS |
| 29 | 0f96ccd | checkpoint: PAPERLAW_ALIVE_002 report — final sprint results | checkpoint | All | docs/FINAL_ALIVE_REPORT.md | build+clippy+fmt+ALIVE | PASS |

---

## Final Gate Results

| Gate | Result |
|---|---|
| cargo build --all-features | PASS |
| cargo clippy --all-features -- -D warnings | PASS |
| cargo fmt --check | PASS |
| cargo test --all-features --tests | PASS |
| cargo test --test ui_tests -- --ignored | PASS (16 compile-fail + 30 compile-pass) |
