# Audit Machinery Ontology — Integration Guide

**Date:** 2026-06-01  
**Files:** 
- Core ontology: `ggen/ontology/audit-machinery.ttl` (900 lines, 45 KB)
- Graph report: `emitted/audit-machinery/graph-report.md` (554 lines, 20 KB)
- This guide: `emitted/audit-machinery/INTEGRATION.md`

---

## Overview

The audit-machinery.ttl ontology is a machine-readable specification of audit law surfaces for wasm4pm-compat. It represents:

- **7 core audit specifications** (RawEvidenceExportedAsAdmitted, WitnessDiscrimination, SilentStructureLoss, BareInvalidInputRefusal, TypeStateAdmissionBoundary, ReceiptProvenance, and 2 more)
- **5 audit templates** (compile-fail fixtures, compile-pass fixtures, runtime validation, static analysis, OCEL event-log conformance)
- **6 proof gates** (AllLawsHaveFixtures, AllFixturesHaveStderr, AllGapsClosed, AllCommitsConventional, AllReceiptsValidate, NoSilentLoss)
- **5 forbidden patterns** with compile-fail proofs
- **3 allowed contexts** (wasm4pm graduation, internal bridge, named projection)
- **3 known gaps** with remediation plans (GAP_001 major, GAP_002 minor, GAP_003 critical)
- **8 hostile assumptions** from the process-mining-chicago-tdd doctrine

---

## File Contents Summary

### ggen/ontology/audit-machinery.ttl

**347 RDF triples** representing the complete audit law graph.

**Structure:**
1. Namespace declarations (10 prefixes: audit:, compat:, prov:, dct:, skos:, sh:, rdf:, rdfs:, xsd:, owl:)
2. Ontology header (OWL declaration, imports wasm4pm-compat.ttl)
3. Class definitions (12 classes: AuditSpec, AuditTemplate, AuditExecutable, AuditGate, ForbiddenPattern, AllowedContext, Gap, CommitEvidence, ClosureClaim, AuxiliaryCommit, CheckpointClaim, HostileAssumption)
4. Property definitions (42 properties)
5. Hostile assumption statements (8 instances)
6. Audit specifications (7 instances)
7. Audit templates (5 instances)
8. Audit gates (6 instances)
9. Forbidden patterns (5 instances)
10. Allowed contexts (3 instances)
11. Known gaps (3 instances)
12. Sample commit evidence (2 instances)
13. Sample closure claims (1 instance)
14. Checkpoint claims (2 instances)
15. Extended audit specifications (3 instances)
16. PROV integration statements

**Key Integration Points with wasm4pm-compat:**
- Imports: `owl:imports <https://wasm4pm-compat.rs/ontology>`
- Cross-references: `compat:WitnessMarker`, `compat:EvidenceState`, `compat:CompileFailLaw`, `compat:GraduationBoundary`, `compat:ProcessForm`
- Links compile-fail/pass fixtures to audit laws
- Maps audit templates to execution patterns in the crate

### emitted/audit-machinery/graph-report.md

**554 lines** of structured documentation.

**Sections:**
1. Executive summary
2. Class hierarchy (13 classes with relationships)
3. Core audit specifications (7 specs with laws, scopes, carriers, verdict types, linked fixtures)
4. Audit templates (5 patterns with instance counts)
5. Audit gates (6 gates with severity levels and conditions)
6. Forbidden patterns (5 patterns with manifestations, proofs, reasons)
7. Allowed contexts (3 contexts with scopes and exceptions)
8. Known gaps (3 gaps with severity, remediation, status)
9. Checkpoint claims (2 sealed ALIVE milestones)
10. Hostile assumptions (8 axioms with audit responses)
11. Integration with wasm4pm-compat type law (mapping table)
12. Property statistics (42 properties in 9 groups)
13. Namespace declarations (10 prefixes with purposes)
14. Query examples (4 SPARQL queries for audit operations)
15. Statistical summary (entity counts)
16. Next steps for audit execution
17. Appendix: W3C vocabulary alignment (PROV, SHACL, SKOS)

---

## Integration Points

### 1. Type Law Integration (wasm4pm-compat.ttl)

The audit machinery is tightly bound to the Rust type law surface:

| Audit Concept | Type Law Class | Link |
|---|---|---|
| AuditSpec | compat:CompileFailLaw | Named law (e.g., RawEvidenceExportedAsAdmitted) |
| AuditTemplate | compat:CompilePassSurface | Lawful code path proved by fixture |
| AuditExecutable | compat:SourceModule | Audit implemented in src/*.rs |
| ForbiddenPattern | compat:CompileFailLaw | Law made unrepresentable by type system |
| CommitEvidence | compat:Receipt | Manufacturing receipt at commit |
| CheckpointClaim | compat:Receipted state | Terminal ALIVE audit checkpoint |

**Query to verify integration:**
```sparql
PREFIX audit: <https://wasm4pm-compat.rs/audit#>
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>

SELECT ?spec ?lawName ?fixture
WHERE {
  ?spec a audit:AuditSpec ;
    audit:lawName ?lawName ;
    audit:applicableTemplate ?tmpl .
  ?tmpl a audit:AuditTemplate ;
    audit:templateKind ?kind .
  ?law a compat:CompileFailLaw ;
    compat:lawName ?lawName ;
    compat:fixtureFile ?fixture .
}
```

### 2. Process-Mining-Chicago-TDD Doctrine Integration

The audit machinery implements all 8 hostile assumptions from the doctrine:

| Assumption | Audit Response | Implementation |
|---|---|---|
| Declared Pipeline ≠ Real | Derive actual process from logs | OCEL conformance template |
| Receipts Emitted Out-of-Cycle | Validate timestamps + lifecycle | ReceiptProvenanceSpec |
| Proof Gates May Pass Falsely | Independent conformance check | OCEL event-log replay |
| Release from Invalid | Verify object lifecycle | EvidenceLifecycleSoundnessSpec |
| Variant Explosion | Drift analysis + metrics | Process cube analysis |
| Stages Skipped | Verify state transitions | OCEL state sequence validation |
| Rework Undetected | Log skeleton + causal graphs | Cross-object causality checking |
| Unsoundness Hidden | wasm4pm verification | SoundnessWitnessed witness path |

### 3. Compile-Fail Fixture Integration

Every compile-fail law in `compat:CompileFailLaw` maps to a fixture in `audit:AuditExecutable`:

**Example mapping:**
```
compat:RawExportedAsAdmittedLaw
  ├── compat:lawName "RawEvidenceExportedAsAdmitted"
  ├── compat:errorCode "E0308"
  ├── compat:fixtureFile "tests/ui/compile_fail/raw_evidence_exported_as_admitted.rs"
  └── compat:stderrFile "tests/ui/compile_fail/raw_evidence_exported_as_admitted.stderr"
    │
    ↓ links to
    │
audit:RawExportedAsAdmittedPattern
  ├── audit:patternName "raw-evidence-exported-as-admitted"
  ├── audit:forbiddenBy compat:RawExportedAsAdmittedLaw
  └── audit:patternDescription "Type system makes this unrepresentable"
    │
    ↓ proves
    │
audit:RawEvidenceExportedAsAdmittedSpec
  ├── audit:lawName "RawEvidenceExportedAsAdmitted"
  ├── audit:applicableTemplate audit:TryBuildCompileFailTemplate
  └── audit:verdictType "boolean"
```

### 4. Gap Tracking Integration

Gaps link audit laws to remediation commitments:

**Example:**
```
audit:Gap001_UnimplementedConformanceAudit
  ├── audit:gapId "GAP_001"
  ├── audit:gapCategory "missing-fixture"
  ├── audit:relatedLaw compat:ConformanceAuthority
  ├── audit:gapSeverity "major"
  ├── audit:remediationPlan "Implement OCEL-based audit of wasm4pm conformance..."
  └── [NO ClosureClaim yet] → Audit gate fails until closed

    → When closed:
    │
    └── audit:ClosureClaim_Gap001_Partial
          ├── audit:closesGap audit:Gap001_UnimplementedConformanceAudit
          ├── audit:closingCommit "2426fac"
          └── audit:closureMethod "fixture-added"
```

### 5. ALIVE Checkpoint Integration

Checkpoints are sealed audit milestones linking to commit evidence:

**Example:**
```
audit:CheckpointAlive002
  ├── audit:checkpointName "PAPERLAW_ALIVE_002"
  ├── audit:checkpointCommit "2426fac"
  ├── audit:allLawsCovered true
  ├── audit:allGapsClosed false (but accepted-debt exceptions apply)
  ├── audit:passedAllGates true
  └── "Sealed; all declared laws covered by fixtures"
```

---

## Usage Patterns

### Pattern 1: Audit Specification Lookup

Find all audit specifications and their verdict types:

```sparql
PREFIX audit: <https://wasm4pm-compat.rs/audit#>
SELECT ?spec ?lawName ?verdictType ?template
WHERE {
  ?spec a audit:AuditSpec ;
    audit:lawName ?lawName ;
    audit:verdictType ?verdictType ;
    audit:applicableTemplate ?template .
}
ORDER BY ?lawName
```

### Pattern 2: Forbidden Pattern Proof Chain

Trace a pattern from its name through proofs to fixtures:

```sparql
PREFIX audit: <https://wasm4pm-compat.rs/audit#>
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>

SELECT ?patternName ?lawName ?errorCode ?fixturePath
WHERE {
  ?pattern a audit:ForbiddenPattern ;
    audit:patternName ?patternName ;
    audit:forbiddenBy ?law .
  ?law compat:lawName ?lawName ;
    compat:errorCode ?errorCode ;
    compat:fixtureFile ?fixturePath .
}
```

### Pattern 3: Gap Status Report

List all gaps with severity and closure status:

```sparql
PREFIX audit: <https://wasm4pm-compat.rs/audit#>

SELECT ?gapId ?severity ?category ?hasClosureClaim ?remediationPlan
WHERE {
  ?gap a audit:Gap ;
    audit:gapId ?gapId ;
    audit:gapSeverity ?severity ;
    audit:gapCategory ?category ;
    audit:remediationPlan ?remediationPlan .
  OPTIONAL { ?claim a audit:ClosureClaim ; audit:closesGap ?gap . }
  BIND(BOUND(?claim) AS ?hasClosureClaim)
}
ORDER BY DESC(?severity)
```

### Pattern 4: ALIVE Gate Audit

Check all gates for a checkpoint:

```sparql
PREFIX audit: <https://wasm4pm-compat.rs/audit#>

SELECT ?gateName ?condition ?severity ?description
WHERE {
  ?gate a audit:AuditGate ;
    audit:gateName ?gateName ;
    audit:gateCondition ?condition ;
    audit:gateSeverity ?severity ;
    audit:gateDescription ?description .
}
ORDER BY DESC(?severity) ?gateName
```

---

## Loading and Querying the Ontology

### Apache Jena (TDB2)

```bash
tdbloader --loc=audit_db ggen/ontology/wasm4pm-compat.ttl ggen/ontology/audit-machinery.ttl
sparql --loc=audit_db --query=query.rq
```

### Oxigraph

```bash
oxigraph load --location audit_store ggen/ontology/audit-machinery.ttl
oxigraph query --location audit_store "SELECT ... WHERE ..."
```

### Python (rdflib)

```python
from rdflib import Graph

g = Graph()
g.parse("ggen/ontology/wasm4pm-compat.ttl", format="turtle")
g.parse("ggen/ontology/audit-machinery.ttl", format="turtle")

query = """
  PREFIX audit: <https://wasm4pm-compat.rs/audit#>
  SELECT ?gap ?severity WHERE {
    ?gap a audit:Gap ; audit:gapSeverity ?severity .
  }
"""
results = g.query(query)
for row in results:
    print(f"{row.gap} → {row.severity}")
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Audit Gate Check

on: [push, pull_request]

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Load ontology
        run: |
          apt-get install -y oxigraph
          oxigraph load --location /tmp/audit_store \
            ggen/ontology/wasm4pm-compat.ttl \
            ggen/ontology/audit-machinery.ttl
      - name: Check all gates pass
        run: |
          oxigraph query --location /tmp/audit_store \
            --query-file ggen/queries/all_gates_pass.rq
      - name: Check no critical gaps
        run: |
          oxigraph query --location /tmp/audit_store \
            --query-file ggen/queries/no_critical_gaps.rq
```

### SPARQL Query Files for CI

**ggen/queries/all_gates_pass.rq:**
```sparql
PREFIX audit: <https://wasm4pm-compat.rs/audit#>
ASK {
  ?gate a audit:AuditGate ;
    audit:gateSeverity "fatal" .
  # For each gate, check its condition passes
  # (implementation depends on gate engine)
}
```

**ggen/queries/no_critical_gaps.rq:**
```sparql
PREFIX audit: <https://wasm4pm-compat.rs/audit#>
ASK {
  FILTER NOT EXISTS {
    ?gap a audit:Gap ;
      audit:gapSeverity "critical" ;
      audit:gapCategory ?cat .
    FILTER NOT EXISTS {
      ?claim a audit:ClosureClaim ;
        audit:closesGap ?gap .
    }
  }
}
```

---

## Maintenance and Updates

### Adding a New Audit Specification

1. **Create the AuditSpec instance:**
   ```turtle
   audit:MyNewSpec
       a audit:AuditSpec ;
       audit:specName "my-new-law" ;
       audit:lawName "MyNewLaw" ;
       audit:auditScope "module" ;
       audit:scopeTarget "src/my_module.rs" ;
       audit:carrierType "Evidence<T, Admitted, W>" ;
       audit:verdictType "boolean" ;
       audit:applicableTemplate audit:TryBuildCompileFailTemplate ;
       rdfs:comment "Audit: ..." .
   ```

2. **Link to a compile-fail law in wasm4pm-compat.ttl:**
   ```turtle
   compat:MyNewLaw
       compat:lawName "MyNewLaw" ;
       compat:fixtureFile "tests/ui/compile_fail/my_new_law.rs" .
   ```

3. **Create the fixture:**
   ```rust
   // tests/ui/compile_fail/my_new_law.rs
   // This should NOT compile
   ```

4. **Create the .stderr file:**
   ```
   error[E0308]: expected type A, found type B
   ...
   ```

### Adding a New Gap

1. **Create the Gap instance:**
   ```turtle
   audit:GapNNN_Description
       a audit:Gap ;
       audit:gapId "GAP_NNN" ;
       audit:gapCategory "missing-fixture" ;
       audit:relatedLaw compat:SomeLaw ;
       audit:gapSeverity "major" ;
       audit:remediationPlan "..." ;
       audit:gapOpenedDate "2026-06-01T..."^^xsd:dateTime .
   ```

2. **Update graph report** with gap details.

3. **When closing, add a ClosureClaim:**
   ```turtle
   audit:ClosureClaimGapNNN
       a audit:ClosureClaim ;
       audit:closesGap audit:GapNNN_Description ;
       audit:closingCommit "abc1234" ;
       audit:closureMethod "fixture-added" ;
       audit:closureTimestamp "2026-06-02T..."^^xsd:dateTime .
   ```

---

## Compliance Checklist

Use this checklist to verify audit machinery is correctly integrated:

- [ ] All 7 core audit specifications have linked compile-fail or compile-pass fixtures
- [ ] All 5 forbidden patterns have proofs (compile-fail fixtures with .stderr files)
- [ ] All 6 audit gates have documented conditions and severity levels
- [ ] All 3 known gaps have remediation plans and target milestones
- [ ] All critical gaps have ClosureClaim instances with commit evidence
- [ ] All commits follow conventional commit format (checked by AllCommitMessagesConventionalGate)
- [ ] All Receipt instances have valid provenance (checked by AllReceiptsValidateGate)
- [ ] No Evidence<T, Projected, W> without LossReport (checked by NoSilentLossInProjectionsGate)
- [ ] Graph report is current (last updated within 7 days)
- [ ] SPARQL queries in CI pass (all gates)

---

## Next Steps

1. **Load the ontology into a triple store** (Apache Jena or Oxigraph)
2. **Configure CI/CD gates** using the SPARQL queries provided
3. **Generate a daily audit report** via SPARQL query results
4. **Track gap closures** as commits are merged
5. **Seal new checkpoints** when all gates pass and critical gaps close

---

**Document Version:** 0.1.0  
**Last Updated:** 2026-06-01  
**Maintainer:** SPARC Audit Machinery
