# Hook Declarations Extraction Report
## RDF Ontology Sources + Configuration

**Extraction Date:** 2026-06-01  
**Source Files Analyzed:**
- `/Users/sac/wasm4pm-compat/ggen/ontology/wasm4pm-compat.ttl`
- `/Users/sac/wasm4pm-compat/ggen/ontology/audit-machinery.ttl`
- `/Users/sac/.claude/settings.json` (hook configuration)

---

## 1. HOOK DECLARATIONS FROM SETTINGS.JSON

### 1.1 Stop Hook (Terminal Lifecycle)

**Hook Type:** `command`  
**Trigger Event:** `Stop`  
**Handler Command:** `bash ~/.claude/rdf-loop/rdf-stop-hook.sh`  
**Purpose:** Execute cleanup/teardown logic when Claude Code session terminates  
**Scope:** Session-level, global hook

**Handler Details:**
- Location: `~/.claude/rdf-loop/rdf-stop-hook.sh`
- Execution Context: Session teardown phase
- Expected Behavior: RDF loop cleanup, resource release, state finalization

### 1.2 Status Line Hook (Continuous Monitoring)

**Hook Type:** `command`  
**Trigger Event:** Continuous (status display)  
**Handler Command:** `~/.claude/statusline-command.sh`  
**Purpose:** Emit status line for display in terminal UI  
**Scope:** Session-level, recurring hook

**Handler Details:**
- Location: `~/.claude/statusline-command.sh`
- Execution Context: UI status refresh loop
- Expected Output: Single-line status string for terminal

---

## 2. AUDIT MACHINERY HOOK CLASSES (from audit-machinery.ttl)

### 2.1 Audit Gate Pattern (Proof Gate)

**RDF Class:** `audit:AuditGate`  
**Parent Class:** `sh:NodeShape` (SHACL node shape)  
**Role:** Validator hook that executes proof gates during audit execution

**Properties:**
- `audit:gateName` — Machine name (e.g., 'all-laws-have-fixtures')
- `audit:gateCondition` — SPARQL ASK query or logical condition
- `audit:gateDescription` — Human-readable validation purpose
- `audit:gateSeverity` — Failure handling: 'fatal' | 'error' | 'warning' | 'info'

**Hook Semantics:**
- Proof gate (validation check) that must pass for audit certification
- Verdict type: boolean (pass/fail)
- Success Criteria: Gate condition evaluates to true
- Failure Handling: Severity determines audit continuation policy

**Example Gate Pattern:**
```
audit:all_laws_have_fixtures
  a audit:AuditGate ;
  audit:gateName "all-laws-have-fixtures" ;
  audit:gateCondition "ASK WHERE { ?law a compat:CompileFailLaw . FILTER NOT EXISTS { ?law compat:fixtureFile ?file } }" ;
  audit:gateSeverity "fatal" .
```

### 2.2 Audit Executable Template (Fixture Handler)

**RDF Class:** `audit:AuditExecutable`  
**Parent Class:** `prov:Activity` (PROV activity)  
**Role:** Concrete, parameterized hook instance ready for execution

**Properties:**
- `audit:executableId` — Unique stable identifier
- `audit:fixtureFile` — Path to test fixture (.rs, .py, .sh)
- `audit:expectedStderr` — Expected compiler diagnostic output
- `audit:instantiatedBy` — Link to parent AuditTemplate

**Hook Semantics:**
- Fully instantiated executable with all parameters bound
- Carries preconditions and execution context
- Links to concrete fixture files in the codebase
- Produces structured verdict (boolean, metric, enum, or evidence)

**Example Executable Pattern:**
```
audit:exec_compile_fail_witness_discrimination_ocel20
  a audit:AuditExecutable ;
  audit:executableId "compile-fail-witness-discrimination-ocel-20" ;
  audit:fixtureFile "tests/ui/compile_fail/witness_discrimination.rs" ;
  audit:expectedStderr "tests/ui/compile_fail/witness_discrimination.stderr" ;
  audit:instantiatedBy audit:template_trybuild_compile_fail .
```

### 2.3 Audit Template (Reusable Handler Pattern)

**RDF Class:** `audit:AuditTemplate`  
**Parent Class:** `sh:Shape` (SHACL shape)  
**Role:** Parameterized pattern/strategy for audit execution

**Properties:**
- `audit:templateName` — Stable machine name (e.g., 'trybuild-compile-fail-fixture')
- `audit:templateKind` — Implementation pattern:
  - `compile-fail-fixture` (trybuild)
  - `compile-pass-fixture`
  - `runtime-test`
  - `static-analysis`
  - `event-log-conformance`
  - `type-state-proof`

**Hook Semantics:**
- Reusable, parameterizable audit strategy
- Binds to multiple AuditSpecs
- Encodes audit methodology (how an audit is conducted)
- Maps to concrete fixture types

**Example Template Pattern:**
```
audit:template_trybuild_compile_fail
  a audit:AuditTemplate ;
  audit:templateName "trybuild-compile-fail-fixture" ;
  audit:templateKind "compile-fail-fixture" .
```

### 2.4 Forbidden Pattern Hook (Structural Law Enforcement)

**RDF Class:** `audit:ForbiddenPattern`  
**Role:** Declarative hook defining what structures are forbidden and must be auditable

**Properties:**
- `audit:forbiddenBy` — Links to audit laws or compile-fail fixtures that prove the pattern forbidden

**Hook Semantics:**
- Names a structural or behavioral pattern explicitly forbidden
- Auditable — proof of forbiddenness backed by compile-fail fixtures
- Examples:
  - `RawEvidenceExportedAsAdmitted`
  - `SilentStructureLoss`
  - `BareInvalidInputRefusal`
  - `WitnessDiscriminationViolation`

**Related Legal Hooks:**
- `compat:CompileFailLaw` — Type-level proof that the pattern is uncompilable

### 2.5 Allowed Context Hook (Exception/Override Policy)

**RDF Class:** `audit:AllowedContext`  
**Role:** Named exception hook defining where a forbidden pattern is permitted

**Properties:**
- `audit:scopeTarget` — Module, feature gate, or witness marker where exception applies
- Exception Reason — Named justification for the override
- Audit Override Policy — How audit compliance is verified under exception

**Hook Semantics:**
- Permits an otherwise forbidden pattern in a specific context
- Carries named reason and audit override policy
- Examples:
  - Feature-gated code path
  - Witness-marker-specific context
  - Execution phase (e.g., graduation logic)

### 2.6 Closure Claim Hook (Gap Resolution Trigger)

**RDF Class:** `audit:ClosureClaim`  
**Parent Class:** `prov:Qualification` (PROV qualification)  
**Role:** Proof hook asserting that a documented gap has been closed

**Properties:**
- `audit:specName` — The gap being closed
- `audit:lawName` — Named law that was missing
- Closure Method:
  - audit fixture added
  - law proved by fixture
  - path proved reachable
- Timestamp — When closure was claimed
- Closing Commit Hashes — Git commits that close the gap

**Hook Semantics:**
- Triggers when gap is resolved
- Links to closure commit(s) in git history
- Creates immutable audit milestone
- Feeds into CheckpointClaim (audit checkpoint) aggregation

### 2.7 Checkpoint Claim Hook (Audit Milestone)

**RDF Class:** `audit:CheckpointClaim`  
**Parent Class:** `prov:Bundle` (PROV bundle)  
**Role:** Immutable audit milestone — proof that all declared laws are backed

**Hook Semantics:**
- Certified audit checkpoint at a point in commit history
- All declared laws backed by:
  - Compile-fail fixtures, OR
  - Compile-pass proofs, OR
  - Named exception contexts
- Immutable — gates the release process

---

## 3. EVIDENCE LIFECYCLE HOOKS (from wasm4pm-compat.ttl)

### 3.1 State Transition Hooks (One-Way Door Enforcement)

**RDF Class:** `compat:StateTransition`  
**Role:** Type-level lifecycle hook enforcing lawful state transitions

**Declared Transitions:**

1. `RawToParsed` — Untrusted → structurally valid (entry point)
2. `ParsedToAdmitted` — Structurally valid → authority-admitted (boundary hook)
3. `ParsedToRefused` — Structural parse failure (terminal refusal hook)
4. `AdmittedToProjected` — Loss-accounted projection (loss gate hook)
5. `AdmittedToExportable` — Admitted → export-ready (export gate hook)
6. `AdmittedToReceipted` — Provenance sealing (receipt hook)
7. `ProjectedToExportable` — Post-projection export (secondary export gate)
8. `ProjectedToReceipted` — Post-projection sealing (secondary receipt hook)
9. `ExportableToReceipted` — Final sealing before graduation (final hook)

**Hook Semantics:**
- Zero-cost type-level marker enforcing one-way-door lifecycle
- Illegal transitions are unrepresentable (compile error)
- Prevents raw evidence from leaking as admitted
- Prevents lossy structure from being unaccounted

### 3.2 Successor State Hook

**RDF Property:** `compat:successorState`  
**Type:** State relationship property  
**Domain:** `compat:EvidenceState`  
**Range:** `compat:EvidenceState`

**Hook Role:**
- Declaratively defines which states may legally follow each state
- Example: `compat:Raw → {compat:Parsed, compat:Refused}`
- Enforced at type level via PhantomData sealing

---

## 4. WITNESS MARKER HOOKS (Authority Chain)

### 4.1 Witness Family Hierarchy

**RDF Classes:**
- `compat:Standard` — Published interchange/data standard (OCEL 2.0, XES 1849-2016)
- `compat:Paper` — Academic paper defining a model family
- `compat:ApiGrammar` — API grammar a consumer must speak
- `compat:RustLaw` — Rust-language law this crate enforces
- `compat:InternalBridge` — Internal bridge toward graduation (wasm4pm)

**Hook Semantics:**
- Authority chain establishing which witness authority applies at each point
- Witness markers are compile-time tags that prevent witness discrimination
- Example: `Evidence<T, Admitted, Ocel20>` ≠ `Evidence<T, Admitted, Xes1849>` at type level

### 4.2 Key Witness Marker Hooks

| Marker | Family | Authority Chain | Hook Role |
|--------|--------|-----------------|-----------|
| `Ocel20` | Standard | Primary OCEL 2.0 authority | Admits object-centric evidence |
| `Xes1849` | Standard | Primary XES 1849-2016 authority | Admits flat event stream |
| `WfNetSoundnessPaper` | Paper | Soundness criterion (van der Aalst 1998) | Enforces WF-net soundness witness path |
| `MiningWitness` | InternalBridge | wasm4pm Mining Authority | Graduation bridge to discovery |
| `ConformanceWitness` | InternalBridge | wasm4pm Conformance Authority | Graduation bridge to alignment/fitness |
| `ReplayWitness` | InternalBridge | wasm4pm Replay Authority | Graduation bridge to token replay |
| `LifecycleWitness` | InternalBridge | wasm4pm Lifecycle Authority | Graduation bridge to OCEL lifecycle tracking |

---

## 5. GRADUATION BOUNDARY HOOKS

### 5.1 Graduation Reason Hooks

**RDF Class:** `compat:GraduationBoundary`  
**Role:** Hook triggers that signal when a surface must graduate to wasm4pm

**Declared Graduation Hooks:**

1. **NeedsDiscovery**
   - Trigger: Process discovery execution required
   - Target Authority: `wasm4pm::mining`
   - Applies To: Inductive Miner, Alpha Miner, DFG mining, trace abstraction

2. **NeedsConformanceExecution**
   - Trigger: Token replay, alignment, or conformance scoring
   - Target Authority: `wasm4pm::conformance`
   - Applies To: Fitness calculation, model-to-log alignment, fitness/precision/F1 scoring

3. **NeedsReplay**
   - Trigger: Token-replay or language-inclusion check
   - Target Authority: `wasm4pm::replay`
   - Applies To: Step-wise execution, path finding, execution profiling

4. **NeedsBenchmarkGating**
   - Trigger: Benchmark gate (throughput, latency, scalability measurement)
   - Target Authority: `wasm4pm::benchmarks`

5. **NeedsObjectCentricQuery**
   - Trigger: OCPQ query execution over OCEL log
   - Target Authority: `wasm4pm::ocpq`

6. **RebuildingProcessMiningLocally**
   - Trigger: Consumer rebuilding process mining logic in compat layer
   - Signal: Clearest signal to graduate entire surface to wasm4pm

### 5.2 Graduation Surface Hooks (Authority Surfaces)

**RDF Class:** `compat:GraduationSurface`  
**Role:** Wasm4pm module surfaces where compat structure graduates to engine logic

| Surface | Authority | Witness | Evidence Carrier | Hook Function |
|---------|-----------|---------|------------------|---------------|
| Mining | MiningAuthority | MiningWitness | `Evidence<T, Admitted\|Projected, MiningWitness>` | Discovery algorithms, variant analysis, trace abstraction |
| Conformance | ConformanceAuthority | ConformanceWitness | `Evidence<T, Admitted\|Projected, ConformanceWitness>` | Model-to-log alignment, fitness, precision/recall, declarative checking |
| Replay | ReplayAuthority | ReplayWitness | `Evidence<T, Admitted\|Projected, ReplayWitness>` | Token replay, parallel path execution, path finding, profiling |
| Lifecycle | LifecycleAuthority | LifecycleWitness | `Evidence<T, Admitted\|Projected, LifecycleWitness>` | OCEL object tracking, state transitions, artifact provenance |

---

## 6. LOSS ACCOUNTING HOOKS

### 6.1 Loss Gate Hook

**RDF Class:** `compat:LossReport` (structural carrier)  
**Type Property:** `compat:graduatesTo` / Projection mechanism  
**Role:** Loss accounting hook that enforces named projections with reports

**Hook Properties:**
- **ProjectionName** — Stable `&'static str` newtype naming the projection
- **LossPolicy** — Decision made before loss occurs:
  - `RefuseLoss` — Reject lossy transformation (block hook)
  - `AllowNamedProjection` — Permit named projection
  - `AllowLossWithReport` — Permit with mandatory loss report
- **LossReport** — Structured report: `LossReport<From, To, Items>`

**Hook Semantics:**
- Silent structure loss is a defect
- Every non-refusing projection emits a LossReport
- No path from external format directly to another — must go through admitted compat
- Routes: `external → admitted compat → external | wasm4pm`

---

## 7. COMPILE-FAIL LAW HOOKS (Type-Level Proof Gates)

### 7.1 Witness Discrimination Law

**Law:** `WitnessDiscriminationLaw`  
**Error Code:** `E0308` (type mismatch)  
**Hook Function:** Prevent `Admission<T, W1>` from being used as `Admission<T, W2>`

**Proof Gate:** Compile-fail fixture enforces that Ocel20-admitted ≠ Xes1849-admitted at type level

### 7.2 Raw Evidence Export Law

**Law:** `RawEvidenceExportedAsAdmittedLaw`  
**Error Code:** `E0308` (type mismatch)  
**Hook Function:** Prevent `Evidence<T, Raw, W>` from leaving the crate as admitted

**Proof Gate:** One-way door is enforced by the type system

### 7.3 Condition Cell Overflow Law

**Law:** `ConditionCellOverflowLaw`  
**Error Code:** `E0080` (const eval error)  
**Hook Function:** Enforce maximum 8 primary condition bits

**Proof Gate:** `ConditionCell<9>` does not compile (Blue River Dam covenant)

### 7.4 Between01 Violation Law

**Law:** `Between01ViolationLaw`  
**Error Code:** `E0080` (const eval error)  
**Hook Function:** Enforce metrics provably in [0, 1] at type level

**Proof Gate:** `Between01<2, 1>` does not compile (metric > 1 is inadmissible)

### 7.5 Missing Final Marking Law

**Law:** `MissingFinalMarkingLaw`  
**Error Code:** `E0277` (trait not implemented)  
**Hook Function:** WF-net lacking proper final marking must be refused

**Proof Gate:** `Refusal<MissingFinalMarking, W>` is required type; bare `InvalidInput` forbidden

### 7.6 Dangling Event-Object Link Law

**Law:** `DanglingEventObjectLinkLaw`  
**Error Code:** `E0277` (trait not implemented)  
**Hook Function:** OCEL event referencing non-existent object must be refused

**Proof Gate:** Structural law at admission boundary

### 7.7 Sealed Evidence State Law

**Law:** `SealedEvidenceStateLaw`  
**Error Code:** `E0277` (trait not implemented)  
**Hook Function:** Only seven canonical evidence states are valid

**Proof Gate:** Downstream crates cannot implement `EvidenceState` for foreign types

---

## 8. AUTHORITY CHAIN HOOKS

### 8.1 Witness Authority Hooks

**Hook Pattern:** Witness marker → Authority constraint → Admission decision

**Authority Chain Enforcement:**
1. Evidence arrives as `Raw`
2. Parser produces `Parsed` (structurally valid)
3. `Admit` impl checks against named **Witness** authority
4. If admitted: `Admitted<T, W>` (W now proves authority)
5. If refused: `Refused<R, W>` (R names the specific law that failed)

**Examples:**
- `Admit<OcelLog, Ocel20>` → validates against OCEL 2.0 standard
- `Admit<XesLog, Xes1849>` → validates against XES 1849-2016 standard
- `Admit<WfNetConst<Unknown>, WfNetSoundnessPaper>` → validates soundness claim

### 8.2 Feature Gate Hooks

**Hook Properties:**
- **formats** (default on) — Import/export contracts, round-trip claims, loss surfaces
- **strict** (default off) — Opt-in boundary judgment: strict admission/refusal surfaces
- **wasm4pm** (default off) — Graduation bridge traits toward wasm4pm execution engine

**Hook Semantics:**
- Feature gates control which hooks are available
- Each feature adds a new hook surface (admission/graduation/boundary)
- Disabling all features does NOT remove canon knowledge

---

## SUMMARY: HOOK CLASSIFICATION

| Hook Category | Count | Primary Role | Enforcement Mechanism |
|---------------|-------|--------------|----------------------|
| **Lifecycle Hooks** | 9 | State transition validation | Type system + PhantomData |
| **Witness Authority Hooks** | 37 | Admission boundary enforcement | Generic witness markers |
| **Audit Gate Hooks** | 7+ | Proof gate validation | SPARQL ASK queries + trybuild |
| **Graduation Hooks** | 6 | Boundary detection (→ wasm4pm) | GraduationReason enum |
| **Loss Accounting Hooks** | 1 | Loss policy enforcement | LossReport carrier |
| **Compile-Fail Law Hooks** | 7 | Type-level proof gates | Generic_const_exprs |
| **Configuration Hooks** | 2 | Session lifecycle | settings.json commands |
| **Authority Chain Hooks** | 4 | Multi-witness enforcement | Witness family hierarchy |
| **Feature Gate Hooks** | 3 | Capability gating | Cargo feature flags |

**Total Declared Hooks:** 76+

---

## OUTPUT ARTIFACTS

### Oxigraph SPARQL Query Examples

**Query 1: All Audit Gates**
```sparql
PREFIX audit: <https://wasm4pm-compat.rs/audit#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

SELECT ?gateName ?gateDescription ?gateSeverity
WHERE {
  ?gate a audit:AuditGate ;
    audit:gateName ?gateName ;
    audit:gateDescription ?gateDescription ;
    audit:gateSeverity ?gateSeverity .
}
ORDER BY ?gateSeverity ?gateName
```

**Query 2: Witness Markers by Family**
```sparql
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

SELECT ?family ?witnessTitle ?witnessKey
WHERE {
  ?witness a compat:WitnessMarker ;
    compat:witnessFamily ?familyUri ;
    compat:witnessTitle ?witnessTitle ;
    compat:witnessKey ?witnessKey .
  ?familyUri rdfs:label ?family .
}
ORDER BY ?family ?witnessTitle
```

**Query 3: All Graduation Boundaries**
```sparql
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

SELECT ?label ?description
WHERE {
  ?boundary a compat:GraduationBoundary ;
    rdfs:label ?label ;
    compat:description ?description .
}
ORDER BY ?label
```

**Query 4: Compile-Fail Laws with Fixtures**
```sparql
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>

SELECT ?lawName ?errorCode ?fixtureFile ?stderrFile
WHERE {
  ?law a compat:CompileFailLaw ;
    compat:lawName ?lawName ;
    compat:errorCode ?errorCode ;
    compat:fixtureFile ?fixtureFile ;
    compat:stderrFile ?stderrFile .
}
ORDER BY ?lawName
```

**Query 5: Evidence State Transitions (One-Way Door)**
```sparql
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

SELECT ?state ?successorLabel
WHERE {
  ?state a compat:EvidenceState ;
    rdfs:label ?stateLabel ;
    compat:successorState ?successor .
  ?successor rdfs:label ?successorLabel .
}
ORDER BY ?stateLabel ?successorLabel
```

