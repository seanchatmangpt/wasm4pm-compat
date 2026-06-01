# SPARC Validation Queries — Complete Index

Checkpoint validation SPARQL queries for PAPERLAW ALIVE certification. Enforces hostile assumptions from process-mining-chicago-tdd doctrine.

**Version:** 0.1  
**Created:** 2026-06-01  
**Status:** Ready for Integration  

---

## Quick Start

Run all four validation gates:

```bash
cd /Users/sac/wasm4pm-compat/ggen

# Gate 1: Receipt evidence
ggen query queries/audit-no-file-count-alive.rq --format sparql-results
# Expected: Result: false (pass) | true (fail)

# Gate 2: Law coverage
ggen query queries/audit-no-commit-count-gate.rq --format sparql-results
# Expected: Result: false (pass) | true (fail)

# Gate 3: Closure traceability
ggen query queries/audit-gap-closure-claims-have-gap-id.rq --format sparql-results
# Expected: Result: (empty - pass) | rows (fail)

# Gate 4: Gap remediation
ggen query queries/audit-critical-gaps-have-remediation.rq --format sparql-results
# Expected: Result: (empty - pass) | rows (fail)
```

**Checkpoint passes if:** All four gates return success values.

---

## File Structure

```
/Users/sac/wasm4pm-compat/ggen/
├── queries/
│   ├── audit-no-file-count-alive.rq                (Gate 1: ASK)
│   ├── audit-no-commit-count-gate.rq               (Gate 2: ASK)
│   ├── audit-gap-closure-claims-have-gap-id.rq     (Gate 3: SELECT)
│   ├── audit-critical-gaps-have-remediation.rq     (Gate 4: SELECT)
│   └── README-VALIDATION-QUERIES.md                (Full documentation)
├── VALIDATION-CHECKPOINT-ARCHITECTURE.txt          (Architecture overview)
├── VALIDATION-QUERY-REFERENCE.md                   (Query reference guide)
└── INDEX-VALIDATION-QUERIES.md                     (This file)
```

---

## Validation Gates at a Glance

| Gate | Query | Type | Validates | Pass Condition |
|------|-------|------|-----------|---|
| **1: Receipt Evidence** | `audit-no-file-count-alive.rq` | ASK | Checkpoints backed by receipts (not file count) | Result = `false` |
| **2: Law Coverage** | `audit-no-commit-count-gate.rq` | ASK | Laws have fixtures (not commit count) | Result = `false` |
| **3: Closure Traceability** | `audit-gap-closure-claims-have-gap-id.rq` | SELECT | Every closure references a gap | Result = `(empty)` |
| **4: Gap Remediation** | `audit-critical-gaps-have-remediation.rq` | SELECT | Critical gaps closed/accepted/PARTIAL | Result = `(empty)` |

---

## Gate 1: Receipt Evidence — `audit-no-file-count-alive.rq`

**Purpose:** Validate that ALIVE checkpoint claims are backed by explicit trybuild receipt evidence, not file count metrics.

**Validates:**
- Every `audit:CheckpointClaim` with `passedAllGates = true` MUST have linked `compat:Receipt` instances
- Receipts must be accessible via `audit:CommitEvidence.carriesReceipt` property chain
- File count (number of source files) is NOT a valid substitute for receipt evidence

**Returns:**
- `false` (PASS) — All checkpoints have receipt evidence
- `true` (FAIL) — At least one checkpoint has zero receipts

**Remediation:**
If gate fails, link receipts to the checkpoint's commit:
```turtle
audit:CommitEvidence_<HASH>
    a audit:CommitEvidence ;
    audit:commitHash "<HASH>" ;
    audit:carriesReceipt compat:Receipt_<ID_1> ,
                         compat:Receipt_<ID_2> .
```

**Related Hostile Assumption:**
- `HA_ProofGatesCanPass_NonConforming` — Gates passing without receipt evidence indicate phantom certification

**Size:** 1.5K | **Lines:** ~35

---

## Gate 2: Law Coverage — `audit-no-commit-count-gate.rq`

**Purpose:** Validate that law coverage is verified by law-to-fixture RDF mapping, not commit count metrics.

**Validates:**
- Every `compat:CompileFailLaw` in scope MUST be linked to an `audit:AuditExecutable` fixture
- The fixture MUST have `audit:fixtureFile` (path to .rs) and `audit:expectedStderr` (path to .stderr)
- Commit count (number of commits since last release) is NOT a substitute for law coverage

**Returns:**
- `false` (PASS) — All laws in scope have linked fixtures
- `true` (FAIL) — At least one law has no linked fixture

**Remediation:**
If gate fails, add fixture for the uncovered law:
```turtle
audit:AuditExecutable_<ID>
    a audit:AuditExecutable ;
    audit:fixtureFile "tests/ui/compile_fail/<law_name>.rs" ;
    audit:expectedStderr "tests/ui/compile_fail/<law_name>.stderr" ;
    audit:instantiatedBy audit:TryBuildCompileFailTemplate .
```

**Related Hostile Assumption:**
- `HA_DeclaredPipelineNotReal` — Declared law coverage (metric: "N laws covered") differs from actual law-fixture mapping

**Size:** 2.0K | **Lines:** ~40

---

## Gate 3: Closure Traceability — `audit-gap-closure-claims-have-gap-id.rq`

**Purpose:** Validate that every gap closure claim is traceable to a documented gap. No orphaned or phantom closures.

**Validates:**
- Every `audit:ClosureClaim` instance MUST have an `audit:closesGap` property
- The target MUST be an `audit:Gap` with `audit:gapId` defined
- Orphaned closure claims (no gap reference) indicate undocumented remediation

**Returns:**
- Empty result set (PASS) — All closure claims reference a gap
- Rows returned (FAIL) — Orphaned closure claims found

**Result Columns:**
- `?closureClaim` — URI of orphaned claim
- `?closingCommit` — Commit hash (undocumented closure)
- `?closureMethod` — Method (fixture-added, law-proved, path-proved-reachable, reclassified)

**Remediation:**
For each orphaned claim, add the missing link:
```turtle
audit:ClosureClaim_<ID>
    audit:closesGap audit:Gap_<ID> .
```

**Related Hostile Assumption:**
- `HA_VariantExplosion` — Hidden rework and phantom closures silently skipped in the audit log

**Size:** 1.4K | **Lines:** ~35

---

## Gate 4: Gap Remediation — `audit-critical-gaps-have-remediation.rq`

**Purpose:** Validate that critical and major gaps are not left open without documentation. Every gap must be closed, accepted, or marked PARTIAL.

**Validates:**
- Every `audit:Gap` with severity `critical` or `major` MUST satisfy ONE of:
  1. Have a `audit:ClosureClaim` with `audit:closureTimestamp` (gap is closed), OR
  2. Have `audit:gapCategory = "accepted-debt"` (gap is accepted as tech debt), OR
  3. Have `audit:remediationPlan` containing `"PARTIAL"` keyword (gap is in progress)
- If none of these conditions hold, the gap is open and blocks certification

**Returns:**
- Empty result set (PASS) — All critical/major gaps are remediated
- Rows returned (FAIL) — Open critical/major gaps found

**Result Columns:**
- `?gapId` — Gap identifier (GAP_XXX)
- `?gapCategory` — Category (missing-fixture, uncovered-law, unresolved-todo, etc.)
- `?gapSeverity` — critical | major
- `?remediationPlan` — Human-readable plan
- `?hasClosureClaim` — true | false
- `?acceptedAsDebt` — true | false

**Remediation Paths:**

**Path A: Close the gap**
```turtle
audit:ClosureClaim_<ID>
    a audit:ClosureClaim ;
    audit:closesGap audit:Gap_<ID> ;
    audit:closingCommit "<HASH>" ;
    audit:closureMethod "law-proved" ;
    audit:closureTimestamp "2026-06-01T12:00:00Z"^^xsd:dateTime .
```

**Path B: Accept as technical debt**
```turtle
audit:Gap_<ID>
    audit:gapCategory "accepted-debt" ;
    audit:remediationPlan "Accepted as technical debt. Rationale: ... Target resolution: ALIVE_005." .
```

**Path C: Document as PARTIAL**
```turtle
audit:Gap_<ID>
    audit:remediationPlan "PARTIAL_REMEDIATION: Basic structure implemented. Pending: ... Target: ALIVE_005." .
```

**Related Hostile Assumptions:**
- `HA_ReleaseMayOccurFromInvalid` — Release can silently occur from incomplete audit state
- `HA_ReceiptsEmittedOutOfCycle` — Gap remediation can be claimed outside lawful object lifecycle

**Size:** 3.3K | **Lines:** ~65

---

## Hostile Assumptions Operationalized

| Assumption | Gate(s) | Rule |
|-----------|---------|------|
| **HA_DeclaredPipelineNotReal** | Gate 2 | Law coverage via RDF, not metric-driven (commits/files) |
| **HA_ReceiptsEmittedOutOfCycle** | Gates 1, 4 | Receipts explicitly linked; gaps have explicit status |
| **HA_ProofGatesCanPass_NonConforming** | Gates 1 & 2 | Gate passage requires BOTH receipt evidence AND law coverage |
| **HA_ReleaseMayOccurFromInvalid** | Gate 4 | All critical gaps must be closed/accepted before release |
| **HA_VariantExplosion** | Gate 3 | Gap closures must reference documented gaps (no phantom closures) |

---

## Documentation Map

| Document | Purpose | Audience | Key Sections |
|----------|---------|----------|--------------|
| **README-VALIDATION-QUERIES.md** | Full query documentation | Engineers, auditors | Query details, remediation paths, ALIVE integration |
| **VALIDATION-CHECKPOINT-ARCHITECTURE.txt** | Architecture overview | Architects, leads | Validation flow, decision table, checkpoint milestones |
| **VALIDATION-QUERY-REFERENCE.md** | Quick reference | Engineers, integrators | SPARQL skeletons, example data, command reference |
| **INDEX-VALIDATION-QUERIES.md** | This file | Everyone | Quick start, file structure, gate summaries |

---

## ALIVE Checkpoint Milestones

These queries are used for:

| Milestone | Queries Used | Status | Note |
|-----------|-------------|--------|------|
| **PAPERLAW_ALIVE_002** | Gates 1, 2 | Active | Receipt evidence + law coverage required |
| **PAPERLAW_ALIVE_003** | Gates 1-4 | Future | All gates required; closure traceability + gap remediation |
| **PAPERLAW_CROWN_ALIVE_004** | Gates 1-4 | Sealed | Previous milestone; foundation for subsequent work |
| **PAPERLAW_ALIVE_005** | Gates 1-4 | Planned | Zero open critical gaps at checkpoint |

---

## Integration with ggen

**Input Ontologies:**
```
ggen/ontology/
├── audit-machinery.ttl       ← Core audit specifications
├── wasm4pm-compat.ttl        ← Type law surfaces
└── papers.ttl                ← Authority and paper coverage
```

**Query Execution:**
```bash
ggen query ggen/queries/<query.rq> --format sparql-results
```

**Output Handling:**
- ASK queries (Gates 1 & 2): Parse `Result: true/false`
- SELECT queries (Gates 3 & 4): Parse result rows or empty set

---

## Decision Logic

```
Checkpoint passes ALIVE certification IFF:
  (Gate1 returns FALSE)
  AND
  (Gate2 returns FALSE)
  AND
  (Gate3 returns EMPTY result set)
  AND
  (Gate4 returns EMPTY result set)

If any gate fails:
  → Checkpoint is BLOCKED
  → Return specific violations for remediation
  → Re-run gates after remediation
```

---

## Example Command Sequence

```bash
#!/bin/bash
set -e

cd /Users/sac/wasm4pm-compat/ggen

echo "=== ALIVE Checkpoint Validation ==="
echo

echo "Gate 1: Receipt Evidence"
result=$(ggen query queries/audit-no-file-count-alive.rq --format sparql-results)
if [[ $result == *"false"* ]]; then
    echo "✓ PASS: Checkpoint has receipt evidence"
else
    echo "✗ FAIL: No receipt evidence found"
    exit 1
fi
echo

echo "Gate 2: Law Coverage"
result=$(ggen query queries/audit-no-commit-count-gate.rq --format sparql-results)
if [[ $result == *"false"* ]]; then
    echo "✓ PASS: All laws have fixtures"
else
    echo "✗ FAIL: Uncovered laws found"
    exit 1
fi
echo

echo "Gate 3: Closure Traceability"
result=$(ggen query queries/audit-gap-closure-claims-have-gap-id.rq --format sparql-results)
if [ -z "$result" ] || [[ $result == *"empty"* ]]; then
    echo "✓ PASS: All closures are traceable"
else
    echo "✗ FAIL: Orphaned closures found:"
    echo "$result"
    exit 1
fi
echo

echo "Gate 4: Gap Remediation"
result=$(ggen query queries/audit-critical-gaps-have-remediation.rq --format sparql-results)
if [ -z "$result" ] || [[ $result == *"empty"* ]]; then
    echo "✓ PASS: All critical gaps are remediated"
else
    echo "✗ FAIL: Open critical gaps found:"
    echo "$result"
    exit 1
fi
echo

echo "=== CHECKPOINT SEALED ✓ ==="
```

---

## Future Enhancements

1. **OCEL Event Log Integration** — Convert query results to OCEL format for pm4py analysis
2. **Conformance Replay** — Compare declared audit process vs. discovered process
3. **Variant Analysis** — Count unique remediation patterns; flag outliers
4. **Temporal Ordering** — Verify closure timestamps are monotonically increasing
5. **Cross-Crate Audits** — Validate graduation boundaries with wasm4pm

---

## Files Manifest

```
audit-no-file-count-alive.rq                 (1.5K)  ASK query, Gate 1
audit-no-commit-count-gate.rq                (2.0K)  ASK query, Gate 2
audit-gap-closure-claims-have-gap-id.rq      (1.4K)  SELECT query, Gate 3
audit-critical-gaps-have-remediation.rq      (3.3K)  SELECT query, Gate 4
README-VALIDATION-QUERIES.md                 (9.3K)  Full documentation
VALIDATION-CHECKPOINT-ARCHITECTURE.txt       (20K)   Architecture overview
VALIDATION-QUERY-REFERENCE.md                (15K)   Query reference guide
INDEX-VALIDATION-QUERIES.md                  (This)  Quick index
```

**Total Size:** ~52.8K

---

## Contact & Support

For questions or issues with these validation queries:

1. **Architecture:** See `VALIDATION-CHECKPOINT-ARCHITECTURE.txt`
2. **Query Details:** See `VALIDATION-QUERY-REFERENCE.md` (SPARQL skeletons, examples)
3. **Full Docs:** See `README-VALIDATION-QUERIES.md` (remediation paths, integration)
4. **Ontology:** See `ggen/ontology/audit-machinery.ttl` (type definitions)

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1 | 2026-06-01 | Initial release: Four validation gates with comprehensive documentation |

---

**Status:** Ready for Integration  
**Last Updated:** 2026-06-01

