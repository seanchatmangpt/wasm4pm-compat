# wasm4pm-compat RDF Ontology Suite

Complete RDF/Turtle ontology for the wasm4pm-compat nightly Rust crate, with integrated open-ontologies standard imports and domain-specific definitions.

## Files Overview

### Primary Integration File

**`wasm4pm-compat-integrated.ttl`** (14 KB)
- Main orchestration file with `@prefix` declarations linking to open-ontologies URIs
- `owl:imports` statements for W3C standards: PROV, DCAT, DCTERMS
- `owl:imports` for process-mining domain standards: PM, OCEL, XES, BPMN
- `owl:imports` for internal domain ontology files
- Meta-level documentation of the entire type-law ecosystem
- Interop bridges linking wasm4pm-compat classes to OCEL 2.0, XES 1849-2016, BPMN 2.0, and general PM standards

### Original Core Ontology

**`wasm4pm-compat.ttl`** (73 KB)
- Complete enumeration of all 37 witness markers from `src/witness.rs`
- All 7 lifecycle stages (Raw, Parsed, Admitted, Refused, Projected, Exportable, Receipted)
- All 9 state-transition markers
- All 28+ process forms (EventLog, PetriNet, ProcessTree, DFG, OCEL, Declare, BPMN, etc.)
- Type constraint instances (Between01, ConditionCell, WorkflowPattern, Soundness tokens)
- Compile-fail laws (unbreakable type-level constraints)
- Compile-pass surfaces (proof gates for lawful paths)
- All 30+ source modules (src/*.rs)
- Graduation boundaries and wasm4pm authority surfaces
- **Base mapping**: Maps all Rust types to their RDF representations

### Domain-Specific Ontology Files

Four specialized domain ontologies, each imported by the main integrated file:

#### **`domain-evidence-structure.ttl`** (16 KB)
- Evidence carrier classes: RawEvidence, ParsedEvidence, AdmittedEvidence, RefusedEvidence, ProjectedEvidence, ExportableEvidence, ReceiptedEvidence
- Witness marker class hierarchy: StandardWitness, PaperWitness, ApiGrammarWitness, RustLawWitness, InternalBridgeWitness
- 6 key witness instances: OCEL 2.0, XES 1849-2016, BPMN 2.0, POWL, WF-net soundness, wasm4pm bridge
- Lifecycle stage class definitions and state transitions
- Admission/Refusal verdict classes
- Loss accounting: LossReport, ProjectionName, LossPolicy classes

**URI**: `https://wasm4pm-compat.rs/domain/evidence-structure`

#### **`domain-process-forms.ttl`** (21 KB)
- Top-level ProcessForm class definition
- Event log family: EventLog, Trace, Event, OcelLog, XesLog, StreamingLog
- Petri net family: PetriNet, Place, Transition, Arc, Marking, WfNet, SeparableWfNet, ObjectCentricPetriNet, WorkflowNet
- Discovery models: ProcessTree, ProcessTreeNode, ProcessTreeOperator, PowlNode, DirectlyFollowsGraph, CausalNet
- Declarative models: DeclareModel, OcpqQuery
- BPMN models: BpmnModel, BpmnTask, BpmnGateway, BpmnEvent
- Conformance forms: ConformanceVerdict, PredictionTarget, ProcessCube, TemporalConstraint
- Object-centric forms: ObjectLifecycle, CausalityGraph
- Multi-perspective and interop: MultiPerspectiveLog, CorrelationSchema, Receipt, InteropBridge
- Interop bridges to standard ontologies (OCEL, XES, BPMN, PM)
- Process form lifecycle properties (graduatesToWasm4pm, graduationReason)

**URI**: `https://wasm4pm-compat.rs/domain/process-forms`

#### **`domain-type-constraints.ttl`** (19 KB)
- TypeConstraint and CompileTimeConstant class hierarchy
- Between01 constraint family with Fitness, Precision, F1 metric classes
- ConditionCell constraint with the Blue River Dam covenant
- Require<{EXPR}>: IsTrue constraint mechanism
- WorkflowPattern ConstParamTy enum with 17 variants
- Soundness token constraints: SoundnessUnknown, SoundnessClaimed, SoundnessWitnessed
- Unbreakable laws with proof gates: WitnessDiscrimination, RawExportedAsAdmitted, MissingFinalMarking, DanglingEventObjectLink, SealedEvidenceState
- Compile-fail fixtures as ALIVE proof gates (7 laws)
- Compile-pass surfaces as lawful-path proof gates (3 surfaces)
- Nightly feature declarations: generic_const_exprs, adt_const_params, const_trait_impl, min_specialization, portable_simd

**URI**: `https://wasm4pm-compat.rs/domain/type-constraints`

#### **`domain-graduation-boundaries.ttl`** (18 KB)
- GraduationBoundary class hierarchy
- Six graduation reasons: NeedsDiscovery, NeedsConformanceExecution, NeedsReplay, NeedsBenchmarkGating, NeedsObjectCentricQuery, RebuildingProcessMiningLocally
- Five wasm4pm authority surfaces:
  - **MiningAuthority**: Alpha, Inductive, Heuristics Miners, DFG mining, variant analysis
  - **ConformanceAuthority**: Alignment, fitness, precision, generalization, F1
  - **ReplayAuthority**: Token-based replay, parallel execution, path finding
  - **LifecycleAuthority**: Object-centric tracking, state machines, provenance
  - **QueryAuthority**: OCPQ query execution (new)
- GraduationCandidate and bridge trait
- Grounded vs Ungrounded candidates
- Input/output form mappings for each authority
- Properties linking process forms to graduation boundaries

**URI**: `https://wasm4pm-compat.rs/domain/graduation-boundaries`

### Supporting Files

**`papers.ttl`** (22 KB)
- Paper coverage ledger: 81 published papers indexed and mapped to compat coverage verdicts
- Links to source modules providing coverage
- Coverage verdicts: COVERED_BY_TYPE, COVERED_BY_FIXTURE, COVERED_BY_GRADUATION_BOUNDARY, PARTIAL_WITH_REASON, MISSING_TYPE_LAW, OUT_OF_SCOPE_WITH_REASON, DUPLICATE_OR_BACKGROUND

**`ggen-substrate.ttl`** (13 KB)
- ggen meta-ontology and substrate definitions
- Relationships between generated artifacts

**`audit-machinery.ttl`** (45 KB)
- Audit and conformance machinery for the type-law crate
- Test coverage mappings

## Prefix Declarations

All files use these standard prefixes:

```turtle
@prefix compat:  <https://wasm4pm-compat.rs/ontology#> .
@prefix paper:   <https://wasm4pm-compat.rs/paper#> .
@prefix domain:  <https://wasm4pm-compat.rs/domain#> .
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs:    <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix skos:    <http://www.w3.org/2004/02/skos/core#> .
@prefix prov:    <http://www.w3.org/ns/prov#> .
@prefix dcat:    <http://www.w3.org/ns/dcat#> .
@prefix dcterms: <http://purl.org/dc/terms/> .

# Standard process-mining ontologies
@prefix pm:      <http://www.purl.org/pm/ontology/2023/05#> .
@prefix ocel:    <https://ocel-standard.org/ontology#> .
@prefix xes:     <http://www.xes-standard.org/ontology#> .
@prefix bpmn:    <https://www.omg.org/spec/BPMN/20100524/MODEL#> .
```

## OWL Imports Chain

The main integrated ontology imports from:

1. **W3C Standards**:
   - `http://www.w3.org/ns/prov` (W3C PROV provenance)
   - `http://www.w3.org/ns/dcat` (W3C DCAT data catalog)
   - `http://purl.org/dc/terms/` (Dublin Core metadata)

2. **Process-Mining Standards**:
   - `http://www.purl.org/pm/ontology/2023/05` (General PM ontology)
   - `https://ocel-standard.org/ontology` (OCEL 2.0)
   - `http://www.xes-standard.org/ontology` (XES 1849-2016)
   - `https://www.omg.org/spec/BPMN/20100524/MODEL` (BPMN 2.0)

3. **Internal Domain Ontologies**:
   - `https://wasm4pm-compat.rs/domain/evidence-structure`
   - `https://wasm4pm-compat.rs/domain/process-forms`
   - `https://wasm4pm-compat.rs/domain/graduation-boundaries`
   - `https://wasm4pm-compat.rs/domain/type-constraints`

## Interop Bridges to Standards

Each major type in wasm4pm-compat maps to corresponding classes in standard ontologies:

### OCEL 2.0 Integration
- `compat:OcelLog` ⟷ `ocel:OcelLog`
- `compat:OcelEvent` ⟷ `ocel:Event`
- `compat:EventObjectLink` ⟷ `ocel:EventObjectLink`
- `compat:ObjectObjectLink` ⟷ `ocel:ObjectObjectLink`
- `compat:ObjectChange` ⟷ `ocel:ObjectChange`

### XES Integration
- `compat:EventLog` ⟷ `xes:Log`
- `compat:Event` ⟷ `xes:Event`
- `compat:XesLog` ⟷ `xes:Log`

### BPMN 2.0 Integration
- `compat:BpmnModel` ⟷ `bpmn:Process`
- `compat:BpmnTask` ⟷ `bpmn:Task`
- `compat:BpmnGateway` ⟷ `bpmn:Gateway`
- `compat:BpmnEvent` ⟷ `bpmn:Event`

### General PM Ontology Integration
- `compat:PetriNet` ⟷ `pm:PetriNet`
- `compat:ProcessTree` ⟷ `pm:ProcessTree`
- `compat:DirectlyFollowsGraph` ⟷ `pm:DirectlyFollowsGraph`
- `compat:DeclareModel` ⟷ `pm:DeclareModel`

## Key Class Hierarchies

### Evidence Lifecycle
```
EvidenceCarrier
├── RawEvidence
├── ParsedEvidence
├── AdmittedEvidence
├── RefusedEvidence
├── ProjectedEvidence
├── ExportableEvidence
└── ReceiptedEvidence
```

### Witness Markers
```
WitnessMarker
├── StandardWitness
│   ├── OcelStandardWitness
│   ├── XesStandardWitness
│   └── BpmnStandardWitness
├── PaperWitness
│   ├── PowlPaperWitness
│   └── WfNetSoundnessPaperWitness
├── ApiGrammarWitness
├── RustLawWitness
└── InternalBridgeWitness
    └── Wasm4pmBridgeWitness
```

### Process Forms
```
ProcessForm
├── EventLog
├── OcelLog
├── XesLog
├── StreamingLog
├── PetriNet
│   ├── WfNet
│   ├── SeparableWfNet
│   ├── ObjectCentricPetriNet
│   └── WorkflowNet
├── ProcessTree
├── PowlNode
├── DirectlyFollowsGraph
├── CausalNet
├── DeclareModel
├── OcpqQuery
├── BpmnModel
├── ConformanceVerdict
├── PredictionTarget
├── ProcessCube
├── TemporalConstraint
├── ObjectLifecycle
├── CausalityGraph
├── MultiPerspectiveLog
├── CorrelationSchema
├── Receipt
└── InteropBridge
```

### Type Constraints
```
TypeConstraint
├── CompileTimeConstant
│   ├── Between01Constraint
│   ├── ConditionCellConstraint
│   ├── RequireIsTrueConstraint
│   │   └── TypedLoopNodeArityConstraint
│   ├── WorkflowPatternConstraint
│   └── SoundnessTokenConstraint
└── UnbreakableLaw
    ├── WitnessDiscriminationLaw
    ├── RawExportedAsAdmittedLaw
    ├── MissingFinalMarkingLaw
    ├── DanglingEventObjectLinkLaw
    └── SealedEvidenceStateLaw
```

### Graduation Surfaces
```
GraduationSurface
├── MiningAuthority
├── ConformanceAuthority
├── ReplayAuthority
├── LifecycleAuthority
├── QueryAuthority
├── PerformanceAuthority
└── MultipleAuthorities
```

## Proof Gates (ALIVE Certification)

All compile-fail and compile-pass fixtures are mapped as proof gates in the ontology:

### Compile-Fail Fixtures (Unbreakable Laws)
- Between01 Violation Law
- ConditionCell Overflow Law
- WitnessDiscrimination Law
- RawExportedAsAdmitted Law
- MissingFinalMarking Law
- DanglingEventObjectLink Law
- SealedEvidenceState Law

### Compile-Pass Fixtures (Lawful Paths)
- SeparableWfNetMarker Pass
- WfNet2Powl Witness Pass
- WorkflowPatternConstParam Pass

Each proof gate carries:
- `compat:lawName`: The named law
- `compat:errorCode`: Expected rustc error code
- `compat:fixtureFile`: Path to fixture in tests/ui/
- `compat:stderrFile`: Path to expected .stderr output
- `domain:proofGateType`: "ALIVE" certification level

## Querying the Ontology

### SPARQL Examples

**Find all witness markers:**
```sparql
SELECT ?witness ?title ?year WHERE {
  ?witness a domain:WitnessMarker ;
           compat:witnessTitle ?title ;
           compat:witnessYear ?year .
}
```

**Find all process forms that graduate to wasm4pm:**
```sparql
SELECT ?form ?reason WHERE {
  ?form a domain:ProcessForm ;
        compat:graduatesToWasm4pm true ;
        compat:graduationReason ?reason .
}
```

**Find all compile-fail laws:**
```sparql
SELECT ?law ?name ?errorCode WHERE {
  ?law a domain:UnbreakableLawClass ;
       compat:lawName ?name ;
       compat:errorCode ?errorCode .
}
```

**Trace graduation boundaries:**
```sparql
SELECT ?reason ?authority WHERE {
  ?reason a domain:GraduationBoundary ;
          domain:graduatesToWasm4pmAuthority ?authority .
}
```

## Ontology Statistics

| Category | Count |
|----------|-------|
| Witness Markers | 37 |
| Lifecycle Stages | 7 |
| State Transitions | 9 |
| Process Forms | 28+ |
| Type Constraints | 6 |
| Unbreakable Laws | 7 |
| Lawful Paths | 3 |
| Source Modules | 30 |
| Graduation Reasons | 6 |
| wasm4pm Authorities | 6 |
| Papers Indexed | 81 |

## Closed-World Semantics

This ontology operates under the **closed-world assumption**:

- All witness markers are enumerated in `domain-evidence-structure.ttl`
- All lifecycle stages are enumerated in `domain-evidence-structure.ttl`
- All process forms are enumerated in `domain-process-forms.ttl`
- All type constraints are enumerated in `domain-type-constraints.ttl`
- All graduation boundaries are enumerated in `domain-graduation-boundaries.ttl`
- All compile-fail laws are backed by fixtures in `tests/ui/compile_fail/`
- All compile-pass surfaces are backed by fixtures in `tests/ui/compile_pass/`

**If a concept is not in this ontology, it is not an enforceable type law in wasm4pm-compat.**

## Integration with wasm4pm

The graduation ontology (`domain-graduation-boundaries.ttl`) bridges to the wasm4pm execution engine:

- **MiningAuthority** → `wasm4pm::mining` — process discovery
- **ConformanceAuthority** → `wasm4pm::conformance` — model-to-log alignment and conformance metrics
- **ReplayAuthority** → `wasm4pm::replay` — token-based simulation
- **LifecycleAuthority** → `wasm4pm::lifecycle` — object-centric process execution
- **QueryAuthority** → `wasm4pm::query` — OCPQ query execution
- **PerformanceAuthority** → `wasm4pm::performance` — benchmark and scalability measurement

Each authority accepts specific process forms as input and produces specific forms as output, with evidence carriers tagged by witness markers.

## Metadata

- **Version**: 1.0.0
- **Created**: 2026-06-01
- **Format**: RDF/Turtle
- **License**: Apache 2.0
- **Creator**: wasm4pm-compat contributors
- **Subject Keywords**: process mining, event logs, object-centric process analysis, type-law enforcement, Rust nightly, zero-cost abstraction

## References

1. **OCEL 2.0 Standard**: https://ocel-standard.org/
2. **XES Standard (IEEE 1849-2016)**: http://www.xes-standard.org/
3. **BPMN 2.0**: https://www.omg.org/spec/BPMN/
4. **W3C PROV**: http://www.w3.org/ns/prov
5. **General PM Ontology**: http://www.purl.org/pm/ontology/

## Validation

To validate RDF syntax:
```bash
cd /Users/sac/wasm4pm-compat/ggen/ontology
# Validate with any RDF validator, e.g., Turtle validator
```

To query with SPARQL:
```bash
# Use Apache Jena SPARQL engine or similar
# $ sparql --data wasm4pm-compat-integrated.ttl --query query.rq
```

---

**Generated by wasm4pm-compat RDF integration workflow**
