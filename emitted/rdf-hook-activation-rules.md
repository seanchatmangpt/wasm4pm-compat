# RDF Hook Activation Rules — wasm4pm-compat

**Date:** 2026-06-01  
**Scope:** RDF/SHACL rule systems that trigger structured hook activation in the wasm4pm-compat type-law crate  
**Source Files:**
- `/ggen/shapes/process-tree.shacl.ttl` — SHACL proof gates
- `/ggen/shapes/loss-accounting.shacl.ttl` — Loss accounting constraints
- `/ggen/ontology/audit-machinery.ttl` — Audit specification rules
- `/ggen/ontology/domain-type-constraints.ttl` — Compile-time constraint laws
- `/ggen/ontology/domain-evidence-structure.ttl` — Evidence lifecycle and state transitions
- `/ggen/ontology/ggen-substrate.ttl` — Generation rule metadata

---

## 1. Semantic Rules (SHACL & SPARQL)

### 1.1 SHACL Node Shapes with SPARQL Constraints

#### ProcessTreeOperator_LoopShape (process-tree.shacl.ttl)

**Rule Trigger Condition:**
- Class: `compat:ProcessTreeOperator_Loop`
- Target: Any RDF node instantiating this class

**Firing Condition:**
```sparql
?this a compat:ProcessTreeOperator_Loop .
```

**Hook Effect Specification:**
- **Gate Name:** `compat:ProcessTreeLoop_hasChildProp`
- **Constraint Type:** `sh:PropertyShape`
- **Assertion Checked:**
  - Property path: `compat:hasChild`
  - Cardinality: `sh:minCount 2, sh:maxCount 2`
  - Node kind: `sh:IRI`
- **Violation Message:** "Loop operator must have exactly 2 children (do-body + redo branch). This is a non-negotiable invariant."
- **Effect:** Type error; loop operator must have exactly 2 children per Leemans 2013

---

#### ProcessTreeOperator_SilentShape (process-tree.shacl.ttl)

**Rule Trigger Condition:**
- Class: `compat:ProcessTreeOperator_Silent`

**Firing Condition:**
```sparql
?this a compat:ProcessTreeOperator_Silent .
```

**Hook Effect Specification:**
- **Gate Name:** `compat:ProcessTreeSilent_hasChildProp`
- **Cardinality Bounds:** `sh:maxCount 0`
- **Effect:** Silent (tau) must have 0 children; it is a leaf node
- **Violation:** Any `compat:hasChild` property on silent operator fails the gate

---

#### TreeProjectableShape with SPARQL Constraint (process-tree.shacl.ttl)

**Rule Trigger Condition:**
- Class: `compat:TreeProjectable`
- Target: All instances of class `TreeProjectable`

**Firing Condition:**
```sparql
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>
SELECT ?this
WHERE {
    ?this a compat:TreeProjectable .
    OPTIONAL { ?this compat:hasOrphanedOperator ?orphan }
    OPTIONAL { ?this compat:hasCycle ?cycle }
    FILTER (BOUND(?orphan) || BOUND(?cycle))
}
```

**Hook Effect Specification:**
- **Gate Name:** `TreeProjectableShape`
- **Constraint Type:** `sh:SPARQLConstraint`
- **Property Assertion Checked:**
  - Absence of `compat:hasOrphanedOperator` (disconnected operators)
  - Absence of `compat:hasCycle` (cycles in operator graph)
- **Violation Message:** "Projection to process tree must preserve block structure: no orphaned operators, no cycles."
- **Effect:** Projection must satisfy block structure invariant; orphaned or cyclic operators cause refusal

---

#### ProcessTreeRefusalShape (process-tree.shacl.ttl)

**Rule Trigger Condition:**
- Class: `compat:ProcessTreeRefusal`

**Firing Condition:**
```sparql
?this a compat:ProcessTreeRefusal .
```

**Hook Effect Specification:**
- **Gate Name:** `compat:ProcessTreeRefusal_hasRefusalReasonProp`
- **Property Path:** `compat:hasRefusalReason`
- **Enum Constraint:** `sh:in (compat:ProcessTreeRefusal_InvalidArity, compat:ProcessTreeRefusal_NonBlockStructured, compat:ProcessTreeRefusal_UnprojectableFromPowl, compat:ProcessTreeRefusal_NonMonotonicReduction)`
- **Cardinality:** `sh:minCount 1, sh:maxCount 1`
- **Effect:** Refusal reason must be one of four named laws; bare/generic reasons forbidden

---

### 1.2 Loss Accounting Constraints (loss-accounting.shacl.ttl)

#### LossReportShape (loss-accounting.shacl.ttl)

**Rule Trigger Condition:**
- Class: `compat:LossReport`

**Firing Condition:**
```sparql
?this a compat:LossReport .
```

**Hook Effect Specification:**
- **Gate Name:** `compat:LossReport_hasProjectionNameProp`
- **Properties to Validate:**
  1. **ProjectionName:** `sh:minCount 1, sh:maxCount 1`, `sh:minLength 1`, `xsd:string`
  2. **LossPolicy:** `sh:minCount 1, sh:maxCount 1`, `sh:in (RefuseLoss | AllowNamedProjection | AllowLossWithReport)`
  3. **Items or Lossless:** `sh:or [ sh:minCount 1 ] | [ sh:hasValue true ]`
- **Violation Message:** "LossReport must have exactly one non-empty ProjectionName, exactly one LossPolicy, and either Items (≥1) or isLossless=true."
- **Effect:** Every lossy projection must carry:
  - Named `ProjectionName` (string identifier)
  - `LossPolicy` decision (before projection)
  - Either items enumerated or marked lossless

---

#### NamedLossShape (loss-accounting.shacl.ttl)

**Rule Trigger Condition:**
- Class: `compat:NamedLoss`

**Firing Condition:**
```sparql
?this a compat:NamedLoss .
```

**Hook Effect Specification:**
- **Property Assertions:**
  - `compat:hasProjectionName` (exactly 1, non-empty string)
  - `compat:hasLossCategory` (exactly 1, from enum: ObjectLoss | AttributeLoss | LinkLoss | StructuralLoss)
- **Effect:** Loss is only valid if categorized as one of four named types

---

## 2. Hook Firing Conditions (Property Assertions and Graph Patterns)

### 2.1 State Transition Hooks (Evidence Lifecycle)

**Source:** `domain-evidence-structure.ttl`

#### Raw → Parsed Transition

**Trigger Property Assertion:**
```sparql
?evidence domain:evidenceState domain:RawStage .
?evidence compat:successorState domain:ParsedStage .
```

**Hook Effect:**
- Firing: When evidence carries state tag `Raw`
- Hook Action: Trigger parsing phase
- Condition: Must reach `Parsed` state as first successor

#### Parsed → Admitted Transition

**Trigger Property Assertion:**
```sparql
?evidence domain:evidenceState domain:ParsedStage .
?evidence compat:successorState domain:AdmittedStage .
```

**Hook Effect:**
- Firing: When evidence is parsed and ready for admission boundary
- Hook Action: Invoke `Admit::admit()` trait implementation
- Condition: Transition is the ONLY sanctioned path to Admitted

#### Admitted → Projected | Exportable | Receipted

**Trigger Property Assertion:**
```sparql
?evidence domain:evidenceState domain:AdmittedStage .
OPTIONAL { ?evidence compat:successorState domain:ProjectedStage . }
OPTIONAL { ?evidence compat:successorState domain:ExportableStage . }
OPTIONAL { ?evidence compat:successorState domain:ReceiptedStage . }
```

**Hook Effect:**
- Firing: When evidence is admitted
- Hook Actions:
  - If Projected: Require `LossReport<From, To, Items>` and `LossPolicy`
  - If Exportable: Verify admitted boundary cleared
  - If Receipted: Require provenance-bearing receipt

#### Refused (Terminal)

**Trigger Property Assertion:**
```sparql
?evidence domain:evidenceState domain:RefusedStage .
```

**Hook Effect:**
- Firing: When evidence fails admission boundary
- Hook Action: Must carry specific named law reason (not bare InvalidInput)
- Condition: No backwards transition to Admitted allowed

---

### 2.2 Witness Discrimination Hooks

**Source:** `audit-machinery.ttl`

#### WitnessDiscriminationLawSpec

**Trigger Condition:**
```sparql
?admission a domain:AdmissionVerdict .
?admission domain:evidenceWitness ?w1 .
OPTIONAL {
    ?admission compat:coercedTo ?witness2 .
    FILTER (?w1 != ?w2)
}
```

**Firing Condition:** Attempted use of `Admission<T, W1>` as `Admission<T, W2>`

**Hook Effect Specification:**
- **Gate Name:** `WitnessDiscriminationLawSpec`
- **Effect:** Type error; witness markers are sealed and non-interchangeable
- **Witness Variants:** `Ocel20`, `Xes1849`, `Bpmn20`, `PowlPaper`, `WfNetSoundnessPaper`, `Wasm4pmBridge`
- **Proof Gate:** Compile-fail fixture at `tests/ui/compile_fail/witness_discrimination.rs`

---

### 2.3 Class Instantiation Hooks

#### CompileFailLaw Instantiation

**Trigger Condition:**
```sparql
?law a compat:CompileFailLaw .
?law compat:lawName ?name .
?law compat:fixtureFile ?fixture .
?law compat:stderrFile ?stderr .
```

**Firing Condition:** A new `CompileFailLaw` instance is declared in ontology

**Hook Effect Specification:**
- **Gate Name:** `AllLawsHaveFixturesGate` (from audit-machinery.ttl)
- **Hook Actions:**
  1. Verify fixture file exists at `compat:fixtureFile`
  2. Verify `.stderr` file exists at `compat:stderrFile`
  3. Fixture must fail for the intended reason (named law), not by accident
  4. Compiler diagnostic in `.stderr` must match actual error
- **Severity:** `FATAL` — missing fixture blocks ALIVE gate

---

#### ProcessTreeRefusalReason Instantiation

**Trigger Condition:**
```sparql
?refusal a compat:ProcessTreeRefusal .
?refusal compat:hasRefusalReason ?reason .
?reason a compat:ProcessTreeRefusalReason .
```

**Firing Condition:** A `ProcessTreeRefusal` carries a reason

**Hook Effect Specification:**
- **Authorized Reasons:** (4 only)
  - `ProcessTreeRefusal_InvalidArity` — child count violates operator law
  - `ProcessTreeRefusal_NonBlockStructured` — result is not block-structured
  - `ProcessTreeRefusal_UnprojectableFromPowl` — POWL cannot map to tree semantics
  - `ProcessTreeRefusal_NonMonotonicReduction` — tree reduction violates Parikh monotonicity
- **Effect:** Bare or unlisted reasons cause audit violation

---

## 3. Hook Effect Specifications

### 3.1 Type-Level Proof Gates (ALIVE Certification)

#### Between01 Metric Bounds

**Rule Source:** `domain-type-constraints.ttl`

**Hook Trigger:**
```rust
Between01<NUM, DEN>  // NUM and DEN are const generics
```

**Effect Specification:**
- **Condition:** `NUM <= DEN` must evaluate to `true` at compile time
- **Violation:** `Between01<2, 1>` does not compile (E0080)
- **Error Code:** `E0080` (evaluates to an illegal value)
- **Proof Gate:** Compile-fail fixture at `tests/ui/compile_fail/between01_violation.rs`
- **Metrics Protected:** Fitness, Precision, F1 — all bounded to [0, 1]

---

#### ConditionCell<BITS> Overflow (Blue River Dam Covenant)

**Hook Trigger:**
```rust
ConditionCell<BITS>  // BITS is a const generic
```

**Effect Specification:**
- **Condition:** `BITS <= 8` must hold at compile time
- **Violation:** `ConditionCell<9>` does not compile (E0080)
- **Error Code:** `E0080`
- **Architectural Impact:** `HIGH` — exceeding limit forces process cube split decision
- **Proof Gate:** Compile-fail fixture at `tests/ui/compile_fail/condition_cell_overflow.rs`
- **Covenant:** "Need 9 means split" — cannot add 9th condition dimension without architectural refactoring

---

#### TypedLoopNode<ARITY> Constraint

**Hook Trigger:**
```rust
TypedLoopNode<ARITY>  // ARITY is a const generic
```

**Effect Specification:**
- **Condition:** `Require<{ ARITY == 2 }>: IsTrue` enforced in where-bounds
- **Violation:** `TypedLoopNode<3>` does not compile
- **Nightly Feature:** `generic_const_exprs` required
- **Leemans 2013 Theorem:** Loop operator in process tree has exactly 2 children (do-body + redo)
- **Proof:** Type itself is non-constructible with wrong arity

---

### 3.2 Admission Boundary Hooks

#### Admit Trait Implementation (Only Sanctioned Path)

**Hook Trigger:**
```sparql
?raw a domain:RawEvidence .
?parsed a domain:ParsedEvidence .
?admitted a domain:AdmittedEvidence .
```

**Effect Specification:**
- **Gate Name:** `TypeStateAdmissionBoundarySpec`
- **Firing Condition:** Code path attempts `Raw → Admitted` transition
- **Hook Actions:**
  1. Invoke `Admit::admit()` on parsed evidence
  2. Return `Admission<T, W>` (success) or `Refusal<R, W>` (failure)
  3. Refusal reason `R` must be a specific named law (e.g., `DanglingEventObjectLink`, `MissingFinalMarking`)
  4. No bare `InvalidInput` enum variant allowed
- **Proof Gate:** Compile-pass fixture at `tests/ui/compile_pass/typestate_admission_boundary.rs`
- **Severity:** `FATAL` — silent structure loss or bare refusals block ALIVE gate

---

#### BareInvalidInputRefusal Detection

**Hook Trigger:**
```sparql
?refusal a domain:RefusalVerdict .
?refusal domain:refusalReasonType "InvalidInput" .
```

**Effect Specification:**
- **Gate Name:** `BareInvalidInputRefusalSpec`
- **Audit Scope:** Module (`src/admission.rs`)
- **Hook Action:** Static analysis scan for instantiations of `Refusal<InvalidInput, W>`
- **Effect:** Any use of bare `InvalidInput` enum variant is a compile-time/audit error
- **Proof Gate:** Static analysis template — scan via grep for forbidden pattern
- **Severity:** `FATAL`

---

### 3.3 Loss Accounting Hooks

#### Loss Report Emission

**Hook Trigger:**
```sparql
?evidence a domain:ProjectedEvidence .
?evidence domain:evidenceState domain:ProjectedStage .
```

**Effect Specification:**
- **Gate Name:** `NoSilentLossInProjectionsGate`
- **Firing Condition:** Any Evidence<T, Projected, W> is created
- **Hook Actions:**
  1. Verify `LossReport<From, To, Items>` is present
  2. Verify `LossPolicy` decision was made BEFORE projection
  3. Verify projection name is non-empty string
  4. If items lost: enumerate them by name and count
  5. If lossless: set `isLossless = true` and explain why
- **Runtime Validation:** Every projection path carries exactly one report
- **Severity:** `FATAL` — silent loss is a structural defect

---

#### Loss Category Classification

**Hook Trigger:**
```sparql
?loss a compat:NamedLoss .
?loss compat:hasLossCategory ?category .
```

**Effect Specification:**
- **Categories (4 total):**
  1. **ObjectLoss** — entire objects were discarded
  2. **AttributeLoss** — object attributes (metadata) were dropped
  3. **LinkLoss** — event-to-object or object-to-object links dropped
  4. **StructuralLoss** — nesting, ordering, concurrency relationships lost
- **Effect:** Loss must fit exactly one category; uncategorized loss is an audit gap
- **Example Projection:** OCEL → XES loses object-to-object links (LinkLoss category)

---

### 3.4 Receipt Provenance Hooks

#### Receipt Creation and Validation

**Hook Trigger:**
```sparql
?evidence a domain:ReceiptedEvidence .
?receipt compat:carriesReceipt ?evidence .
?receipt audit:commitHash ?hash .
?receipt audit:commitAuthor ?author .
?receipt audit:commitTimestamp ?ts .
```

**Effect Specification:**
- **Gate Name:** `AllReceiptsValidateGate`, `ReceiptProvenanceSpec`
- **Firing Condition:** Evidence enters Receipted state
- **Hook Actions:**
  1. Verify commit hash is valid and reachable in git history
  2. Verify author email matches commit metadata
  3. Verify timestamp is in ISO 8601 format and monotonically increasing
  4. Verify witness marker matches evidence witness
  5. Verify receipt cannot be backdated or forged
- **Provenance Chain:** Commit → Receipt → Evidence → Witness
- **Severity:** `FATAL` — forged receipts invalidate entire audit chain

---

## 4. Generation Rule Activation (ggen Substrate)

**Source:** `ggen-substrate.ttl`

### 4.1 Witness Marker Generation Hook

**Rule:** `substrate:WitnessGenRule`

**Query File:** `queries/extract-witnesses.rq`

**Trigger Condition:**
```sparql
?witness a compat:WitnessMarker .
?witness compat:rustType ?rustType .
?witness compat:witnessKey ?key .
?witness compat:witnessTitle ?title .
?witness compat:witnessYear ?year .
```

**Hook Effect:**
- **Template:** `templates/witness-marker.tera`
- **Output:** `src/generated/witnesses.rs` (single file)
- **Effect:** Renders one zero-sized marker enum per WitnessMarker instance
- **Activation:** Each new witness in ontology triggers code generation

---

### 4.2 Compile-Fail Fixture Generation Hook

**Rule:** `substrate:CompileFailGenRule`

**Query File:** `queries/extract-compile-fail-laws.rq`

**Trigger Condition:**
```sparql
?law a compat:CompileFailLaw .
?law compat:fixtureFile ?fixturePath .
?law compat:stderrFile ?stderrPath .
```

**Hook Effect:**
- **Template:** `templates/compile-fail-fixture.tera`
- **Output:** `tests/ui/compile_fail/` (one file per law)
- **Effect:** Generates trybuild fixture .rs file and expected .stderr diagnostic
- **Activation:** Each CompileFailLaw instance triggers fixture generation
- **Proof Gate:** ALIVE gate requires all generated fixtures to compile and fail correctly

---

### 4.3 Compile-Pass Fixture Generation Hook

**Rule:** `substrate:CompilePassGenRule`

**Query File:** `queries/extract-process-forms.rq`

**Trigger Condition:**
```sparql
?form a compat:ProcessForm .
?surface a compat:CompilePassSurface .
?surface compat:formsFrom ?form .
```

**Hook Effect:**
- **Template:** `templates/compile-pass-fixture.tera`
- **Output:** `tests/ui/compile_pass/` (one file per ProcessForm/Surface)
- **Effect:** Generates trybuild fixture proving lawful path is open
- **Activation:** Each ProcessForm triggers compile-pass proof generation

---

### 4.4 Audit Script Generation Hook

**Rule:** `substrate:AuditScriptGenRule`

**Query File:** `queries/extract-source-modules.rq`

**Trigger Condition:**
```sparql
?module a compat:SourceModule .
?module compat:filePath ?path .
?module compat:moduleType ?type .
```

**Hook Effect:**
- **Template:** `templates/audit-script.tera`
- **Output:** `scripts/audit/` (one shell script per SourceModule)
- **Effect:** Renders per-module audit script checking invariants:
  - Doctest coverage
  - No `unsafe_code`
  - No engine logic (only structure)
  - No forbidden patterns
- **Activation:** Each SourceModule instance triggers audit script generation

---

## 5. Audit Gate Activation (ALIVE Certification)

**Source:** `audit-machinery.ttl`

### 5.1 ALIVE Gate Requirements

#### AllLawsHaveFixturesGate

**Trigger:** During audit certification

**Condition:**
```sparql
?law a compat:CompileFailLaw .
FILTER NOT EXISTS { ?law compat:fixtureFile ?file . }
```

**Effect:**
- **Severity:** `FATAL`
- **Hook Action:** Scan all CompileFailLaw instances
- **Success Criterion:** Every law must have a fixture file
- **Failure:** Audit cannot pass; gap opened for missing fixture

---

#### AllFixturesHaveStderrGate

**Trigger:** During audit certification

**Condition:**
```sparql
?fixture compat:fixtureFile ?path .
FILTER NOT EXISTS { ?fixture compat:stderrFile ?stderrPath . }
```

**Effect:**
- **Severity:** `FATAL`
- **Hook Action:** Verify `.stderr` files exist for all compile-fail fixtures
- **Success Criterion:** Every fixture must have expected compiler diagnostic
- **Trybuild Requirement:** Without `.stderr`, fixture output cannot be validated

---

#### AllGapsClosedOrAcceptedGate

**Trigger:** Before release

**Condition:**
```sparql
?gap a audit:Gap .
?gap audit:gapSeverity ?severity .
FILTER (?severity IN ("critical", "major"))
FILTER NOT EXISTS { ?gap audit:closesGap audit:ClosureClaim . }
```

**Effect:**
- **Severity:** `FATAL`
- **Hook Action:** Enumerate all documented gaps
- **Success Criterion:** All critical/major gaps must have ClosureClaim with committing evidence
- **Alternative:** Gap must be reclassified as `accepted-debt` with justification

---

#### NoSilentLossInProjectionsGate

**Trigger:** During runtime testing

**Condition:**
```sparql
?projected a domain:ProjectedEvidence .
FILTER NOT EXISTS { ?projected compat:carriesReport ?report . }
```

**Effect:**
- **Severity:** `FATAL`
- **Hook Action:** Scan all Evidence<T, Projected, W> instances
- **Success Criterion:** Every projection must carry exactly one LossReport
- **Failure:** Silent loss is a structural defect; blocks audit

---

## 6. Hostile Assumption Activation Rules

**Source:** `audit-machinery.ttl` (lines 473–501)

### 6.1 HA_DeclaredPipelineNotReal

**Trigger Condition:**
```sparql
?pipeline compat:declaredStages ?stages .
?log a ocel:EventLog .
```

**Hook Effect:**
- **Firing Condition:** Whenever declared manufacturing pipeline is compared against actual event log
- **Hook Action:** Derive actual process from event log via process discovery
- **Verification:** Compare declared model against discovered actual process
- **Failure Case:** Stages may be skipped or repeated without detection in code alone

---

### 6.2 HA_ReceiptsEmittedOutOfCycle

**Trigger Condition:**
```sparql
?receipt compat:hasTimestamp ?ts .
?event ocel:timestamp ?eventTs .
FILTER (?ts > ?eventTs)  // Receipt dated before event occurrence
```

**Hook Effect:**
- **Firing Condition:** Receipt timestamp is checked against event log causality
- **Hook Action:** Verify receipt provenance chains are consistent with causality
- **Failure Case:** Receipts may be emitted outside lawful object lifecycles

---

### 6.3 HA_ProofGatesCanPass_NonConforming

**Trigger Condition:**
```sparql
?gate sh:hasValue true .  // Gate claims pass
?log a ocel:EventLog .
```

**Hook Effect:**
- **Firing Condition:** Proof gate passes but OCEL conformance check fails
- **Hook Action:** Derive actual conformance from OCEL event logs
- **Verification:** Run process-mining conformance check (via wasm4pm)
- **Failure Case:** Gate may pass despite non-conforming execution

---

## 7. Summary Table: RDF Rules → Hook Activation

| RDF Rule | Trigger | Hook Type | Severity | Effect |
|----------|---------|-----------|----------|--------|
| ProcessTreeOperator_LoopShape | Class instantiation | Type constraint | FATAL | Loop must have exactly 2 children |
| TreeProjectableShape (SPARQL) | Property assertion | Graph constraint | FATAL | No orphaned operators; no cycles |
| LossReportShape | Evidence creation | Loss accounting | FATAL | Must carry ProjectionName, LossPolicy, Items/Lossless |
| WitnessDiscriminationLawSpec | Witness coercion attempt | Type error | FATAL | Witness markers sealed, non-interchangeable |
| Admit trait (admission boundary) | Parsed evidence | State transition | FATAL | Only sanctioned Raw → Admitted path |
| Between01<NUM, DEN> | Const generic bound | Type constraint | FATAL | NUM ≤ DEN at compile time |
| ConditionCell<BITS> | Const generic bound | Type constraint | FATAL | BITS ≤ 8 (Blue River Dam) |
| TypedLoopNode<ARITY> | Const generic bound | Type constraint | FATAL | ARITY == 2 (Leemans 2013) |
| Receipt creation | Receipted evidence | Provenance | FATAL | Commit hash, author, timestamp, witness validation |
| WitnessGenRule (ggen) | WitnessMarker ontology | Code generation | N/A | Generate `witnesses.rs` module |
| CompileFailGenRule (ggen) | CompileFailLaw ontology | Fixture generation | FATAL | Generate .rs and .stderr files |
| AllLawsHaveFixturesGate | ALIVE certification | Audit gate | FATAL | Every law must have fixture |
| NoSilentLossInProjectionsGate | Projection creation | Audit gate | FATAL | Every projection must carry LossReport |
| HA_DeclaredPipelineNotReal | OCEL event log | Hostile assumption | MAJOR | Compare declared model against discovered process |

---

## References

1. **SHACL Specification:** https://www.w3.org/TR/shacl/
2. **SPARQL 1.1 Query Language:** https://www.w3.org/TR/sparql11-query/
3. **Leemans, S. J. J. (2013).** "Robust Process Mining with Guarantees" — Loop operator arity invariant
4. **van der Aalst, W. M. P. (1998).** "The Application of Petri Nets to Workflow Management" — WF-net soundness
5. **Kourani, F. & van Zelst, S. J. (2023).** "Partially Ordered Workflow Language (POWL)" — Block structure preservation
6. **Russell, N., van der Aalst, W. M. P., & ter Hofstede, A. H. M. (2016).** "Workflow Patterns: The Definitive Guide" — 17 of 20 patterns covered

---

**End of RDF Hook Activation Rules**
