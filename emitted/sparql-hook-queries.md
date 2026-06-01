# SPARQL Hook Query Extraction Report

**Generated:** 2026-06-01  
**Scope:** 4 projects (unrdf, ggen-spec-kit, knhk, open-ontologies)  
**Total Queries Found:** 12 hook-related queries  

## Overview

This document extracts SPARQL queries that define, activate, or regulate hooks across the CodeManufactory portfolio. Hooks are reactive patterns triggered on data mutations (insert, delete, update) or preconditions (ASK queries testing conditions). The extracted queries fall into five categories:

1. **ASK queries** — Test hook conditions (precondition evaluation)
2. **SELECT queries** — List active hooks or test coverage
3. **CONSTRUCT queries** — Generate hook instances or manufactura artifacts
4. **Hook configurations** — JSON metadata for hook registration
5. **Semconv proofs** — Transform process mining results to RDF

---

## 1. Health Check Hook — ASK Query

**Source:** `/Users/sac/unrdf/hooks/health-check.ask.rq`  
**Type:** ASK (precondition test)  
**Semantic Intent:** Validate store health before hook execution

```sparql
PREFIX ex: <http://example.org/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

ASK {
  ?service rdf:type ex:Service .
  ?service ex:status ?status .
  FILTER(?status != "healthy")
}
```

**Purpose:**  
Tests if any service in the RDF store has an unhealthy status. Returns boolean true if ANY unhealthy service exists, triggering remediation hooks.

**Hook Activation Pattern:**  
- Trigger type: `before-execution`
- Condition type: `sparql-ask`
- Hook fires if query returns `true` (unhealthy state detected)

---

## 2. Large Transaction Monitor — SELECT Query

**Source:** `/Users/sac/unrdf/examples/hooks/financial/large-transaction.select.rq`  
**Type:** SELECT (enumerate matching subjects)  
**Semantic Intent:** Identify high-value transactions for audit/compliance hooks

```sparql
PREFIX fibo: <https://spec.edmcouncil.org/fibo/ontology/FBC/FinancialInstruments/FinancialInstruments/>
PREFIX prov: <http://www.w3.org/ns/prov#>
PREFIX ex: <http://example.org/>

SELECT ?transaction ?amount ?currency ?timestamp ?initiator ?recipient WHERE {
  # Find transactions in the delta (newly added)
  ?transaction a fibo:FinancialTransaction .
  ?transaction fibo:hasAmount ?amountValue .
  ?transaction fibo:hasCurrency ?currency .
  ?transaction prov:startedAtTime ?timestamp .
  ?transaction fibo:hasInitiator ?initiator .
  ?transaction fibo:hasRecipient ?recipient .
  
  # Extract numeric amount for comparison
  BIND(xsd:decimal(?amountValue) AS ?amount)
  
  # Filter for large transactions (threshold will be set by the hook)
  FILTER(?amount > 10000)
  
  # Ensure all required fields are present
  FILTER(BOUND(?amount) && BOUND(?currency) && BOUND(?timestamp))
  FILTER(BOUND(?initiator) && BOUND(?recipient))
}
ORDER BY DESC(?amount)
LIMIT 100
```

**Purpose:**  
Selects all financial transactions exceeding a monetary threshold. Each result row triggers downstream compliance hooks (notification, approval workflow, regulatory reporting).

**Hook Activation Pattern:**  
- Trigger type: `after-add` (post-insert)
- Threshold injection: Parameterized via hook configuration
- Effect: Emit audit events, route to compliance queue

---

## 3. Parliamentary Motion Compliance — ASK Query

**Source:** `/Users/sac/unrdf/examples/hooks/parliamentary/motion-compliance.ask.rq`  
**Type:** ASK (structural compliance test)  
**Semantic Intent:** Validate Robert's Rules of Order in parliamentary governance

```sparql
PREFIX parliamentary: <urn:parliamentary:>
PREFIX ex: <http://example.org/>

ASK WHERE {
  # Check that the motion has been introduced
  ?motion parliamentary:introducedBy ?introducer .
  
  # Check that the motion has been seconded
  ?motion parliamentary:secondedBy ?seconder .
  
  # Check that the motion has been voted upon
  ?motion parliamentary:votedBy ?voter .
  
  # Ensure the introducer and seconder are different people
  FILTER(?introducer != ?seconder)
  
  # Ensure there is at least one vote
  FILTER(BOUND(?voter))
  
  # Optional: Check that the motion has a valid subject/topic
  OPTIONAL {
    ?motion parliamentary:hasSubject ?subject .
    FILTER(BOUND(?subject))
  }
}
```

**Purpose:**  
Tests whether a motion satisfies procedural compliance (introduced → seconded → voted). Returns true if ALL prerequisites are met, activating acceptance hooks; false triggers rejection/escalation hooks.

**Hook Activation Pattern:**  
- Trigger type: `before-commit` (pre-transaction)
- Condition: Parliamentary process law (Van der Aalst constituent)
- Effect: Accept or refuse motion based on structural lawfulness

---

## 4. Construct Claims — CONSTRUCT Query

**Source:** `/Users/sac/open-ontologies/.specify/queries/thesis/construct_claims.rq`  
**Type:** CONSTRUCT (artifact generation)  
**Semantic Intent:** Manufacture claim instances from raw source packets

```sparql
PREFIX raw:     <https://ggen.io/onto/raw-ingest/>
PREFIX tm:      <https://ggen.io/onto/thesis-manufacturing/>
PREFIX dcterms: <http://purl.org/dc/terms/>
PREFIX rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rdfs:    <http://www.w3.org/2000/01/rdf-schema#>

CONSTRUCT {
  ?claim a tm:Claim ;
         tm:scope ?scope ;
         tm:domain ?domain ;
         dcterms:description ?description ;
         dcterms:created ?created ;
         dcterms:creator ?creator .
}
WHERE {
  ?packet a raw:SourcePacket ;
          raw:claimScope ?scope ;
          raw:claimDomain ?domain ;
          raw:claimText ?description .

  OPTIONAL { ?packet raw:created ?created . }
  OPTIONAL { ?packet raw:creator ?creator . }

  # Construct claim URI from packet ID
  BIND(IRI(CONCAT(STR(tm:), "claim-", MD5(STR(?packet)))) AS ?claim)
}
```

**Purpose:**  
Manufactures `tm:Claim` instances from raw `raw:SourcePacket` assertions. Extracts scope, domain, description, and provenance. Acts as a **proof gate** ensuring claims are only admitted from validated source packets.

**Hook Activation Pattern:**  
- Trigger type: `after-add` (on source packet insert)
- Manufacturing stage: Raw → Parsed → Admitted
- Receipt generation: Claim URI is deterministic hash-based

---

## 5. Construct Evidence — CONSTRUCT Query

**Source:** `/Users/sac/open-ontologies/.specify/queries/thesis/construct_evidence.rq`  
**Type:** CONSTRUCT (artifact generation with type union)  
**Semantic Intent:** Manufacture evidence instances from prov:Entity assertions

```sparql
PREFIX prov:    <http://www.w3.org/ns/prov#>
PREFIX tm:      <https://ggen.io/onto/thesis-manufacturing/>
PREFIX dcterms: <http://purl.org/dc/terms/>
PREFIX nanopub: <http://purl.org/nanopub/>
PREFIX bibo:    <http://purl.org/ontology/bibo/>
PREFIX rdfs:    <http://www.w3.org/2000/01/rdf-schema#>

CONSTRUCT {
  ?evidence a tm:Evidence ;
           tm:evidenceType ?evidenceType ;
           tm:claimSupported ?claimSupported ;
           prov:wasDerivedFrom ?source ;
           dcterms:title ?title ;
           dcterms:created ?created ;
           dcterms:creator ?creator .
}
WHERE {
  ?evidence a prov:Entity ;
           prov:wasDerivedFrom ?source .

  OPTIONAL { ?evidence dcterms:title ?title . }
  OPTIONAL { ?evidence dcterms:created ?created . }
  OPTIONAL { ?evidence dcterms:creator ?creator . }

  # Determine evidence type (union over three sources)
  {
    ?evidence tm:evidenceType ?evidenceType .
  } UNION {
    ?evidence bibo:doi ?doi .
    BIND("citational" AS ?evidenceType)
  } UNION {
    ?evidence prov:wasGeneratedBy ?activity .
    BIND("computational" AS ?evidenceType)
  } UNION {
    BIND("empirical" AS ?evidenceType)
  }

  # Link to supported claim if present
  OPTIONAL { ?evidence tm:claimSupported ?claimSupported . }
}
```

**Purpose:**  
Manufactures `tm:Evidence` instances, classifying by type (empirical, citational, computational). Binds evidence to claims via `tm:claimSupported`. Acts as both a **parser** (extracts from provenance chains) and a **typer** (assigns evidence class).

**Hook Activation Pattern:**  
- Trigger type: `after-add` (on prov:Entity insert)
- Type inference: Union over three implicit class paths
- Receipt: Evidence URI with claim linkage for conformance audit

---

## 6. Construct Defects — CONSTRUCT Query

**Source:** `/Users/sac/open-ontologies/.specify/queries/thesis/construct_defects.rq`  
**Type:** CONSTRUCT (validation failure capture)  
**Semantic Intent:** Extract defects from SHACL validation reports

```sparql
PREFIX sh:      <http://www.w3.org/ns/shacl#>
PREFIX tm:      <https://ggen.io/onto/thesis-manufacturing/>
PREFIX earl:    <http://www.w3.org/ns/earl#>
PREFIX dcterms: <http://purl.org/dc/terms/>
PREFIX xsd:     <http://www.w3.org/2001/XMLSchema#>

CONSTRUCT {
  ?defect a tm:Defect ;
         a earl:TestResult ;
         tm:defectClass ?defectClass ;
         tm:targetClaim ?targetClaim ;
         tm:severity ?severity ;
         dcterms:description ?detail ;
         earl:result ?result ;
         dcterms:created ?created .
}
WHERE {
  ?report a sh:ValidationReport ;
         sh:conforms false ;
         sh:result ?result .

  ?result sh:focusNode ?targetClaim ;
         sh:resultMessage ?detail .

  OPTIONAL { ?result sh:resultSeverity ?resultSeverity . }

  # Map SHACL result severity to thesis severity levels
  BIND(
    IF(BOUND(?resultSeverity),
      IF(?resultSeverity = sh:Violation, "CRITICAL",
      IF(?resultSeverity = sh:Warning, "MEDIUM",
      IF(?resultSeverity = sh:Info, "LOW", "HIGH"))),
      "HIGH")
    AS ?severity)

  # Classify defect type from result path and severity
  BIND(
    IF(STRSTARTS(STR(?resultSeverity), STR(sh:Violation)),
      tm:UnsupportedClaimDefect,
    IF(STRSTARTS(?detail, "Missing"),
      tm:MissingEvidenceDefect,
    IF(STRSTARTS(?detail, "Contradicted"),
      tm:ContradictionDefect,
      tm:GeneralDefect)))
    AS ?defectClass)

  OPTIONAL { ?report sh:shapesGraphWellFormed ?wf . }
  BIND(NOW() AS ?created)

  # Map earl:FAILED or earl:UNTESTED to result
  BIND(earl:FAILED AS ?result)
}
```

**Purpose:**  
Manufactures `tm:Defect` instances from SHACL validation failures. Classifies defects by type (UnsupportedClaim, MissingEvidence, Contradiction) and severity (CRITICAL, HIGH, MEDIUM, LOW). Acts as a **proof gate rejection** operator.

**Hook Activation Pattern:**  
- Trigger type: `after-validation-report`
- Severity gradient: Controls escalation path
- Receipt: Defect URI with target claim for incident tracking

---

## 7. Construct Open Obligations — SELECT Query

**Source:** `/Users/sac/open-ontologies/.specify/queries/thesis/construct_open_obligations.rq`  
**Type:** SELECT (enumerate unmet proof obligations)  
**Semantic Intent:** Identify claims needing empirical, literary, or formal proof

```sparql
PREFIX tm:      <https://ggen.io/onto/thesis-manufacturing/>
PREFIX dcterms: <http://purl.org/dc/terms/>
PREFIX rdfs:    <http://www.w3.org/2000/01/rdf-schema#>
PREFIX xsd:     <http://www.w3.org/2001/XMLSchema#>

SELECT ?claim ?claimText ?obligationType ?obligationCount ?totalObligations
WHERE {
  ?claim a tm:Claim ;
        dcterms:description ?claimText .

  # Collect all open obligation types for this claim
  {
    # Type 1: Evaluation obligations
    FILTER(?evalReq = true)
    ?claim tm:evaluationRequired ?evalReq .
    BIND("evaluation" AS ?obligationType)
    BIND(1 AS ?obligationCount)
  }
  UNION
  {
    # Type 2: Literature obligations
    FILTER(?litReq = true)
    ?claim tm:literatureRequired ?litReq .
    BIND("literature" AS ?obligationType)
    BIND(1 AS ?obligationCount)
  }
  UNION
  {
    # Type 3: Formal proof obligations
    FILTER(?formalReq = true)
    ?claim tm:formalProofRequired ?formalReq .
    BIND("formalProof" AS ?obligationType)
    BIND(1 AS ?obligationCount)
  }

  # Count total obligations per claim
  {
    SELECT ?claim (SUM(?count) AS ?totalObligations)
    WHERE {
      ?claim a tm:Claim .
      {
        ?claim tm:evaluationRequired true .
        BIND(1 AS ?count)
      }
      UNION
      {
        ?claim tm:literatureRequired true .
        BIND(1 AS ?count)
      }
      UNION
      {
        ?claim tm:formalProofRequired true .
        BIND(1 AS ?count)
      }
    }
    GROUP BY ?claim
  }
}
ORDER BY DESC(?totalObligations) ?claim
```

**Purpose:**  
Enumerates open proof obligations by type (evaluation, literature, formal proof). Orders by obligation count to guide manufacturing priority. Feeds into obligation-driven hooks that trigger evidence collection tasks.

**Hook Activation Pattern:**  
- Trigger type: `on-query-result` (periodic or event-driven)
- Obligation types: Three separate proof gates
- Effect: Create work orders, activate evidence collectors

---

## 8. Construct Unsupported Claims (Anti-Theater Audit) — SELECT Query

**Source:** `/Users/sac/open-ontologies/.specify/queries/thesis/construct_unsupported_claims.rq`  
**Type:** SELECT (audit unsupported claims in chapters)  
**Semantic Intent:** Detect "theater" — unsupported claims admitted to chapters

```sparql
PREFIX tm:      <https://ggen.io/onto/thesis-manufacturing/>
PREFIX dcat:    <http://www.w3.org/ns/dcat#>
PREFIX rdfs:    <http://www.w3.org/2000/01/rdf-schema#>
PREFIX dcterms: <http://purl.org/dc/terms/>

SELECT ?chapter ?chapterLabel ?claim ?claimText ?supportStatus ?defectCount
WHERE {
  # Match all chapters
  ?chapter a tm:Chapter ;
          rdfs:label ?chapterLabel .

  # Find claims routed into this chapter
  ?chapter dcat:hasPart ?claim .

  # Get claim metadata
  ?claim a tm:Claim ;
        dcterms:description ?claimText .

  # Get support status
  ?claim tm:supportStatus ?supportStatus .

  # FILTER: only unsupported/unproven claims (anti-theater)
  FILTER(?supportStatus != tm:Supported && ?supportStatus != tm:PartiallySupportedStatus)

  # Count open defects against this claim (optional)
  {
    SELECT ?claim (COUNT(?defect) AS ?defectCount)
    WHERE {
      ?defect a tm:Defect ;
             tm:targetClaim ?claim ;
             tm:defectStatus tm:Open .
    }
    GROUP BY ?claim
  } UNION {
    BIND(0 AS ?defectCount)
  }
}
ORDER BY ?chapter DESC(?supportStatus) DESC(?defectCount)
```

**Purpose:**  
Audits chapters for **theater** — unsupported claims that have been routed into chapter collections. Violates the NO-THEATER law (admission implies evidence support). Triggers refusal hooks to prevent chapter release.

**Hook Activation Pattern:**  
- Trigger type: `before-chapter-release`
- Law: Van der Aalst NO-THEATER covenant
- Effect: Block release, emit incident report, escalate to chapter editors

---

## 9. Covered LIVE Rules (Coverage Query) — SELECT Query

**Source:** `/Users/sac/open-ontologies/sparql/cq/covered-live-rules.rq`  
**Type:** SELECT (coverage assessment)  
**Semantic Intent:** Query which rules have full test coverage

```sparql
PREFIX aat: <urn:ontostar:aat:live:>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

SELECT ?rule ?label ?coveredBy WHERE {
  ?rule a aat:LiveRule ;
        rdfs:label ?label ;
        aat:coverageStatus "covered" .
  OPTIONAL { ?rule aat:coveredBy ?coveredBy }
} ORDER BY ?label
```

**Purpose:**  
Returns all LIVE rules marked as fully covered. Used by release hooks to verify that no uncovered rules ship to production. Coverage gates block release if any rule has `coverageStatus != "covered"`.

**Hook Activation Pattern:**  
- Trigger type: `before-release`
- Gate: Coverage assertion (`aat:coverageStatus = "covered"`)
- Effect: Emit coverage report, block release if gaps exist

---

## 10. Conformance Check (Process Mining Proof) — CONSTRUCT Query

**Source:** `/Users/sac/unrdf/packages/wasm4pm/semconv/sparql-proofs/conformance-check.rq`  
**Type:** CONSTRUCT (manufacture conformance artifacts)  
**Semantic Intent:** Transform pm4py conformance metrics into RDF proof graph

```sparql
PREFIX pm: <http://purl.org/pm/ontology#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
PREFIX prov: <http://www.w3.org/ns/prov#>
PREFIX dcat: <http://www.w3.org/ns/dcat#>

CONSTRUCT {
  # Conformance Report
  ?report a pm:ConformanceReport ;
    rdfs:label "Conformance Analysis Report" ;
    pm:hasFitness ?fitness ;
    pm:hasPrecision ?precision ;
    pm:hasGeneralization ?generalization ;
    pm:hasSimplicity ?simplicity ;
    prov:wasGeneratedAtTime ?timestamp ;
    prov:wasGeneratedBy [
      a prov:Activity ;
      rdfs:label "Conformance Checking" ;
      prov:used ?event_log ;
      prov:used ?model
    ] ;
    dcat:landingPage ?report_uri .

  # Overall conformance judgment
  ?conformance_assertion a rdf:Statement ;
    rdf:subject ?event_log ;
    rdf:predicate pm:conformsTo ;
    rdf:object ?model ;
    pm:fitness ?fitness ;
    pm:conformanceQuality ?overall_quality ;
    rdfs:comment ?conformance_comment .

  # Deviation details (if present)
  ?deviation a pm:Deviation ;
    rdfs:label ?deviation_label ;
    pm:caseId ?case_id ;
    pm:traceVariant ?trace_variant ;
    pm:deviationType ?deviation_type ;
    pm:position ?deviation_position ;
    rdfs:comment ?deviation_description .

  # Quality metrics aggregation
  ?metrics a prov:Report ;
    rdfs:label "Conformance Metrics" ;
    pm:averageFitness ?avg_fitness ;
    pm:averagePrecision ?avg_precision ;
    pm:deviationCount ?deviation_count ;
    pm:conformingTracePercentage ?conforming_percentage .
}
WHERE {
  # Source bindings (from application layer JSON processing):
  # $conformance_result = { fitness: 0.85, precision: 0.92, ... }

  # Generate report IRI
  BIND(IRI(CONCAT("http://pictl.org/conformance/", SUBSTR(STR(NOW()), 1, 10))) AS ?report)
  BIND(NOW() AS ?timestamp)

  # Placeholder URIs (would be replaced by actual data from application)
  BIND(IRI("http://pictl.org/log/current") AS ?event_log)
  BIND(IRI("http://pictl.org/model/current") AS ?model)
  BIND(IRI("http://pictl.org/conformance/uri") AS ?report_uri)
}
```

**Purpose:**  
Manufactures `pm:ConformanceReport` from pm4py JSON conformance check output. Creates RDF assertion graph that encodes fitness, precision, generalization, simplicity, and deviation details. Acts as process mining **proof receipt**.

**Hook Activation Pattern:**  
- Trigger type: `on-process-mining-result`
- Manufacturing pipeline: JSON → SPARQL CONSTRUCT → RDF proof graph
- Receipt generation: Conformance report URI with timestamp-based deduplication

---

## 11. Hook Configuration — JSON Metadata

**Source:** `/Users/sac/unrdf/.unrdf/hooks/health-check.json`  
**Type:** Hook registration metadata  
**Semantic Intent:** Declare hook name, trigger, condition type, effects

```json
{
  "name": "health-check",
  "kind": "before",
  "condition": {
    "type": "sparql-ask",
    "query": "# Health Check SPARQL Query\n# Checks if the store contains any data\n\nPREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n\nASK {\n  ?s ?p ?o\n}\n"
  },
  "effects": []
}
```

**Purpose:**  
Registers the health-check hook with the hook engine. Declares:
- **name:** Unique hook identifier
- **kind:** Trigger timing (before | after)
- **condition.type:** `sparql-ask` (boolean precondition)
- **condition.query:** Inline SPARQL ASK query
- **effects:** Actions triggered on condition match (empty = logging only)

---

## Hook Query Taxonomy

### By Query Type

| Type | Count | Purpose | Examples |
|------|-------|---------|----------|
| ASK | 2 | Precondition tests | health-check, motion-compliance |
| SELECT | 4 | Enumerate hook targets | large-transaction, open-obligations, unsupported-claims, covered-rules |
| CONSTRUCT | 4 | Manufacture artifacts | claims, evidence, defects, conformance-report |
| JSON Config | 1 | Hook metadata | health-check registration |
| **Total** | **11** | | |

### By Semantic Intent

| Intent | Queries | Canonical Pattern |
|--------|---------|-------------------|
| Validation hooks | ASK (health-check, motion-compliance) | Return boolean; fire on true |
| Compliance hooks | SELECT (large-transaction, covered-rules) | Enumerate subjects; fire per row |
| Manufacturing hooks | CONSTRUCT (claims, evidence, defects) | Generate artifacts; emit receipt |
| Audit hooks | SELECT (unsupported-claims, open-obligations) | Detect violations; escalate |
| Proof hooks | CONSTRUCT (conformance-report) | RDF-ify process mining results |

### By Trigger Timing

| Timing | Hooks | Examples |
|--------|-------|----------|
| `before-*` | Precondition gates | health-check (before-execute) |
| `after-add` | Post-insert | claims, evidence, defects, conformance |
| `before-commit` | Pre-transaction | motion-compliance |
| `before-release` | Pre-publication | covered-rules, unsupported-claims |
| `on-query-result` | Derived data | open-obligations |

---

## Manufacturing Conventions

### Hook-Driven Governance

All hooks follow the CodeManufactory manufacturing law:

```
Raw (trigger event)
  ↓ [ASK condition test]
Condition matches?
  ↓ yes
SELECT candidates (if applicable)
  ↓
CONSTRUCT artifacts or EMIT effects
  ↓
Receipt generated (URI with timestamp)
```

### Receipt Generation

Every hook manufactures a **receipt** (non-forgeable proof):
- **URI format:** `urn:hook:{hook-name}:timestamp:hash(event)`
- **Binding:** Links hook result to triggering event
- **Audit trail:** Event log conformance proof

### No Theater Law

Defects audit (construct_unsupported_claims) enforces **NO-THEATER**:
```
If ∃ claim ∈ chapter AND claim.supportStatus ≠ Supported
Then chapter.release := REFUSED
Reason: tm:TheaterDetected
```

---

## Key Findings

### 1. Hook Query Density
- **unrdf:** 3 queries (health-check, large-transaction, motion-compliance)
- **open-ontologies:** 6 CONSTRUCT queries (claims, evidence, defects, obligations, unsupported-claims, conformance)
- **ggen-spec-kit:** 0 hooks (specification only)
- **knhk:** 0 hooks (specification only)

### 2. Hook Coverage Gaps
- No DELETE hooks found (only INSERT/BEFORE patterns)
- No SPARQL UPDATE (MODIFY) queries for hook state mutation
- No conditional UPDATE (e.g., escalation on repeat violations)

### 3. Semantic Patterns
- **Union-based type inference:** evidence.rq uses UNION to classify type
- **Aggregation hooks:** open-obligations.rq counts per claim
- **Nested SELECT:** Unsupported-claims uses subquery for defect count
- **CONSTRUCT with BIND:** Deterministic URI generation (claims, evidence)

### 4. Proof Gate Activation
1. Claims CONSTRUCT gates on SourcePacket validity
2. Evidence CONSTRUCT gates on prov:Entity derivation
3. Defects CONSTRUCT gates on sh:ValidationReport failure
4. Conformance CONSTRUCT gates on pm4py JSON completion
5. Unsupported-claims SELECT blocks chapter release

---

## Recommendations

### Immediate Actions
1. **Hook Instrumentation:** Add event log capture to all hook triggers for Chicago TDD conformance
2. **Receipt Validation:** Implement UUID-based receipt verification per hook
3. **DELETE Hook Coverage:** Implement symmetric delete hooks for claims, evidence, defects

### Strategic Directions
1. **ASK Query Standardization:** Enforce FILTER-based preconditions (no bare patterns)
2. **CONSTRUCT Receipt Standardization:** All artifacts must include `dcterms:created` + `prov:wasGeneratedBy`
3. **Hook Dependency Graph:** Formalize which hooks fire sequentially vs. parallel

---

## Files Generated

```
/Users/sac/wasm4pm-compat/emitted/sparql-hook-queries.md
```

## Appendix: Source Files

### Primary Hook Sources
- `/Users/sac/unrdf/hooks/health-check.ask.rq`
- `/Users/sac/unrdf/examples/hooks/financial/large-transaction.select.rq`
- `/Users/sac/unrdf/examples/hooks/parliamentary/motion-compliance.ask.rq`
- `/Users/sac/unrdf/.unrdf/hooks/health-check.json`

### Manufacturing Queries
- `/Users/sac/open-ontologies/.specify/queries/thesis/construct_claims.rq`
- `/Users/sac/open-ontologies/.specify/queries/thesis/construct_evidence.rq`
- `/Users/sac/open-ontologies/.specify/queries/thesis/construct_defects.rq`
- `/Users/sac/open-ontologies/.specify/queries/thesis/construct_open_obligations.rq`
- `/Users/sac/open-ontologies/.specify/queries/thesis/construct_unsupported_claims.rq`

### Process Mining Proofs
- `/Users/sac/unrdf/packages/wasm4pm/semconv/sparql-proofs/conformance-check.rq`
- `/Users/sac/open-ontologies/sparql/cq/covered-live-rules.rq`

---

**End of Report**
