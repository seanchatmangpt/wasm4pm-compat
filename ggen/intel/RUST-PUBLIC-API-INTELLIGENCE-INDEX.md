# Rust Public API Surface Intelligence Index

**Extraction Date:** 2026-06-01  
**Crate:** wasm4pm-compat v26.6.8  
**Toolchain:** nightly (5 unconditional features)  
**Total Public Modules:** 37 (42 files including ts/ and wasm/ subdirs)

---

## Four Intelligence Ledgers Generated

This intelligence extraction produces four complementary ledgers that fully map the public API surface of wasm4pm-compat, classify every public type by projection capability, and document the graduation bridge to wasm4pm.

### 1. **rust-public-api-map.json** (411 lines)

**Purpose:** Complete API tree indexed by module.

**Contents:**
- All 37 public modules listed with purpose and public types
- Law tag modules: `state`, `evidence`, `admission`, `witness`, `loss`, `law`
- Domain model modules: `eventlog`, `ocel`, `ids`, `petri`, `powl`, `process_tree`, `xes`, `bpmn`, `dfg`, `conformance`, `receipt`
- Feature-gated modules: `formats`, `strict`, `engine_bridge`, `ts`, `wasm`
- Nightly foundry: `nightly_foundry` (4 experimental law surfaces)
- Per-module public type counts (318 total public types)
- Sealed traits (9 identified)
- Zero-cost abstractions documented

**Key Sections:**
- `core_law_modules` — the typestate machinery (State, Evidence, Admission/Refusal, Witness, Loss, Law)
- `domain_model_modules` — projectable domain shapes (EventLog, OCEL, Petri nets, POWL, process trees, XES, BPMN, DFG)
- `feature_gated_modules` — capability stages (formats, strict, wasm4pm, ts, wasm)
- `public_surface_summary` — type counts, sealed traits, zero-cost invariants
- `invariants` — the 6 non-negotiable rules (no unsafe, exactly 3 features, refusal law, loss accountability, no engines, documentation)

**Usage:** High-level reference for the entire public API structure; starting point for navigation.

---

### 2. **projectable-type-ledger.yaml** (306 lines)

**Purpose:** Types that CAN be exported to TypeScript, WASM ABI, and WIT (WebAssembly Interface Types).

**Projection Categories:**

#### Event-Log Domain
- **Event** → `interface WasmEvent { activity: string; timestamp_ns?: bigint; ... }`
- **Trace** → `interface WasmTrace { case_id: string; events: WasmEvent[]; }`
- **EventLog** → `interface WasmEventLog { traces: WasmTrace[]; ... }`

#### OCEL 2.0 Extensions
- **OcelLog** → extends EventLog with E2O/O2O links and object changes
- **EventObjectLink** → `interface WasmE2OLink { event_id: string; object_id: string; ... }`
- **ObjectObjectLink** → `interface WasmO2OLink { source_id: string; target_id: string; ... }`
- **ObjectChange** → `interface WasmObjectChange { object_id: string; attribute: string; value: any; ... }`

#### Identifiers (Zero-Cost Newtype Wrappers)
- **ActivityId, CaseId, EventId, ObjectId, ResourceId** → each projects via **Branded type** in TypeScript
  - TS: `type ActivityId = Branded<string, 'ActivityId'>`
  - WASM: `struct ActivityId([u8]) #[repr(transparent)]`
  - WIT: `type activity-id = string`
  - **Reason:** #[repr(transparent)] makes newtype wrapper zero-cost; Branded type in TS prevents accidental confusion

#### Process Models
- **PetriNet** → `interface WasmPetriNet { places, transitions, arcs, initial_marking }`
- **Place, Transition, Arc** → direct field mapping to TS
- **ProcessTree** → recursive tree structure → nested TS objects
- **PowlModel** → nodes + partial order (flattened as edge list)

#### Conformance Metrics
- **Fitness, Precision, Generalization** → each a rational number `{numerator: u32, denominator: u32}`
- **ConformanceVerdict** → bundles all 4 metrics
- **Metric<KIND, NUM, DEN>** → projects as `{kind: string, numerator: u32, denominator: u32}`
  - KIND encoded as string (e.g., "Fitness")
  - Bounds [0,1] enforced by Rust; WASM side trusts the constraint

#### Format Interchanges
- **XesLog, XesTrace, XesEvent** → standard XES structure
- **BpmnModel, BpmnTask, BpmnGateway** → OMG BPMN notation
- **DirectlyFollowsGraph** → activities + edge list

**Projection Cost Classification:**
- **Zero-cost:** Newtype wrapper erasure (ActivityId → string), field layout preservation (PetriNet → TS array of records), direct field mapping (Place → record)
- **Minimal-cost:** One vector copy (Vec<Trace> → TS Trace[]), field reordering (rational pairs), or edge list flattening (POWL partial order)
- **Structural-translation:** Tree flattening (ProcessTree parent-child relationships), linearization (partial order → adjacency list)

**Witness Projection Strategy:**
All witnesses (40+ markers) project as **metadata only**:
- `Witness::KEY` → string (e.g., "ocel-2.0")
- `Witness::FAMILY` → string ("Standard", "Paper", "ApiGrammar", "RustLaw", "InternalBridge")
- `Witness::TITLE` → string (e.g., "OCEL 2.0")
- `Witness::YEAR` → Option<u16>

**Projection Constraints (Hard Rules):**
1. All types must have `repr(C)` or `repr(transparent)` for WASM ABI
2. No generic parameters in projected types (monomorphize before ABI)
3. No lifetime parameters (all values owned at boundary)
4. No unsized types (use `Vec<T>` instead of `[T]`)
5. All identifiers project via string + TS branded type

---

### 3. **non-projectable-type-ledger.yaml** (225 lines)

**Purpose:** Types that CANNOT be exported to language boundaries; internal machinery that stays inside Rust.

**Non-Projectable Categories:**

#### Sealed Trait Implementations (Cannot Export)
- **EvidenceState trait** — sealed by `private::Sealed` supertrait
  - Implementations: `Raw`, `Parsed`, `Admitted`, `Refused`, `Projected`, `Exportable`, `Receipted`
  - Why: The typestate invariant depends on exhaustiveness. Exporting would allow downstream crates to forge an 8th stage, breaking the admission law.
  - **Export Strategy:** Don't export the stage marker. Export the *outcome*: "Evidence is now Admitted" means export the Admitted-stage value, not the Raw value.

- **Witness trait** — implements sealed pattern (cannot instantiate)
  - 41 witness markers (empty enums, uninhabited)
  - Why: Markers are phantom types designed to disappear at type level. Exporting them would expose internal type machinery.
  - **Export Strategy:** Extract const metadata (`Witness::KEY`, `TITLE`, `YEAR`, `FAMILY`) as strings.

- **LossyFormatExport, TreeProjectable, FollowsRelation, ReceiptCarrier, OcelValidator, OcelRoundTripClaim, GraduateToWasm4pm** — all sealed
  - Why: Sealed traits enforce compile-time contracts. Exporting would expose the sealing mechanism, defeating its purpose.
  - **Export Strategy:** Implement inside Rust; project the *result* (transformed value or boolean verdict), not the trait itself.

#### PhantomData-Carrying Types (Zero-Sized Type Tags)
- **Evidence<T, State, W>**
  - Phantom fields: `state: PhantomData<State>`, `witness: PhantomData<W>`
  - Actual data: `value: T`
  - Why: PhantomData tags are compile-time markers with zero runtime representation. Exporting them would require fabricating a value for something that doesn't exist.
  - **Export Strategy:** Project only `T` and export State/W as metadata (witness_key, witness_family).

- **Admission<T, W>** — phantom: `witness: PhantomData<W>`
- **Refusal<R, W>** — phantom: `witness: PhantomData<W>`
- **StateTransition markers** — RawToParsed, ParsedToAdmitted, etc. — all zero-sized structs
  - Why: Pure type-level proofs of specific transitions; no runtime meaning.
  - **Export Strategy:** Don't export markers. Export the outcome of the transition.

#### const_generic_params Machinery (Cannot Export Naively)
- **ConditionCell<BITS>** — compile-time assertion cell
  - `adt_const_params` feature required
  - BITS is compile-time checked; at runtime it's unit type `()`
  - Why: Exporting would require proving BITS at runtime — impossible without a theorem prover on the WASM side.

- **Between01<NUM, DEN>** — const-generic bounded rational [0,1]
  - `adt_const_params` feature required
  - NUM, DEN checked at compile-time to ensure 0 <= NUM <= DEN, DEN != 0
  - Why: Constraint is compile-time only; WASM side cannot prove it.
  - **Export Strategy:** Project as tuple (u32, u32); document that bounds are *trusted* to have been checked by Rust.

- **EvidenceMode** — const-generic enum (Raw, Parsed, Admitted, ...)
  - `adt_const_params` feature required
  - Why: Mirrors typestate tokens as const values; exists only in const-generic positions.
  - **Export Strategy:** Don't export. Export lifecycle stage as string metadata.

- **Metric<KIND, NUM, DEN>** — const-generic conformance metric
  - KIND: MetricKind (ConstParamTy-constrained)
  - NUM, DEN: rational bounds [0,1]
  - **Export Strategy:** Project as `{kind: string, numerator: u32, denominator: u32}`

#### min_specialization Machinery
- **nightly_foundry** types using `min_specialization`
  - Specialization narrows type law for specific Petri net subclasses
  - Why: Narrowed types exist only at compile-time for more-specific implementations.
  - **Export Strategy:** Project the underlying domain type (PetriNet), not the specialization wrapper.

#### portable_simd Machinery
- **nightly_foundry token_law** using `portable_simd`
  - SIMD token operations are compile-time type-law surfaces
  - Why: WASM lacks standard SIMD interface matching Rust's `simd<T>` types.
  - **Export Strategy:** Project token markings (vectors of counts), not the SIMD type law.

#### Generic/Parameterized Types (Cannot Export Without Monomorphization)
- **Evidence<T, State, W>** — unconstrained T, sealed State, phantom W
  - Why: Cannot export generic form to WASM ABI.
  - **Export Strategy:** Specialize to concrete types: Evidence<OcelLog, Admitted, Ocel20>. Each specialization is a distinct projectable type.

- **LossReport<From, To, Items>** — unconstrained generics
  - **Export Strategy:** Specialize: LossReport<OcelLog, XesLog, EventCount>

#### Internal/pub(crate) Machinery
- **Evidence::sealed(T)** — pub(crate) constructor for Admitted evidence
  - Why: Intentionally hidden to prevent Raw evidence from bypassing the Admit trait.
  - **Export Strategy:** Use public Evidence::raw() constructor; transition via Admit trait before crossing boundary.

- **Witness implementations** — cannot be instantiated (uninhabited enums)
  - Why: Witness trait is implemented once per marker at declaration; downstream crates cannot fabricate new witnesses.

#### nightly_foundry Module (Always-On Staging)
- **petri_law, powl_law, evidence_law, token_law** — four experimental law surfaces
- Uses: `generic_const_exprs`, `adt_const_params`, `const_trait_impl`, `min_specialization`, `portable_simd`
- Why: Type-law machinery using unstable compiler features; not meant to be exported as public types.
- **Export Strategy:** Implement law surfaces inside Rust; project only the domain types (PetriNet, ProcessTree) that *use* the laws.

**Non-Projectable Boundary Rule:**
> If a type is parameterized by a sealed trait, a const generic, or a PhantomData tag, it is non-projectable in its parameterized form. **Specialize/monomorphize it in Rust**, then project only the specialized concrete type. The machinery stays inside Rust; only domain types cross the boundary.

---

### 4. **graduation-surface-ledger.yaml** (393 lines)

**Purpose:** Bridge traits and witnesses that hand evidence to the wasm4pm execution engine; the one-way graduation protocol.

**Core Graduation Protocol:**

```
Compat Boundary                              Engine Boundary
────────────────────                        ───────────────
Raw → Admit → Admitted                      Admitted → Semantic Validation → Grounded
      (structural)                                       (semantic)
      
      Evidence<T, Admitted, Witness>
                  ↓
      GraduationCandidate
                  ↓
      is_grounded() ?
                  ├─ True  → GraduateToWasm4pm → Evidence<T, Admitted, Wasm4pmBridge>
                  └─ False → GraduationReason (specific block reason)
```

**Sealed Bridge Trait: GraduateToWasm4pm**
- Located in: `engine_bridge.rs` (feature: wasm4pm)
- Purpose: The *only* sanctioned protocol for handing typed compat Evidence to wasm4pm engine
- **Sealed:** Prevents downstream crates from fabricating fake graduation paths
- Trait signature:
  ```rust
  pub trait GraduateToWasm4pm {
    type CompatEvidence: Evidence<...>;
    type EngineInput: ...;
    fn graduate(evidence: Self::CompatEvidence) -> Result<Self::EngineInput, GraduationError>;
  }
  ```

**GraduationCandidate Type**
- Wraps Evidence<T, Admitted, W> with groundedness judgment
- Methods:
  - `is_grounded(&self) → bool` — is the evidence ready to graduate?
  - `graduation_reason(&self) → &GraduationReason` — if not grounded, why?
  - `into_evidence(self) → Evidence<T, Admitted, W>` — if grounded, extract the evidence
- Semantics:
  - **Grounded:** Structurally Admitted and ready for engine validation. No further context is missing.
  - **Ungrounded:** Structurally Admitted but missing context (loss policy, refusal path, object lifecycle, temporal ordering, etc.).

**GraduationReason Enum** — Specific Block Reasons
1. **GroundedAtFullAdmission** — Admission was complete; Evidence is ready.
2. **UngroundedMissingLossPolicy** — Lossy projection occurred but no LossPolicy recorded. Engine cannot trust the shape.
3. **UngroundedMissingRefusalPath** — Refusal surface not fully explored. Evidence may contain hidden refusals.
4. **UngroundedHiddenProcessMiningGrowth** — Process evidence grew (new objects/activities) after admission. Model is unstable.
5. **UngroundedTemporalOrdering** — Events lack timestamp context or temporal ordering is incomplete.
6. **UngroundedObjectLife** — Object lifecycle incomplete (missing create/destroy or violating transitions).

**Per-Domain Graduation Paths** — Step-by-Step Workflows

#### OCEL 2.0 Path
```
Step 1 (Compat boundary):
  Raw OCEL JSON/XML → Admit → Evidence<OcelLog, Admitted, Ocel20>
  Validation: Structural (E2O/O2O links, object types, attributes)
  Authority: Ocel20 (Standard witness)

Step 2 (Graduation gate):
  GraduationCandidate::is_grounded()?
  If False: Block with reason (e.g., UngroundedObjectLife)

Step 3 (Engine boundary):
  Evidence<OcelLog, Admitted, Wasm4pmBridge> → Wasm4pmOcelEngine
  Engine validates: Object lifecycle state transitions, cross-object causality, event→object binding consistency
  Output: Engine's own Admitted<OcelLog> (different authority, richer validation)

Expected compat-boundary refusals:
  - MissingObjectType
  - MissingEventType
  - DanglingEventObjectLink
  - InvalidAttributeValue
  - MalformedTimestamp

Expected engine-boundary refusals:
  - ObjectLifecycleViolation
  - CausalOrderingCycle
  - InconsistentObjectBinding
  - TemporalConstraintViolation
```

#### Petri Net Path
```
Step 1: Evidence<PetriNet, Admitted, WfNetSoundnessPaper>
  Validation: Structural (nodes, arcs, initial marking exist)

Step 2: Is this a workflow net? Is soundness provable?
  GraduationReason: UngroundedHiddenProcessMiningGrowth if net was grown incrementally

Step 3: Wasm4pmPetriEngine
  Validates: Murata rules, reachability graph, safety, liveness
  Asserts: Evidence<WfNet, Admitted, WfNetSoundnessPaper> (nested witness)

Compat refusals: MissingInitialMarking, DisconnectedSubnet, DeadTransition
Engine refusals: UnsoundWfNet, UnsafeMarking, DeadlockDetected
```

#### ProcessTree Path
```
Step 1: Evidence<ProcessTree, Admitted, InductiveMiner>
  Validation: Tree structure (parent-child relationships valid)

Step 2: Temporal context present?
  GraduationReason: UngroundedTemporalOrdering if mined without timestamps

Step 3: Wasm4pmProcessTreeEngine
  Computes: Fitness, precision, generalization, simplicity
  Output: Verdict attached to evidence

Compat refusals: InvalidLoopArity (loop must have 2 children), MissingLoopExitCondition
Engine refusals: DeadActivityInModel, UnobservedActivity
```

#### Conformance Metrics Path
```
Step 1: Evidence<ConformanceVerdict, Admitted, Wasm4pmBridge>
  Validation: All 4 metrics in [0,1] (enforced by Between01)

Step 2: GroundedAtFullAdmission (metrics are numeric; structure complete)

Step 3: Engine stores verdict (read-only once graduated)

Compat refusals: MetricOutOfBounds
Engine refusals: None (engine is read-only)
```

**Witness Responsibility Matrix**

| Witness Family | Example | Compat Responsibility | Engine Responsibility |
|---|---|---|---|
| **Standard** | Ocel20, Xes1849 | Check syntax/structure | Validate object lifecycle, event ordering |
| **Paper** | WfNetSoundnessPaper, InductiveMiner | Check shape | Verify soundness/fitness via algorithm |
| **RustLaw** | RustTypestateLaw | Enforce at type-level | Trust the typestate |
| **InternalBridge** | Wasm4pmBridge | Mark graduation boundary | Responsible for all semantic checking |

**Graduation Errors**
1. **GraduationUngrounded** — Evidence is not grounded; carried reason explains why
2. **GraduationTypeMismatch** — Expected type vs. actual type
3. **GraduationWitnessMismatch** — Witness does not match engine's expected authority

**Feature Gating**
- Feature flag: `wasm4pm` (default: disabled)
- Reason: Graduation is optional. A compat-only downstream crate never graduates.
- When disabled: GraduateToWasm4pm and GraduationCandidate are unavailable.

**Integration Patterns**

Pattern 1: Admit → Check → Graduate
```rust
let raw_ocel = OcelJson::parse(json_string)?;
let admitted = OcelAdmitter::admit(Evidence::raw(raw_ocel))?;
let candidate = GraduationCandidate::from(admitted);
if !candidate.is_grounded() {
  eprintln!("Cannot graduate: {}", candidate.graduation_reason().tag());
  return Err(GraduationUngrounded);
}
let engine_input = OcelGraduation::graduate(candidate.into_evidence())?;
let engine = Wasm4pmOcelEngine::new(engine_input);
let verdict = engine.check_conformance(log, model)?;
```

Pattern 2: Engine Receives Trusted Admitted Evidence
```rust
impl Engine {
  fn admit_evidence<T>(
    evidence: Evidence<T, Admitted, Wasm4pmBridge>
  ) -> Result<EngineAdmitted<T>> {
    let value = evidence.into_inner();
    // Value is structurally sound (compat crate proved it)
    // Engine focuses on semantic validation
    self.validate_semantics(&value)
      .map(|()| EngineAdmitted { value })
  }
}
```

**Graduation Invariants** (Hard Rules)
1. Structural soundness: No Evidence graduates unless `Evidence<T, Admitted, W>`
2. Witness consistency: Witness at graduation must match engine's expected authority
3. No silent loss: Evidence with loss must document it; cannot graduate silently
4. Typed refusal path: Every GraduationReason is specific, never bare "Ungrounded"
5. One-way graduation: Engine output cannot return to compat boundary

---

## Quick Navigation by Use Case

### "I want to export a type to TypeScript/WASM/WIT"
→ **projectable-type-ledger.yaml**
- Find your type in the ledger
- Copy the `to_ts`, `to_wasm`, `to_wit` signatures
- Check the "zero_cost_impl" note for ABI optimization strategy

### "I'm writing code in the compat crate and want to know what's internal"
→ **non-projectable-type-ledger.yaml**
- PhantomData fields? Stay inside Rust (typestate machinery).
- Sealed trait impl? Implement inside; project the result, not the trait.
- Generic type parameters? Monomorphize before crossing boundary.

### "I'm building a wasm4pm engine and need to know what to expect at the graduation boundary"
→ **graduation-surface-ledger.yaml**
- GraduateToWasm4pm is the contract. Implement it for each domain model.
- GraduationCandidate tells you if evidence is grounded; GraduationReason tells you why it's blocked.
- Witness metadata (KEY, FAMILY, TITLE, YEAR) is exported as runtime struct.

### "I need the complete API tree"
→ **rust-public-api-map.json**
- All 37 modules listed
- Per-module type counts
- Feature-gating documented
- Sealed traits and invariants listed

---

## Key Statistics

| Metric | Count |
|---|---|
| Public modules | 37 |
| Public types | 318 |
| Sealed traits | 9 |
| Witness markers | 41 |
| Projectable types | ~100 |
| Non-projectable types | ~200+ (internal machinery, generics, phantoms) |
| Feature-gated capability stages | 5 (formats, strict, wasm4pm, ts, wasm) |
| Nightly features unconditional | 5 (generic_const_exprs, adt_const_params, const_trait_impl, min_specialization, portable_simd) |

---

## Fundamental Invariants

1. **#![forbid(unsafe_code)]** — Absolute. No exceptions.
2. **Exactly 3 public features** — formats, strict, wasm4pm. Per-format flags break the contract.
3. **Refusal law** — Every refusal carries a specific named law. No bare InvalidInput.
4. **Loss accountability** — All lossy projections go through Project trait with LossPolicy + LossReport.
5. **No engine logic** — Discovery, conformance, replay, alignment are not in this crate. Graduate to wasm4pm.
6. **Documentation** — Every public type, module, fn requires rustdoc stating what it IS, is NOT, and when to graduate.
7. **Typestate enforcement** — Illegal lifecycle transitions are **unrepresentable** at compile time. The type system IS the law enforcement.

---

## Files in This Intelligence Index

```
ggen/intel/
├── RUST-PUBLIC-API-INTELLIGENCE-INDEX.md (this file)
├── rust-public-api-map.json (complete API tree; JSON)
├── projectable-type-ledger.yaml (types that export to TS/WASM/WIT)
├── non-projectable-type-ledger.yaml (internal machinery; sealed, phantom, const-generic)
└── graduation-surface-ledger.yaml (wasm4pm bridge; GraduateToWasm4pm protocol)
```

All four ledgers are complementary and should be read as a unified intelligence package on the wasm4pm-compat public API surface.
