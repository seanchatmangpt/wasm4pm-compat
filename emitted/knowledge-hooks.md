# Knowledge-Specific Hook Patterns in wasm4pm-compat

**Scanned:** `/Users/sac/wasm4pm-compat/src/`  
**Crate Type:** Nightly-only, structure-only process-evidence standard  
**Philosophy:** Zero-cost type law via PhantomData markers and const generics; no runtime engines

---

## 1. Ontology Loaders & Knowledge Base Hooks

### Pattern: Witness-Driven Authority Registration

**Location:** `src/witness.rs`

The crate does **not** implement classical "ontology loaders" — instead, it encodes authority declaratively:

```rust
/// Witness markers — empty enums naming a named authority (standard, paper, grammar, law)
pub trait Witness {
    const KEY: &'static str;      // Machine-facing stable key
    const FAMILY: WitnessFamily;  // Classification: Standard|Paper|ApiGrammar|RustLaw|InternalBridge
    const TITLE: &'static str;    // Human-facing title
    const YEAR: Option<u16>;      // Publication year
}
```

**Witness Family Categories (implicit ontology classification):**

| Family | Role | Examples |
|--------|------|----------|
| `Standard` | Published interchange/data format standard | OCEL 2.0, XES 1849-2016 |
| `Paper` | Academic/research model definition | POWL, WF-net soundness, OC-Petri-nets, OCPQ |
| `ApiGrammar` | Consumer-facing call contract | pm4py grammar, pmax consumer grammar |
| `RustLaw` | Language-level enforcement rule | typestate admission, `forbid(unsafe_code)` |
| `InternalBridge` | Graduation/engine bridge | wasm4pm engine bridge |

**How it works:** Witnesses are zero-sized `PhantomData` markers that thread through `Evidence<T, State, W>`. Each witness carries only metadata constants — **no validation logic**. The metadata is the hook: a diagnostic, linter, or reviewer reads the witness family to determine what class of authority governs the boundary.

**No "loader"** exists because witnesses are compile-time constants embedded in the type system. A host asks "what authority names this evidence?" by inspecting `W::KEY`, `W::FAMILY`, etc.

---

## 2. Knowledge Base Hooks (Structured Rules)

### Pattern: Diagnostic Law Vocabulary

**Location:** `src/diagnostic.rs`

A named law surface that codifies the structural invariants a compat boundary must satisfy:

```rust
pub enum CompatDiagnostic {
    MissingWitness,                    // Every admitted surface must name its authority
    MissingRoundTripFixture,           // Round-trip claims need a fixture proving it
    RawEvidenceExportedAsAdmitted,     // Raw→Admitted transition only via Admit impl
    LossyProjectionWithoutPolicy,      // Lossy transform requires LossPolicy decision
    HiddenFlattening,                  // Loss must emit itemized LossReport
    MissingRefusalPath,                // Admit/Project must have named Reason type
    MissingReceiptShape,               // Provenance-bearing evidence needs receipt envelope
    UnreachablePrimitive,              // All canon types must be wired to a boundary
    MigrationRecommended,              // Surface has outgrown compat; graduate
}
```

**How it works:** Each `CompatDiagnostic` variant is a **named structural law** plus a **satisfaction condition**. A CI gate, linter, or reviewer walks the codebase, checks these diagnostics, and knows exactly what remediation is needed:

- `MissingWitness` → Add a witness to the evidence type
- `LossyProjectionWithoutPolicy` → Implement via `Project` trait under explicit `LossPolicy`
- `HiddenFlattening` → Emit `LossReport` itemizing discarded fields
- `UnreachablePrimitive` → Connect type to an admission/projection/export contract or remove

**Knowledge hook:** A linter hooking into this enum can ask "is this surface paper-complete?" — the diagnostics are the checklist.

---

## 3. Event Handlers for Knowledge Updates

### Pattern: State Transition Markers (Typestate)

**Location:** `src/state.rs` + `src/evidence.rs`

The lifecycle is enforced **structurally**, not imperatively. There is no event-handler registration — instead, type transitions are the events:

```rust
// Canonical lifecycle stages (empty enums — zero-sized, uninhabited)
pub enum Raw {}      // Untrusted, just parsed
pub enum Parsed {}   // Well-formed, not yet judged
pub enum Admitted {} // Passed an Admit impl boundary
pub enum Refused {}  // Rejected at boundary; carries named law reason
pub enum Projected {} // Lossy transformation applied; carries LossReport
pub enum Exportable {} // Exit visa granted directly from Admitted
pub enum Receipted {} // Wrapped in receipt envelope; ready for hand-off

// Typestate carrier enforces illegal transitions at compile time
pub struct Evidence<T, State: EvidenceState, W> {
    pub value: T,
    pub state: PhantomData<State>,    // Type-level lifecycle marker
    pub witness: PhantomData<W>,      // Type-level authority marker
}
```

**How lifecycle "events" work:**

```
Raw ──parse──▶ Parsed ──admit(Admit impl)──▶ Admitted ──▶ {Projected | Exportable | Receipted}
  │                                              ▲
  └────────────── refuse(Refusal) ────────────┴──▶ Refused
```

**No callback hooks:** State transitions are type-checked, not event-registered. A function that demands `Evidence<T, Admitted, W>` cannot accept `Evidence<T, Raw, W>` — the boundary law is enforced by the Rust type system.

**Knowledge hook point:** Every transition method (`into_parsed`, `into_admitted`, `into_refused`, `into_projected`, `into_exportable`, `into_receipted`) is a **potential observation point** where a host could log, measure, or inspect state flow. The crate provides infallible builder methods, but a host wrapping these in observability middleware is the natural extension.

### Pattern: Admission Gate (Structure-Only Boundary Verdict)

**Location:** `src/admission.rs`

The **only** path from `Raw` to `Admitted` is through an `Admit` impl:

```rust
pub trait Admit {
    type Admitted;
    type Reason;  // Specific named refusal reason, never "InvalidInput"
    
    fn admit(value: Self) -> Result<Admission<Self::Admitted, W>, Refusal<Self::Reason, W>>;
}

// Returned verdicts are first-class types, not Result<(), String>
pub struct Admission<T, W> {
    pub value: T,
    witness: PhantomData<W>,  // Names the authority this value answers to
}

pub struct Refusal<R, W> {
    pub reason: R,            // Specific law that was broken
    witness: PhantomData<W>,
}
```

**Hook pattern:** Every `Admit::admit()` implementation is a **knowledge update checkpoint**:
- It names the specific law being checked
- It returns a named `Reason` if it refuses
- A host monitoring admissions sees (1) which witness governed it, (2) which specific law was checked, (3) whether it passed or failed with what reason

Example integration:
```rust
impl Admit for LinkedOcel {
    type Admitted = AdmittedOcel;
    type Reason = OcelAdmissionRefusal;  // Named enum, not string
    
    fn admit(value: Self) -> Result<Admission<Self::Admitted, Ocel20>, Refusal<OcelAdmissionRefusal, Ocel20>> {
        // Check structural laws...
        if has_dangling_links(&value) {
            return Err(Refusal::new(OcelAdmissionRefusal::DanglingEventObjectLink));
        }
        Ok(Admission::new(AdmittedOcel { /* ... */ }))
    }
}
```

A logging/audit middleware can hook here:
```rust
let result = LinkedOcel::admit(raw_ocel);
// Log: witness=Ocel20, reason_type=OcelAdmissionRefusal, passed=result.is_ok()
// Forward the result unchanged
```

---

## 4. Proof Chain Hooks

### Pattern: Receipt Envelope (Provenance Carrier)

**Location:** `src/receipt.rs`

Receipts are **shaped, not computed** — they carry digests and replay hints produced elsewhere:

```rust
pub trait WellShaped {
    fn well_shaped(&self) -> bool;  // Shape check: presence of required fields
}

pub struct ReceiptShape {
    pub witness: String,            // What law/paper this receipt is judged against
    pub digest: Digest,             // Carried digest (not computed here)
    pub replay_hint: ReplayHint,    // Carried replay recipe (not executed here)
}

pub struct ReceiptEnvelope {
    pub subject: String,            // What is being receipted (case id, run id, artifact)
    pub witness: String,            // Authority name
    pub digest: Digest,             // Content digest
    pub replay_hint: ReplayHint,    // How to replay/verify
}

pub struct ReceiptChain {
    pub head: ReceiptEnvelope,
    pub prior: Option<Box<ReceiptChain>>,  // Linked-list proof chain
}
```

**Proof chain hook pattern:**

1. **Shape verification:** `receipt.well_shaped()` checks that all required fields are non-empty (witness, digest, replay_hint all present)
2. **Chaining:** `ReceiptChain::prior` links receipts into a chain; a host can walk the chain to verify sequential provenance
3. **No verification:** The receipt module never hashes, signs, verifies digests, or executes replays — **all such work belongs in `wasm4pm`**

**Hook integration point:** A host wrapping evidence in a receipt can:
- Compute the digest (e.g., BLAKE3 hash of the admitted log)
- Store the receipt in a proof-chain database
- On later access, verify the chain: walk the `prior` links, check each digest against computed values, and flag any mismatches

Example pattern:
```rust
let admitted_log = /* ... Evidence<OcelLog, Admitted, Ocel20> ... */;

// Host computes receipt at emission time
let digest = Digest::new(blake3(&serialize(&admitted_log)));
let receipt = ReceiptEnvelope::new(
    format!("case-{}", case_id),
    Ocel20::KEY,
    digest,
    ReplayHint::new(format!("run:plan#{}", run_id))
);

// Host wraps in Receipted state
let receipted = admitted_log.into_receipted(receipt);

// Later, on retrieval: host walks ReceiptChain::prior, verifies digests
```

---

## 5. Receipt Handlers

### Pattern: Graduation Candidates (Execution Handoff)

**Location:** `src/engine_bridge.rs`

When a compat surface hits a wall that structure cannot solve, it produces a **graduation candidate** describing why and what should leave the compat layer:

```rust
pub enum GraduationReason {
    NeedsDiscovery,                       // Model must be discovered
    NeedsConformanceExecution,            // Conformance must be computed
    NeedsReplay,                          // Log must be replayed
    NeedsReceipts,                        // Provenance receipts must be minted
    NeedsBenchmarkGate,                   // Benchmark must run
    NeedsObjectCentricQueryExecution,     // OCPQ must be executed
    RebuildingProcessMiningLocally,       // Host is re-implementing mining
}

pub struct GraduationCandidate {
    pub reason: GraduationReason,
    pub subject: String,                  // What is graduating (e.g., "p2p OCEL log")
    pub evidence_ref: String,             // Opaque reference to grounding evidence
}

pub trait GraduateToWasm4pm {
    fn candidate(&self) -> GraduationCandidate;
}
```

**Receipt/graduation hook pattern:**

1. A compat value (e.g., an admitted log) implements `GraduateToWasm4pm`
2. It produces a `GraduationCandidate` naming:
   - **Why** it must graduate (the reason enum variant)
   - **What** is graduating (the subject)
   - **Where** the evidence is (the reference)
3. A host (or the `wasm4pm` engine intake) reads the candidate and decides:
   - Whether the graduation request is justified
   - Which engine path to invoke (discovery, conformance, replay, …)
   - How to handle the evidence

**Hook points:**

```rust
impl GraduateToWasm4pm for PendingOcelLog {
    fn candidate(&self) -> GraduationCandidate {
        GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,  // Hard signal: must execute
            "OCEL event log, 50K events",
            format!("blake3:{}", self.content_hash)
        )
    }
}

// Host reading the candidate:
let candidate = pending_log.candidate();
match candidate.reason {
    GraduationReason::NeedsDiscovery => {
        let model = wasm4pm::discover(&pending_log, candidate.evidence_ref)?;
        // ...
    }
    _ => {}
}
```

---

## 6. Loss & Projection Hooks

### Pattern: Lossy Transformation with Policy & Report

**Location:** `src/loss.rs`

Lossy projections are **first-class, accountable boundaries:**

```rust
pub enum LossPolicy {
    RefuseLoss,                  // Reject any loss
    AllowNamedProjection,        // Allow loss under an explicit name
    AllowLossWithReport,         // Allow loss + itemize discarded items
}

pub struct ProjectionName(pub &'static str);  // Named projection (e.g., "ocel→xes:by-order")

pub struct LossReport<From, To, Items> {
    pub projection: ProjectionName,
    pub policy: LossPolicy,
    pub discarded: Items,        // Enumeration of what was lost
}

pub trait Project {
    type Output;
    type Items;
    type Reason;
    
    fn project(value: Self, policy: LossPolicy) -> Result<(Self::Output, LossReport<_, _, Self::Items>), Self::Reason>;
}
```

**Hook pattern:** Every lossy transformation is **named, gated, and reported:**

1. **Name:** `ProjectionName` makes the transformation auditable (same name = same semantics)
2. **Policy:** Caller decides in advance: refuse, allow-by-name, or allow-with-report
3. **Report:** Discarded items are enumerated so loss is **not silent**

**Hook integration:**

```rust
impl Project for OcelToXesProjection {
    type Output = XesLog;
    type Items = Vec<DiscardedLink>;
    type Reason = ProjectionRefusal;
    
    fn project(ocel: OcelLog, policy: LossPolicy) -> Result<(XesLog, LossReport<OcelLog, XesLog, Vec<DiscardedLink>>), ProjectionRefusal> {
        // Decide policy
        match policy {
            LossPolicy::RefuseLoss if ocel.has_multi_object_events() => {
                return Err(ProjectionRefusal::CannotFlattenMultiObject);
            }
            LossPolicy::AllowLossWithReport => {
                // Flatten, collect discarded links
                let (xes, discarded_links) = flatten(ocel);
                let report = LossReport::new(
                    ProjectionName("ocel→xes:by-order"),
                    policy,
                    discarded_links
                );
                return Ok((xes, report));
            }
            _ => { /* ... */ }
        }
    }
}

// Host reading the report
let (xes_log, loss_report) = OcelToXesProjection::project(admitted_ocel, policy)?;
// Log: projection_name, policy, items_lost_count
// Audit trail: what was lost, when, why
```

---

## 7. Authorization & Witness-Driven Validation Hooks

### Pattern: Strict Boundary Judgment (Opt-in)

**Location:** `src/strict.rs`

An optional (`strict` feature) boundary judgment layer that enforces additional structural laws:

```rust
pub struct ProcessBoundary {
    pub kind: BoundaryKind,
    pub name: String,
}

pub enum StrictViolation {
    MissingLossPolicy,                    // Lossy path without explicit policy
    MissingRefusalPath,                   // No named Reason type
    HiddenProcessMiningGrowth,            // Host is re-implementing mining
    MissingReceiptShape,                  // Provenance not wrapped
}

pub trait StrictCheck {
    fn check(&self) -> Result<(), StrictViolation>;
}
```

**Hook pattern:** A host enables the `strict` feature to get compile-time + runtime checks that surfaces are "paper-complete":
- Every admitted surface has a witness
- Every lossy projection has a policy and report
- No raw evidence leaks out unvetted
- Receipts wrap provenance-bearing values

Example:
```rust
// In code with `strict` feature
let boundary = ProcessBoundary::fully_attested(BoundaryKind::Admission, "ocel-intake");
boundary.check()?;  // Enforces all strict laws
```

---

## 8. Canon Type Reachability (Knowledge Inventory)

### Pattern: Module-Level Canon Documentation

**Location:** `src/lib.rs` module list

The crate defines a **canon** — the set of all process-evidence shapes it knows:

```
Canon modules (always-on, no cfg gate):
├── law.rs                  (const-generic law machinery, ConditionCell, Between01, Metric)
├── eventlog.rs             (Event, Trace, EventLog, EventStream)
├── ocel.rs                 (OcelLog, OcelEvent, EventObjectLink, ObjectObjectLink)
├── xes.rs                  (XesLog, XesEvent, attributes)
├── bpmn.rs                 (BPMN shapes, process elements)
├── petri.rs                (PetriNet, WfNet, soundness witness)
├── powl.rs                 (POWL, process tree, arity constraints)
├── dfg.rs                  (DFG, multi-perspective DFG, object-centric DFG)
├── conformance.rs          (Metric<KIND, NUM, DEN>, fitness/precision bounds)
├── prediction.rs           (Predictive patterns, outcome forecasting)
├── declare.rs              (Declare constraints, constraint language)
├── ocpq.rs                 (Object-centric process queries)
├── process_tree.rs         (Process trees with typed arity)
├── object_lifecycle.rs     (Object state/transition shapes)
├── causality.rs            (Causal link types, causal chains)
├── temporal.rs             (Temporal constraints, ordering)
├── ids.rs                  (Zero-cost ID newtypes)
├── receipt.rs              (Receipt shapes, WellShaped trait)
├── evidence.rs             (Evidence<T, State, W> carrier)
├── admission.rs            (Admit trait, Admission/Refusal verdicts)
├── loss.rs                 (LossPolicy, LossReport, Project trait)
├── witness.rs              (Witness trait, witness markers)
├── state.rs                (Lifecycle stages)
├── engine_bridge.rs        (GraduationReason, GraduateToWasm4pm)
├── diagnostic.rs           (CompatDiagnostic law checklist)
└── nightly_foundry.rs      (Experimental type-law surfaces)
```

**Hook pattern:** The `UnreachablePrimitive` diagnostic checks that **every canon type is wired to at least one boundary** (admission, projection, export, or graduation). A linter walks the type tree and verifies:
- Every struct/enum in the canon has a way in (via `Admit` or builder)
- Every admitted value has a way out (via export, project, or graduation)
- No orphaned types exist

This is **knowledge reachability**: ensuring the ontology is fully connected and actualized.

---

## Summary: Hook Taxonomy

| Hook Type | Location | Pattern | Mechanism |
|-----------|----------|---------|-----------|
| **Ontology Loader** | `witness.rs` | Authority registration via trait constants | `Witness::KEY/FAMILY/TITLE/YEAR` metadata |
| **Knowledge Base** | `diagnostic.rs` | Structural law vocabulary | `CompatDiagnostic` enum checklist |
| **Event Handlers** | `evidence.rs` + `state.rs` | State transition markers | Typestate `Evidence<T, State, W>` |
| **Admission Gate** | `admission.rs` | Boundary verdict surface | `Admit` trait, named `Reason` type |
| **Proof Chain** | `receipt.rs` | Provenance carrier + shape checker | `ReceiptChain::prior`, `WellShaped` trait |
| **Graduation Candidate** | `engine_bridge.rs` | Execution handoff request | `GraduateToWasm4pm` trait + `GraduationReason` |
| **Lossy Projection** | `loss.rs` | Accountability for data loss | `Project` trait + `LossPolicy` + `LossReport` |
| **Strict Boundary Check** | `strict.rs` | Paper-completeness enforcement | `StrictCheck` trait (feature-gated) |
| **Canon Reachability** | `lib.rs` (module tree) | Knowledge inventory verification | `UnreachablePrimitive` diagnostic |

---

## Integration Example: A Complete Hook Chain

```rust
// 1. Witness declares authority
use wasm4pm_compat::witness::Ocel20;
use wasm4pm_compat::evidence::Evidence;

// 2. Raw evidence enters
let raw_ocel = Evidence::<OcelLog, Raw, Ocel20>::raw(parsed_bytes);

// 3. Admission gate (hook: log which witness, which reason if refused)
let admitted = LinkedOcelAdmitter::admit(raw_ocel.value)?;  // Returns Admission<_, Ocel20> or Refusal<OcelRefusal, Ocel20>

// 4. Admitted evidence lifecycle (hook: observe state transition)
let admitted_ev = admitted.into_evidence();

// 5. Lossy projection (hook: record policy, discarded items)
let (projected, loss_report) = OcelToXesProjection::project(admitted_ev.value, LossPolicy::AllowLossWithReport)?;

// 6. Receipt wrapping (hook: compute digest, store proof chain)
let digest = Digest::new(blake3_hash(&serialize(&projected)));
let receipt = ReceiptEnvelope::new("case-1", Ocel20::KEY, digest, ReplayHint::new("run:plan#1"));
let receipted_ev = admitted_ev.into_receipted(receipt);

// 7. Graduation decision (hook: check if further execution is needed)
impl GraduateToWasm4pm for ProjectedXesLog {
    fn candidate(&self) -> GraduationCandidate {
        GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            "Flattened OCEL→XES, 50K events",
            receipt.digest.as_inner().to_string()
        )
    }
}

// 8. Host observes graduation need and forwards to wasm4pm engine
let candidate = receipted_ev.value.candidate();
if candidate.is_grounded() {
    wasm4pm::discover(receipted_ev.value, candidate)?;
}
```

Each step is **named**, **witnessed**, and **auditable** — no silent transformations.
