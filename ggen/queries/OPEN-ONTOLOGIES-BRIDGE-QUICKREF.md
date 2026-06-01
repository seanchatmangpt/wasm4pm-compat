# Open Ontologies Bridge Queries — Quick Reference

**Deployed:** 2026-06-01  
**Files:** 9 SPARQL queries + 2 markdown guides  
**Purpose:** Join wasm4pm-compat type-law surface with open-ontologies standard definitions

---

## One-Command Cheat Sheet

```bash
# Master bridge (local only)
ggen query open-ontologies-bridge.rq --format json

# Witness registry (canonical mapping)
ggen query extract-witness-to-external-mapping.rq --format json

# Validation audit (find issues)
ggen query validate-bridge-alignment.rq --format json

# OCEL-specific deep dive
ggen query extract-ocel-compat-join.rq --format json

# BPMN-specific deep dive
ggen query extract-bpmn-compat-join.rq --format json

# Petri/WF-net deep dive
ggen query extract-petri-compat-join.rq --format json

# Process tree deep dive
ggen query extract-process-tree-compat-join.rq --format json

# Conformance metrics deep dive
ggen query extract-conformance-metrics-bridge.rq --format json

# Live federation (requires remote endpoint)
ggen query open-ontologies-bridge-federation.rq \
  --local-endpoint http://localhost:3030/compat/query \
  --remote-endpoint http://sparql.open-ontologies.org/query
```

---

## File Manifest

| File | Type | Purpose | Output |
|---|---|---|---|
| `open-ontologies-bridge.rq` | SPARQL SELECT | Master join query | All aligned forms + external defs |
| `extract-witness-to-external-mapping.rq` | SPARQL SELECT | Witness registry (canonical) | 1 row per witness; used by all others |
| `extract-ocel-compat-join.rq` | SPARQL SELECT | OCEL 2.0 specific | OcelLog, OcelEvent, links, changes |
| `extract-bpmn-compat-join.rq` | SPARQL SELECT | BPMN 2.0 specific | BpmnProcess, Tasks, Gateways, etc. |
| `extract-petri-compat-join.rq` | SPARQL SELECT | Petri net specific | PetriNet, WfNet, OCPN, soundness |
| `extract-process-tree-compat-join.rq` | SPARQL SELECT | Process tree specific | ProcessTree, TreeNode, operators |
| `extract-conformance-metrics-bridge.rq` | SPARQL SELECT | Metrics specific | Fitness, Precision, F-score, etc. |
| `validate-bridge-alignment.rq` | SPARQL SELECT | Alignment audit | 4 audit kinds (orphans, ambiguity, etc.) |
| `open-ontologies-bridge-federation.rq` | SPARQL SELECT+SERVICE | Federation bridge | Same as master, with live remote data |
| `OPEN-ONTOLOGIES-BRIDGE-INDEX.md` | Markdown | Full reference | Query catalog + integration guide |
| `OPEN-ONTOLOGIES-BRIDGE-QUICKREF.md` | Markdown | This file | Command cheat sheet |

---

## Key Concepts

### Witness Marker
A compile-time tag in wasm4pm-compat that names a standard or paper. Examples:
- `Ocel20` → witness key `'ocel-2.0'`
- `Bpmn20` → witness key `'bpmn-2.0'`
- `PetriNet` → witness key `'petri-net'`

### ProcessForm
A primary process-evidence shape in compat (structure-only):
- OcelLog, BpmnProcess, PetriNet, ProcessTree, etc.
- Each is admitted against a specific witness marker
- Never executes discovery, conformance, replay (those graduate to wasm4pm)

### External Definition
A counterpart in open-ontologies:
- ocel:ObjectCentricEventLog, bpmn:Process, petri:PetriNet, etc.
- Carries constraints, rules, formula references
- Linked via witness key normalization

### Alignment Basis
Why compat and external join:
- `'standard-namespace'` — same standard (ocel-2.0 to ocel:Component)
- `'structure-isomorphic'` — shapes match structurally
- `'law-equivalent'` — type laws are equivalent
- `'fuzzy-match-label'` — text match on label/comment (federation only)

---

## Witness Key → External Namespace Mapping

| Witness Key | Title | External URI |
|---|---|---|
| `ocel-2.0` | OCEL 2.0 | https://open-ontologies.org/ocel# |
| `bpmn-2.0` | BPMN 2.0 | https://open-ontologies.org/bpmn# |
| `petri-net` | Petri Net | https://open-ontologies.org/petri# |
| `wf-net` | WF-net | https://open-ontologies.org/wfnet# |
| `object-centric-petri-net` | OC Petri Net | https://open-ontologies.org/ocpn# |
| `process-tree` | Process Tree | https://open-ontologies.org/processtree# |
| `xes-1849` | XES 1.8.4.9 | https://open-ontologies.org/xes# |
| `dfg` | DFG | https://open-ontologies.org/dfg# |
| `pm4py-api-grammar` | pm4py API | https://open-ontologies.org/pm4py# |

---

## Query Categories

### 1. **Registry** (Start here)
- `extract-witness-to-external-mapping.rq` — canonical witness ledger
- **Run once, cache result** (used by all other queries)

### 2. **Master Bridge** (Unified view)
- `open-ontologies-bridge.rq` — all forms with external defs
- `open-ontologies-bridge-federation.rq` — master + live remote data

### 3. **Form-Specific Deep Dives** (Pick your standard)
- `extract-ocel-compat-join.rq` — OCEL 2.0 forms only
- `extract-bpmn-compat-join.rq` — BPMN 2.0 forms only
- `extract-petri-compat-join.rq` — Petri/WF-net forms only
- `extract-process-tree-compat-join.rq` — Process tree forms only
- `extract-conformance-metrics-bridge.rq` — Metrics forms only

### 4. **Audit & Validation** (Find issues)
- `validate-bridge-alignment.rq` — 4 audit types:
  - `ORPHANED_FORM` — compat form, no external match
  - `AMBIGUOUS_ALIGNMENT` — compat form, multiple external matches
  - `TYPE_LAW_VIOLATION` — external constraint, not in compat
  - `WITNESS_KEY_MISMATCH` — declared ≠ inferred witness

---

## Quick Deployment

```bash
# 1. Load compat.ttl into triple store
curl -X POST http://localhost:3030/compat/data \
  --data-binary @ggen/ontology/wasm4pm-compat.ttl \
  -H 'Content-Type: text/turtle'

# 2. Extract and cache witness registry
ggen query extract-witness-to-external-mapping.rq --format json > /tmp/_witness-registry.json

# 3. Run master bridge
ggen query open-ontologies-bridge.rq --format json > /tmp/_bridge.json

# 4. Run audit
ggen query validate-bridge-alignment.rq --format json > /tmp/_audit.json

# 5. Review audit report for issues
jq '.[] | select(.auditKind != "OK")' < /tmp/_audit.json
```

---

## Output Columns by Query

### Master Bridge
- `compatForm`, `compatRustType`, `compatSourceFile`
- `externalDef`, `externalLabel`, `witnessKey`, `alignmentBasis`
- `externalConstraintCount`

### Witness Registry
- `witnessKey`, `witnessTitle`, `witnessFamily`, `witnessYear`
- `externalNamespace`, `externalPrefix`, `externalNamespaceLabel`
- `compatProcessFormCount`, `witnessDescription`

### OCEL Join
- `compatForm`, `compatRustType`, `compatSourceFile`
- `ocelComponent`, `ocelComponentType`, `compatDxSurface`, `compatBuilderMethods`
- `ocelConstraintName`

### BPMN Join
- `compatForm`, `compatRustType`, `compatSourceFile`
- `bpmnElement`, `bpmnElementType`, `bpmnElementLabel`, `gatewayType`
- `bpmnConstraints`, `propertyCount`

### Petri Join
- `compatForm`, `compatRustType`, `compatSourceFile`
- `petriComponent`, `petriComponentType`, `petriComponentLabel`
- `soundnessConstraint`, `isSoundWfNet`, `objectCentricVariant`, `arcPropertyCount`

### Process Tree Join
- `compatForm`, `compatRustType`, `compatSourceFile`
- `treeComponent`, `treeComponentType`, `treeComponentLabel`
- `supportedOperators`, `loopArity`, `isProjectable`, `constraintCount`

### Metrics Join
- `metricForm`, `metricKind`, `compatMinBound`, `compatMaxBound`
- `compatNumRestriction`, `compatDenomRestriction`
- `externalMetricDef`, `externalMetricLabel`, `externalFormulaOrReference`
- `externalValidationRule`, `paperCitation`

### Audit
- `auditKind`, `compatForm`, `compatRustType`, `compatSourceFile`
- `issue`, `remediation`

---

## Tips & Tricks

### Filter by witness key
```bash
ggen query open-ontologies-bridge.rq --format json | jq '.[] | select(.witnessKey == "ocel-2.0")'
```

### Count forms per witness
```bash
ggen query extract-witness-to-external-mapping.rq --format json | jq '.[] | "\(.witnessKey): \(.compatProcessFormCount)"'
```

### Find orphaned forms
```bash
ggen query validate-bridge-alignment.rq --format json | jq '.[] | select(.auditKind == "ORPHANED_FORM")'
```

### Export metrics to CSV
```bash
ggen query extract-conformance-metrics-bridge.rq --format json | jq -r '.[] | [.metricKind, .compatMinBound, .compatMaxBound] | @csv'
```

### Validate no audit issues
```bash
ISSUE_COUNT=$(ggen query validate-bridge-alignment.rq --format json | jq 'length')
if [ "$ISSUE_COUNT" -gt 0 ]; then echo "FAIL: $ISSUE_COUNT audit issues"; exit 1; fi
```

---

## Troubleshooting

### "No results"
- **Cause:** Triple store not loaded or SPARQL endpoint unreachable
- **Fix:** Load wasm4pm-compat.ttl, verify endpoint URI

### "SERVICE request timeout"
- **Cause:** Open-ontologies endpoint unreachable or slow
- **Fix:** Check remote endpoint URL, try local-only queries first

### "Orphaned forms" in audit
- **Cause:** ProcessForm admitted against witness with no external namespace mapping
- **Fix:** Add witness key to mapping in extract-witness-to-external-mapping.rq BIND clause

### "Ambiguous alignment" in audit
- **Cause:** Multiple external definitions with same name
- **Fix:** Add compat:canonicalExternalDef property to ProcessForm

---

## Integration Checklist

- [ ] Triple store (Fuseki/Virtuoso/Stardog) running
- [ ] wasm4pm-compat.ttl loaded into triple store
- [ ] SPARQL endpoint reachable (test: curl http://localhost:3030/compat/query)
- [ ] extract-witness-to-external-mapping.rq runs without error
- [ ] open-ontologies-bridge.rq returns rows (no empty result)
- [ ] validate-bridge-alignment.rq has 0 ORPHANED_FORM rows
- [ ] All form-specific queries return expected columns
- [ ] Federation queries enabled (if using open-ontologies endpoint)
- [ ] Response times acceptable (<5s local, <10s federation)
- [ ] Audit report reviewed monthly

---

## See Also

- `OPEN-ONTOLOGIES-BRIDGE-INDEX.md` — Full reference guide
- `ggen/ontology/wasm4pm-compat.ttl` — Source RDF
- `CLAUDE.md` — Type law architecture
- https://open-ontologies.org/ — External standards

---

**Last checked:** 2026-06-01
