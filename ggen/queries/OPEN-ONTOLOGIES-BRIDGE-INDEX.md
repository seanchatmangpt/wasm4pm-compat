# Open Ontologies Bridge Queries — Index & Reference

**Last Updated:** 2026-06-01  
**Purpose:** Master SPARQL bridge collection uniting wasm4pm-compat type-law ontology with open-ontologies standard definitions  
**Status:** Authoritative query set for production deployment

---

## Quick Start

### Master Bridge Query
```bash
# Primary join query (single-shot, local only)
ggen query open-ontologies-bridge.rq --format json

# With federation to live open-ontologies endpoint
ggen query open-ontologies-bridge-federation.rq \
  --local-endpoint http://localhost:3030/compat/query \
  --remote-endpoint http://sparql.open-ontologies.org/query
```

### Validation Audit
```bash
# Check alignment consistency, detect orphans, ambiguities, type law violations
ggen query validate-bridge-alignment.rq --format json
```

---

## Query Catalog

### 1. **open-ontologies-bridge.rq** (Master Bridge)
**Type:** CONSTRUCT + SELECT (universal)  
**Input Graphs:** wasm4pm-compat.ttl (primary), open-ontologies data (optional)  
**Output:** Aligned process forms with external definitions and constraint mappings

**What it does:**
- Lists all `compat:ProcessForm` instances (OCEL, BPMN, Petri nets, process trees, etc.)
- Maps each to external standard definitions via witness marker key
- Counts external constraints applying to each form
- Shows alignment basis (standard-namespace, structure-isomorphic, law-equivalent)

**Key columns:**
- `compatForm` — URI in compat: namespace
- `compatRustType` — Rust struct name (e.g., OcelLog, BpmnProcess)
- `compatSourceFile` — src/module.rs file
- `externalDef` — URI in open-ontologies
- `externalLabel` — Human-facing name from external
- `witnessKey` — Bridge identifier ('ocel-2.0', 'bpmn-2.0', etc.)
- `alignmentBasis` — Join rationale
- `externalConstraintCount` — How many rules apply

**Example output:**
```json
{
  "compatForm": "https://wasm4pm-compat.rs/ontology#OcelLogForm",
  "compatRustType": "OcelLog",
  "compatSourceFile": "src/ocel.rs",
  "externalDef": "https://open-ontologies.org/ocel#ObjectCentricEventLog",
  "witnessKey": "ocel-2.0",
  "alignmentBasis": "standard-namespace",
  "externalConstraintCount": 5
}
```

---

### 2. **extract-witness-to-external-mapping.rq** (Witness Registry)
**Type:** SELECT (registry)  
**Purpose:** Authoritative witness-to-external mapping ledger  
**Output:** 1 row per witness marker; used by all downstream queries

**What it does:**
- Lists all `compat:WitnessMarker` instances
- Maps each to its canonical open-ontologies namespace
- Shows SPARQL prefix abbreviation (ocel, bpmn, petri, etc.)
- Counts ProcessForm instances admitted against this witness
- Provides human-facing namespace labels

**Key columns:**
- `witnessKey` — Stable compat identifier ('ocel-2.0')
- `witnessTitle` — Human-facing title ('OCEL 2.0')
- `witnessFamily` — StandardDocument, Paper, ApiGrammar, RustLaw, InternalBridge
- `externalNamespace` — Open-ontologies URI
- `externalPrefix` — SPARQL prefix abbrev (ocel)
- `compatProcessFormCount` — How many compat forms use this witness
- `witnessDescription` — Definition from compat

**This is the canonical ledger.** All other bridge queries join through this.

**Example output:**
```json
{
  "witnessKey": "ocel-2.0",
  "witnessTitle": "OCEL 2.0",
  "witnessFamily": "StandardDocument",
  "externalNamespace": "https://open-ontologies.org/ocel#",
  "externalPrefix": "ocel",
  "compatProcessFormCount": 6,
  "witnessDescription": "OCEL 2.0 — the object-centric event log standard."
}
```

---

### 3. **extract-ocel-compat-join.rq** (OCEL Forms)
**Type:** SELECT (extraction)  
**Purpose:** OCEL 2.0 compatibility — deep form-by-form mapping  
**Output:** One row per OCEL compat form with external definition and constraints

**What it does:**
- Extracts all compat OCEL forms: OcelLog, OcelEvent, EventObjectLink, ObjectObjectLink, ObjectChange
- Joins with external OCEL standard component definitions
- Shows DX surfaces (builder API methods)
- Lists OCEL-specific constraints from external standard

**Key columns:**
- `compatForm`, `compatRustType`, `compatSourceFile`
- `ocelComponent` — External OCEL component URI
- `ocelComponentType` — OCEL component class (Event, Link, Change, etc.)
- `compatDxSurface` — Builder API style (fluent chain, constructor, etc.)
- `compatBuilderMethods` — Available builder chains
- `ocelConstraintName` — External constraint label

**Witness filter:** `'ocel-2.0'`

**Example output:**
```json
{
  "compatRustType": "OcelLog",
  "compatSourceFile": "src/ocel.rs",
  "ocelComponent": "https://open-ontologies.org/ocel#ObjectCentricEventLog",
  "ocelComponentType": "https://open-ontologies.org/ocel#ObjectCentricEventLog",
  "compatBuilderMethods": "new; with_events; with_object_changes",
  "ocelConstraintName": "event_id_unique"
}
```

---

### 4. **extract-bpmn-compat-join.rq** (BPMN Forms)
**Type:** SELECT (extraction)  
**Purpose:** BPMN 2.0 compatibility — element-wise mapping  
**Output:** One row per BPMN compat form with external definition and gateway specialization

**What it does:**
- Extracts all compat BPMN forms: BpmnProcess, BpmnTask, BpmnGateway, BpmnEvent, BpmnPool, BpmnLane, BpmnSubProcess, BpmnMessageFlow
- Maps gateway types (XOR, AND, OR) separately
- Joins with external BPMN element definitions
- Counts BPMN properties (isExecutable, isTriggeredByEvent, etc.)
- Lists BPMN structural constraints

**Key columns:**
- `compatForm`, `compatRustType`, `compatSourceFile`
- `bpmnElement` — External BPMN element URI
- `bpmnElementType` — BPMN class (Task, Gateway, Event, etc.)
- `gatewayType` — XOR, AND, OR (if applicable)
- `bpmnConstraints` — Constraint names (e.g., exclusive-no-merging)
- `propertyCount` — How many BPMN properties defined

**Witness filter:** `'bpmn-2.0'`

**Example output:**
```json
{
  "compatRustType": "BpmnGateway",
  "compatSourceFile": "src/bpmn.rs",
  "bpmnElementType": "https://open-ontologies.org/bpmn#Gateway",
  "gatewayType": "XOR",
  "bpmnConstraints": "exclusive-no-merging; immediate-fire",
  "propertyCount": 4
}
```

---

### 5. **extract-petri-compat-join.rq** (Petri Forms)
**Type:** SELECT (extraction)  
**Purpose:** Petri net compatibility — WF-net soundness + object-centric variants  
**Output:** One row per Petri net compat form with soundness constraints and OC variants

**What it does:**
- Extracts all compat Petri forms: PetriNet, Place, Transition, Arc, Marking, WfNet, ObjectCentricPetriNet
- Maps WF-net soundness constraints (proper termination, single source/sink)
- Shows object-centric Petri net variant presence
- Joins with external Petri/WF-net/OCPN standard definitions
- Counts arc properties for reachability analysis

**Key columns:**
- `compatForm`, `compatRustType`, `compatSourceFile`
- `petriComponent` — External Petri component URI
- `petriComponentType` — Petri class (Place, Transition, Arc, etc.)
- `soundnessConstraint` — WF-net law name (e.g., option-to-complete)
- `isSoundWfNet` — Boolean soundness claim
- `objectCentricVariant` — OCPN variant if present
- `arcPropertyCount` — Reachability property count

**Witness filters:** `'petri-net'`, `'wf-net'`, `'object-centric-petri-net'`

**Example output:**
```json
{
  "compatRustType": "WfNet",
  "compatSourceFile": "src/petri.rs",
  "soundnessConstraint": "option-to-complete",
  "isSoundWfNet": true,
  "objectCentricVariant": null,
  "arcPropertyCount": 3
}
```

---

### 6. **extract-process-tree-compat-join.rq** (Process Tree Forms)
**Type:** SELECT (extraction)  
**Purpose:** Process tree compatibility — operator support and projectability  
**Output:** One row per process tree form with supported operators and constraints

**What it does:**
- Extracts all compat process tree forms: ProcessTree, TreeNode, LoopNode, ActivityNode, GatewayNode
- Shows supported tree operators (sequence, choice, parallel, loop)
- Maps loop arity constraints (TypedLoopNode<ARITY>)
- Shows projectability status (can be projected to lower forms?)
- Joins with external process tree standard definitions

**Key columns:**
- `compatForm`, `compatRustType`, `compatSourceFile`
- `treeComponent` — External process tree component URI
- `treeComponentType` — Tree component class
- `supportedOperators` — CSV of operator names
- `loopArity` — Loop arity constraint value
- `isProjectable` — Boolean projectability claim
- `constraintCount` — How many external constraints apply

**Witness filter:** `'process-tree'`

**Example output:**
```json
{
  "compatRustType": "ProcessTree",
  "compatSourceFile": "src/process_tree.rs",
  "supportedOperators": "sequence, choice, parallel, loop",
  "loopArity": 2,
  "isProjectable": true,
  "constraintCount": 5
}
```

---

### 7. **extract-conformance-metrics-bridge.rq** (Metrics)
**Type:** SELECT (extraction)  
**Purpose:** Conformance metrics — join compat Metric<KIND, NUM, DEN> with external standards  
**Output:** One row per metric form with Between01 bounds and external formula

**What it does:**
- Extracts all compat metric forms: Fitness, Precision, Generalization, Simplicity, F-score
- Maps Between01<NUM, DEN> compile-time bounds to external numeric constraints
- Joins with external metrics standard definitions
- Shows external formula reference and validation rules
- Links to covering papers

**Key columns:**
- `metricForm`, `metricKind` — Fitness, Precision, etc.
- `compatMinBound`, `compatMaxBound` — Between01 range
- `compatNumRestriction`, `compatDenomRestriction` — Numerator/denominator types
- `externalMetricDef` — External metric URI
- `externalFormulaOrReference` — Math formula or standard reference
- `externalValidationRule` — External constraint URI
- `paperCitation` — Covering paper key

**Example output:**
```json
{
  "metricKind": "Fitness",
  "compatMinBound": 0,
  "compatMaxBound": 1,
  "compatNumRestriction": "AlignedTokens",
  "externalMetricDef": "https://open-ontologies.org/metrics#FitnessMetric",
  "externalFormulaOrReference": "aligned_moves / total_moves",
  "paperCitation": "van-der-aalst-process-mining-2016"
}
```

---

### 8. **validate-bridge-alignment.rq** (Audit)
**Type:** SELECT (validation)  
**Purpose:** Alignment consistency audit — detect orphans, ambiguities, type law violations  
**Output:** Multi-row audit report with issues and remediations

**What it does:**
- **AUDIT 1 — Orphaned Forms:** compat ProcessForm with no external definition
- **AUDIT 2 — Ambiguous Alignments:** compat ProcessForm mapped to multiple external URIs
- **AUDIT 3 — Type Law Violations:** external constraint not reflected in compat type
- **AUDIT 4 — Witness Key Mismatches:** declared witness key ≠ inferred witness key

**Audit columns (all reports):**
- `auditKind` — ORPHANED_FORM, AMBIGUOUS_ALIGNMENT, TYPE_LAW_VIOLATION, WITNESS_KEY_MISMATCH
- `compatForm`, `compatRustType`, `compatSourceFile`
- `issue` — Human-readable problem description
- `remediation` — Recommended fix

**Example output:**
```json
{
  "auditKind": "ORPHANED_FORM",
  "compatRustType": "CustomProcessForm",
  "compatSourceFile": "src/custom.rs",
  "issue": "No external definition found for this form",
  "remediation": "Create external alignment record in open-ontologies or update witness key."
}
```

---

### 9. **open-ontologies-bridge-federation.rq** (Federation)
**Type:** SELECT with SERVICE (federation)  
**Purpose:** Live federation query against remote open-ontologies SPARQL endpoint  
**Output:** Same as master bridge, but with live external data

**What it does:**
- Executes master bridge logic locally against compat.ttl
- For each witness key, federates via SERVICE to open-ontologies endpoint
- Supports exact matching (standard-namespace) and fuzzy label matching
- Computes alignment distance (0 = exact, 1 = fuzzy)
- Filters by `isAligned` boolean

**Key columns (as in master bridge, plus):**
- `alignmentDistance` — 0 (exact match), 1 (fuzzy label match)
- `isAligned` — True for exact matches

**Deployment:**
```bash
# Update SERVICE URIs in query to point to actual endpoints:
# SERVICE <http://YOUR_LOCAL_COMPAT_ENDPOINT/query> { ... }
# SERVICE <http://YOUR_REMOTE_ONTOLOGIES_ENDPOINT/query> { ... }

ggen query open-ontologies-bridge-federation.rq \
  --local-endpoint http://localhost:3030/compat/query \
  --remote-endpoint http://sparql.open-ontologies.org/query
```

---

## Namespace Mappings

Authoritative witness-to-external mapping (computed by extract-witness-to-external-mapping.rq):

| Witness Key | Title | External Namespace | Prefix |
|---|---|---|---|
| `ocel-2.0` | OCEL 2.0 | https://open-ontologies.org/ocel# | ocel |
| `bpmn-2.0` | BPMN 2.0 | https://open-ontologies.org/bpmn# | bpmn |
| `petri-net` | Petri Net | https://open-ontologies.org/petri# | petri |
| `wf-net` | WF-net | https://open-ontologies.org/wfnet# | wfnet |
| `object-centric-petri-net` | OC Petri Net | https://open-ontologies.org/ocpn# | ocpn |
| `process-tree` | Process Tree | https://open-ontologies.org/processtree# | ptree |
| `xes-1849` | XES 1.8.4.9 | https://open-ontologies.org/xes# | xes |
| `dfg` | Directly-Follows Graph | https://open-ontologies.org/dfg# | dfg |
| `pm4py-api-grammar` | pm4py API | https://open-ontologies.org/pm4py# | pm4py |

---

## Integration Guide

### Step 1: Load wasm4pm-compat.ttl into Triple Store

```bash
# Using Apache Jena Fuseki
curl -X POST http://localhost:3030/compat/data \
  --data-binary @ggen/ontology/wasm4pm-compat.ttl \
  -H 'Content-Type: text/turtle'

# Or using Virtuoso
isql localhost 1111 dba password
SPARQL CLEAR GRAPH <https://wasm4pm-compat.rs/ontology>;
LOAD <file:///path/to/wasm4pm-compat.ttl> INTO <https://wasm4pm-compat.rs/ontology>;
```

### Step 2: Execute Witness Registry Query

```bash
# Extract witness-to-external mapping once, store result
ggen query extract-witness-to-external-mapping.rq --format json > _witness-registry.json

# Use _witness-registry.json in bridge query parameterization
```

### Step 3: Run Bridge Queries

```bash
# Master bridge (local only)
ggen query open-ontologies-bridge.rq --format json > _bridge-results.json

# Or with federation (requires remote endpoint)
ggen query open-ontologies-bridge-federation.rq \
  --local-endpoint http://localhost:3030/compat/query \
  --remote-endpoint http://sparql.open-ontologies.org/query \
  --format json > _bridge-results-federated.json
```

### Step 4: Run Validation Audit

```bash
ggen query validate-bridge-alignment.rq --format json > _audit-report.json

# Review audit report for issues
jq '.[] | select(.auditKind == "ORPHANED_FORM")' < _audit-report.json
```

### Step 5: Run Form-Specific Extractions

```bash
# OCEL deep dive
ggen query extract-ocel-compat-join.rq --format json > _ocel-forms.json

# BPMN deep dive
ggen query extract-bpmn-compat-join.rq --format json > _bpmn-forms.json

# Petri nets deep dive
ggen query extract-petri-compat-join.rq --format json > _petri-forms.json

# Process trees deep dive
ggen query extract-process-tree-compat-join.rq --format json > _process-trees.json

# Metrics deep dive
ggen query extract-conformance-metrics-bridge.rq --format json > _metrics.json
```

---

## Testing & Validation

### Unit Test: Namespace Mapping Completeness
```sparql
# Assert: every compat ProcessForm has a witness key
SELECT ?form WHERE {
  ?form a compat:ProcessForm .
  FILTER NOT EXISTS { ?form compat:admittedAgainst ?w . }
}
# Expected: empty result set
```

### Unit Test: No Orphaned Forms
```sparql
# Run validate-bridge-alignment.rq
# Assert: no rows with auditKind = "ORPHANED_FORM"
```

### Unit Test: Witness Key Consistency
```sparql
# Run validate-bridge-alignment.rq
# Assert: no rows with auditKind = "WITNESS_KEY_MISMATCH"
```

### Integration Test: Round-trip Alignment
```bash
# Export process forms to external format
ggen query extract-ocel-compat-join.rq | jq '.[] | .compatRustType' | sort -u

# Verify each compat form appears in open-ontologies
# (requires live endpoint)
```

---

## Performance Tuning

### For Large Graphs (>100M triples)

**Index external URIs:**
```sparql
# Pre-compute external namespace membership
INSERT INTO <https://wasm4pm-compat.rs/index> {
  ?externalDef
    compat:inNamespace ?namespace ;
    compat:externalPrefix ?prefix .
}
WHERE {
  GRAPH <https://open-ontologies.org/data> {
    ?externalDef a ?type .
    BIND (STRBEFORE(STR(?type), '#') AS ?namespace)
    BIND (SUBSTR(?namespace, 32) AS ?prefix)  # Extract from URI
  }
}
```

**Cache witness registry:**
```sparql
# Run extract-witness-to-external-mapping.rq once
# Store result as named graph <https://wasm4pm-compat.rs/witness-index>
# Modify bridge queries to use cached index instead of computed BIND
```

### Query Optimization Tips

1. **Filter early:** Add witness key FILTER in WHERE clause, not later
2. **Use UNION sparingly:** Federation queries should use OPTIONAL with SERVICE
3. **Bind before FILTER:** Compute ?alignmentBasis with BIND, then FILTER on it
4. **Group efficiently:** GROUP BY on minimal columns, use SAMPLE for optional facts

---

## Deployment Checklist

- [ ] Triple store loaded with wasm4pm-compat.ttl
- [ ] SPARQL endpoint reachable (default: http://localhost:3030/compat/query)
- [ ] extract-witness-to-external-mapping.rq executed and cached
- [ ] open-ontologies remote endpoint configured (if using federation)
- [ ] validate-bridge-alignment.rq run, audit report reviewed
- [ ] No ORPHANED_FORM, WITNESS_KEY_MISMATCH, TYPE_LAW_VIOLATION issues
- [ ] All form-specific extractions (OCEL, BPMN, Petri, trees, metrics) validated
- [ ] Query response times <5s for local, <10s for federation
- [ ] Alerting configured: re-run audit monthly

---

## See Also

- `ggen/ontology/wasm4pm-compat.ttl` — Source ontology
- `ggen/ontology/papers.ttl` — Paper coverage index
- `ggen/VALIDATION-QUERY-REFERENCE.md` — Other validation queries
- `CLAUDE.md` — Type law architecture
- Open Ontologies standards: https://open-ontologies.org/

---

**Questions?** Contact the wasm4pm-compat team.
