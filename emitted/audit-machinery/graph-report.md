# SPARC Audit Machinery Ontology — Graph Report

**Generated:** 2026-06-01  
**Source:** `/Users/sac/wasm4pm-compat/ggen/ontology/audit-machinery.ttl`  
**Triples:** 347 (core classes, properties, instances, and relationships)  
**Imports:** wasm4pm-compat.ttl (witness markers, evidence states, compile-fail laws)  
**External Vocabularies:** prov:, dct:, skos:, sh: (W3C standard ontologies)

---

## Executive Summary

The SPARC Audit Machinery Ontology is a machine-readable specification of audit law surfaces for the wasm4pm-compat nightly-first Rust crate. It represents:

1. **Core Audit Abstractions**
   - `AuditSpec` — Named audit contracts and their scope
   - `AuditTemplate` — Patterns for conducting audits
   - `AuditExecutable` — Concrete audit instances ready to run
   - `AuditGate` — Proof gates (validation checkpoints)

2. **Audit Artifacts**
   - `ForbiddenPattern` — Structurally unbreakable laws
   - `AllowedContext` — Named exceptions where patterns are permitted
   - `Gap` — Documented audit discrepancies with remediation plans
   - `CommitEvidence` — Git commits carrying manufacturing receipts

3. **Closure and Certification**
   - `ClosureClaim` — Assertions that gaps have been closed
   - `AuxiliaryCommit` — Supporting commits advancing audit readiness
   - `CheckpointClaim` — Immutable ALIVE milestones (e.g., PAPERLAW_ALIVE_002)

4. **Hostile Assumptions**
   - Process-Mining-Chicago-TDD doctrine: "If the code says it worked but the event log cannot prove a lawful process happened, then it did not work."
   - Eight hostile assumptions documented and auditable.

---

## Class Hierarchy

```
rdfs:Class (W3C base)
├── audit:AuditSpec (subClassOf prov:Plan)
├── audit:AuditTemplate (subClassOf sh:Shape)
├── audit:AuditExecutable (subClassOf prov:Activity)
├── audit:AuditGate (subClassOf sh:NodeShape)
├── audit:ForbiddenPattern (skos:related compat:CompileFailLaw)
├── audit:AllowedContext (skos:related compat:WitnessMarker)
├── audit:Gap (skos:related prov:Fault)
├── audit:CommitEvidence (skos:related prov:EntityBundle)
├── audit:ClosureClaim (skos:related prov:Qualification)
├── audit:AuxiliaryCommit (skos:related prov:Entity)
├── audit:CheckpointClaim (skos:related prov:Bundle)
└── audit:HostileAssumption (Chicago-TDD doctrine)
```

---

## Core Audit Specifications (7 Instances)

### 1. RawEvidenceExportedAsAdmittedSpec
- **Law:** RawEvidenceExportedAsAdmitted
- **Scope:** Crate-wide (wasm4pm-compat)
- **Carrier:** `Evidence<T, Raw, W>`
- **Verdict Type:** Boolean (pass/fail)
- **Template:** TryBuildCompileFailTemplate
- **Linked Fixture:** `compat:RawExportedAsAdmittedLaw` (compile-fail proof)

### 2. WitnessDiscriminationLawSpec
- **Law:** WitnessDiscrimination
- **Scope:** Crate-wide
- **Carrier:** `Admission<T, W>`
- **Verdict Type:** Boolean
- **Template:** TryBuildCompileFailTemplate
- **Linked Fixture:** `compat:WitnessDiscriminationLaw` (E0308 compile error)

### 3. SilentStructureLossSpec
- **Law:** SilentStructureLoss
- **Scope:** Crate-wide
- **Carrier:** `Evidence<T, Projected, W>`
- **Verdict Type:** Boolean
- **Template:** RuntimeLossAuditTemplate
- **Runtime Check:** Every projected evidence must carry `LossReport<From, To, Items>`

### 4. BareInvalidInputRefusalSpec
- **Law:** BareInvalidInputRefusal
- **Scope:** Module-specific (src/admission.rs)
- **Carrier:** `Refusal<R, W>`
- **Verdict Type:** Boolean
- **Template:** StaticAnalysisTemplate
- **Enforcement:** Grep/clippy to ensure no InvalidInput enum instantiation

### 5. TypeStateAdmissionBoundarySpec
- **Law:** TypeStateAdmissionBoundary
- **Scope:** Crate-wide
- **Carrier:** `Evidence<T, Admitted, W>`
- **Verdict Type:** Boolean
- **Template:** TryBuildCompilePassTemplate
- **Proof:** Compile-pass fixture demonstrating lawful admission path

### 6. ReceiptProvenanceSpec
- **Law:** ReceiptProvenance
- **Scope:** Crate-wide
- **Carrier:** `Evidence<T, Receipted, W>`
- **Verdict Type:** Enum (proof verdict)
- **Template:** OcelConformanceTemplate
- **Audit Method:** OCEL event log derivation + conformance checking

### 7. (Additional Specs in extension)
- **ReceiptConsistencySpec**
- **EvidenceLifecycleSoundnessSpec**
- **LossReportCompletenessSpec**

---

## Audit Templates (5 Patterns)

### 1. TryBuildCompileFailTemplate
Each forbidden law backed by `tests/ui/compile_fail/*.rs` fixture:
- Fixture must fail with intended error code (e.g., E0308, E0277, E0080)
- Paired `.stderr` file with expected rustc diagnostic
- Proves law is unbreakable at compile time

**Instances Using This Template:**
- RawEvidenceExportedAsAdmittedSpec
- WitnessDiscriminationLawSpec

### 2. TryBuildCompilePassTemplate
Each lawful path backed by `tests/ui/compile_pass/*.rs` fixture:
- Fixture must compile successfully
- Proves the law is not overly restrictive; the lawful path is open

**Instances Using This Template:**
- TypeStateAdmissionBoundarySpec
- Examples: `separable_wfnet_marker.rs`, `wfnet2powl_witness.rs`, `workflow_pattern_const_param.rs`

### 3. RuntimeLossAuditTemplate
Loss audits executed at runtime:
- Inspects `LossReport<From, To, Items>` carriers
- Validates: FromType, ToType, Items (by name and count)
- Detects silent loss (lossy transformation without report)

**Instances Using This Template:**
- SilentStructureLossSpec

### 4. StaticAnalysisTemplate
Code analysis (grep, clippy, cargo-audit):
- Scans for forbidden pattern instantiations
- Example: No bare `InvalidInput` enum uses

**Instances Using This Template:**
- BareInvalidInputRefusalSpec

### 5. OcelConformanceTemplate
Event log conformance checking (wasm4pm integration):
- Derives OCEL event log of audit execution
- Runs process mining (discovery, conformance, replay)
- Compares declared audit model vs. discovered actual process

**Instances Using This Template:**
- ReceiptProvenanceSpec
- ReceiptConsistencySpec
- EvidenceLifecycleSoundnessSpec

---

## Audit Gates (6 Proof Gates)

### 1. AllLawsHaveFixturesGate
**Severity:** Fatal (ALIVE gate)  
**Condition:** Every `compat:CompileFailLaw` has non-empty `compat:fixtureFile`  
**Failure:** ALIVE certification blocked

### 2. AllFixturesHaveStderrGate
**Severity:** Fatal (ALIVE gate)  
**Condition:** Every compile-fail fixture has paired `.stderr` file  
**Failure:** Fixture is not properly validated by trybuild

### 3. AllGapsClosedOrAcceptedGate
**Severity:** Fatal (ALIVE gate)  
**Condition:** Every `Gap` has either `ClosureClaim` or `gapSeverity: "accepted-debt"` with justification  
**Failure:** Open audit gaps block certification

### 4. AllCommitMessagesConventionalGate
**Severity:** Error (blocks most releases)  
**Condition:** All commits follow `type(scope): description` format  
**Failure:** Commit history not machine-parseable

### 5. AllReceiptsValidateGate
**Severity:** Fatal (ALIVE gate)  
**Condition:** Every `Receipt` has valid provenance (commit hash, author, timestamp, witness)  
**Failure:** Provenance chain is broken or forged

### 6. NoSilentLossInProjectionsGate
**Severity:** Fatal (ALIVE gate)  
**Condition:** Every `Evidence<T, Projected, W>` carries exactly one `LossReport`  
**Failure:** Silent loss detected (structural defect)

---

## Forbidden Patterns (5 Core Patterns)

### 1. RawExportedAsAdmittedPattern
- **Manifestation:** Using `Evidence<T, Raw, W>` where `Admitted` required
- **Proof:** Compile-fail fixture with E0308 type error
- **Why Forbidden:** One-way-door law requires Raw → Parsed → Admitted progression

### 2. WitnessDiscriminationViolationPattern
- **Manifestation:** Using `Admission<T, W1>` as `Admission<T, W2>`
- **Proof:** Compile-fail fixture with E0308 type error
- **Why Forbidden:** Witnesses are sealed; no cross-witness coercion

### 3. SilentStructureLossPattern
- **Manifestation:** Lossy transformation without `LossReport`
- **Proof:** Runtime validation; static patterns in src/loss.rs
- **Allowed In:** `NamedProjectionException` (only if LossReport present)
- **Why Forbidden:** Loss must be accounted and recorded

### 4. ConditionCellOverflowPattern
- **Manifestation:** Using `ConditionCell<BITS>` with BITS > 8
- **Proof:** Compile-fail fixture with E0080 illegal-value error
- **Why Forbidden:** Blue River Dam covenant: max 8 condition bits

### 5. SoundnessForgedPattern
- **Manifestation:** Constructing `WfNetConst<SoundnessWitnessed>` outside wasm4pm
- **Proof:** Compile-fail fixture with E0277 trait-bound error
- **Why Forbidden:** Soundness verification is a wasm4pm privilege; tokens are non-forgeable

---

## Allowed Contexts (3 Named Exceptions)

### 1. Wasm4pmGraduationContext
- **Pattern:** Some type-construction patterns forbidden in the compat layer
- **Reason:** wasm4pm is a trusted execution boundary with its own audit machinery
- **Scope:** `feature(wasm4pm)`
- **Module:** src/graduation.rs

### 2. InternalBridgeAssemblyContext
- **Pattern:** Bridge implementation layer has internal exceptions
- **Reason:** Bridge assembly requires compatibility work
- **Scope:** `witness(Wasm4pmBridge, InternalBridge)`
- **Module:** src/witness.rs

### 3. NamedProjectionException
- **Pattern:** Lossy transformations allowed only if accompanied by LossReport
- **Reason:** Loss is accounted and on the record
- **Scope:** `module(src/loss.rs)`
- **Condition:** LossReport<From, To, Items> must be present

---

## Known Gaps (3 Documented Discrepancies)

### GAP_001: Unimplemented Conformance Audit
- **Category:** missing-fixture
- **Severity:** Major
- **Related Law:** `compat:ConformanceAuthority`
- **Opened:** 2026-05-30
- **Remediation:** Implement OCEL-based audit of wasm4pm conformance execution
- **Blocker:** Requires wasm4pm integration (out of scope for compat)
- **Status:** Open (conditional acceptance as "requires-graduation")

### GAP_002: Uncovered Process Tree Construction Laws
- **Category:** uncovered-law
- **Severity:** Minor
- **Related Law:** `compat:ProcessTree`
- **Opened:** 2026-05-30
- **Remediation:** Add compile-fail fixtures for ProcessTree block-structure invariants
- **Blocker:** May require extending generic_const_exprs
- **Status:** Open (future enhancement)

### GAP_003: Unverified Graduation Paths
- **Category:** unresolved-todo
- **Severity:** Critical
- **Related Law:** `compat:GraduationCandidate`
- **Opened:** 2026-05-30
- **Remediation:** Integrate with wasm4pm to verify each GraduationBoundary has real entry point
- **Blocker:** Cross-crate audit required
- **Status:** Open (critical — must be resolved before crown release)

---

## Checkpoint Claims (2 Sealed ALIVE Milestones)

### PAPERLAW_ALIVE_002
- **Commit:** 2426fac (nightly_foundry type-law surfaces)
- **Timestamp:** 2026-05-30 12:00 UTC
- **All Laws Covered:** ✓ Yes
- **All Gaps Closed:** ✗ No (major/minor gaps remain)
- **All Gates Passed:** ✓ Yes
- **Status:** Sealed; foundation for subsequent work

### PAPERLAW_CROWN_ALIVE_004 (Previous)
- **Commit:** 277d528 (initial wasm4pm-compat)
- **Timestamp:** 2026-05-28 14:00 UTC
- **Receipts:** 196 compile-fail + 406 compile-pass fixtures
- **Papers Covered:** 98 papers
- **Status:** Sealed; baseline for current work

---

## Hostile Assumptions (Chicago-TDD Doctrine)

Per the process-mining-chicago-tdd covenant, the audit must NOT trust:

### HA_DeclaredPipelineNotReal
The declared manufacturing pipeline is not the real runtime process. Stages may be skipped, repeated, or nested without detection.

**Audit Response:** Derive actual process from event logs; compare against declared model.

### HA_ReceiptsEmittedOutOfCycle
Receipts may be emitted outside lawful object lifecycles or backdated.

**Audit Response:** Validate receipt timestamps against commit history; check lifecycle consistency with event log.

### HA_ProofGatesCanPass_NonConforming
Proof gates (ALIVE certification, trybuild success) may pass despite non-conforming execution.

**Audit Response:** Derive actual process from OCEL event logs; check conformance independently.

### HA_ReleaseMayOccurFromInvalid
Release may be declared from incomplete or invalid artifact histories.

**Audit Response:** Verify object lifecycle soundness before accepting release claim.

### HA_VariantExplosion
System may appear deterministic while logs reveal variant explosion, hidden loops, retries, or rework.

**Audit Response:** Conduct drift analysis; compute variant metrics; flag process complexity growth.

### HA_StagesSkipped
Manufacturing stages may be skipped entirely without visible signal.

**Audit Response:** Verify no impossible state transitions in event log.

### HA_ReworkUndetected
Silent rework, retries, and compensating transactions may occur and go unlogged.

**Audit Response:** Use log skeleton and causal graph analysis to detect impossible order violations.

### HA_UnsoundnessHidden
Soundness claims may be attached without proof.

**Audit Response:** Run wasm4pm soundness verification on all WfNetConst<SoundnessWitnessed> tokens.

---

## Integration with wasm4pm-compat Type Law

The audit machinery is tightly integrated with the Rust type law surface:

| Audit Concept | Type Law Equivalent | Link |
|---|---|---|
| AuditSpec | compat:CompileFailLaw, compat:TypeConstraint | Named laws enforced by the type system |
| AuditTemplate | compat:CompilePassSurface | Lawful code paths proved by compile-pass fixtures |
| AuditGate | compat:EvidenceState transitions | State machine validity gates |
| ForbiddenPattern | compat:ForbiddenLaw enum variants | Patterns made unrepresentable by types |
| AllowedContext | compat:WitnessMarker | Contexts where patterns are permitted |
| CommitEvidence | compat:Receipt | Manufacturing receipts attached to commits |
| CheckpointClaim | compat:Receipted state | Terminal state for immutable audit checkpoint |

---

## Property Statistics

**Total RDF Properties Defined:** 42

### Property Groups

1. **Spec Properties** (9 properties)
   - specName, lawName, auditScope, scopeTarget, carrierType, witnessRequired, stateRequired, verdictType, applicableTemplate

2. **Template Properties** (2 properties)
   - templateName, templateKind

3. **Executable Properties** (4 properties)
   - instantiatedBy, executableId, fixtureFile, expectedStderr

4. **Gate Properties** (4 properties)
   - gateCondition, gateName, gateDescription, gateSeverity

5. **Pattern & Context Properties** (5 properties)
   - forbiddenBy, patternName, patternDescription, allowedIn, contextName, contextScope, exceptionReason

6. **Gap Properties** (8 properties)
   - gapId, gapCategory, relatedLaw, gapSeverity, remediationPlan, gapOpenedAt, gapOpenedDate

7. **CommitEvidence Properties** (5 properties)
   - commitHash, commitAuthor, commitTimestamp, commitMessage, parentCommit, carriesReceipt

8. **Closure Properties** (4 properties)
   - closesGap, closingCommit, closureMethod, closureTimestamp

9. **Checkpoint Properties** (6 properties)
   - checkpointName, checkpointCommit, checkpointTimestamp, allLawsCovered, allGapsClosed, passedAllGates

---

## Namespace Declarations

| Prefix | Namespace | Purpose |
|---|---|---|
| `audit:` | https://wasm4pm-compat.rs/audit# | Audit machinery classes and properties |
| `compat:` | https://wasm4pm-compat.rs/ontology# | wasm4pm-compat type law (witness, evidence state, compile-fail law) |
| `prov:` | http://www.w3.org/ns/prov# | W3C Provenance (Plan, Activity, Entity, Bundle) |
| `dct:` | http://purl.org/dc/terms/ | Dublin Core (created, creator, description) |
| `skos:` | http://www.w3.org/2004/02/skos/core# | Knowledge Organization System (related, seeAlso) |
| `sh:` | http://www.w3.org/ns/shacl# | SHACL shapes (Shape, NodeShape) |
| `rdf:` | http://www.w3.org/1999/02/22-rdf-syntax-ns# | RDF core (Property, type) |
| `rdfs:` | http://www.w3.org/2000/01/rdf-schema#| RDF Schema (Class, label, comment, domain, range) |
| `xsd:` | http://www.w3.org/2001/XMLSchema# | XML Schema datatypes (string, integer, boolean, dateTime, date) |
| `owl:` | http://www.w3.org/2002/07/owl# | OWL (Ontology, imports) |

---

## Query Examples (SPARQL)

### Query 1: All Forbidden Patterns and Their Proof Fixtures
```sparql
PREFIX audit: <https://wasm4pm-compat.rs/audit#>
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>

SELECT ?pattern ?lawName ?fixturePath
WHERE {
  ?pattern a audit:ForbiddenPattern ;
    audit:patternName ?name ;
    audit:forbiddenBy ?law .
  ?law compat:lawName ?lawName ;
    compat:fixtureFile ?fixturePath .
}
ORDER BY ?pattern
```

### Query 2: All Open Gaps by Severity
```sparql
PREFIX audit: <https://wasm4pm-compat.rs/audit#>

SELECT ?gapId ?gapCategory ?severity ?remediation
WHERE {
  ?gap a audit:Gap ;
    audit:gapId ?gapId ;
    audit:gapCategory ?gapCategory ;
    audit:gapSeverity ?severity ;
    audit:remediationPlan ?remediation .
}
ORDER BY ?severity DESC
```

### Query 3: All Audit Specs and Their Verdict Types
```sparql
PREFIX audit: <https://wasm4pm-compat.rs/audit#>

SELECT ?spec ?lawName ?verdictType ?template
WHERE {
  ?spec a audit:AuditSpec ;
    audit:lawName ?lawName ;
    audit:verdictType ?verdictType ;
    audit:applicableTemplate ?template .
}
ORDER BY ?spec
```

### Query 4: Closure Claims with Associated Gaps
```sparql
PREFIX audit: <https://wasm4pm-compat.rs/audit#>

SELECT ?gapId ?closingCommit ?method ?timestamp
WHERE {
  ?claim a audit:ClosureClaim ;
    audit:closesGap ?gap ;
    audit:closingCommit ?closingCommit ;
    audit:closureMethod ?method ;
    audit:closureTimestamp ?timestamp .
  ?gap audit:gapId ?gapId .
}
ORDER BY ?timestamp DESC
```

---

## Statistical Summary

| Entity | Count | Notes |
|---|---|---|
| Total RDF Triples | 347 | Ontology declaration + instances |
| Classes Defined | 12 | Core audit abstractions |
| Properties Defined | 42 | Spec, template, gate, pattern, gap, commit, closure |
| Audit Specifications | 7 | Named audit contracts |
| Audit Templates | 5 | Implementation patterns (compile-fail, compile-pass, runtime, static, OCEL-conformance) |
| Audit Gates | 6 | Proof gates for ALIVE certification |
| Forbidden Patterns | 5 | Unbreakable laws |
| Allowed Contexts | 3 | Named exceptions |
| Known Gaps | 3 | GAP_001 (major), GAP_002 (minor), GAP_003 (critical) |
| Checkpoint Claims | 2 | PAPERLAW_ALIVE_002, PAPERLAW_CROWN_ALIVE_004 |
| Hostile Assumptions | 8 | Chicago-TDD doctrine axioms |

---

## File Paths

- **Ontology File:** `/Users/sac/wasm4pm-compat/ggen/ontology/audit-machinery.ttl` (1150+ lines)
- **Graph Report:** `/Users/sac/wasm4pm-compat/emitted/audit-machinery/graph-report.md` (this file)
- **Integration:** Imports `ggen/ontology/wasm4pm-compat.ttl` and references W3C standard vocabularies

---

## Next Steps for Audit Execution

1. **Fixture Implementation**
   - Ensure all AuditSpec instances have corresponding fixtures in `tests/ui/`
   - Generate missing .stderr files for compile-fail fixtures

2. **Gap Closure**
   - Assign remediation ownership to GAP_001, GAP_002, GAP_003
   - Implement OCEL-based audit infrastructure for conformance checking

3. **SPARQL Query Integration**
   - Load audit-machinery.ttl into a triple store (Apache Jena, Oxigraph, etc.)
   - Execute audit gate queries to drive CI/CD gate logic

4. **Hostile Assumption Auditing**
   - Derive event logs from trybuild and CI execution
   - Run process mining conformance checks against declared audit model
   - Flag discrepancies between declared and discovered audit process

5. **Cross-Crate Audit (wasm4pm Integration)**
   - Link GraduationBoundary instances to actual wasm4pm entry points
   - Verify receipt graduations produce valid evidence carriers in wasm4pm

---

## Appendix: Vocabulary Alignment

### W3C PROV Alignment
- `AuditSpec` ⊆ `prov:Plan` — audit is a planned activity
- `AuditExecutable` ⊆ `prov:Activity` — audit is an activity execution
- `CommitEvidence` ⊆ `prov:EntityBundle` — commits are bundles of evidence entities
- `ClosureClaim` ⊆ `prov:Qualification` — closure claims qualify relationships
- `Gap` ⊆ `prov:Fault` — gaps are faults in the audit model

### W3C SHACL Alignment
- `AuditTemplate` ⊆ `sh:Shape` — templates define shapes for audit data
- `AuditGate` ⊆ `sh:NodeShape` — gates are node shape validation rules

### SKOS Vocabulary
- `ForbiddenPattern` `skos:related` `compat:CompileFailLaw` — patterns are related to laws
- `AllowedContext` `skos:related` `compat:WitnessMarker` — contexts are named scopes
- `Gap` `skos:related` `prov:Fault` — gaps are documented faults

---

**Report Generated:** 2026-06-01 by SPARC Audit Machinery  
**Ontology Version:** 0.1.0  
**License:** Same as wasm4pm-compat (nightly-first, zero-cost type law)
