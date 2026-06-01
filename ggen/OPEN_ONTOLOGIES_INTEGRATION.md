# wasm4pm-compat & open-ontologies Integration

**Status:** Integration architecture documentation  
**Date:** 2026-06-01  
**Authority:** wasm4pm-compat CROWN_ALIVE_004 (type law) + open-ontologies OntoStar (admission gates)

---

## 1. Namespace Alignment

### wasm4pm-compat Ontology Stack

wasm4pm-compat defines four namespace families:

| Namespace | Base URI | Purpose | Authority |
|---|---|---|---|
| **compat** | `https://wasm4pm-compat.rs/ontology#` | Core type-law surfaces: WitnessMarker, EvidenceState, ProcessForm, TypeConstraint, CompileFailLaw, SourceModule | src/witness.rs, src/state.rs, src/evidence.rs |
| **paper** | `https://wasm4pm-compat.rs/paper#` | Paper inventory, citations, coverage records | ggen/ontology/papers.ttl |
| **substrate** | `https://wasm4pm-compat.rs/substrate#` | ggen manufacturing pipeline metadata: GenerationRule, QueryFile, TemplateFile, OutputArtifact | ggen/ontology/ggen-substrate.ttl |
| **audit** | `https://wasm4pm-compat.rs/audit#` | Proof gates, audit machinery, receipt records | ggen/ontology/audit-machinery.ttl |

### open-ontologies (OntoStar) Namespace Stack

open-ontologies exports three core public vocabularies:

| Namespace | Base URI | Purpose | Module |
|---|---|---|---|
| **ghf** (GHF Core) | `https://open-ontologies.org/profile/github-factory#` | GitHub factory metadata: repositories, labels, environments, branch protection | ontology/ghf-core.ttl |
| **public-alignment** | (schema.org + prov + dcat + odrl + dcterms + skos) | Pure W3C standard vocabularies for actions, executions, agents, receipts, evidence | ontology/public-alignment.ttl |
| **cell8** | `https://open-ontologies.org/cell8#` | 13-gate attestation machinery (A1–A13), EARL emission, BLAKE3 chain, Ed25519 sealing | ontology/cell8-shapes.ttl |

### Alignment Layer

**Cross-namespace mapping is bidirectional and non-exclusive:**

```turtle
# In ggen/ontology/wasm4pm-compat.ttl
compat:WitnessMarker
    rdfs:seeAlso ghf:ContributionUnit ;
    owl:equivalentClass prov:Entity .

# In open-ontologies/ontology/public-alignment.ttl (incoming link)
prov:Entity
    skos:closeMatch compat:WitnessMarker ;
    rdfs:comment "Captures evidence units, receipts, and witness markers from downstream process-mining type law." .

# Cell8 receipt chains reference wasm4pm-compat ALIVE gates
cell8:Attestation
    cell8:attests compat:CompileFailLaw, compat:CompilePassSurface ;
    cell8:producedBy [ rdf:value "cargo test --test ui_tests" ] .
```

**Key principle:** open-ontologies adopts **no wasm4pm-compat internals**. It references wasm4pm-compat only through its public class URIs (compat:WitnessMarker, compat:EvidenceState) and papers (paper:Aalst2021, paper:VanDerAalst2022).

---

## 2. Example Queries

### Query 1: Find all WitnessMarkers admitted in wasm4pm-compat by papers in open-ontologies

**File:** `ggen/queries/witness_marker_paper_coverage.rq`

```sparql
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>
PREFIX paper: <https://wasm4pm-compat.rs/paper#>
PREFIX prov: <http://www.w3.org/ns/prov#>
PREFIX dct: <http://purl.org/dc/terms/>

# Query: Which WitnessMarkers derive from papers cataloged in open-ontologies?
SELECT ?witness ?witnessTitle ?paperKey ?paperTitle ?paperYear
WHERE {
  ?witness
    a compat:WitnessMarker ;
    compat:witnessTitle ?witnessTitle ;
    compat:citePaper ?paperKey .
  
  ?paperKey
    a paper:Paper ;
    rdfs:label ?paperTitle ;
    dct:issued ?paperYear .
  
  # Optional: filter for papers that are also in open-ontologies thesis
  FILTER (?paperYear > 2015)
}
ORDER BY ?paperYear ?witnessTitle
```

**Use case:** Verify that every WitnessMarker in wasm4pm-compat cites a published paper. Used in ALIVE gate `paperlaw_witness_receipt_gate`.

---

### Query 2: Discover process forms that graduate to wasm4pm via Cell8 attestation

**File:** `ggen/queries/graduation_forms_with_cell8_sealing.rq`

```sparql
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>
PREFIX cell8: <https://open-ontologies.org/cell8#>
PREFIX prov: <http://www.w3.org/ns/prov#>

# Query: Which ProcessForms graduate to wasm4pm and are sealed by Cell8 attestation?
SELECT ?form ?formLabel ?gradReason ?cell8Gate ?sealStatus
WHERE {
  # Process form in wasm4pm-compat
  ?form
    a compat:ProcessForm ;
    rdfs:label ?formLabel .
  
  # Form graduates to wasm4pm
  ?form
    compat:graduatesTo ?boundary .
  ?boundary
    a compat:GraduationBoundary ;
    compat:reason ?gradReason .
  
  # Attestation chain from Cell8
  ?attestation
    cell8:attests ?form ;
    cell8:gate ?cell8Gate ;
    cell8:sealStatus ?sealStatus .
  
  # Restrict to sealed (not pending) attestations
  FILTER (?sealStatus = "sealed"^^xsd:string)
}
ORDER BY ?formLabel ?gradReason
```

**Use case:** For wasm4pm integration, ensure that every graduating form carries a Cell8-sealed receipt before admission to the execution engine.

---

### Query 3: Audit trail: CompileFailLaw receipts -> Paper -> Cell8 Seal Chain

**File:** `ggen/queries/compile_fail_audit_chain.rq`

```sparql
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>
PREFIX paper: <https://wasm4pm-compat.rs/paper#>
PREFIX audit: <https://wasm4pm-compat.rs/audit#>
PREFIX cell8: <https://open-ontologies.org/cell8#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX prov: <http://www.w3.org/ns/prov#>

# Query: Full audit chain from a compile-fail law through paper authority to Cell8 seal
SELECT ?law ?lawName ?rustcError ?fixturePath ?paperKey ?paperTitle ?cellGate ?blake3Hash
WHERE {
  # Compile-fail law with receipt
  ?law
    a compat:CompileFailLaw ;
    rdfs:label ?lawName ;
    compat:rustcError ?rustcError ;
    compat:fixturePath ?fixturePath ;
    compat:citePaper ?paperKey .
  
  # Paper authority
  ?paperKey
    a paper:Paper ;
    rdfs:label ?paperTitle .
  
  # Audit receipt linking law to Cell8
  ?auditRecord
    audit:validates ?law ;
    audit:producedBy [ rdf:value "cargo test --test ui_tests -- --ignored" ] ;
    prov:wasDerivedFrom [ rdfs:comment "compile-fail .stderr receipt" ] .
  
  # Cell8 attestation sealing the audit record
  ?attestation
    cell8:attests ?auditRecord ;
    cell8:gate ?cellGate ;
    cell8:blake3Hash ?blake3Hash .
}
LIMIT 10
```

**Use case:** For adversarial verification—prove that a compile-fail law is real, backed by a trybuild fixture, cited to a paper, and sealed by Cell8's cryptographic chain.

---

### Query 4: Find gaps that open-ontologies should address in wasm4pm-compat

**File:** `ggen/queries/integration_gap_register.rq`

```sparql
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX dct: <http://purl.org/dc/terms/>
PREFIX skos: <http://www.w3.org/2004/02/skos/core#>

# Query: Which GraduationBoundaries lack Cell8 sealing or LLM manufacturing receipts?
SELECT ?boundary ?boundaryName ?missingAttestation ?requiredCell8Gate
WHERE {
  ?boundary
    a compat:GraduationBoundary ;
    rdfs:label ?boundaryName .
  
  OPTIONAL {
    ?boundary
      compat:gappedSince ?gapDate ;
      skos:editorialNote ?missingAttestation .
  }
  
  # Cell8 sealing is required for graduation
  OPTIONAL {
    ?attestation
      cell8:attests ?boundary ;
      cell8:gate ?requiredCell8Gate .
  }
  
  # Filter: only boundaries without Cell8 sealing
  FILTER ( !BOUND(?attestation) )
}
```

**Use case:** Identify integration gaps where open-ontologies must extend Cell8 or manufacturing machinery to cover wasm4pm-compat boundaries.

---

## 3. Authority Hierarchy

### The Three-Tier Authority Stack

```
┌─────────────────────────────────────────────────────────┐
│ TIER 1: wasm4pm-compat (Type Law Authority)             │
├─────────────────────────────────────────────────────────┤
│ • Owns: compile-time process-mining type surfaces       │
│ • Location: src/*.rs + ggen/ontology/wasm4pm-compat.ttl │
│ • Gate: CROWN_ALIVE_004 (196 compile-fail + 406 pass)   │
│ • Lifecycle: Raw → Parsed → Admitted → {Projected|...}  │
│ • Issues receipts for: witness markers, evidence states, │
│   type constraints, graduation boundaries               │
│ • Publishes: 98 papers, 37 source modules, 23 audits    │
└─────────────────────────────────────────────────────────┘
                         △
                         │ cites
                         │
┌─────────────────────────────────────────────────────────┐
│ TIER 2: open-ontologies (Manufacturing Authority)       │
├─────────────────────────────────────────────────────────┤
│ • Owns: admission gates, cryptographic receipts         │
│ • Location: /open-ontologies (OntoStar)                 │
│ • Gate: Cell8 13-gate attestation (A1–A13)              │
│ • Machinery:                                            │
│   - Groq LLM + DSPy + pm4py POWL candidates            │
│   - 9-breed cognition swarm (Rust + AtomVM + Hearsay-II)│
│   - Manufacturing (IaC, Rust, Erlang, AtomVM)           │
│   - BLAKE3 receipt chain + opt-in Ed25519 seal          │
│   - 50+ MCP tools (onto_*) over stdio/HTTP              │
│ • Admits: requirements, work orders, artifacts          │
│ • Issues receipts for: Cell8 attestation chains         │
└─────────────────────────────────────────────────────────┘
                         △
                         │ seals
                         │
┌─────────────────────────────────────────────────────────┐
│ TIER 3: wasm4pm (Execution Authority)                   │
├─────────────────────────────────────────────────────────┤
│ • Owns: conformance, discovery, replay, alignment       │
│ • Location: wasm4pm crate (not yet imported by compat)  │
│ • Machinery: mining algorithms, token simulation        │
│ • Constraint: ONLY imports from compat.graduation       │
│ • Issues receipts for: mining/conformance/replay results│
│ • Open gap: GAP_001 (compat-to-wasm bridge)             │
└─────────────────────────────────────────────────────────┘
```

### Command Chain (Upstream Authority)

**Who issues commands to whom:**

1. **wasm4pm-compat issues to open-ontologies:**
   - "Admit this CompileFailLaw as a type-law surface"
   - "Seal this GraduationBoundary with Cell8 attestation A9"
   - "Manufacture an example_graduation_candidate proof-of-concept"

2. **open-ontologies issues to wasm4pm:**
   - "Migrate Admitted evidence to wasm4pm.Mining with receipt"
   - "Run conformance replay on this Evidence<OcelLog, Admitted>"
   - "Link execution results back to wasm4pm-compat witness marker"

3. **Refusal chain (no reverse flow):**
   - If open-ontologies denies Cell8 attestation: wasm4pm-compat MUST NOT publish
   - If wasm4pm fails conformance: open-ontologies does NOT retry manufacturing
   - If wasm4pm-compat witness is undefined: neither open-ontologies nor wasm4pm admits the evidence

### Proof of Authority

**Authority is proved by receipts, not credentials.**

| Tier | Proof Document | Location | Refresh |
|---|---|---|---|
| 1 (compat) | Paper (108 pages, LaTeX, peer-ready) | wasm4pm-compat/paper/main.pdf | At each ALIVE gate closure |
| 2 (open-ontologies) | Cell8 13-gate attestation record | open-ontologies/.ggen/receipts/latest.json | After each manufacturing run |
| 3 (wasm4pm) | Conformance replay receipt | wasm4pm/receipts/conformance-YYYYMMDD.json | After each mining/replay |

**Recursive proof:** The paper in Tier 1 cites the papers that authorize Tier 2 and Tier 3. Tier 2 seals Tier 1 receipts with cryptography. Tier 3 validates Tier 2 manufacturing output.

---

## 4. Integration Points

### Point A: Type-Law Admission (wasm4pm-compat → open-ontologies)

**When:** Every time a compile-fail fixture is added or a law changes.

**What flows:**
- compat:WitnessMarker instances + paper citations
- compat:CompileFailLaw + .stderr receipts
- compat:TypeConstraint definitions

**How:**
```bash
# In wasm4pm-compat
cargo test --test ui_tests -- --ignored                 # Emit type-law receipts
cargo run --release --example ggen-export-compat.ttl    # Export wasm4pm-compat.ttl

# In open-ontologies
onto admit --type type-law --source wasm4pm-compat.ttl   # Submit to Cell8 gates
```

### Point B: Manufacturing (open-ontologies → wasm4pm)

**When:** A GraduationBoundary passes Cell8 A9 (form readiness).

**What flows:**
- compat:ProcessForm instances
- compat:GraduationBoundary + rationale
- Evidence carriers with compat witness markers

**How:**
```bash
# In open-ontologies
onto manufacture --candidate wasm4pm-compat.rs/CROWN_004 --gate A9

# In wasm4pm
cargo add --git https://github.com/fabio-rovai/wasm4pm-compat wasm4pm-compat
wasm4pm import-evidence --from open-ontologies --receipt $RECEIPT_ID
```

### Point C: Conformance Feedback (wasm4pm → wasm4pm-compat)

**When:** A mined process or replayed trace shows unexpected behavior.

**What flows:**
- Conformance violations
- Missing or inconsistent witness markers
- Suggested type-law corrections

**How:**
```bash
# In wasm4pm
wasm4pm conformance-check --log ocel.json --model petri.net --receipt output.json

# In wasm4pm-compat (if violations found)
cargo test --all-features -- --include-ignored
# Then fix src/*.rs laws and re-submit
```

---

## 5. Repository Links

### wasm4pm-compat (This Repository)

- **GitHub:** https://github.com/fabio-rovai/wasm4pm-compat
- **Main branch:** [main](https://github.com/fabio-rovai/wasm4pm-compat)
- **Ontologies:** `/ggen/ontology/`
- **Papers registry:** `/ggen/ontology/papers.ttl`
- **Type-law surfaces:** `/src/`

### open-ontologies (OntoStar)

- **GitHub:** https://github.com/fabio-rovai/open-ontologies
- **Main branch:** [ontostar-integration](https://github.com/fabio-rovai/open-ontologies/tree/ontostar-integration)
- **Cell8 sealing:** `/src/cell8.rs`
- **Manufacturing:** `/src/manufacturing/`
- **Ontologies:** `/ontology/`
- **Documentation:** `/docs/00-overview.md`

### wasm4pm (Future Integration)

- **GitHub:** https://github.com/fabio-rovai/wasm4pm (planned)
- **Graduation bridges:** `/src/graduation/` (to be added)
- **Conformance engines:** `/src/conformance/`, `/src/discovery/`, `/src/replay/`

---

## 6. Integration Checklist

### Phase 1: Namespace Registration (Done)

- [x] compat namespace published in `ggen/ontology/wasm4pm-compat.ttl`
- [x] paper namespace linked to papers in `/ggen/ontology/papers.ttl`
- [x] substrate namespace metadata in `/ggen/ontology/ggen-substrate.ttl`
- [x] audit namespace machinery in `/ggen/ontology/audit-machinery.ttl`

### Phase 2: Query Suite (In Progress)

- [ ] `ggen/queries/witness_marker_paper_coverage.rq` ✓ (drafted)
- [ ] `ggen/queries/graduation_forms_with_cell8_sealing.rq` ✓ (drafted)
- [ ] `ggen/queries/compile_fail_audit_chain.rq` ✓ (drafted)
- [ ] `ggen/queries/integration_gap_register.rq` ✓ (drafted)
- [ ] Register queries in `ggen/ggen.toml`
- [ ] Add SPARQL test fixtures in `ggen/queries/tests/`

### Phase 3: Template Suite

- [ ] `ggen/templates/cell8_attestation_request.tera`
- [ ] `ggen/templates/graduation_candidate_brief.tera`
- [ ] `ggen/templates/witness_audit_report.tera`

### Phase 4: Manifest Registration

- [ ] Add `[graduation]` table to `ggen/ggen.toml`
- [ ] Link Cell8 gates: A9 (form readiness) + A13 (release seal)
- [ ] Define MCP tool surface: `onto admit` + `onto verify`

### Phase 5: Documentation

- [x] This file: `OPEN_ONTOLOGIES_INTEGRATION.md`
- [ ] Reference in `/README.md`
- [ ] Add example in `/examples/graduation_with_open_ontologies.rs`

### Phase 6: GAP_001 Closure (wasm4pm Bridge)

- [ ] wasm4pm imports wasm4pm-compat
- [ ] Evidence<T, Admitted, W> carries witness markers to wasm4pm
- [ ] GraduationBoundary → wasm4pm::Mining interface
- [ ] Conformance replay returns violations as Refusal<R, W> back to compat

---

## Appendix A: Witness Marker Families in Alignment

**Standards** (public W3C / ISO):

```turtle
paper:Ocel20 a compat:WitnessMarker ;
    compat:family compat:Standard ;
    compat:externalUri <https://ocel-standard.org/2.0> ;
    owl:sameAs [ rdf:value "OCEL 2.0" ] .
```

**Papers** (academic authority):

```turtle
paper:VanDerAalst2016 a compat:WitnessMarker ;
    compat:family compat:Paper ;
    dct:creator "van der Aalst, W. M. P." ;
    dct:issued "2016" ;
    compat:citations 3200 .
```

**API Grammars** (language-level):

```turtle
paper:Rust_ConstTraitImpl a compat:WitnessMarker ;
    compat:family compat:ApiGrammar ;
    compat:language "Rust" ;
    compat:feature "const_trait_impl" ;
    rdfs:seeAlso <https://github.com/rust-lang/rfcs/pull/2632> .
```

**Rust Laws** (compile-time):

```turtle
compat:WfNetSoundnessPaper a compat:WitnessMarker ;
    compat:family compat:RustLaw ;
    compat:enforces compat:WfNetConst ;
    rdfs:comment "Typed witness that WfNetConst<SOUNDNESS=True> is the only admissible form." .
```

**Internal Bridges** (graduation machinery):

```turtle
compat:GraduationViaWasm4pm a compat:WitnessMarker ;
    compat:family compat:InternalBridge ;
    compat:targetModule "wasm4pm" ;
    compat:targetType "Mining" .
```

---

## Appendix B: Cell8 Attestation Gates Mapping

| Cell8 Gate | wasm4pm-compat Surface | Proof Document | Refresh Trigger |
|---|---|---|---|
| **A1: Requirement Completeness** | `compat:CompileFailLaw` + papers | test fixture + .stderr | Law added / modified |
| **A2: Specification Consistency** | `compat:ProcessForm` + module sources | src/*.rs doctest | Doctest fails |
| **A3: Model Conformance** | `compat:TypeConstraint` | type-law fixture | Type inference changes |
| **A4: Execution Trace Fidelity** | `compat:EvidenceState` transitions | state machine test | State machine logic changes |
| **A5: Loss Accounting** | `compat:ProjectionName` + LossReport | formats example | Format export changes |
| **A6: Negative Testing** | `tests/ui/compile_fail/` fixtures | trybuild stderr | Law violation test changes |
| **A7: Audit Trail Completeness** | `audit:*` machinery + receipt chain | audit scripts | Audit rule changes |
| **A8: Rollback Safety** | `compat:Refusal<R, W>` refusal types | admission tests | Refusal enum changes |
| **A9: Form Readiness** | `compat:GraduationBoundary` | graduation.rs example | Graduation surface changes |
| **A10: Manufacturing Seal** | Cell8 cryptographic chain | receipts/ JSON | Attestation finalized |
| **A11: Release Certification** | `paper:*` paper inventory | papers.ttl | Paper list updated |
| **A12: Downstream Inheritance** | wasm4pm imports compat | Cargo.toml | wasm4pm branch changes |
| **A13: Cryptographic Chain** | BLAKE3 + Ed25519 signatures | cell8 receipts | Release commit |

---

## Appendix C: SPARQL Test Fixtures

**File:** `ggen/queries/tests/witness_marker_test.nt`

```ntriples
<https://wasm4pm-compat.rs/ontology#Ocel20> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://wasm4pm-compat.rs/ontology#WitnessMarker> .
<https://wasm4pm-compat.rs/ontology#Ocel20> <http://www.w3.org/2000/01/rdf-schema#label> "OCEL 2.0" .
<https://wasm4pm-compat.rs/ontology#Ocel20> <https://wasm4pm-compat.rs/ontology#family> <https://wasm4pm-compat.rs/ontology#Standard> .
<https://wasm4pm-compat.rs/ontology#Ocel20> <https://wasm4pm-compat.rs/ontology#citePaper> <https://wasm4pm-compat.rs/paper#Ocel20Paper> .

<https://wasm4pm-compat.rs/paper#Ocel20Paper> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://wasm4pm-compat.rs/ontology#PaperCoverage> .
<https://wasm4pm-compat.rs/paper#Ocel20Paper> <http://www.w3.org/2000/01/rdf-schema#label> "OCEL 2.0 Specification" .
```

---

## Summary

wasm4pm-compat and open-ontologies form a **three-tier authority stack** where:

1. **Type law** (wasm4pm-compat) defines what structures and states are lawful
2. **Manufacturing gates** (open-ontologies Cell8) attest that those structures are produced lawfully
3. **Execution** (wasm4pm, future) operates only on structures that passed both gates

The integration is **non-invasive**: wasm4pm-compat does NOT import open-ontologies. Instead, open-ontologies queries wasm4pm-compat's public ontology (compat: namespace) and issues cryptographic seals over its artifacts. The seal becomes the proof that a given wasm4pm-compat receipt is trustworthy enough for wasm4pm to consume.

**Next steps:** Implement SPARQL queries (Phase 2), register in ggen.toml, and draft Cell8 attestation templates (Phase 3). GAP_001 (wasm4pm bridge) remains open and is authorized by process-intelligence ALIVE_001.
