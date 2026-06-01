# RDF Hook Dependency Chains and Invocation Order

**Generated:** 2026-06-01  
**Scope:** wasm4pm-compat project audit machinery ontology  
**Sources:** ggen/ontology/audit-machinery.ttl, domain-process-forms.ttl, wasm4pm-compat.ttl

---

## Executive Summary

This document extracts hook dependency chains, execution DAGs, and ordering constraints from the RDF ontology in the wasm4pm-compat project. The project uses a **declarative audit specification model** where:

- **Hooks** are represented as `AuditSpec` (specifications), `AuditTemplate` (implementation patterns), and `AuditExecutable` (concrete audit instances)
- **Dependency relations** are expressed via RDF properties: `applicableTemplate`, `instantiatedBy`, `relatedLaw`, `supportsGapClosure`, `closesGap`
- **Execution order** is imposed by state machines (evidence lifecycle), proof gates (ALIVE certification), and closure claims (gap remediation dependencies)
- **Mutual exclusion** and lock patterns are implicit in the type-law covenant and the one-way-door evidence lifecycle

---

## 1. Hook Properties and Dependency Relations

### 1.1 Core Hook Properties

| Property | Domain | Range | Semantics |
|----------|--------|-------|-----------|
| `audit:applicableTemplate` | `AuditSpec` | `AuditTemplate` | Links audit specification to its implementation template(s) |
| `audit:instantiatedBy` | `AuditExecutable` | `AuditTemplate` | Reverse: binds a concrete executable to its template |
| `audit:relatedLaw` | `Gap` | `CompileFailLaw`, `GraduationBoundary`, `AuditSpec` | Names the law that a gap affects |
| `audit:supportsGapClosure` | `AuxiliaryCommit` | `ClosureClaim` | An auxiliary commit advances (without directly closing) a gap |
| `audit:closesGap` | `ClosureClaim` | `Gap` | A closure claim asserts that a specific gap is closed |
| `audit:carriesReceipt` | `CommitEvidence` | `compat:Receipt` | A commit carries manufacturing receipts proving audit passage |
| `audit:closingCommit` | `ClosureClaim` | `xsd:string` (commit hash) | The commit that closes a gap |
| `audit:parentCommit` | `CommitEvidence` | `xsd:string` (commit hash) | Git parent commit(s); enables DAG traversal |

### 1.2 Precondition and State Properties

| Property | Domain | Semantics |
|----------|--------|-----------|
| `audit:gateCondition` | `AuditGate` | SPARQL ASK or logical condition; gate passes if true |
| `audit:gateSeverity` | `AuditGate` | Failure severity: fatal, error, warning, info |
| `audit:stateRequired` | `AuditSpec` | Evidence state(s) under audit (Admitted, Projected, Receipted) |
| `audit:witnessRequired` | `AuditSpec` | Witness marker(s) that must be present |
| `audit:verdictType` | `AuditSpec` | Type of verdict: boolean, metric, enum, evidence |

### 1.3 Constraint and Exception Properties

| Property | Domain | Semantics |
|----------|--------|-----------|
| `audit:forbiddenBy` | `ForbiddenPattern` | Links pattern to compile-fail law that proves it forbidden |
| `audit:allowedIn` | `ForbiddenPattern` | Named exception context where the pattern is permitted |
| `audit:contextScope` | `AllowedContext` | Scope of exception: module, feature, witness, phase |
| `audit:exceptionReason` | `AllowedContext` | Why the forbidden pattern is permitted here |

---

## 2. Hook Execution DAG (Directed Acyclic Graph)

### 2.1 ALIVE Certification Gate Ordering

The ALIVE gate is the top-level proof gate. It depends on these sub-gates in order:

```
┌─────────────────────────────────────────────────────────────┐
│                  ALIVE CERTIFICATION GATE                   │
├─────────────────────────────────────────────────────────────┤
│ Condition: All three gates below must pass (AND conjunction) │
│ Severity: fatal (blocks release if any gate fails)           │
└─────────────────────────────────────────────────────────────┘
         │
         ├─────────────────┬───────────────────┬──────────────────┐
         ▼                 ▼                   ▼                  ▼
    ┌─────────┐      ┌──────────┐        ┌──────────┐        ┌─────────────┐
    │ Gate 1  │      │ Gate 2   │        │ Gate 3   │        │ Gate 4      │
    └─────────┘      └──────────┘        └──────────┘        └─────────────┘
    AllLaws        AllFixtures        AllGapsClosedOr    AllReceiptsValidate
    HaveFixtures   HaveStderr         Accepted
    (fatal)        (fatal)            (fatal)            (fatal)

         ├─────────────────────────────────┬───────────────────┐
         ▼                                 ▼                   ▼
    ┌──────────────┐               ┌────────────────┐    ┌──────────────┐
    │ Gate 5       │               │ Gate 6         │    │ Gate 7       │
    └──────────────┘               └────────────────┘    └──────────────┘
    AllCommitMessages          NoSilentLossIn      (Additional gates as needed)
    Conventional               Projections
    (error)                    (fatal)
```

### 2.2 Gate Dependency Definitions (RDF)

```ttl
audit:AllLawsHaveFixturesGate
    a audit:AuditGate ;
    audit:gateName "all-laws-have-fixtures" ;
    audit:gateSeverity "fatal" ;
    # Precondition: compat:CompileFailLaw instances must exist
    # Condition: SPARQL ASK checking compat:fixtureFile is non-empty for all laws
    # Blocks: AllFixturesHaveStderrGate (if this fails, stderr check is moot)
    .

audit:AllFixturesHaveStderrGate
    a audit:AuditGate ;
    audit:gateName "all-fixtures-have-expected-stderr" ;
    audit:gateSeverity "fatal" ;
    # Precondition: AllLawsHaveFixturesGate must pass
    # Condition: SPARQL ASK checking compat:stderrFile for all fixtures
    # Blocks: AllGapsClosedOrAcceptedGate
    .

audit:AllGapsClosedOrAcceptedGate
    a audit:AuditGate ;
    audit:gateName "all-gaps-closed-or-accepted" ;
    audit:gateSeverity "fatal" ;
    # Precondition: AllFixturesHaveStderrGate must pass
    # Condition: SPARQL ASK checking all audit:Gap instances have status closed or accepted
    # Blocks: AllReceiptsValidateGate
    .

audit:AllReceiptsValidateGate
    a audit:AuditGate ;
    audit:gateName "all-receipts-validate" ;
    audit:gateSeverity "fatal" ;
    # Precondition: AllGapsClosedOrAcceptedGate must pass
    # Condition: All compat:Receipt instances carry valid provenance
    .

audit:AllCommitMessagesConventionalGate
    a audit:AuditGate ;
    audit:gateName "all-commits-conventional-format" ;
    audit:gateSeverity "error" ;
    # Independent gate (can run in parallel with others)
    # Condition: All audit:CommitEvidence instances follow conventional commit format
    .

audit:NoSilentLossInProjectionsGate
    a audit:AuditGate ;
    audit:gateName "no-silent-loss-in-projections" ;
    audit:gateSeverity "fatal" ;
    # Precondition: AllReceiptsValidateGate must pass (receipts validate before loss check)
    # Condition: Every Evidence<T, Projected, W> carries a LossReport
    .
```

### 2.3 Execution Ordering Constraints (RDF Representation)

```ttl
# Define ordering as rdf:first / rdf:rest chains (RDF List pattern)

audit:AliveCertificationSequence
    a rdf:List ;
    rdf:first audit:AllLawsHaveFixturesGate ;
    rdf:rest [
        rdf:first audit:AllFixturesHaveStderrGate ;
        rdf:rest [
            rdf:first audit:AllGapsClosedOrAcceptedGate ;
            rdf:rest [
                rdf:first audit:AllReceiptsValidateGate ;
                rdf:rest [
                    rdf:first audit:NoSilentLossInProjectionsGate ;
                    rdf:rest rdf:nil
                ]
            ]
        ]
    ] .

# Independent sequence (can interleave with ordered sequence)
audit:ParallelConstraintSequence
    a rdf:List ;
    rdf:first audit:AllCommitMessagesConventionalGate ;
    rdf:rest rdf:nil .
```

---

## 3. Evidence Lifecycle Hooks and State Transitions

The **one-way-door** lifecycle enforces hook ordering through Rust's type system, but the semantic ordering is captured in RDF:

### 3.1 Evidence State Machine

```
Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ {Projected | Exportable | Receipted}
  │                                  ▲
  └────────────── refuse ────────────┴──▶ Refused  (terminal; carries named law)
```

Each transition is guarded by hooks (implicit in type-law enforcement):

```ttl
# Hook 1: Raw to Parsed (parse hook)
compat:ParseTransition
    a compat:EvidenceTransition ;
    compat:fromState compat:Raw ;
    compat:toState compat:Parsed ;
    compat:transitionHook "parse" ;
    # No type-law gates; parsing is structural only
    .

# Hook 2: Parsed to Admitted (admit hook) — THE CRITICAL GATE
compat:AdmitTransition
    a compat:EvidenceTransition ;
    compat:fromState compat:Parsed ;
    compat:toState compat:Admitted ;
    compat:transitionHook "admit" ;
    audit:applicableTemplate audit:TryBuildCompilePassTemplate ;
    # PRECONDITION: Evidence must be Parsed (enforced at type level)
    # GATE: Admit trait impl must succeed or fail with specific Refusal<R, W>
    # ENFORCEMENT: Only Admit::admit() can produce Evidence<T, Admitted, W>
    .

# Hook 3: Admitted to Projected (project hook) — LOSS ACCOUNTING GATE
compat:ProjectTransition
    a compat:EvidenceTransition ;
    compat:fromState compat:Admitted ;
    compat:toState compat:Projected ;
    compat:transitionHook "project" ;
    audit:applicableTemplate audit:RuntimeLossAuditTemplate ;
    # PRECONDITION: Evidence must be Admitted
    # GATE: Must carry LossPolicy (RefuseLoss | AllowNamedProjection | AllowLossWithReport)
    # GATE: Must emit LossReport<From, To, Items> for all non-refusing paths
    # ENFORCEMENT: Type-enforced via Project trait and LossReport carrier
    .

# Hook 4: Admitted to Exportable (export hook) — FORMAT INTEROP GATE
compat:ExportTransition
    a compat:EvidenceTransition ;
    compat:fromState compat:Admitted ;
    compat:toState compat:Exportable ;
    compat:transitionHook "export" ;
    # PRECONDITION: Evidence must be Admitted
    # GATE: Format interop (OCEL → XES requires ProjectionName and LossPolicy)
    # ENFORCEMENT: No direct external→external route; must go through Admitted
    .

# Hook 5: Admitted to Receipted (receipt hook) — PROVENANCE MINTING GATE
compat:ReceiptTransition
    a compat:EvidenceTransition ;
    compat:fromState compat:Admitted ;
    compat:toState compat:Receipted ;
    compat:transitionHook "receipt" ;
    audit:applicableTemplate audit:OcelConformanceTemplate ;
    # PRECONDITION: Evidence must be Admitted (never Projected or Refused)
    # GATE: Receipt must carry non-forgeable witness path
    # GATE: Commit hash, author, timestamp must be consistent with CommitEvidence
    # GRADUATION: Receipt minting graduates to wasm4pm
    .

# Refused state (terminal, cannot transition)
compat:RefusedState
    a compat:EvidenceState ;
    compat:isTerminal true ;
    # PROPERTIES: Refusal<R, W> carries a specific named law R
    # NO OUTBOUND TRANSITIONS
    .
```

### 3.2 Admission Refusal Ordering

Every refusal must name a specific law. The refusal hierarchy (ordered by severity):

```ttl
# Tier 1: Structural laws (compile-fail enforcement)
compat:DanglingEventObjectLink
    a compat:NamedRefusalLaw ;
    rdfs:label "Dangling Event-Object Link" ;
    compat:severity "fatal" ;
    compat:precedent "OCEL structural law: every event-to-object link must reference a declared object" ;
    .

compat:MissingFinalMarking
    a compat:NamedRefusalLaw ;
    rdfs:label "Missing Final Marking" ;
    compat:severity "fatal" ;
    compat:precedent "WF-net law: sink place must have an initial marking" ;
    .

compat:IllegalNodeNesting
    a compat:NamedRefusalLaw ;
    rdfs:label "Illegal Process Tree Node Nesting" ;
    compat:severity "fatal" ;
    compat:precedent "Block-structure law: loop operator must have exactly 2 children" ;
    .

# Tier 2: Witness discrimination laws
compat:WitnessDiscriminationViolation
    a compat:NamedRefusalLaw ;
    rdfs:label "Witness Discrimination Violation" ;
    compat:severity "error" ;
    compat:precedent "Type-law: Admission<T, W1> ≠ Admission<T, W2>" ;
    .

# Bare "InvalidInput" is FORBIDDEN
compat:BareInvalidInputRefusal
    a audit:ForbiddenPattern ;
    audit:patternName "bare-invalid-input-refusal" ;
    audit:patternDescription "Refusal<InvalidInput, W> must never be instantiated" ;
    audit:forbiddenBy compat:BareInvalidInputRefusalLaw ;
    .
```

---

## 4. Gap Closure Dependency Chains

### 4.1 Gap Class Hierarchy

```ttl
# Gap types with their preconditions

audit:Gap001_UnimplementedConformanceAudit
    a audit:Gap ;
    audit:gapId "GAP_001" ;
    audit:gapCategory "missing-fixture" ;
    audit:gapSeverity "major" ;
    audit:relatedLaw compat:ConformanceAuthority ;
    # PRECONDITION: ConformanceAuthority must exist in domain ontology
    # CLOSURE PATH: Implement OCEL-based audit deriving event logs of wasm4pm conformance execution
    # BLOCKER: Requires wasm4pm integration (external crate)
    .

audit:Gap002_UncoveredProcessTreeConstruction
    a audit:Gap ;
    audit:gapId "GAP_002" ;
    audit:gapCategory "uncovered-law" ;
    audit:gapSeverity "minor" ;
    audit:relatedLaw compat:ProcessTree ;
    # PRECONDITION: ProcessTree must be defined (it is)
    # CLOSURE PATH: Add compile-fail fixtures proving block-structure invariants
    # DEPENDENCY: Generic_const_exprs feature may need extension
    .

audit:Gap003_UnverifiedGraduationPaths
    a audit:Gap ;
    audit:gapId "GAP_003" ;
    audit:gapCategory "unresolved-todo" ;
    audit:gapSeverity "critical" ;
    audit:relatedLaw compat:GraduationCandidate ;
    # PRECONDITION: GraduationBoundary instances must map to real wasm4pm entry points
    # CLOSURE PATH: Cross-crate audit verifying each boundary has receipt-bearing execution
    # BLOCKER: Requires verified integration with wasm4pm
    # BLOCKS: Any release or ALIVE checkpoint
    .
```

### 4.2 Closure Claim Dependency Graph

```ttl
# Example: GAP_001 partial closure

audit:ClosureClaim_Gap001_Partial
    a audit:ClosureClaim ;
    audit:closesGap audit:Gap001_UnimplementedConformanceAudit ;
    audit:closingCommit "2426fac" ;
    audit:closureMethod "fixture-added" ;
    # PRECONDITION: commit 2426fac must exist and be reachable
    # PRECONDITION: commit must carry audit:CommitEvidence
    # PRECONDITION: CommitEvidence must link to compat:Receipt instances
    # BLOCKS: Further releases (major severity gap still open)
    # SUPPORTS: AllGapsClosedOrAcceptedGate (partial satisfaction)
    .

# Auxiliary commit supporting closure

audit:AuxiliaryCommit_Docs_Gap001
    a audit:AuxiliaryCommit ;
    audit:supportsGapClosure audit:ClosureClaim_Gap001_Partial ;
    audit:advancesAuditReadiness "Documentation describing OCEL conformance audit structure, preconditions for wasm4pm integration" ;
    # PRECONDITION: None (auxiliary commits have no blocking preconditions)
    # EFFECT: Advances readiness toward closure without closing
    # INDEPENDENCE: Can be applied in parallel with other auxiliary commits
    .
```

### 4.3 Closure Ordering as RDF List

```ttl
# Mandatory closure order (for ALIVE gate to pass)

audit:ClosureSequence
    a rdf:List ;
    rdf:first audit:ClosureClaim_Gap002 ;  # Minor gap; nice-to-have
    rdf:rest [
        rdf:first audit:ClosureClaim_Gap001 ;  # Major gap; must close before release
        rdf:rest [
            rdf:first audit:ClosureClaim_Gap003 ;  # Critical gap; blocks ALIVE
            rdf:rest rdf:nil
        ]
    ] .
```

---

## 5. Mutual Exclusion and Lock Patterns

### 5.1 Type-Law Mutual Exclusion (State Tags)

The Rust type system enforces mutual exclusion via the state parameter in `Evidence<T, State, W>`:

```ttl
# These are mutually exclusive at the type level (different types)

compat:RawAndAdmittedMutualExclusion
    a rdf:Statement ;
    rdf:subject compat:Evidence_Raw ;
    rdf:predicate compat:mutuallyExcludesWith ;
    rdf:object compat:Evidence_Admitted ;
    rdfs:comment "Evidence<T, Raw, W> and Evidence<T, Admitted, W> are different types. A value cannot be both Raw and Admitted simultaneously. The one-way-door law prevents regression." ;
    .

compat:ProjectedAndReceiptedMutualExclusion
    a rdf:Statement ;
    rdf:subject compat:Evidence_Projected ;
    rdf:predicate compat:mutuallyExclusesWith ;
    rdf:object compat:Evidence_Receipted ;
    rdfs:comment "Evidence<T, Projected, W> and Evidence<T, Receipted, W> are different types. A projected value (lossy) cannot simultaneously carry a receipt (provenance), as receipts apply only to Admitted or Receipted states." ;
    .

# Refusal is terminal (no outbound transitions)

compat:RefusedIsTerminal
    a rdf:Statement ;
    rdf:subject compat:Evidence_Refused ;
    rdf:predicate compat:mutuallyExcludesWith ;
    rdf:object [
        rdf:type rdf:List ;
        rdf:first compat:Evidence_Parsed ;
        rdf:rest [
            rdf:first compat:Evidence_Admitted ;
            rdf:rest [
                rdf:first compat:Evidence_Projected ;
                rdf:rest [
                    rdf:first compat:Evidence_Exportable ;
                    rdf:rest [
                        rdf:first compat:Evidence_Receipted ;
                        rdf:rest rdf:nil
                    ]
                ]
            ]
        ]
    ] ;
    rdfs:comment "Refused evidence is terminal. No state transition out of Refused is possible." ;
    .
```

### 5.2 Witness Discrimination Lock

```ttl
# Witness markers prevent cross-witness contamination

compat:WitnessDiscriminationLock
    a rdf:Statement ;
    rdf:subject compat:Admission_Ocel20 ;
    rdf:predicate compat:cannotBeUsedAs ;
    rdf:object compat:Admission_Xes1849 ;
    rdfs:comment "Admission<T, Ocel20> and Admission<T, Xes1849> are different types. A value admitted against the OCEL 2.0 standard cannot be used as a value admitted against XES 1849-2016." ;
    # Lock enforced at compile time via Rust type system
    # No runtime check needed; attempted coercion is a type error
    .
```

### 5.3 Loss Policy Lock (Lossy Transformation Barrier)

```ttl
# Loss accounting requires explicit decision before transformation

compat:LossPolicyLock
    a rdf:Statement ;
    rdf:subject compat:Evidence_Admitted ;
    rdf:predicate compat:cannotProjectWithout ;
    rdf:object compat:LossPolicy ;
    rdfs:comment "An Evidence<T, Admitted, W> value cannot be projected (transitioned to Projected) without an explicit LossPolicy decision: RefuseLoss, AllowNamedProjection, or AllowLossWithReport. This lock prevents silent loss." ;
    # Enforcement: Project trait requires LossPolicy parameter
    # Enforcement: No implicit coercion from Admitted to Projected exists
    .

# Loss report requirement (runtime enforcement)

audit:NoSilentLossInProjectionsGate
    a audit:AuditGate ;
    audit:gateName "no-silent-loss-in-projections" ;
    rdfs:comment "Runtime enforcement: every Evidence<T, Projected, W> must carry exactly one LossReport<From, To, Items>. This is the audit gate that validates the loss lock was honored." ;
    .
```

---

## 6. Hook Invocation Order Summary (SPARQL-Compatible View)

### 6.1 Sequential Ordering (SPARQL ASK Pattern)

```sparql
# Query: Are all audit gates ordered correctly?

PREFIX audit: <https://wasm4pm-compat.rs/audit#>
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>

ASK WHERE {
    # Gate 1 must fire before Gate 2
    audit:AllLawsHaveFixturesGate ?p1 ?o1 .
    audit:AllFixturesHaveStderrGate ?p2 ?o2 .
    
    # Implicit ordering: if Gate 1 fails (severity fatal), Gate 2 is unreachable
    audit:AllLawsHaveFixturesGate audit:gateSeverity "fatal" .
    audit:AllFixturesHaveStderrGate audit:gateSeverity "fatal" .
    
    # Verify precedence chain
    FILTER NOT EXISTS {
        # No gate can execute if its predecessor failed
        # (This is semantic, not syntactic — enforced by audit harness)
    }
}
```

### 6.2 Dependency Extraction (SPARQL CONSTRUCT Pattern)

```sparql
# Construct hook dependency relations

PREFIX audit: <https://wasm4pm-compat.rs/audit#>
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

CONSTRUCT {
    ?spec audit:dependsOn ?template .
    ?template audit:implementedBy ?executable .
    ?gap audit:blockedBy ?relatedLaw .
    ?closure audit:requiresCommit ?commit .
}
WHERE {
    # Spec depends on its applicable template
    ?spec audit:applicableTemplate ?template .
    
    # Template is instantiated as executable
    ?executable audit:instantiatedBy ?template .
    
    # Gap is blocked by related law
    ?gap audit:relatedLaw ?relatedLaw .
    
    # Closure claim requires a specific commit
    ?closure audit:closesGap ?gap .
    ?closure audit:closingCommit ?commit .
}
```

---

## 7. RDF DAG Representation

### 7.1 Hook Dependency DAG as N-Triples

```nt
<https://wasm4pm-compat.rs/audit#AllLawsHaveFixturesGate> <https://wasm4pm-compat.rs/audit#blocksGate> <https://wasm4pm-compat.rs/audit#AllFixturesHaveStderrGate> .

<https://wasm4pm-compat.rs/audit#AllFixturesHaveStderrGate> <https://wasm4pm-compat.rs/audit#blocksGate> <https://wasm4pm-compat.rs/audit#AllGapsClosedOrAcceptedGate> .

<https://wasm4pm-compat.rs/audit#AllGapsClosedOrAcceptedGate> <https://wasm4pm-compat.rs/audit#blocksGate> <https://wasm4pm-compat.rs/audit#AllReceiptsValidateGate> .

<https://wasm4pm-compat.rs/audit#AllReceiptsValidateGate> <https://wasm4pm-compat.rs/audit#blocksGate> <https://wasm4pm-compat.rs/audit#NoSilentLossInProjectionsGate> .

<https://wasm4pm-compat.rs/audit#AllCommitMessagesConventionalGate> <https://wasm4pm-compat.rs/audit#isParallelWith> <https://wasm4pm-compat.rs/audit#AllLawsHaveFixturesGate> .

<https://wasm4pm-compat.rs/audit#Gap003_UnverifiedGraduationPaths> <https://wasm4pm-compat.rs/audit#blocksALIVERelease> <https://wasm4pm-compat.rs/audit#CheckpointCrownAlive004> .
```

---

## 8. Hook Preconditions and Effect Summary Table

| Hook | Type | Preconditions | Effects | Dependencies |
|------|------|---|---|---|
| **Parse** | Transition | Raw evidence exists | Evidence becomes Parsed | None (root) |
| **Admit** | Transition | Evidence is Parsed | Evidence becomes Admitted or Refused | TryBuildCompilePassTemplate |
| **Project** | Transition | Evidence is Admitted | Evidence becomes Projected with LossReport | RuntimeLossAuditTemplate |
| **Export** | Transition | Evidence is Admitted | Evidence becomes Exportable in target format | Format-specific LossPolicy |
| **Receipt** | Transition | Evidence is Admitted | Evidence becomes Receipted with provenance | OcelConformanceTemplate |
| **AllLawsHaveFixtures** | Gate | CompileFailLaw instances exist | Audit can proceed to fixture validation | None (root gate) |
| **AllFixturesHaveStderr** | Gate | AllLawsHaveFixtures passes | Audit can proceed to gap validation | AllLawsHaveFixtures |
| **AllGapsClosedOrAccepted** | Gate | AllFixturesHaveStderr passes | ALIVE can proceed to receipt validation | AllFixturesHaveStderr |
| **AllReceiptsValidate** | Gate | AllGapsClosedOrAccepted passes | ALIVE can proceed to loss validation | AllGapsClosedOrAccepted |
| **NoSilentLossInProjections** | Gate | AllReceiptsValidate passes | ALIVE gate is sealed (all checks pass) | AllReceiptsValidate |
| **AllCommitMessagesConventional** | Gate | CommitEvidence instances exist | Commit history is auditable | None (parallel) |

---

## 9. Conclusion

The wasm4pm-compat audit machinery is **declaratively defined in RDF** with these key features:

1. **Hook Properties**: `applicableTemplate`, `instantiatedBy`, `relatedLaw`, `supportsGapClosure`, `closesGap`, `carriesReceipt`
2. **Execution DAG**: Linear sequence of ALIVE gates (fatal failures block release); parallel non-fatal gates
3. **Ordering Constraints**: Type-enforced one-way-door lifecycle; witness discrimination; loss policy locks
4. **Mutual Exclusion**: Evidence state tags, witness markers, loss policy requirements all prevent simultaneous incompatible states
5. **Preconditions**: Each gate checks existence and consistency of preceding artifacts (laws, fixtures, gaps, receipts)
6. **RDF Representation**: N-Triples, Turtle, SPARQL ASK/CONSTRUCT patterns enable machine-readable audit definition

The entire audit machinery is **structure-only** in the compat crate; execution and verdict logic graduate to wasm4pm.

---

**Document End**
