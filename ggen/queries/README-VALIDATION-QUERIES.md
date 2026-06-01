# SPARC Audit Validation Queries

Four checkpoint validation queries for the PAPERLAW architecture. These queries enforce the hostile assumptions from **process-mining-chicago-tdd doctrine**: if the event log (RDF graph) cannot prove a lawful audit happened, then it did not happen.

---

## Query 1: `audit-no-file-count-alive.rq` — Reject ALIVE from File Count

**Type:** ASK Query (Boolean result)

**Purpose:** Validates that ALIVE checkpoint claims are **evidence-based**, not file volume metrics.

**What it validates:**
- Every checkpoint claiming `passedAllGates = true` and `allLawsCovered = true` MUST have explicit receipt evidence
- Receipt evidence is linked via `audit:carriesReceipt` properties pointing to `compat:Receipt` instances
- If a checkpoint's commit has zero receipts in the knowledge graph, the gate claim is invalid

**Returns:**
- `false` (pass) — All checkpoints have receipt evidence backing their gate claims
- `true` (failure) — At least one checkpoint gate claim lacks receipt evidence

**Example failure scenario:**
```
Checkpoint: PAPERLAW_ALIVE_002
  - checkpointCommit: 2426fac
  - passedAllGates: true
  - NO audit:carriesReceipt properties found
  → FAILURE: File count metric alone does not constitute proof
```

**Rationale:**
ALIVE certification is not about commit velocity or file growth. It is a proof gate that requires:
1. Trybuild receipts (compile-fail/pass fixtures pass)
2. Type-law surfaces are complete
3. Witness markers and state transitions are enforced at compile time

---

## Query 2: `audit-no-commit-count-gate.rq` — Reject ALIVE from Commit Count Alone

**Type:** ASK Query (Boolean result)

**Purpose:** Enforces that gate passage requires **law-to-fixture mapping**, not commit count metrics.

**What it validates:**
- Every `compat:CompileFailLaw` instance in scope MUST be linked to an `audit:AuditExecutable` fixture
- The fixture must have `audit:fixtureFile` (path to trybuild .rs file) and `audit:expectedStderr` (path to .stderr)
- If any law lacks a linked fixture, the gate claim `allLawsCovered = true` is invalid

**Returns:**
- `false` (pass) — All laws in scope have fixtures
- `true` (failure) — At least one law has no linked fixture

**Example failure scenario:**
```
Law: RawEvidenceExportedAsAdmitted
  - no audit:AuditExecutable fixture found
  - Checkpoint claims allLawsCovered = true
  → FAILURE: Law is uncovered; commit count is irrelevant
```

**Rationale:**
From **process-mining-chicago-tdd doctrine**:
> If the event log cannot prove a lawful process happened, then it did not happen.

Commit count is an audit anti-pattern. We validate law coverage through RDF triple patterns:
1. Count the `compat:CompileFailLaw` instances
2. For each law, verify `audit:applicableTemplate` exists
3. For each template, verify `audit:AuditExecutable` instantiations exist
4. For each executable, verify fixture file exists in the knowledge graph

---

## Query 3: `audit-gap-closure-claims-have-gap-id.rq` — Gap Closure Claims Must Reference Gaps

**Type:** SELECT Query (Rows = violations)

**Purpose:** Validates that every gap closure is **traceable to a documented gap**.

**What it validates:**
- Every `audit:ClosureClaim` instance MUST have an `audit:closesGap` property
- The target of `audit:closesGap` must be an `audit:Gap` with `audit:gapId` defined
- Orphaned closure claims (no gap reference) indicate undocumented remediation

**Returns:**
- Empty result set (pass) — All closure claims reference a gap
- Rows with `?closureClaim`, `?closingCommit`, `?closureMethod` (failure) — Orphaned claims

**Example failure row:**
```
?closureClaim                    ?closingCommit   ?closureMethod
audit:ClosureClaim_Unknown_001   a1b2c3d4e5f6     fixture-added
  ↑ This claim has no audit:closesGap link
```

**Remediation:**
For each orphaned claim, add the missing triple:
```sparql
audit:ClosureClaim_Unknown_001 audit:closesGap audit:Gap001_SomeGap .
```

**Rationale:**
Hostile assumption: undocumented gap closures can silently mask audit defects. Every closure must:
1. Name the gap it closes (via `audit:gapId`)
2. State the closing method (fixture-added, law-proved, path-proved-reachable, reclassified)
3. Carry a timestamp (`audit:closureTimestamp`)

---

## Query 4: `audit-critical-gaps-have-remediation.rq` — Critical Gaps Must Be Closed, PARTIAL, or Accepted

**Type:** SELECT Query (Rows = violations)

**Purpose:** Validates that **critical and major gaps are not left open without documentation**.

**What it validates:**
- Every `audit:Gap` with severity `critical` or `major` MUST have one of:
  1. A `audit:ClosureClaim` with `audit:closureTimestamp` (gap is closed)
  2. `audit:gapCategory` reclassified to `"accepted-debt"` (gap is accepted as tech debt)
  3. `audit:remediationPlan` containing `"PARTIAL"` keyword (gap is in progress, documented)
- If none of these conditions hold, the gap is "open" and blocks audit certification

**Returns:**
- Empty result set (pass) — All critical/major gaps are remediated or documented
- Rows with `?gapId`, `?gapSeverity`, `?remediationPlan` (failure) — Open critical gaps

**Example failure row:**
```
?gapId        ?gapSeverity ?gapCategory    ?hasClosureClaim ?acceptedAsDebt ?remediationPlan
GAP_003       critical     unresolved-todo false            false          "Integrate with wasm4pm"
  ↑ Open critical gap; no closure, not accepted, not marked PARTIAL
```

**Remediation strategies:**

**Strategy A: Close the gap**
```turtle
audit:ClosureClaim_Gap003_Closed
    a audit:ClosureClaim ;
    audit:closesGap audit:Gap003_UnverifiedGraduationPaths ;
    audit:closingCommit "abc123def456" ;
    audit:closureMethod "law-proved" ;
    audit:closureTimestamp "2026-06-01T12:00:00Z"^^xsd:dateTime .
```

**Strategy B: Accept as technical debt**
```turtle
audit:Gap003_UnverifiedGraduationPaths
    audit:gapCategory "accepted-debt" ;
    audit:remediationPlan "Accepted as technical debt: wasm4pm integration is out of scope for PAPERLAW_CROWN_ALIVE_004. Tracked in GAP_003 for ALIVE_005 sprint." .
```

**Strategy C: Document as PARTIAL**
```turtle
audit:Gap003_UnverifiedGraduationPaths
    audit:remediationPlan "PARTIAL_REMEDIATION: Basic graduation structure implemented in src/graduation.rs. Pending: cross-crate verification with wasm4pm. Target closure: ALIVE_005." .
```

**Rationale:**
Gaps are first-class audit defects. From **process-mining-chicago-tdd doctrine**:
> Model-vs-log mismatch is not a discrepancy. It is a defect.

Critical gaps left unresolved and undocumented can silently break the audit model. Every gap MUST be:
1. **Closed** (evidence provided), OR
2. **Accepted** (explicit risk acceptance), OR
3. **Documented as PARTIAL** (remediation in progress)

---

## Integration with ALIVE Gate

These four queries form the **validation checkpoint layer** of ALIVE certification:

| Gate | Query | Validation |
|------|-------|-----------|
| **Receipt Evidence** | `audit-no-file-count-alive.rq` | Checkpoints must carry trybuild receipts |
| **Law Coverage** | `audit-no-commit-count-gate.rq` | All laws must have fixtures (not metric-driven) |
| **Gap Traceability** | `audit-gap-closure-claims-have-gap-id.rq` | Every closure must reference a gap |
| **Gap Remediation** | `audit-critical-gaps-have-remediation.rq` | Critical gaps must be closed/accepted/partial |

A checkpoint passes ALIVE certification only if **all four queries return validation success**:
- Queries 1 & 2 (ASK): must return `false`
- Queries 3 & 4 (SELECT): must return empty result set

---

## Usage

Run individually:
```bash
ggen query audit-no-file-count-alive.rq --format sparql-results
ggen query audit-no-commit-count-gate.rq --format sparql-results
ggen query audit-gap-closure-claims-have-gap-id.rq --format sparql-results
ggen query audit-critical-gaps-have-remediation.rq --format sparql-results
```

Run as a validation suite:
```bash
# Would require wrapper script (future enhancement)
./scripts/validate-alive-checkpoint.sh --ontology ggen/ontology/ --queries ggen/queries/
```

---

## Hostile Assumptions Enforced

These queries operationalize the **process-mining-chicago-tdd doctrine**:

1. **HA_DeclaredPipelineNotReal** → Query 2 validates law-fixture mapping (not declared coverage)
2. **HA_ReceiptsEmittedOutOfCycle** → Query 1 validates receipts are explicitly linked
3. **HA_ProofGatesCanPass_NonConforming** → Queries 1 & 2 require evidence + law coverage
4. **HA_ReleaseMayOccurFromInvalid** → Query 4 blocks release until gaps are remediated
5. **HA_VariantExplosion** → Query 3 ensures closure evidence is traceable (not silent/phantom)

---

## Future Enhancements

- **OCEL Event Log Integration:** Derive these queries from actual OCEL traces of the audit process
- **Conformance Replay:** Use pm4py to verify checkpoint claims against discovered audit process
- **Variant Analysis:** Count unique gap remediation patterns and flag outliers
- **Temporal Ordering:** Verify closure timestamps are monotonically increasing
- **Cross-Crate Audits:** Extend queries to validate graduation boundaries with wasm4pm

---

## References

- **Ontology:** `ggen/ontology/audit-machinery.ttl` — Full audit machinery specification
- **Doctrine:** `~/.claude/rules/process-mining-chicago-tdd.md` — Hostile assumptions
- **Type Law:** `ggen/ontology/wasm4pm-compat.ttl` — Type law surfaces
- **Papers:** `ggen/ontology/papers.ttl` — Authority index

