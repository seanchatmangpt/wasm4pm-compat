# RDF Hook Properties Reference

**Document version**: 1.0  
**Generated from**: wasm4pm-compat RDF ontologies (audit-machinery.ttl, wasm4pm-compat.ttl, domain-evidence-structure.ttl, domain-type-constraints.ttl)  
**Scope**: Complete semantic mapping of RDF hook properties used in the wasm4pm-compat nightly type-law crate.

---

## Overview

This reference maps all RDF hook properties and their semantic meaning across the wasm4pm-compat ontology ecosystem. Each property is indexed by:

1. **Domain/Range Classes** — what subjects and objects it connects
2. **Formal Definitions** — the RDF/OWL definitions from ontologies
3. **Usage Patterns** — how the property is used in audit and evidence lifecycles
4. **Cardinality Constraints** — whether it's functional, inverse-functional, multi-valued
5. **Inverse/Related Properties** — linked properties forming semantic networks

The document is organized by **property family** (audit spec properties, evidence lifecycle properties, witness properties, etc.) for ease of navigation.

---

## Property Families

### 1. AUDIT SPECIFICATION PROPERTIES

These properties describe audit specifications (AuditSpec class instances) that declare named audit contracts.

#### `audit:specName`

**Domain**: `audit:AuditSpec`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The stable machine name of an AuditSpec, used as a unique identifier.

**Examples**:
- `"witness-discrimination-law"`
- `"raw-evidence-exported-as-admitted"`
- `"receipt-consistency-audit"`

**Related Properties**:
- Inverse of: none (data property)
- Similar to: `audit:lawName`, `audit:templateName`, `audit:gateName`

**Usage Pattern**:
```sparql
?spec a audit:AuditSpec ;
       audit:specName "witness-discrimination-law" .
```

---

#### `audit:lawName`

**Domain**: `audit:AuditSpec`, `audit:AuditGate`, `audit:ForbiddenPattern`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The human-readable name of the law this audit enforces. Corresponds to `compat:CompileFailLaw` instances.

**Examples**:
- `"WitnessDiscrimination"` (witness tags are sealed)
- `"RawEvidenceExportedAsAdmitted"` (one-way door)
- `"SilentStructureLoss"` (all projections must report)
- `"ReceiptProvenance"` (receipts carry non-forgeable provenance)

**Related Properties**:
- Links to: `compat:lawName` in the type-law ontology
- Cross-references: `compat:CompileFailLaw` instances

**Usage Pattern**:
```sparql
?spec audit:lawName "RawEvidenceExportedAsAdmitted" .
?law a compat:CompileFailLaw ;
      compat:lawName "RawEvidenceExportedAsAdmitted" .
```

---

#### `audit:auditScope`

**Domain**: `audit:AuditSpec`  
**Range**: `xsd:string` (enum-like: "module", "crate", "feature", "cross-crate")  
**Cardinality**: Functional (1:1)  
**Definition**: The scope of audit authority. Defines whether the audit applies to a single module, entire crate, a cargo feature, or spans multiple crates.

**Allowed Values**:
- `"module"` — single src/*.rs file (e.g., src/admission.rs)
- `"crate"` — entire wasm4pm-compat crate
- `"feature"` — guarded by cargo feature flag (e.g., `formats`, `strict`, `wasm4pm`)
- `"cross-crate"` — multiple crates (wasm4pm-compat + wasm4pm + other integrations)

**Related Properties**:
- Paired with: `audit:scopeTarget` (specifies which module/feature/crate)
- Complements: `audit:carrierType` (what type is under audit)

**Usage Pattern**:
```sparql
?spec audit:auditScope "module" ;
       audit:scopeTarget "src/admission.rs" .
```

---

#### `audit:scopeTarget`

**Domain**: `audit:AuditSpec`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The specific module, feature name, or cross-crate reference targeted by the audit scope.

**Examples**:
- `"src/admission.rs"` (module scope)
- `"wasm4pm-compat"` (crate scope)
- `"formats"` (feature scope)
- `"wasm4pm-compat:src/witness.rs"` (cross-crate reference)

**Related Properties**:
- Paired with: `audit:auditScope` (type of scope)
- Links to: source module URIs in `compat:sourceFile`

**Usage Pattern**:
```sparql
?spec audit:auditScope "crate" ;
       audit:scopeTarget "wasm4pm-compat" .
```

---

#### `audit:carrierType`

**Domain**: `audit:AuditSpec`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The Rust type signature of the evidence carrier under audit. Fully qualified type string.

**Examples**:
- `"Evidence<T, Admitted, W>"` — admitted evidence
- `"Admission<T, W>"` — admission verdict only
- `"Refusal<R, W>"` — refusal verdict with named reason
- `"Evidence<T, Projected, W>"` — projected evidence with loss accounting
- `"Receipt"` — provenance-bearing receipt envelope
- `"LossReport<From, To, Items>"` — loss report

**Related Properties**:
- Links to: `audit:witnessRequired`, `audit:stateRequired` (constraints on the carrier)
- Cross-references: `compat:EvidenceState`, `compat:WitnessMarker`

**Usage Pattern**:
```sparql
?spec audit:carrierType "Evidence<T, Admitted, W>" ;
       audit:stateRequired compat:Admitted .
```

---

#### `audit:witnessRequired`

**Domain**: `audit:AuditSpec`  
**Range**: `compat:WitnessMarker`  
**Cardinality**: Multi-valued (0..*)  
**Definition**: The witness marker(s) that must be present in evidence for this audit to apply. If unspecified, audit applies to all witnesses.

**Examples**:
- `compat:Ocel20` (OCEL 2.0 standard witness)
- `compat:Xes1849` (XES IEEE 1849-2016 witness)
- `compat:WfNetSoundnessPaper` (WF-net soundness paper witness)
- `compat:MiningWitness` (wasm4pm mining authority bridge)

**Related Properties**:
- Range: instances of `compat:WitnessMarker`
- Inverse relation: witness markers "required-by" audit specs (implicit)
- Links to: `domain:evidenceWitness` (evidence-level witness tag)

**Usage Pattern**:
```sparql
?spec audit:witnessRequired compat:Ocel20 ;
       audit:witnessRequired compat:Xes1849 .
```

---

#### `audit:stateRequired`

**Domain**: `audit:AuditSpec`  
**Range**: `compat:EvidenceState`  
**Cardinality**: Multi-valued (0..*)  
**Definition**: The evidence state(s) under audit. Examples: Admitted, Projected, Receipted. If unspecified, audit applies to all legal states.

**Allowed Values** (7 canonical lifecycle stages):
- `compat:Raw` — untrusted input (entry stage)
- `compat:Parsed` — structurally parsed, not yet judged
- `compat:Admitted` — admitted across the boundary against a witness
- `compat:Refused` — terminal refusal with named reason
- `compat:Projected` — result of named lossy projection with LossReport
- `compat:Exportable` — cleared to leave the crate
- `compat:Receipted` — sealed in provenance-bearing receipt (terminal)

**Related Properties**:
- Range: instances of `compat:EvidenceState`
- Links to: `domain:evidenceState` (evidence-level state tag)
- Succession defined by: `compat:successorState` (legal state transitions)

**Usage Pattern**:
```sparql
?spec audit:stateRequired compat:Admitted ;
       audit:stateRequired compat:Projected .
```

---

#### `audit:verdictType`

**Domain**: `audit:AuditSpec`  
**Range**: `xsd:string` (enum: "boolean", "metric", "enum", "evidence")  
**Cardinality**: Functional (1:1)  
**Definition**: The type of verdict this audit spec produces.

**Allowed Values**:
- `"boolean"` — pass/fail verdict (True/False)
- `"metric"` — numeric score (e.g., fitness 0.87)
- `"enum"` — named verdict variant (e.g., Admit/Refuse/Uncertain)
- `"evidence"` — structured evidence bundle (e.g., ConformanceVerdict with multiple metrics)

**Related Properties**:
- Determines: verdict carrier type paired with `audit:carrierType`
- Links to: `audit:gateCondition` (how verdict is evaluated)

**Usage Pattern**:
```sparql
?spec audit:verdictType "enum" ;
       audit:carrierType "Refusal<R, W>" .
```

---

#### `audit:applicableTemplate`

**Domain**: `audit:AuditSpec`  
**Range**: `audit:AuditTemplate`  
**Cardinality**: Multi-valued (1..*)  
**Definition**: Links an AuditSpec to its AuditTemplate(s). One spec may have multiple templates covering different implementations or scope levels.

**Related Properties**:
- Inverse of: `audit:instantiatedBy` (from template perspective)
- Links to: `audit:AuditTemplate` instances

**Usage Pattern**:
```sparql
?spec audit:applicableTemplate ?template .
?template a audit:AuditTemplate ;
          audit:templateName "trybuild-compile-fail-fixture" .
```

---

### 2. AUDIT TEMPLATE PROPERTIES

Templates define implementation patterns for how audits are conducted.

#### `audit:templateName`

**Domain**: `audit:AuditTemplate`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The stable machine name of an AuditTemplate, used as a unique identifier.

**Examples**:
- `"trybuild-compile-fail-fixture"`
- `"trybuild-compile-pass-fixture"`
- `"runtime-loss-validation"`
- `"static-refusal-analysis"`
- `"ocel-event-log-conformance"`

**Related Properties**:
- Inverse: none (data property)
- Similar to: `audit:specName`, `audit:gateName`

**Usage Pattern**:
```sparql
?template a audit:AuditTemplate ;
          audit:templateName "trybuild-compile-fail-fixture" .
```

---

#### `audit:templateKind`

**Domain**: `audit:AuditTemplate`  
**Range**: `xsd:string` (enum-like)  
**Cardinality**: Functional (1:1)  
**Definition**: The audit implementation pattern category.

**Allowed Values**:
- `"compile-fail-fixture"` — trybuild fixture proving a law is unbreakable
- `"compile-pass-fixture"` — trybuild fixture proving a lawful path is open
- `"runtime-test"` — runtime unit or integration test
- `"static-analysis"` — code analysis (grep, clippy, cargo-audit)
- `"event-log-conformance"` — OCEL/process-mining based audit
- `"type-state-proof"` — Rust type system proof via typestate pattern

**Related Properties**:
- Describes implementation of: `audit:AuditTemplate`
- Determines: fixture file format, expected output validation

**Usage Pattern**:
```sparql
?template audit:templateKind "compile-fail-fixture" ;
          audit:fixtureFile "tests/ui/compile_fail/witness_discrimination.rs" .
```

---

#### `audit:instantiatedBy`

**Domain**: `audit:AuditExecutable`  
**Range**: `audit:AuditTemplate`  
**Cardinality**: Functional (1:1)  
**Definition**: Links an AuditExecutable to its parent AuditTemplate. The executable is the template with all parameters bound.

**Related Properties**:
- Inverse of: (no formal inverse; semantic inverse is "instantiates")
- Paired with: `audit:executableId` (unique identifier of the instantiation)

**Usage Pattern**:
```sparql
?executable a audit:AuditExecutable ;
            audit:instantiatedBy ?template ;
            audit:executableId "compile-fail-witness-discrimination-ocel-20" .
```

---

### 3. AUDIT EXECUTABLE PROPERTIES

Executables are fully instantiated audits ready to run.

#### `audit:executableId`

**Domain**: `audit:AuditExecutable`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The stable unique identifier for this AuditExecutable. Combines template name with witness/state binding.

**Examples**:
- `"compile-fail-witness-discrimination-ocel-20"`
- `"runtime-admission-boundary-xes"`
- `"loss-audit-ocel2xes-projection"`

**Related Properties**:
- Links to: `audit:instantiatedBy` (parent template)
- Paired with: `audit:fixtureFile` (concrete fixture path)

**Usage Pattern**:
```sparql
?exec a audit:AuditExecutable ;
       audit:executableId "compile-fail-witness-discrimination-ocel-20" ;
       audit:fixtureFile "tests/ui/compile_fail/witness_discrimination.rs" .
```

---

#### `audit:fixtureFile`

**Domain**: `audit:AuditExecutable`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: Path to a test fixture file (trybuild .rs file, pytest conftest, runtime test module) relative to the crate root.

**Examples**:
- `"tests/ui/compile_fail/witness_discrimination.rs"`
- `"tests/ui/compile_pass/typestate_admission_boundary.rs"`
- `"tests/runtime/loss_accounting.rs"`

**Related Properties**:
- Paired with: `audit:expectedStderr` (for compile-fail fixtures)
- Links to: source files in the repository

**Usage Pattern**:
```sparql
?exec audit:fixtureFile "tests/ui/compile_fail/raw_evidence_exported.rs" ;
       audit:expectedStderr "tests/ui/compile_fail/raw_evidence_exported.stderr" .
```

---

#### `audit:expectedStderr`

**Domain**: `audit:AuditExecutable`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: Path to the .stderr file containing expected compiler diagnostic output for compile-fail fixtures. Relative to crate root.

**Usage Pattern**:
```sparql
?exec audit:templateKind "compile-fail-fixture" ;
       audit:fixtureFile "tests/ui/compile_fail/between01_violation.rs" ;
       audit:expectedStderr "tests/ui/compile_fail/between01_violation.stderr" .
```

---

### 4. AUDIT GATE PROPERTIES

Proof gates validate that audits pass certain conditions.

#### `audit:gateName`

**Domain**: `audit:AuditGate`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The stable machine name of an AuditGate.

**Examples**:
- `"all-laws-have-fixtures"` — every law has a test backing it
- `"all-fixtures-have-expected-stderr"` — every compile-fail fixture has .stderr
- `"all-gaps-closed-or-accepted"` — no open critical/major gaps
- `"all-receipts-validate"` — receipt provenance is consistent
- `"no-silent-loss-in-projections"` — every projection carries LossReport

**Related Properties**:
- Similar to: `audit:specName`, `audit:templateName`

---

#### `audit:gateCondition`

**Domain**: `audit:AuditGate`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: A SPARQL ASK query or other logical condition that the gate evaluates. Returns boolean.

**Examples**:
```sparql
# Query: all laws have fixtures
ASK {
  ?law a compat:CompileFailLaw .
  FILTER NOT EXISTS { ?law compat:fixtureFile ?file . }
}
```

**Related Properties**:
- Used by: audit execution engines to determine pass/fail
- Links to: verdict RDF graphs to evaluate conditions

---

#### `audit:gateSeverity`

**Domain**: `audit:AuditGate`  
**Range**: `xsd:string` (enum: "fatal", "error", "warning", "info")  
**Cardinality**: Functional (1:1)  
**Definition**: Gate failure severity level.

**Allowed Values**:
- `"fatal"` — audit fails, block release
- `"error"` — audit fails, must remediate before next release
- `"warning"` — log and continue
- `"info"` — informational only

**Related Properties**:
- Determines: release blocking behavior
- Paired with: `audit:gateDescription` (human explanation)

---

### 5. FORBIDDEN PATTERN PROPERTIES

#### `audit:patternName`

**Domain**: `audit:ForbiddenPattern`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The stable machine name of a ForbiddenPattern.

**Examples**:
- `"raw-evidence-exported-as-admitted"`
- `"witness-discrimination-violation"`
- `"silent-structure-loss"`
- `"condition-cell-overflow"`

---

#### `audit:forbiddenBy`

**Domain**: `audit:ForbiddenPattern`  
**Range**: `compat:CompileFailLaw`  
**Cardinality**: Multi-valued (1..*)  
**Definition**: Links a ForbiddenPattern to the compile-fail laws or fixtures that prove it forbidden.

**Related Properties**:
- Range: `compat:CompileFailLaw` instances
- Links to: proof gates that enforce the pattern

**Usage Pattern**:
```sparql
?pattern a audit:ForbiddenPattern ;
         audit:patternName "raw-evidence-exported-as-admitted" ;
         audit:forbiddenBy compat:RawExportedAsAdmittedLaw .
```

---

#### `audit:allowedIn`

**Domain**: `audit:ForbiddenPattern`  
**Range**: `audit:AllowedContext`  
**Cardinality**: Multi-valued (0..*)  
**Definition**: Links a ForbiddenPattern to AllowedContext instances where it is permitted under a named exception.

**Examples**:
- Pattern: `"silent-structure-loss"`  
  AllowedContext: `"named-projection"` (loss allowed only in src/loss.rs with LossReport)
- Pattern: `"soundness-forged"`  
  AllowedContext: `"wasm4pm-graduation"` (only wasm4pm can produce SoundnessWitnessed)

**Related Properties**:
- Multi-valued: one pattern may have multiple exceptions
- Conditional: exceptions apply only in their specific scope

---

### 6. ALLOWED CONTEXT PROPERTIES

#### `audit:contextName`

**Domain**: `audit:AllowedContext`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The stable machine name of an AllowedContext.

**Examples**:
- `"wasm4pm-graduation"` — graduation to wasm4pm engine
- `"internal-bridge-assembly"` — bridge implementation layer
- `"named-projection"` — lossy transformation with LossReport

---

#### `audit:contextScope`

**Domain**: `audit:AllowedContext`  
**Range**: `xsd:string` (enum: "module", "feature", "witness", "phase")  
**Cardinality**: Functional (1:1)  
**Definition**: The scope(s) in which this exception applies.

**Allowed Values**:
- `"module"` — specific src/*.rs file (e.g., src/graduation.rs)
- `"feature"` — cargo feature gate (e.g., wasm4pm, formats)
- `"witness"` — specific witness marker (e.g., Wasm4pmBridge)
- `"phase"` — development phase (e.g., development, release)

---

#### `audit:exceptionReason`

**Domain**: `audit:AllowedContext`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: Human-readable reason why the forbidden pattern is permitted in this context. Must be explicit and documented.

**Examples**:
- "The wasm4pm engine is a trusted execution boundary with its own audit machinery."
- "Loss in src/loss.rs is conditional — allowed only with LossReport emitted."
- "Bridge assembly requires witness construction that would normally be forbidden."

---

### 7. GAP PROPERTIES

#### `audit:gapId`

**Domain**: `audit:Gap`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The stable unique identifier for a Gap, following pattern "GAP_NNN".

**Examples**:
- `"GAP_001_MISSING_CONFORMANCE_FIXTURE"`
- `"GAP_002_UNCOVERED_PROCESS_TREE_LAWS"`
- `"GAP_003_UNVERIFIED_GRADUATION_PATHS"`

---

#### `audit:gapCategory`

**Domain**: `audit:Gap`  
**Range**: `xsd:string` (enum)  
**Cardinality**: Functional (1:1)  
**Definition**: The gap category/type.

**Allowed Values**:
- `"missing-fixture"` — audit law without test fixture
- `"uncovered-law"` — law without compile-fail proof
- `"unreachable-path"` — code path without proof it is reachable
- `"unresolved-todo"` — documented TODO in code
- `"unverified-graduation"` — graduation boundary without verification

---

#### `audit:gapSeverity`

**Domain**: `audit:Gap`  
**Range**: `xsd:string` (enum)  
**Cardinality**: Functional (1:1)  
**Definition**: Gap severity level.

**Allowed Values**:
- `"critical"` — breaks ALIVE gate, blocks release
- `"major"` — must remediate before next release
- `"minor"` — nice-to-have
- `"accepted-debt"` — intentional, accepted risk

---

#### `audit:relatedLaw`

**Domain**: `audit:Gap`  
**Range**: `compat:CompileFailLaw`, `compat:GraduationBoundary`, `audit:AuditSpec`  
**Cardinality**: Multi-valued (1..*)  
**Definition**: Links a Gap to the law, compile-fail fixture, or graduation boundary that is gapped.

**Related Properties**:
- Range: three classes of audit entities
- Links to: core ALIVE requirements

---

#### `audit:remediationPlan`

**Domain**: `audit:Gap`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: Human-readable plan to close this gap. May reference specific commits, papers, or refactoring plans.

---

#### `audit:gapOpenedAt`, `audit:gapOpenedDate`

**Domain**: `audit:Gap`  
**Range**: `xsd:string` (hash), `xsd:dateTime`  
**Cardinality**: Functional (1:1) each  
**Definition**: Git commit hash and timestamp when this gap was identified or created.

---

### 8. COMMIT EVIDENCE PROPERTIES

#### `audit:commitHash`

**Domain**: `audit:CommitEvidence`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The full git commit hash (SHA-1 or SHA-256) of this CommitEvidence.

---

#### `audit:commitAuthor`

**Domain**: `audit:CommitEvidence`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The author email or name of the commit.

**Example**: `"Sean Chatman <xpointsh@gmail.com>"`

---

#### `audit:commitTimestamp`

**Domain**: `audit:CommitEvidence`  
**Range**: `xsd:dateTime`  
**Cardinality**: Functional (1:1)  
**Definition**: The commit timestamp in ISO 8601 format.

**Example**: `"2026-05-30T10:15:00Z"`

---

#### `audit:commitMessage`

**Domain**: `audit:CommitEvidence`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The commit message (first line and body). Must follow conventional commits format.

**Format**: `type(scope): description`  
**Example**: `"feat(nightly): implement zero-cost type-law surfaces in nightly_foundry"`

---

#### `audit:parentCommit`

**Domain**: `audit:CommitEvidence`  
**Range**: `xsd:string`  
**Cardinality**: Multi-valued (1..2)  
**Definition**: The parent commit hash(es). For merge commits, may have multiple parents.

---

#### `audit:carriesReceipt`

**Domain**: `audit:CommitEvidence`  
**Range**: `compat:Receipt`  
**Cardinality**: Multi-valued (0..*)  
**Definition**: Links a CommitEvidence to one or more Receipt instances proving audit passage or gap closure.

**Related Properties**:
- Range: `compat:Receipt` (provenance-bearing receipt envelopes)
- Links to: manufacturing receipts that prove audit success

---

### 9. CLOSURE CLAIM PROPERTIES

#### `audit:closesGap`

**Domain**: `audit:ClosureClaim`  
**Range**: `audit:Gap`  
**Cardinality**: Functional (1:1)  
**Definition**: Links a ClosureClaim to the Gap it claims to close.

---

#### `audit:closingCommit`

**Domain**: `audit:ClosureClaim`  
**Range**: `xsd:string`  
**Cardinality**: Multi-valued (1..*)  
**Definition**: The commit hash(es) that close (or begin to close) the gap. May reference multiple commits in a sequence.

---

#### `audit:closureMethod`

**Domain**: `audit:ClosureClaim`  
**Range**: `xsd:string` (enum)  
**Cardinality**: Functional (1:1)  
**Definition**: How the gap was closed.

**Allowed Values**:
- `"fixture-added"` — new trybuild fixture added
- `"law-proved"` — compile-fail or compile-pass proof added
- `"path-proved-reachable"` — code path proved reachable
- `"reclassified"` — gap reclassified as accepted-debt or out-of-scope

---

#### `audit:closureTimestamp`

**Domain**: `audit:ClosureClaim`  
**Range**: `xsd:dateTime`  
**Cardinality**: Functional (1:1)  
**Definition**: The timestamp when the gap was declared closed.

---

### 10. AUXILIARY COMMIT PROPERTIES

#### `audit:supportsGapClosure`

**Domain**: `audit:AuxiliaryCommit`  
**Range**: `audit:ClosureClaim`  
**Cardinality**: Multi-valued (0..*)  
**Definition**: Links an AuxiliaryCommit to one or more ClosureClaim instances it advances (without directly closing the gap).

---

#### `audit:advancesAuditReadiness`

**Domain**: `audit:AuxiliaryCommit`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: Free-text description of how this auxiliary commit advances audit readiness.

**Examples**:
- "Improves trybuild fixture clarity with additional comments"
- "Refactors admission.rs for better auditability"
- "Documents the Blue River Dam covenant decision"

---

### 11. CHECKPOINT CLAIM PROPERTIES

#### `audit:checkpointName`

**Domain**: `audit:CheckpointClaim`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The stable name of a checkpoint milestone, following pattern "PAPERLAW_ALIVE_NNN" or "PAPERLAW_CROWN_ALIVE_NNN".

**Examples**:
- `"PAPERLAW_ALIVE_001"`
- `"PAPERLAW_ALIVE_002"`
- `"PAPERLAW_CROWN_ALIVE_004"`

---

#### `audit:checkpointCommit`

**Domain**: `audit:CheckpointClaim`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The commit hash at which this checkpoint is declared.

---

#### `audit:checkpointTimestamp`

**Domain**: `audit:CheckpointClaim`  
**Range**: `xsd:dateTime`  
**Cardinality**: Functional (1:1)  
**Definition**: The timestamp at which this checkpoint was certified.

---

#### `audit:allLawsCovered`, `audit:allGapsClosed`, `audit:passedAllGates`

**Domain**: `audit:CheckpointClaim`  
**Range**: `xsd:boolean`  
**Cardinality**: Functional (1:1) each  
**Definition**: Boolean flags asserting checkpoint certification status.

- `allLawsCovered`: True if all declared laws are covered by compile-fail or compile-pass fixtures
- `allGapsClosed`: True if all non-accepted-debt gaps are closed
- `passedAllGates`: True if all configured AuditGate instances pass

---

### 12. EVIDENCE LIFECYCLE PROPERTIES

Properties from `domain-evidence-structure.ttl` describing evidence carrier lifecycle.

#### `domain:carriedValue`

**Domain**: `domain:EvidenceCarrier`  
**Range**: `owl:Thing`  
**Cardinality**: Functional (1:1)  
**Definition**: The actual data value T carried inside an Evidence<T, State, W> envelope. Type T is application-specific (EventLog, PetriNet, ConformanceVerdict, etc.).

---

#### `domain:evidenceState`

**Domain**: `domain:EvidenceCarrier`  
**Range**: `domain:LifecycleStageClass`  
**Cardinality**: Functional (1:1)  
**Definition**: The lifecycle stage (Raw, Parsed, Admitted, Refused, Projected, Exportable, Receipted) of this evidence as a PhantomData tag.

**Related Properties**:
- Links to: `compat:EvidenceState` instances
- Defines: legal transitions via `compat:successorState`

---

#### `domain:evidenceWitness`

**Domain**: `domain:EvidenceCarrier`  
**Range**: `domain:WitnessMarker`  
**Cardinality**: Functional (1:1)  
**Definition**: The witness marker (W parameter in Evidence<T, State, W>) this evidence answers to. Discriminates Ocel20 from Xes1849, etc.

**Related Properties**:
- Links to: `compat:WitnessMarker` instances
- Sealed: witness tags cannot be changed or merged without re-admission

---

#### `domain:admissionVerdictOf`

**Domain**: `domain:AdmissionVerdict`  
**Range**: `domain:EvidenceCarrier`  
**Cardinality**: Functional (1:1)  
**Definition**: Relates an Admission verdict to the evidence that triggered it.

---

#### `domain:refusalReasonType`

**Domain**: `domain:RefusalVerdict`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The specific named law reason R of a Refusal<R, W>. 

**Examples**:
- `"DanglingEventObjectLink"`
- `"MissingFinalMarking"`
- `"WitnessDiscrimination"`

**Constraint**: Bare `"InvalidInput"` is forbidden.

---

#### `domain:refusalVerdictOf`

**Domain**: `domain:RefusalVerdict`  
**Range**: `domain:EvidenceCarrier`  
**Cardinality**: Functional (1:1)  
**Definition**: Relates a Refusal verdict to the evidence that triggered it.

---

### 13. LOSS ACCOUNTING PROPERTIES

#### `domain:lossFromType`

**Domain**: `domain:LossReport`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The source type in a LossReport<From, To, Items>.

**Example**: `"OcelLog"` (when converting OCEL to XES)

---

#### `domain:lossToType`

**Domain**: `domain:LossReport`  
**Range**: `xsd:string`  
**Cardinality**: Functional (1:1)  
**Definition**: The target type in a LossReport<From, To, Items>.

**Example**: `"XesLog"`

---

#### `domain:lossItemCount`

**Domain**: `domain:LossReport`  
**Range**: `xsd:integer`  
**Cardinality**: Functional (1:1)  
**Definition**: The number of items lost in a projection (cardinality of the Items set in LossReport<From, To, Items>).

---

#### `domain:lossPolicy`

**Domain**: `domain:LossReport`  
**Range**: `domain:LossPolicy`  
**Cardinality**: Functional (1:1)  
**Definition**: The LossPolicy that governed this projection.

**Range Values**:
- `"RefuseLoss"` — refuse the projection if loss would occur
- `"AllowNamedProjection"` — allow loss only with a named ProjectionName
- `"AllowLossWithReport"` — allow loss if LossReport is emitted

---

### 14. TYPE CONSTRAINT PROPERTIES

#### `domain:hasCompileFailLaw`

**Domain**: `domain:TypeConstraint`  
**Range**: `compat:CompileFailLaw`  
**Cardinality**: Multi-valued (1..*)  
**Definition**: Links a TypeConstraint to the compile-fail law(s) that prove it unbreakable.

**Example**:
```sparql
domain:Between01Constraint domain:hasCompileFailLaw domain:Between01ViolationLaw .
```

---

#### `domain:requiresNightlyFeature`

**Domain**: `domain:TypeConstraint`  
**Range**: `xsd:string`  
**Cardinality**: Multi-valued (0..*)  
**Definition**: Nightly Rust features required to enforce this constraint.

**Examples**:
- `"generic_const_exprs"` (for Require<{EXPR}>: IsTrue)
- `"adt_const_params"` (for ConstParamTy enums)
- `"const_trait_impl"` (for const function traits)

---

## Cardinality Summary Table

| Property | Domain | Cardinality | Notes |
|----------|--------|-------------|-------|
| `audit:specName` | AuditSpec | 1:1 | Functional, unique identifier |
| `audit:lawName` | AuditSpec, AuditGate, ForbiddenPattern | 1:1 | Functional |
| `audit:auditScope` | AuditSpec | 1:1 | Functional, enum-valued |
| `audit:scopeTarget` | AuditSpec | 1:1 | Functional, string |
| `audit:carrierType` | AuditSpec | 1:1 | Functional, Rust type signature |
| `audit:witnessRequired` | AuditSpec | 0..* | Multi-valued, links to WitnessMarker |
| `audit:stateRequired` | AuditSpec | 0..* | Multi-valued, links to EvidenceState |
| `audit:verdictType` | AuditSpec | 1:1 | Functional, enum-valued |
| `audit:applicableTemplate` | AuditSpec | 1..* | Multi-valued, at least one |
| `audit:templateName` | AuditTemplate | 1:1 | Functional, unique identifier |
| `audit:templateKind` | AuditTemplate | 1:1 | Functional, enum-valued |
| `audit:instantiatedBy` | AuditExecutable | 1:1 | Functional |
| `audit:executableId` | AuditExecutable | 1:1 | Functional, unique identifier |
| `audit:fixtureFile` | AuditExecutable | 1:1 | Functional |
| `audit:expectedStderr` | AuditExecutable | 0..1 | Optional, compile-fail only |
| `audit:gateName` | AuditGate | 1:1 | Functional, unique identifier |
| `audit:gateCondition` | AuditGate | 1:1 | Functional, SPARQL query |
| `audit:gateSeverity` | AuditGate | 1:1 | Functional, enum-valued |
| `audit:forbiddenBy` | ForbiddenPattern | 1..* | Multi-valued, at least one |
| `audit:patternName` | ForbiddenPattern | 1:1 | Functional, unique identifier |
| `audit:allowedIn` | ForbiddenPattern | 0..* | Multi-valued, optional |
| `audit:contextName` | AllowedContext | 1:1 | Functional, unique identifier |
| `audit:contextScope` | AllowedContext | 1:1 | Functional, enum-valued |
| `audit:exceptionReason` | AllowedContext | 1:1 | Functional, human-readable |
| `audit:gapId` | Gap | 1:1 | Functional, unique identifier |
| `audit:gapCategory` | Gap | 1:1 | Functional, enum-valued |
| `audit:gapSeverity` | Gap | 1:1 | Functional, enum-valued |
| `audit:relatedLaw` | Gap | 1..* | Multi-valued, at least one |
| `audit:remediationPlan` | Gap | 1:1 | Functional, human-readable |
| `audit:gapOpenedAt` | Gap | 1:1 | Functional, commit hash |
| `audit:gapOpenedDate` | Gap | 1:1 | Functional, ISO 8601 |
| `audit:commitHash` | CommitEvidence | 1:1 | Functional, unique identifier |
| `audit:commitAuthor` | CommitEvidence | 1:1 | Functional |
| `audit:commitTimestamp` | CommitEvidence | 1:1 | Functional, ISO 8601 |
| `audit:commitMessage` | CommitEvidence | 1:1 | Functional, conventional commits |
| `audit:parentCommit` | CommitEvidence | 1..2 | Multi-valued (1 or 2 for merge) |
| `audit:carriesReceipt` | CommitEvidence | 0..* | Multi-valued, optional |
| `audit:closesGap` | ClosureClaim | 1:1 | Functional |
| `audit:closingCommit` | ClosureClaim | 1..* | Multi-valued, at least one |
| `audit:closureMethod` | ClosureClaim | 1:1 | Functional, enum-valued |
| `audit:closureTimestamp` | ClosureClaim | 1:1 | Functional, ISO 8601 |
| `audit:checkpointName` | CheckpointClaim | 1:1 | Functional, unique identifier |
| `audit:checkpointCommit` | CheckpointClaim | 1:1 | Functional, commit hash |
| `audit:checkpointTimestamp` | CheckpointClaim | 1:1 | Functional, ISO 8601 |
| `audit:allLawsCovered` | CheckpointClaim | 1:1 | Functional, boolean |
| `audit:allGapsClosed` | CheckpointClaim | 1:1 | Functional, boolean |
| `audit:passedAllGates` | CheckpointClaim | 1:1 | Functional, boolean |
| `domain:carriedValue` | EvidenceCarrier | 1:1 | Functional |
| `domain:evidenceState` | EvidenceCarrier | 1:1 | Functional |
| `domain:evidenceWitness` | EvidenceCarrier | 1:1 | Functional |
| `domain:lossFromType` | LossReport | 1:1 | Functional |
| `domain:lossToType` | LossReport | 1:1 | Functional |
| `domain:lossItemCount` | LossReport | 1:1 | Functional |
| `domain:lossPolicy` | LossReport | 1:1 | Functional |

---

## Usage Pattern Examples

### Example 1: Witness Discrimination Audit Specification

```sparql
# AuditSpec instance linking to template and forbidden pattern
?spec a audit:AuditSpec ;
       audit:specName "witness-discrimination-law" ;
       audit:lawName "WitnessDiscrimination" ;
       audit:auditScope "crate" ;
       audit:scopeTarget "wasm4pm-compat" ;
       audit:carrierType "Admission<T, W>" ;
       audit:verdictType "boolean" ;
       audit:applicableTemplate audit:TryBuildCompileFailTemplate ;
       rdfs:comment "Admission<T, W1> cannot be used as Admission<T, W2>" .

# Template instance
audit:TryBuildCompileFailTemplate
       a audit:AuditTemplate ;
       audit:templateName "trybuild-compile-fail-fixture" ;
       audit:templateKind "compile-fail-fixture" .

# Executable instance
?exec a audit:AuditExecutable ;
       audit:executableId "compile-fail-witness-discrimination-ocel-20" ;
       audit:instantiatedBy audit:TryBuildCompileFailTemplate ;
       audit:fixtureFile "tests/ui/compile_fail/witness_discrimination.rs" ;
       audit:expectedStderr "tests/ui/compile_fail/witness_discrimination.stderr" .

# Forbidden pattern instance
?pattern a audit:ForbiddenPattern ;
         audit:patternName "witness-discrimination-violation" ;
         audit:forbiddenBy compat:WitnessDiscriminationLaw ;
         rdfs:comment "Witness tags are sealed and non-interchangeable" .
```

### Example 2: Gap Closure with Commit Evidence

```sparql
# Gap instance
audit:Gap001_UnimplementedConformanceAudit
       a audit:Gap ;
       audit:gapId "GAP_001" ;
       audit:gapCategory "missing-fixture" ;
       audit:relatedLaw compat:ConformanceAuthority ;
       audit:gapSeverity "major" ;
       audit:remediationPlan "Implement OCEL-based audit..." ;
       audit:gapOpenedDate "2026-05-30T00:00:00Z"^^xsd:dateTime .

# Commit evidence with receipts
audit:CommitAliveEvidence_2426fac
       a audit:CommitEvidence ;
       audit:commitHash "2426fac" ;
       audit:commitAuthor "Sean Chatman <xpointsh@gmail.com>" ;
       audit:commitTimestamp "2026-05-30T11:30:00Z"^^xsd:dateTime ;
       audit:commitMessage "feat(nightly): implement zero-cost type-law surfaces..." ;
       audit:parentCommit "f94762a" ;
       audit:carriesReceipt [some receipt URI] .

# Closure claim linking commit to gap
audit:ClosureClaim_Gap001_Partial
       a audit:ClosureClaim ;
       audit:closesGap audit:Gap001_UnimplementedConformanceAudit ;
       audit:closingCommit "2426fac" ;
       audit:closureMethod "fixture-added" ;
       audit:closureTimestamp "2026-05-30T11:45:00Z"^^xsd:dateTime .
```

### Example 3: Evidence Lifecycle with Loss Accounting

```sparql
# Admitted evidence
?admitted a domain:AdmittedEvidence ;
          domain:carriedValue ?ocelLogValue ;
          domain:evidenceState domain:AdmittedStage ;
          domain:evidenceWitness compat:Ocel20 .

# Projection with loss report
?projected a domain:ProjectedEvidence ;
           domain:carriedValue ?xesLogValue ;
           domain:evidenceState domain:ProjectedStage ;
           domain:evidenceWitness compat:Xes1849 ;
           domain:lossReport [
               a domain:LossReport ;
               domain:lossFromType "OcelLog" ;
               domain:lossToType "XesLog" ;
               domain:lossItemCount 42 ;
               domain:lossPolicy "AllowLossWithReport"
           ] .
```

---

## Inverse and Related Properties

| Property | Inverse/Related | Relationship |
|----------|-----------------|--------------|
| `audit:applicableTemplate` | (implicit) "instantiates" | Template → Spec (many:many) |
| `audit:instantiatedBy` | (implicit) "instantiates" | Executable → Template (many:one) |
| `audit:witnessRequired` | (implicit) "required-by" | WitnessMarker → Spec (many:many) |
| `audit:stateRequired` | (implicit) "required-by" | EvidenceState → Spec (many:many) |
| `audit:forbiddenBy` | (implicit) "forbids" | CompileFailLaw → ForbiddenPattern (many:many) |
| `audit:allowedIn` | (implicit) "allows" | AllowedContext → ForbiddenPattern (many:many) |
| `audit:relatedLaw` | (implicit) "is-gapped-by" | CompileFailLaw/GraduationBoundary → Gap (many:many) |
| `audit:carriesReceipt` | (implicit) "carried-by" | Receipt → CommitEvidence (many:one) |
| `audit:closesGap` | (implicit) "closed-by" | Gap → ClosureClaim (one:many) |
| `audit:closingCommit` | (implicit) "closes" | CommitEvidence → ClosureClaim (many:many) |
| `audit:supportsGapClosure` | (implicit) "supported-by" | ClosureClaim → AuxiliaryCommit (many:many) |
| `domain:admissionVerdictOf` | `domain:refusalVerdictOf` | Verdict ↔ Evidence (one:one or error) |
| `domain:evidenceState` | `compat:successorState` | State → NextState (defines legal transitions) |
| `domain:evidenceWitness` | `compat:witnessFamily` | Witness → Family (categorizes witness) |

---

## Closed-World Statement

All properties declared in this reference are **exhaustive** for the wasm4pm-compat audit and evidence ontologies as of version 1.0. No audit spec, template, executable, gate, gap, or evidence property exists outside this inventory that is valid in the SPARC audit machinery.

---

## Related Documentation

- **wasm4pm-compat.ttl** — Type-law ontology (WitnessMarker, EvidenceState, ProcessForm, TypeConstraint)
- **audit-machinery.ttl** — Audit specification ontology (AuditSpec, AuditTemplate, AuditGate, Gap, CommitEvidence)
- **domain-evidence-structure.ttl** — Evidence lifecycle domain ontology (evidence carriers, admission/refusal)
- **domain-type-constraints.ttl** — Type constraint domain ontology (compile-time invariants, proof gates)
- **process-mining-chicago-tdd doctrine** — Hostile assumptions underpinning audit semantics
- **WASM4PM-COMPAT-PRD-ARD.md** — ALIVE gate requirements and nightly-first covenant

---

**End of RDF Hook Properties Reference**
