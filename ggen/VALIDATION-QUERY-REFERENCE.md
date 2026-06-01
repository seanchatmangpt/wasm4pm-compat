# SPARC Validation Query Reference

Complete reference for four checkpoint validation SPARQL queries that enforce hostile assumptions from the process-mining-chicago-tdd doctrine.

---

## Quick Reference Table

| Query | File | Type | Validates | Returns on Pass |
|-------|------|------|-----------|-----------------|
| Gate 1: Receipt Evidence | `audit-no-file-count-alive.rq` | ASK | Evidence-backed checkpoints (not file count) | `false` |
| Gate 2: Law Coverage | `audit-no-commit-count-gate.rq` | ASK | Law-fixture mapping (not commit count) | `false` |
| Gate 3: Closure Traceability | `audit-gap-closure-claims-have-gap-id.rq` | SELECT | Every closure references a gap | Empty |
| Gate 4: Gap Remediation | `audit-critical-gaps-have-remediation.rq` | SELECT | Critical gaps closed/accepted/PARTIAL | Empty |

---

## Query 1: Receipt Evidence Gate

**File:** `audit-no-file-count-alive.rq`

**SPARQL Form:** ASK

**Validates:** ALIVE checkpoints must be backed by explicit receipt evidence, not file volume metrics.

### SPARQL Skeleton
```sparql
ASK
WHERE {
  ?checkpoint a audit:CheckpointClaim ;
              audit:checkpointName ?name ;
              audit:passedAllGates true ;
              audit:allLawsCovered true .

  ?checkpoint audit:checkpointCommit ?commitHash .

  FILTER NOT EXISTS {
    ?commit a audit:CommitEvidence ;
            audit:commitHash ?commitHash ;
            audit:carriesReceipt ?receipt .
    ?receipt a compat:Receipt .
  }
}
```

### Logic
```
For each CheckpointClaim claiming passedAllGates=true and allLawsCovered=true:
  IF (checkpointCommit.carriesReceipt is empty)
    THEN return TRUE (failure: no receipt evidence)
  ELSE continue to next checkpoint
If all checkpoints have receipt evidence:
  RETURN FALSE (pass)
```

### Return Values
- **FALSE** (pass) — All checkpoints have `audit:carriesReceipt` links
- **TRUE** (fail) — At least one checkpoint has zero receipts

### Example Data (Passing Checkpoint)
```turtle
audit:CheckpointAlive002
    a audit:CheckpointClaim ;
    audit:checkpointName "PAPERLAW_ALIVE_002" ;
    audit:checkpointCommit "2426fac" ;
    audit:passedAllGates true ;
    audit:allLawsCovered true .

audit:CommitAliveEvidence_2426fac
    a audit:CommitEvidence ;
    audit:commitHash "2426fac" ;
    audit:carriesReceipt compat:Receipt_2426fac_001 ,
                         compat:Receipt_2426fac_002 .

# Query returns FALSE (pass)
```

### Example Data (Failing Checkpoint)
```turtle
audit:CheckpointAlive002
    a audit:CheckpointClaim ;
    audit:checkpointName "PAPERLAW_ALIVE_002" ;
    audit:checkpointCommit "2426fac" ;
    audit:passedAllGates true ;
    audit:allLawsCovered true .

# No CommitEvidence with commitHash "2426fac" or no carriesReceipt triples

# Query returns TRUE (fail: no receipt evidence found)
```

---

## Query 2: Law Coverage Gate

**File:** `audit-no-commit-count-gate.rq`

**SPARQL Form:** ASK

**Validates:** Law coverage is verified by law-to-fixture mapping, not commit count metrics.

### SPARQL Skeleton
```sparql
ASK
WHERE {
  ?checkpoint a audit:CheckpointClaim ;
              audit:checkpointName ?name ;
              audit:allLawsCovered true .

  ?law a compat:CompileFailLaw ;
       compat:definedInModule ?module .

  FILTER NOT EXISTS {
    ?law rdfs:label ?lawLabel ;
         audit:lawName ?lawNameStr .
    ?fixture a audit:AuditExecutable ;
             audit:fixtureFile ?fixtureFilePath ;
             audit:instantiatedBy ?template .
    ?template audit:templateKind "compile-fail-fixture" .
    ?fixture_spec a audit:AuditSpec ;
                  audit:applicableTemplate ?template ;
                  audit:lawName ?lawNameStr .
  }
}
```

### Logic
```
For each CheckpointClaim claiming allLawsCovered=true:
  For each compat:CompileFailLaw in scope:
    IF (law has no linked audit:AuditExecutable fixture)
      THEN return TRUE (failure: uncovered law)
    ELSE continue to next law
If all laws have linked fixtures:
  RETURN FALSE (pass)
```

### Return Values
- **FALSE** (pass) — All `compat:CompileFailLaw` instances have `audit:AuditExecutable` fixtures
- **TRUE** (fail) — At least one law has no linked fixture

### Example Data (Passing Checkpoint)
```turtle
compat:RawExportedAsAdmittedLaw
    a compat:CompileFailLaw ;
    rdfs:label "RawExportedAsAdmitted" ;
    audit:lawName "RawExportedAsAdmitted" ;
    compat:definedInModule compat:SourceModule_evidence .

audit:AuditExecutable_001
    a audit:AuditExecutable ;
    audit:fixtureFile "tests/ui/compile_fail/raw_exported_as_admitted.rs" ;
    audit:expectedStderr "tests/ui/compile_fail/raw_exported_as_admitted.stderr" ;
    audit:instantiatedBy audit:TryBuildCompileFailTemplate .

audit:AuditSpec_RawExportedAsAdmitted
    a audit:AuditSpec ;
    audit:lawName "RawExportedAsAdmitted" ;
    audit:applicableTemplate audit:TryBuildCompileFailTemplate .

# Query returns FALSE (pass: law has fixture)
```

### Example Data (Failing Checkpoint)
```turtle
compat:UncoveredLaw
    a compat:CompileFailLaw ;
    rdfs:label "UncoveredLaw" ;
    audit:lawName "UncoveredLaw" ;
    compat:definedInModule compat:SourceModule_process_tree .

# No audit:AuditExecutable linked to this law

# Query returns TRUE (fail: no fixture found)
```

---

## Query 3: Closure Traceability Gate

**File:** `audit-gap-closure-claims-have-gap-id.rq`

**SPARQL Form:** SELECT

**Validates:** Every `audit:ClosureClaim` must reference a documented `audit:Gap` via `audit:closesGap`.

### SPARQL Skeleton
```sparql
SELECT ?closureClaim ?closingCommit ?closureMethod
WHERE {
  ?closureClaim a audit:ClosureClaim ;
                audit:closingCommit ?closingCommit ;
                audit:closureMethod ?closureMethod .

  FILTER NOT EXISTS {
    ?closureClaim audit:closesGap ?gapReference .
    ?gapReference a audit:Gap ;
                  audit:gapId ?gapId .
  }
}
ORDER BY ?closingCommit
```

### Logic
```
For each audit:ClosureClaim:
  IF (no audit:closesGap property exists)
    THEN add to result set (violation)
  ELSE continue to next closure claim
Return all orphaned closure claims
If result set is empty:
  PASS (all closures are traceable)
If result set has rows:
  FAIL (undocumented closures found)
```

### Return Values
- **Empty result** (pass) — All closure claims reference a gap
- **Rows** (fail) — Orphaned closure claims with no gap reference

### Result Columns
- `?closureClaim` — URI of the orphaned `audit:ClosureClaim`
- `?closingCommit` — Commit hash (undocumented closure)
- `?closureMethod` — Method type (fixture-added, law-proved, etc.)

### Example Data (Passing Checkpoint)
```turtle
audit:ClosureClaim_Gap001_001
    a audit:ClosureClaim ;
    audit:closesGap audit:Gap001_UnimplementedConformanceAudit ;
    audit:closingCommit "a1b2c3d4e5f6" ;
    audit:closureMethod "fixture-added" ;
    audit:closureTimestamp "2026-06-01T10:00:00Z"^^xsd:dateTime .

audit:Gap001_UnimplementedConformanceAudit
    a audit:Gap ;
    audit:gapId "GAP_001" ;
    audit:gapCategory "missing-fixture" .

# Query returns NO ROWS (pass: closure is traceable)
```

### Example Data (Failing Checkpoint)
```turtle
audit:ClosureClaim_Unknown_001
    a audit:ClosureClaim ;
    # Missing: audit:closesGap property
    audit:closingCommit "a1b2c3d4e5f6" ;
    audit:closureMethod "fixture-added" ;
    audit:closureTimestamp "2026-06-01T10:00:00Z"^^xsd:dateTime .

# Query returns 1 ROW:
#   ?closureClaim = audit:ClosureClaim_Unknown_001
#   ?closingCommit = "a1b2c3d4e5f6"
#   ?closureMethod = "fixture-added"
# (fail: orphaned closure)
```

### Remediation
For each orphaned closure claim, add the missing triple:
```turtle
audit:ClosureClaim_Unknown_001
    audit:closesGap audit:GapXXX_SomeGap .
```

---

## Query 4: Gap Remediation Gate

**File:** `audit-critical-gaps-have-remediation.rq`

**SPARQL Form:** SELECT

**Validates:** Critical and major gaps must be closed, accepted as debt, or documented as PARTIAL.

### SPARQL Skeleton
```sparql
SELECT ?gapId ?gapCategory ?gapSeverity ?remediationPlan ?hasClosureClaim ?acceptedAsDebt
WHERE {
  ?gap a audit:Gap ;
       audit:gapId ?gapId ;
       audit:gapCategory ?gapCategory ;
       audit:gapSeverity ?gapSeverity ;
       audit:remediationPlan ?remediationPlan .

  FILTER (?gapSeverity = "critical" || ?gapSeverity = "major")

  OPTIONAL {
    ?closureClaim audit:closesGap ?gap ;
                  audit:closureTimestamp ?closureTs .
    BIND(true AS ?hasClosureClaim)
  }
  BIND(COALESCE(?hasClosureClaim, false) AS ?hasClosureClaim)

  OPTIONAL {
    ?gap audit:gapCategory "accepted-debt" .
    BIND(true AS ?acceptedAsDebt)
  }
  BIND(COALESCE(?acceptedAsDebt, false) AS ?acceptedAsDebt)

  OPTIONAL {
    FILTER REGEX(?remediationPlan, "PARTIAL|partial|PARTIAL_REMEDIATION", "i")
    BIND(true AS ?isPartial)
  }
  BIND(COALESCE(?isPartial, false) AS ?isPartial)

  FILTER (!?hasClosureClaim && !?acceptedAsDebt && !?isPartial)
}
ORDER BY ?gapSeverity DESC ?gapId
```

### Logic
```
For each audit:Gap with severity ∈ {critical, major}:
  Check1 = (has audit:ClosureClaim with closureTimestamp)
  Check2 = (gapCategory = "accepted-debt")
  Check3 = (remediationPlan contains "PARTIAL")
  IF NOT (Check1 OR Check2 OR Check3)
    THEN add to result set (violation: open gap)
  ELSE continue to next gap
If result set is empty:
  PASS (all critical gaps are remediated)
If result set has rows:
  FAIL (open critical gaps found)
```

### Return Values
- **Empty result** (pass) — All critical/major gaps are closed, accepted, or documented as PARTIAL
- **Rows** (fail) — Open critical/major gaps without remediation

### Result Columns
- `?gapId` — Gap identifier (e.g., GAP_001)
- `?gapCategory` — Category (missing-fixture, uncovered-law, unresolved-todo, unverified-graduation)
- `?gapSeverity` — critical | major | minor | accepted-debt
- `?remediationPlan` — Human-readable remediation description
- `?hasClosureClaim` — true | false (whether a closure claim exists)
- `?acceptedAsDebt` — true | false (whether accepted-debt status applies)

### Example Data (Passing Checkpoint)
```turtle
# Case A: Gap is closed
audit:Gap001_UnimplementedConformanceAudit
    a audit:Gap ;
    audit:gapId "GAP_001" ;
    audit:gapSeverity "major" ;
    audit:gapCategory "missing-fixture" ;
    audit:remediationPlan "Implement OCEL-based audit..." .

audit:ClosureClaim_Gap001_Closed
    a audit:ClosureClaim ;
    audit:closesGap audit:Gap001_UnimplementedConformanceAudit ;
    audit:closureTimestamp "2026-06-01T10:00:00Z"^^xsd:dateTime .

# Case B: Gap is accepted as debt
audit:Gap002_UncoveredProcessTree
    a audit:Gap ;
    audit:gapId "GAP_002" ;
    audit:gapSeverity "critical" ;
    audit:gapCategory "accepted-debt" ;
    audit:remediationPlan "Accepted as technical debt. Rationale: ProcessTree construction is complex and will be addressed in ALIVE_005." .

# Case C: Gap is documented as PARTIAL
audit:Gap003_UnverifiedGraduation
    a audit:Gap ;
    audit:gapId "GAP_003" ;
    audit:gapSeverity "critical" ;
    audit:gapCategory "unresolved-todo" ;
    audit:remediationPlan "PARTIAL_REMEDIATION: Basic graduation structure in src/graduation.rs. Pending cross-crate wasm4pm integration. Target: ALIVE_005." .

# Query returns NO ROWS (pass: all critical gaps are remediated)
```

### Example Data (Failing Checkpoint)
```turtle
audit:Gap003_UnverifiedGraduation
    a audit:Gap ;
    audit:gapId "GAP_003" ;
    audit:gapSeverity "critical" ;
    audit:gapCategory "unresolved-todo" ;
    audit:remediationPlan "Integrate with wasm4pm to verify graduation paths." .

# No audit:ClosureClaim linked to this gap
# gapCategory is NOT "accepted-debt"
# remediationPlan does NOT contain "PARTIAL"

# Query returns 1 ROW:
#   ?gapId = "GAP_003"
#   ?gapSeverity = "critical"
#   ?gapCategory = "unresolved-todo"
#   ?remediationPlan = "Integrate with wasm4pm..."
#   ?hasClosureClaim = false
#   ?acceptedAsDebt = false
# (fail: open critical gap)
```

### Remediation Paths

**Path A: Close the gap**
```turtle
audit:ClosureClaim_Gap003_Closed
    a audit:ClosureClaim ;
    audit:closesGap audit:Gap003_UnverifiedGraduation ;
    audit:closingCommit "xyz789abc123" ;
    audit:closureMethod "law-proved" ;
    audit:closureTimestamp "2026-06-01T12:00:00Z"^^xsd:dateTime .
```

**Path B: Accept as technical debt**
```turtle
audit:Gap003_UnverifiedGraduation
    audit:gapCategory "accepted-debt" ;
    audit:remediationPlan "Accepted as technical debt: wasm4pm integration is out of scope for PAPERLAW_CROWN_ALIVE_004. Tracked in GAP_003 for ALIVE_005 sprint." .
```

**Path C: Document as PARTIAL**
```turtle
audit:Gap003_UnverifiedGraduation
    audit:remediationPlan "PARTIAL_REMEDIATION: Basic graduation structure implemented in src/graduation.rs. Pending: cross-crate verification with wasm4pm. Target closure: ALIVE_005." .
```

---

## Validation Checkpoint Command Reference

### Run All Validation Queries

```bash
# Sequential validation (stop on first failure)
set -e
ggen query audit-no-file-count-alive.rq --format sparql-results
ggen query audit-no-commit-count-gate.rq --format sparql-results
ggen query audit-gap-closure-claims-have-gap-id.rq --format sparql-results
ggen query audit-critical-gaps-have-remediation.rq --format sparql-results
echo "All checkpoints passed."
```

### Interpret Results

| Query | Result | Interpretation |
|-------|--------|-----------------|
| Gate 1 (ASK) | `false` | ✓ Checkpoints have receipt evidence |
| Gate 1 (ASK) | `true` | ✗ BLOCK: No receipt evidence found |
| Gate 2 (ASK) | `false` | ✓ All laws have fixtures |
| Gate 2 (ASK) | `true` | ✗ BLOCK: Uncovered laws found |
| Gate 3 (SELECT) | Empty | ✓ All closures are traceable |
| Gate 3 (SELECT) | Rows | ✗ BLOCK: Orphaned closures found |
| Gate 4 (SELECT) | Empty | ✓ All critical gaps are remediated |
| Gate 4 (SELECT) | Rows | ✗ BLOCK: Open critical gaps found |

---

## Integration with ALIVE Certification

These four queries form a **mandatory checkpoint sequence** for ALIVE milestone certification:

1. **PAPERLAW_ALIVE_002:** Receipt evidence + law coverage required
2. **PAPERLAW_ALIVE_003:** All closure claims traceable, no orphaned remediations
3. **PAPERLAW_CROWN_ALIVE_004:** All critical gaps closed or accepted with full traceability
4. **PAPERLAW_ALIVE_005** (future): Zero open critical gaps at checkpoint

A milestone is **SEALED** only when:
- Gate 1 returns `false` (receipt evidence present)
- Gate 2 returns `false` (law coverage verified)
- Gate 3 returns empty (closures traceable)
- Gate 4 returns empty (gaps remediated)

---

## Hostile Assumptions Operationalized

| Assumption | Enforced By | Verification |
|------------|-------------|--------------|
| **HA_DeclaredPipelineNotReal** | Query 2 | Law coverage via RDF triples, not commit count |
| **HA_ReceiptsEmittedOutOfCycle** | Query 1 | Receipts explicitly linked to commitments |
| **HA_ProofGatesCanPass_NonConforming** | Queries 1 & 2 | Receipt evidence + law coverage both required |
| **HA_ReleaseMayOccurFromInvalid** | Query 4 | All critical gaps must be closed/accepted |
| **HA_VariantExplosion** | Query 3 | Closures must reference documented gaps |

---

## Files and References

| File | Purpose | Lines |
|------|---------|-------|
| `audit-no-file-count-alive.rq` | Gate 1 validation query | ~35 |
| `audit-no-commit-count-gate.rq` | Gate 2 validation query | ~40 |
| `audit-gap-closure-claims-have-gap-id.rq` | Gate 3 validation query | ~35 |
| `audit-critical-gaps-have-remediation.rq` | Gate 4 validation query | ~65 |
| `README-VALIDATION-QUERIES.md` | Full documentation | ~300 |
| `VALIDATION-CHECKPOINT-ARCHITECTURE.txt` | Architecture overview | ~400 |
| `VALIDATION-QUERY-REFERENCE.md` | This reference guide | ~500 |

---

## See Also

- **Ontology:** `ggen/ontology/audit-machinery.ttl` — Full audit machinery specification
- **Architecture:** `ggen/VALIDATION-CHECKPOINT-ARCHITECTURE.txt` — Checkpoint flow diagram
- **Doctrine:** `~/.claude/rules/process-mining-chicago-tdd.md` — Hostile assumptions
- **Type Law:** `ggen/ontology/wasm4pm-compat.ttl` — Type law surfaces

