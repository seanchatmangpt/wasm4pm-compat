Checkpoint: PAPERLAW_PARTIAL_003
Commit count produced: 85
Current paper count: 24
Covered by type: 3
Covered by graduation: 5
Duplicate/background: 3
Out of scope: 10
Partial: 3
MISSING_TYPE_LAW: 0
Compile-pass fixtures: 34
Compile-fail fixtures: 35
.stderr receipts: 35
Fast loop result: 0.12s
ALIVE gate result: PASS
Clippy result: PASS
Fmt result: PASS
Feature audit: PASS
Stable-language audit: PASS
Engine-creep audit: PASS
Verdict: PAPERLAW_PARTIAL_003
Residuals:
  - compile-fail 35 < 40 required (need 5 more fixtures)
  - compile-pass 34 < 60 required (need 26 more fixtures)
  - papers 24 < 32 required (need 8 more papers scanned and ledgered)
Next recommended workflow: Run paperlaw-003-100-commit-sprint to close the 3 residuals:
  1. Add 8 papers to PAPER_COVERAGE_LEDGER.md (OCEL 2.0 spec, XES IEEE 1849,
     Inductive Miner, Declare/LTL, Alpha Miner, Log Skeleton, OC-Petri nets,
     alignment paper by Adriansyah et al.)
  2. Add 5 compile-fail fixtures with .stderr receipts (forged non-separable
     WF-net→POWL, OCPQ type mixing, XES→OCED without loss policy,
     ComplianceConstraintWitness violation, one additional law boundary)
  3. Add 26 compile-pass fixtures covering the gaps listed in ALIVE_003_CHECKPOINT.md
  Then re-run this gate to achieve PAPERLAW_ALIVE_003 and tag wasm4pm-compat-paperlaw-alive-003.
