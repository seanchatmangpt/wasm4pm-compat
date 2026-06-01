# Hook Contracts and Interfaces — wasm4pm-compat

**Generated:** 2026-06-01  
**Crate:** `wasm4pm-compat` — nightly-only, structure-only process-evidence standard  
**Purpose:** Catalog all hook trait interfaces, their contracts, lifecycle guarantees, and example implementations

---

## Executive Summary

This crate defines **eight distinct hook surfaces**, each with a specific contract and lifecycle. None of them run execution engines; all are structure-only. They are the **seams** where a host (or the `wasm4pm` engine) can observe or intervene:

| Hook | Trait | Location | Purpose | Lifecycle |
|------|-------|----------|---------|-----------|
| **Ontology Loader** | `Witness` | `src/witness.rs` | Name the authority (standard, paper, grammar, law) governing a boundary | Type-level metadata constants |
| **Admission Gate** | `Admit` | `src/admission.rs` | Judge raw evidence against a named law; produce `Admission` or named `Refusal` | `Raw → Admitted` transition only |
| **Lossy Projection** | `Project` | `src/loss.rs` | Account for data loss; emit policy + named loss report | Lossy transform w/ named policy & items |
| **Graduation Bridge** | `GraduateToWasm4pm` | `src/engine_bridge.rs` | Declare why evidence must leave compat layer and enter execution engine | Candidate production (not execution) |
| **Strict Boundary Check** | `StrictCheck` | `src/strict.rs` | Enforce paper-completeness (feature-gated, opt-in) | Compile-time + runtime check |
| **Receipt Verification** | `WellShaped` | `src/receipt.rs` | Verify proof envelope has required fields | Shape check (presence, not content) |
| **Witness Authority** | `Join`, `WithTop` | `src/witness.rs` | Compose witnesses; define top authority | Metadata algebra only |
| **State Transition** | (Typestate) | `src/state.rs` + `src/evidence.rs` | Enforce legal lifecycle progression via type system | Compile-time state machine |

---

## 1. Witness Trait (Ontology Loader)

**Location:** `src/witness.rs` (lines 33–85)

### Interface

```rust
pub trait Witness {
    /// A stable, lowercase, machine-facing key (e.g. "ocel-2.0").
    const KEY: &'static str;
    
    /// The family this witness belongs to.
    const FAMILY: WitnessFamily;
    
    /// A human-facing title (e.g. "OCEL 2.0").
    const TITLE: &'static str;
    
    /// The publication year, if the authority has a dated edition.
    const YEAR: Option<u16>;
}
```

### Contract

**What the hook must implement:**
- Four associated constants that name an authority
- No methods; no validation logic
- Must be zero-sized (typically an empty enum)

**What guarantees it provides:**
- Type-level proof that a value is answered to a named authority
- Machines can read `KEY` to index by standard/paper/grammar
- Humans can read `TITLE` to understand the context
- Timeline queries can filter by `YEAR`

**Lifecycle guarantees:**
- Immutable across the lifetime of a piece of evidence
- Compile-time constant; zero runtime cost
- Witness type flows through `Evidence<T, State, Witness>`, `Admission<T, Witness>`, and `Refusal<R, Witness>`

### Variant: WitnessFamily

```rust
pub enum WitnessFamily {
    Standard,      // Published interchange standard (OCEL, XES)
    Paper,         // Academic paper (POWL, WF-net, OCPQ, Declare)
    ApiGrammar,    // Consumer-facing call contract (pm4py, pmax)
    RustLaw,       // Language-level enforcement (typestate, forbid)
    InternalBridge // Engine bridge (wasm4pm graduation)
}
```

### Examples

**Example 1: Standard Witness**
```rust
// From src/witness.rs
witness_marker!(
    Ocel20, "ocel-2.0", WitnessFamily::Standard, "OCEL 2.0", Some(2023)
);
// Now Ocel20::KEY == "ocel-2.0", Ocel20::FAMILY == Standard, etc.
```

**Example 2: Paper Witness**
```rust
witness_marker!(
    WfNetSoundnessPaper,
    "wfnet-soundness-paper",
    WitnessFamily::Paper,
    "The Application of Petri Nets to Workflow Management (soundness)",
    Some(1998)
);
```

**Example 3: Host-side usage (reading witness metadata)**
```rust
fn audit_evidence<T, W: Witness>(ev: &Evidence<T, _, W>) {
    println!("Witness: {}", W::TITLE);
    println!("Family: {:?}", W::FAMILY);
    println!("Key: {}", W::KEY);
    if let Some(year) = W::YEAR {
        println!("Published: {}", year);
    }
}
```

### Hook Points for Integration

1. **Linter:** Walk the type tree and enumerate all witnesses; check they are known to the schema
2. **Audit:** On admission, log `W::KEY` and `W::FAMILY` for the audit trail
3. **Graduation:** Forward `W` to the `wasm4pm` engine as the authority to verify against
4. **Metadata Index:** Build a map of {`W::KEY` → witness} for quick lookup

---

## 2. Admit Trait (Admission Gate)

**Location:** `src/admission.rs` (lines 179–241)

### Interface

```rust
pub trait Admit {
    /// The raw shape arriving at this boundary.
    type Raw;
    /// The admitted shape produced on success.
    type Admitted;
    /// The *named* refusal reason produced on failure (never "InvalidInput").
    type Reason;
    /// The authority this boundary judges against.
    type Witness;

    /// Judges `raw` against the named law for this boundary.
    fn admit(
        raw: Evidence<Self::Raw, Raw, Self::Witness>,
    ) -> Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>>;
}
```

### Associated Types

| Type | Role | Example |
|------|------|---------|
| `Raw` | Shape of untrusted input | `LinkedOcelLogRaw` (struct with dangling refs) |
| `Admitted` | Shape after judgment | `AdmittedOcelLog` (guaranteed well-formed) |
| `Reason` | Named law broken on refusal | `OcelAdmissionRefusal` enum |
| `Witness` | Authority being checked | `Ocel20` |

### Contract

**What the hook must implement:**
- `admit(raw: Evidence<Raw, Raw, Witness>) → Result<Admission<Admitted, Witness>, Refusal<Reason, Witness>>`
- Logic checks a *named structural law* (e.g. "all event-object links resolve")
- On success: return `Admission::new(admitted_value)`
- On failure: return `Refusal::new(reason_enum_variant)` — **not a string**

**What guarantees it provides:**
- Type-level proof that a value passed judgment
- Named law is recorded in the reason type (not hidden in a string)
- Host can pattern-match on the reason to understand what failed
- No silent acceptance; every case is explicit

**Lifecycle guarantees:**
- The **only** path from `Raw` to `Admitted` state
- Once `Admitted`, the value cannot regress to `Raw`
- `Witness` type parameter preserves which authority judged it

### Return Types

**On Success: `Admission<T, W>`**
```rust
pub struct Admission<T, W> {
    pub value: T,
    witness: PhantomData<W>,
}

// Bridge to Admitted evidence
pub fn into_evidence(self) -> Evidence<T, crate::state::Admitted, W>
```

**On Failure: `Refusal<R, W>`**
```rust
pub struct Refusal<R, W> {
    pub reason: R,  // Must be a named enum, not String
    witness: PhantomData<W>,
}

// Extracts the reason
pub fn into_reason(self) -> R
```

### Example Implementation

**From tests/admission_refusal.rs:**

```rust
/// Toy OCEL admission: the raw value is `true` iff every event-object link
/// resolves to a real object. We refuse with a *specific named law*.
enum LinkedOcel {}

#[derive(Debug, PartialEq, Eq)]
enum OcelRefusal {
    DanglingEventObjectLink,
}

impl Admit for LinkedOcel {
    type Raw = bool;
    type Admitted = bool;
    type Reason = OcelRefusal;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<bool, Raw, Ocel20>,
    ) -> Result<Admission<bool, Ocel20>, Refusal<OcelRefusal, Ocel20>> {
        if raw.value {
            Ok(Admission::new(true))
        } else {
            Err(Refusal::new(OcelRefusal::DanglingEventObjectLink))
        }
    }
}

// Usage:
let raw = Evidence::raw(true);
let admission = LinkedOcel::admit(raw)?;  // Returns Admission<_, Ocel20>
let admitted_ev = admission.into_evidence();  // Evidence<_, Admitted, Ocel20>
```

### More Complex Example: Real OCEL Admission

**From tests/blue_river_dam_bridge.rs (conceptual):**

```rust
enum OcelAdmitter {}

#[derive(Debug)]
enum OcelAdmissionRefusal {
    MissingObjectType,
    DanglingEventObjectLink,
    MissingFinalMarking,
    ObjectIdConflict,
}

impl Admit for OcelAdmitter {
    type Raw = OcelLogRaw;  // Parsed but not yet judged
    type Admitted = AdmittedOcelLog;
    type Reason = OcelAdmissionRefusal;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<OcelLogRaw, Raw, Ocel20>,
    ) -> Result<Admission<AdmittedOcelLog, Ocel20>, Refusal<OcelAdmissionRefusal, Ocel20>> {
        // Check: every event has at least one object link
        for event in &raw.value.events {
            if event.object_links.is_empty() {
                return Err(Refusal::new(OcelAdmissionRefusal::DanglingEventObjectLink));
            }
            // Check: each link references a known object
            for link in &event.object_links {
                if !raw.value.objects.contains_key(&link.object_id) {
                    return Err(Refusal::new(OcelAdmissionRefusal::DanglingEventObjectLink));
                }
            }
        }

        // All checks passed; construct the admitted shape
        Ok(Admission::new(AdmittedOcelLog {
            events: raw.value.events,
            objects: raw.value.objects,
            // ... other admitted fields
        }))
    }
}
```

### Hook Points for Integration

1. **Middleware:** Wrap `admit()` calls to log (witness, reason_type, pass/fail)
2. **Metrics:** Count admits vs refusals per witness
3. **Audit trail:** Record which boundary was crossed and with what law
4. **Test harness:** Verify every reason type is reachable and tested
5. **Graduation:** If deeper checks are needed, produce a `GraduationCandidate`

---

## 3. Project Trait (Lossy Projection)

**Location:** `src/loss.rs` (lines 953–974)

### Interface

```rust
pub trait Project {
    /// The shape being projected from.
    type From;
    /// The shape being projected to.
    type To;
    /// The concrete record of discarded evidence.
    type Lost;
    /// The *named* refusal reason when loss is not permitted.
    type Reason;

    /// Projects under `policy`, either reporting the loss or refusing it.
    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason>;
}
```

### Associated Types

| Type | Role | Example |
|------|------|---------|
| `From` | Lossful source shape | `OcelLog` (rich object-centric) |
| `To` | Target shape after flattening | `XesLog` (single-case) |
| `Lost` | Enumeration of what was discarded | `Vec<DiscardedEventObjectLink>` |
| `Reason` | Named law preventing loss | `OcelToXesProjectionRefusal` |

### Support Type: LossPolicy

```rust
pub enum LossPolicy {
    /// Loss is not tolerated; projection must refuse.
    RefuseLoss,
    /// Loss permitted under an explicitly named projection.
    AllowNamedProjection,
    /// Loss permitted; must emit itemized LossReport.
    AllowLossWithReport,
}

// Guard helpers (no pattern-matching required)
impl LossPolicy {
    pub const fn is_refusing(self) -> bool { /* ... */ }
    pub const fn is_named(self) -> bool { /* ... */ }
    pub const fn is_reporting(self) -> bool { /* ... */ }
}
```

### Support Type: ProjectionName

```rust
pub struct ProjectionName(pub &'static str);

// Examples:
ProjectionName("ocel-flatten-to-xes:by-order")
ProjectionName("dfg-to-sequence")
ProjectionName("powl-to-bpmn:flatten-loops")
```

### Support Type: LossReport

```rust
pub struct LossReport<From, To, Items> {
    pub projection: ProjectionName,
    pub policy: LossPolicy,
    pub discarded: Items,  // The actual lost items (Vec, Set, Count, etc.)
}

impl<From, To, Items> LossReport<From, To, Items> {
    pub fn new(projection: ProjectionName, policy: LossPolicy, discarded: Items) -> Self
    pub fn is_lossless(&self) -> bool  // where Items: IsEmpty
    pub fn summary(&self) -> NamedLoss  // (projection, loss_category)
}
```

### Contract

**What the hook must implement:**
- Decide policy beforehand: refuse, allow-by-name, or allow-with-report
- If loss occurs, emit `LossReport` with:
  - Named `ProjectionName` (stable identifier)
  - The `LossPolicy` that governed it
  - Enumerated `Lost` items (not a count or hash; actual items)
- If policy is `RefuseLoss` and loss would occur: return `Refusal`

**What guarantees it provides:**
- Every lossy transformation is **named and accountable**
- Host can audit which projections were applied
- Discarded items are enumerable (can be logged, persisted, recovered)
- Silent loss is structurally impossible

**Lifecycle guarantees:**
- The **only** path to lossy transformation (no free format→format conversions)
- Loss is gated by policy decided *before* transformation
- Report is returned alongside the projected value

### Example Implementation

**Conceptual OCEL → XES Flattening:**

```rust
enum OcelToXesProjection {}

#[derive(Debug, Clone)]
struct DiscardedEventObjectLink {
    event_id: EventId,
    object_type: String,
    object_id: ObjectId,
}

#[derive(Debug)]
enum OcelToXesProjectionRefusal {
    CannotFlattenMultiObject,
    CannotSelectCaseNotion,
}

impl Project for OcelToXesProjection {
    type From = OcelLog;
    type To = XesLog;
    type Lost = Vec<DiscardedEventObjectLink>;
    type Reason = OcelToXesProjectionRefusal;

    fn project(
        ocel: OcelLog,
        policy: LossPolicy,
    ) -> Result<LossReport<OcelLog, XesLog, Vec<DiscardedEventObjectLink>>, OcelToXesProjectionRefusal> {
        // Decide policy
        if policy.is_refusing() && ocel.has_multi_object_events() {
            return Err(OcelToXesProjectionRefusal::CannotFlattenMultiObject);
        }

        // Select a case notion (e.g., the most common object type)
        let case_notion = ocel.select_case_notion()
            .ok_or(OcelToXesProjectionRefusal::CannotSelectCaseNotion)?;

        // Flatten: map each event to a XES trace
        let mut xes_log = XesLog::new();
        let mut discarded = Vec::new();

        for event in ocel.events {
            let xe = XesEvent::from(&event);
            xes_log.add_event(xe);

            // Record what was discarded
            for link in &event.object_links {
                if link.object_type != case_notion {
                    discarded.push(DiscardedEventObjectLink {
                        event_id: event.id.clone(),
                        object_type: link.object_type.clone(),
                        object_id: link.object_id.clone(),
                    });
                }
            }
        }

        let report = LossReport::new(
            ProjectionName("ocel→xes:by-order"),
            policy,
            discarded
        );

        Ok(report)
    }
}

// Usage:
let admitted_ocel = /* Evidence<OcelLog, Admitted, Ocel20> */;
let result = OcelToXesProjection::project(
    admitted_ocel.value,
    LossPolicy::AllowLossWithReport
);

match result {
    Ok(loss_report) => {
        println!("Projected via {}", loss_report.projection.as_str());
        println!("Discarded {} links", loss_report.discarded.len());
        for link in loss_report.discarded {
            println!("  Event {} → {} {}", link.event_id, link.object_type, link.object_id);
        }
    }
    Err(e) => eprintln!("Refusal: {:?}", e),
}
```

### Hook Points for Integration

1. **Accountability log:** Record every projection by name, policy, and item count
2. **Loss auditor:** Verify discarded items meet a domain-specific recovery threshold
3. **Policy enforcer:** Reject projections if policy doesn't match business rules
4. **Loss recovery:** Persist discarded items to a recovery database
5. **Round-trip testing:** Use loss report to validate round-trip fixtures

---

## 4. GraduateToWasm4pm Trait (Graduation Bridge)

**Location:** `src/engine_bridge.rs` (lines 161–194)

### Interface

```rust
pub trait GraduateToWasm4pm {
    /// Produce the graduation case for `self`.
    fn candidate(&self) -> GraduationCandidate;
}
```

### Support Type: GraduationCandidate

```rust
pub struct GraduationCandidate {
    pub reason: GraduationReason,
    pub subject: String,           // What is graduating
    pub evidence_ref: String,      // Opaque ref to grounding evidence
}

impl GraduationCandidate {
    pub fn new(
        reason: GraduationReason,
        subject: impl Into<String>,
        evidence_ref: impl Into<String>,
    ) -> Self { /* ... */ }

    pub fn is_grounded(&self) -> bool {  // evidence_ref and subject non-empty
        !self.evidence_ref.trim().is_empty() && !self.subject.trim().is_empty()
    }
}
```

### Support Type: GraduationReason

```rust
pub enum GraduationReason {
    /// Process model must be discovered from log (algorithmic job).
    NeedsDiscovery,
    /// Conformance result must be computed (not claimed).
    NeedsConformanceExecution,
    /// Log must be replayed against model.
    NeedsReplay,
    /// Provenance receipts must be minted and chained.
    NeedsReceipts,
    /// Benchmark gate must run to admit result.
    NeedsBenchmarkGate,
    /// Object-centric query must be executed.
    NeedsObjectCentricQueryExecution,
    /// Host is rebuilding process mining locally (strongest signal).
    RebuildingProcessMiningLocally,
}

impl GraduationReason {
    pub const fn tag(self) -> &'static str {
        // "needs_discovery", "needs_replay", etc.
    }

    pub const fn is_hard_signal(self) -> bool {
        // true for NeedsDiscovery, NeedsConformanceExecution, NeedsReplay,
        // NeedsObjectCentricQueryExecution, RebuildingProcessMiningLocally
    }
}
```

### Contract

**What the hook must implement:**
- `candidate(&self) → GraduationCandidate`
- Name the reason (enum variant) the value must graduate
- Provide a human-readable subject (what is graduating)
- Provide an evidence reference (opaque string, e.g., hash or URI)
- Ensure `is_grounded()` returns `true` (no empty fields)

**What guarantees it provides:**
- Typed declaration of why compat layer cannot proceed further
- Evidence is tied to the graduation claim (via `evidence_ref`)
- Host (or engine intake) has a reviewable case to make graduation decision
- No silent escalation; every graduation is explicit

**Lifecycle guarantees:**
- Producing a candidate does **not** perform graduation
- Engine decides whether to accept, retry, or reject
- Candidate is immutable once produced

### Example Implementation

**From tests/graduation.rs (conceptual):**

```rust
struct PendingOcelLog {
    log: AdmittedOcelLog,
    digest: String,  // blake3 hash
}

impl GraduateToWasm4pm for PendingOcelLog {
    fn candidate(&self) -> GraduationCandidate {
        GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            format!("OCEL log, {} events, {} objects", 
                    self.log.event_count(), 
                    self.log.object_count()),
            format!("blake3:{}", self.digest),
        )
    }
}

// Usage:
let pending = PendingOcelLog { /* ... */ };
let candidate = pending.candidate();

if candidate.is_grounded() {
    match candidate.reason {
        GraduationReason::NeedsDiscovery => {
            let model = wasm4pm::discover(&pending.log, &candidate.evidence_ref)?;
            // ...
        }
        _ => {}
    }
} else {
    eprintln!("Candidate not grounded; missing subject or evidence_ref");
}
```

### Multi-Reason Example

```rust
impl GraduateToWasm4pm for ComputedConformanceResult {
    fn candidate(&self) -> GraduationCandidate {
        // After discovering the model, we need to *verify* conformance with an algorithm
        GraduationCandidate::new(
            GraduationReason::NeedsConformanceExecution,
            format!("Discovered model (Petri net, {} transitions) vs admitted log",
                    self.model.transition_count()),
            format!("ref:model-{}-log-{}", self.model.hash, self.log.hash),
        )
    }
}

// Another example: replay
impl GraduateToWasm4pm for LogReplayRequest {
    fn candidate(&self) -> GraduationCandidate {
        GraduationCandidate::new(
            GraduationReason::NeedsReplay,
            "Replay discovered model against event log",
            format!("model:{} log:{}", self.model_id, self.log_id),
        )
    }
}

// Another example: query execution
impl GraduateToWasm4pm for OcpqQuery {
    fn candidate(&self) -> GraduationCandidate {
        GraduationCandidate::new(
            GraduationReason::NeedsObjectCentricQueryExecution,
            format!("OCPQ: {}", self.query_text),
            format!("query-id:{}", self.id),
        )
    }
}
```

### Hook Points for Integration

1. **Engine intake:** Read candidate and route to appropriate service (discover, conform, replay, query, …)
2. **Graduation log:** Record every graduation with reason and subject
3. **Metrics:** Count graduations by reason; identify bottlenecks
4. **Grounding verifier:** Check that evidence_ref can be resolved to actual artifact
5. **Policy gate:** Reject graduation if reason is not on allowlist
6. **Feedback loop:** On graduation failure, return error with root cause

---

## 5. StrictCheck Trait (Strict Boundary Judgment)

**Location:** `src/strict.rs` (lines 208–296)

### Interface

```rust
pub trait StrictCheck {
    /// Check this declaration against the boundary covenant,
    /// collecting *all* named violations.
    fn check(&self) -> Result<(), Vec<StrictViolation>>;
}
```

### Support Type: ProcessBoundary

```rust
pub struct ProcessBoundary {
    pub kind: ProcessBoundaryKind,
    pub name: String,
    
    // Flags (set to true if the boundary meets the requirement)
    pub has_witness: bool,
    pub has_round_trip_fixture: bool,
    pub has_loss_policy: bool,
    pub has_conformance_fields: bool,
    pub has_receipt_shape: bool,
    pub has_refusal_path: bool,
    pub exports_raw_evidence: bool,
    pub hidden_pm_growth: bool,
}

pub enum ProcessBoundaryKind {
    EmitsEvents,
    EmitsObjectRelations,
    ImportsFormat,
    ExportsFormat,
    ClaimsConformance,
    ClaimsReceipt,
    ClaimsReplay,
    ClaimsProcessMiningSupport,
}

impl ProcessBoundary {
    pub fn fully_attested(kind: ProcessBoundaryKind, name: impl Into<String>) -> Self {
        // All flags set to true (fully compliant)
    }
}
```

### Support Type: StrictViolation

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrictViolation {
    MissingWitness,                 // Every boundary must name authority
    MissingRoundTripFixture,        // Format I/O must prove round-trip
    MissingLossPolicy,              // Lossy export must declare policy
    MissingConformanceFields,       // Conformance claim needs metrics
    MissingReceiptShape,            // Receipt claim needs envelope
    MissingRefusalPath,             // Serious boundary needs named refusal
    RawEvidenceExported,            // Raw must never cross a boundary
    HiddenProcessMiningGrowth,      // PM work must be explicit (graduation)
}

impl StrictViolation {
    pub fn law(&self) -> &'static str {
        // Returns human-readable law name
    }
}
```

### Contract

**What the hook must implement:**
- `check(&self) → Result<(), Vec<StrictViolation>>`
- Return `Ok(())` if all flags match the boundary's requirements
- Return `Err(violations)` with **all** violations found (not just the first)

**What guarantees it provides:**
- Opt-in (feature-gated) enforcement of "paper-completeness"
- Host can verify every boundary meets documentary standards
- All violations returned at once (no multi-pass iteration needed)
- Named violations are specific to boundary kind

**Lifecycle guarantees:**
- Runtime check (unlike witness type-checking)
- Can be called at any point in the lifecycle
- Non-blocking (returns violations for audit; does not panic)

### Implementation Details (from src/strict.rs)

```rust
impl StrictCheck for ProcessBoundary {
    fn check(&self) -> Result<(), Vec<StrictViolation>> {
        use ProcessBoundaryKind as K;
        let mut v = Vec::new();

        // Hidden growth and raw-evidence export are refused for ANY boundary kind.
        if self.hidden_pm_growth {
            v.push(StrictViolation::HiddenProcessMiningGrowth);
        }
        if self.exports_raw_evidence {
            v.push(StrictViolation::RawEvidenceExported);
        }

        // A witness is owed by every boundary that emits or translates structure.
        let owes_witness = matches!(
            self.kind,
            K::EmitsEvents
                | K::EmitsObjectRelations
                | K::ImportsFormat
                | K::ExportsFormat
                | K::ClaimsReceipt
        );
        if owes_witness && !self.has_witness {
            v.push(StrictViolation::MissingWitness);
        }

        // Import/export owe a round-trip fixture.
        if matches!(self.kind, K::ImportsFormat | K::ExportsFormat) && !self.has_round_trip_fixture {
            v.push(StrictViolation::MissingRoundTripFixture);
        }

        // Export owes a loss policy.
        if matches!(self.kind, K::ExportsFormat) && !self.has_loss_policy {
            v.push(StrictViolation::MissingLossPolicy);
        }

        // Conformance claims owe conformance fields.
        if matches!(self.kind, K::ClaimsConformance) && !self.has_conformance_fields {
            v.push(StrictViolation::MissingConformanceFields);
        }

        // Receipt claims owe a receipt shape.
        if matches!(self.kind, K::ClaimsReceipt) && !self.has_receipt_shape {
            v.push(StrictViolation::MissingReceiptShape);
        }

        // Every serious, trust-bearing boundary owes a first-class refusal path.
        let owes_refusal = matches!(
            self.kind,
            K::ImportsFormat
                | K::ExportsFormat
                | K::ClaimsConformance
                | K::ClaimsReceipt
                | K::ClaimsReplay
                | K::ClaimsProcessMiningSupport
        );
        if owes_refusal && !self.has_refusal_path {
            v.push(StrictViolation::MissingRefusalPath);
        }

        if v.is_empty() {
            Ok(())
        } else {
            Err(v)
        }
    }
}
```

### Example Usage

```rust
#[cfg(feature = "strict")]
#[test]
fn strict_export_boundary_must_have_loss_policy() {
    let mut b = ProcessBoundary::fully_attested(
        ProcessBoundaryKind::ExportsFormat,
        "xes-out"
    );
    b.has_loss_policy = false;  // Violate the law
    
    let violations = b.check().unwrap_err();
    assert!(violations.contains(&StrictViolation::MissingLossPolicy));
}

#[cfg(feature = "strict")]
#[test]
fn strict_check_accumulates_all_violations() {
    let mut b = ProcessBoundary::fully_attested(
        ProcessBoundaryKind::ImportsFormat,
        "ocel-in"
    );
    b.has_witness = false;
    b.has_round_trip_fixture = false;
    b.has_refusal_path = false;
    
    let violations = b.check().unwrap_err();
    assert_eq!(violations.len(), 3);
    assert!(violations.contains(&StrictViolation::MissingWitness));
    assert!(violations.contains(&StrictViolation::MissingRoundTripFixture));
    assert!(violations.contains(&StrictViolation::MissingRefusalPath));
}
```

### Hook Points for Integration

1. **Pre-release gate:** Run before graduation; reject if violations > 0
2. **Documentation audit:** Verify every boundary is documented in specs
3. **Linter:** As part of CI; fail build if violations introduced
4. **Health dashboard:** Track violation count over time
5. **Capability audit:** Ensure declared features are actually implemented

---

## 6. WellShaped Trait (Receipt Verification)

**Location:** `src/receipt.rs`

### Interface

```rust
pub trait WellShaped {
    /// Check that this envelope has all required fields (witness, digest, replay_hint present).
    fn well_shaped(&self) -> bool;
}
```

### Support Type: ReceiptEnvelope

```rust
pub struct ReceiptEnvelope {
    pub subject: String,           // What is being receipted
    pub witness: String,           // Authority name
    pub digest: Digest,            // Content hash
    pub replay_hint: ReplayHint,   // How to verify/replay
}

pub struct Digest {
    // opaque content
}

pub struct ReplayHint {
    // opaque recipe
}

impl ReceiptEnvelope {
    pub fn new(
        subject: impl Into<String>,
        witness: impl Into<String>,
        digest: Digest,
        replay_hint: ReplayHint,
    ) -> Self { /* ... */ }

    pub fn well_shaped(&self) -> bool {
        // All fields non-empty and present
    }
}

pub struct ReceiptChain {
    pub head: ReceiptEnvelope,
    pub prior: Option<Box<ReceiptChain>>,  // Linked-list proof chain
}
```

### Contract

**What the hook must implement:**
- `well_shaped(&self) → bool`
- Check that witness, digest, replay_hint are all **present** (not empty)
- Return `true` only if shape is valid for use

**What guarantees it provides:**
- Structural check (presence of fields, not semantic validity)
- Host can reject malformed receipts before using them
- No validation of digest correctness or replay success
- Fast check (no hashing, signing, or cryptography)

**Lifecycle guarantees:**
- Can be called at any time on a receipt envelope
- Does not modify the receipt
- Result may change if receipt is mutated

### Example Implementation

```rust
impl WellShaped for ReceiptEnvelope {
    fn well_shaped(&self) -> bool {
        !self.subject.trim().is_empty()
            && !self.witness.trim().is_empty()
            && self.digest.is_present()
            && self.replay_hint.is_present()
    }
}

impl WellShaped for ReceiptChain {
    fn well_shaped(&self) -> bool {
        self.head.well_shaped()
            && self.prior.as_ref().map_or(true, |p| p.well_shaped())
        // All envelopes in the chain must be well-shaped
    }
}
```

### Example Usage

```rust
// Host receives a receipt from external source
let receipt = receive_receipt_from_network();

// Shape check before use
if receipt.well_shaped() {
    // Receipt is structurally valid; proceed
    let candidate = GraduationCandidate::new(
        GraduationReason::NeedsReceipts,
        "Receipt verification",
        receipt.digest.to_string(),
    );
    wasm4pm::verify_receipt_chain(receipt, candidate)?;
} else {
    eprintln!("Receipt malformed; missing required fields");
}
```

### Hook Points for Integration

1. **Receipt intake:** Reject malformed receipts before processing
2. **Digest verifier:** After shape check, compute digest and compare
3. **Chain walker:** Traverse `prior` links and check each is well-shaped
4. **Proof store:** Before persisting, verify well_shaped()
5. **Recovery auditor:** On replay, log which replay_hints were used

---

## 7. Witness Algebra (Join, WithTop)

**Location:** `src/witness.rs` (lines 150–200+, conceptual)

### Interfaces (Simplified)

```rust
/// Compose two witnesses into a joint authority.
pub trait Join {
    type Output;
    fn join(&self, other: &impl Witness) -> Self::Output;
}

/// The top authority in a witness hierarchy.
pub trait WithTop {
    fn top() -> Self;  // The most permissive authority
}
```

### Contract

**What these hooks enable:**
- Reasoning about witness hierarchies (which standard is "higher" than another)
- Composing authorities (OCEL + XES = multi-format witness)
- Defining a top element (no restrictions)

**Example:**
```rust
// A boundary that accepts both OCEL and XES admits evidence witnessed by either
enum DualStandardBoundary {}

impl Admit for DualStandardBoundary {
    type Witness = Ocel20;  // Or Xes1849? Use a composed witness here.
    // ...
}
```

### Hook Points for Integration

1. **Authority composition:** Build complex witness types for multi-standard boundaries
2. **Hierarchy ordering:** Determine which authority subsumes another
3. **Lattice structure:** Compute witness meet/join for complex scenarios

---

## 8. Typestate Lifecycle (State Transitions)

**Location:** `src/state.rs` + `src/evidence.rs`

### Lifecycle States

```rust
pub enum Raw {}      // Untrusted
pub enum Parsed {}   // Well-formed
pub enum Admitted {} // Judged and accepted
pub enum Refused {}  // Judged and rejected
pub enum Projected {} // Lossy transform applied
pub enum Exportable {} // Ready to leave crate
pub enum Receipted {} // Wrapped in receipt envelope
```

### Evidence Carrier

```rust
pub struct Evidence<T, State: EvidenceState, W> {
    pub value: T,
    pub state: PhantomData<State>,    // Type-level lifecycle marker
    pub witness: PhantomData<W>,      // Type-level authority marker
}

impl<T, W> Evidence<T, Raw, W> {
    pub fn raw(value: T) -> Self { /* ... */ }
}

impl<T, W> Evidence<T, Raw, W> {
    pub fn into_parsed(self) -> Evidence<T, Parsed, W> { /* ... */ }
}

impl<T, W> Evidence<T, Parsed, W> {
    pub fn into_admitted(self, admission: Admission<T, W>) -> Evidence<T, Admitted, W> {
        // Only via Admit impl
    }
}

impl<T, W> Evidence<T, Admitted, W> {
    pub fn into_projected(self, report: LossReport<_, _, _>) -> Evidence<T, Projected, W> {
        // Only via Project impl
    }

    pub fn into_exportable(self) -> Evidence<T, Exportable, W> { /* ... */ }

    pub fn into_receipted(self, receipt: ReceiptEnvelope) -> Evidence<T, Receipted, W> {
        // Only via Receipt wrapper
    }
}
```

### Contract

**What the type system enforces:**
- No backward transitions (can't go `Admitted` → `Raw`)
- No skipped stages (can't go `Raw` → `Receipted` without `Admitted`)
- Witnesses must be consistent across transitions

**What guarantees it provides:**
- Compile-time proof of legal lifecycle
- Impossible states are not representable
- Each transition is explicit and named

### Transition Diagram

```
Raw ──into_parsed──▶ Parsed ──admit()──▶ Admitted ─┬─into_projected──▶ Projected
  │                                        │        └─into_exportable─▶ Exportable
  │                                        └────────into_receipted───▶ Receipted
  └────────refuse()────────▶ Refused
```

### Hook Points for Integration

1. **Observability middleware:** Wrap transition methods to log state changes
2. **Metrics:** Count transitions per state; identify bottlenecks
3. **Assertion hooks:** Verify invariants hold at each state
4. **State machine visualization:** Generate diagrams from type-level states
5. **Graduation decision:** Inspect current state before graduation

---

## Cross-Cutting Integration Patterns

### Pattern 1: Admission + Witness + Graduation Pipeline

```rust
// 1. Raw evidence enters with witness
let raw_ev = Evidence::<OcelLog, Raw, Ocel20>::raw(parsed_bytes);

// 2. Admission gate (hook: log witness, reason if refused)
let admission = OcelAdmitter::admit(raw_ev)?;
let admitted_ev = admission.into_evidence();

// 3. Later: check if graduation is needed
let candidate = admitted_ev.value.candidate();
if candidate.reason.is_hard_signal() {
    // Escalate with grounded evidence
    wasm4pm::handle_graduation(candidate)?;
}
```

### Pattern 2: Strict Boundary Enforcement

```rust
#[cfg(feature = "strict")]
{
    let boundary = ProcessBoundary::fully_attested(
        ProcessBoundaryKind::ImportsFormat,
        "ocel-intake"
    );
    boundary.check()?;  // Enforces all strict laws
}
```

### Pattern 3: Lossy Projection with Loss Audit

```rust
let admitted_ocel = /* ... */;

let result = OcelToXesProjection::project(
    admitted_ocel.value,
    LossPolicy::AllowLossWithReport
);

match result {
    Ok(loss_report) => {
        // Audit trail: what was lost, when, why
        eprintln!("Projection: {}", loss_report.projection.as_str());
        eprintln!("Items lost: {}", loss_report.discarded.len());
        
        // Store report for later recovery
        loss_db.insert(loss_report);
    }
    Err(refusal) => {
        // Named refusal; log and handle
        eprintln!("Refusal: {:?}", refusal);
    }
}
```

### Pattern 4: Receipt Chain Verification

```rust
let receipt = ReceiptEnvelope::new(
    format!("case-{}", case_id),
    Ocel20::KEY,
    Digest::compute(blake3(&serialize(&admitted_log))),
    ReplayHint::new(format!("run:plan#{}", run_id))
);

if receipt.well_shaped() {
    let receipted_ev = admitted_ev.into_receipted(receipt);
    
    // Later: verify chain integrity
    verify_receipt_chain(&receipted_ev.value.receipt)?;
} else {
    eprintln!("Receipt envelope malformed");
}
```

---

## Test Fixtures

All hooks have explicit test cases in:

| Module | Test File | Coverage |
|--------|-----------|----------|
| `admission.rs` | `tests/admission_refusal.rs` | `Admit` trait, named refusals, witness metadata |
| `loss.rs` | `tests/loss_projection.rs` | `Project` trait, `LossPolicy`, `LossReport` |
| `engine_bridge.rs` | `tests/graduation.rs` | `GraduateToWasm4pm`, `GraduationCandidate` |
| `strict.rs` | `tests/strict_contracts.rs` | `StrictCheck`, `ProcessBoundary` |
| `receipt.rs` | `tests/receipt_shapes.rs` | `WellShaped`, `ReceiptEnvelope` |
| `witness.rs` | `tests/witness_authority.rs` | `Witness` trait, metadata, families |
| `evidence.rs` | `tests/evidence_lifecycle.rs` | State transitions, `Evidence` carrier |

---

## Summary Table

| Hook | Trait | Location | Input | Output | Lifecycle | Fails By |
|------|-------|----------|-------|--------|-----------|----------|
| **Witness** | `Witness` | `witness.rs` | — | Metadata constants | Type-level; immutable | N/A (zero-sized) |
| **Admission** | `Admit` | `admission.rs` | `Evidence<Raw, Witness>` | `Admission<Witness>` \| `Refusal<Reason, Witness>` | `Raw → Admitted` | Named `Reason` enum |
| **Projection** | `Project` | `loss.rs` | `(Self, LossPolicy)` | `LossReport<Lost>` \| `Refusal<Reason>` | Lossy transform | Named `Reason` enum |
| **Graduation** | `GraduateToWasm4pm` | `engine_bridge.rs` | `Self` | `GraduationCandidate` | Candidate production | `is_grounded()` false |
| **Strict Check** | `StrictCheck` | `strict.rs` | `ProcessBoundary` | `Ok(())` \| `Err(Vec<StrictViolation>)` | Runtime check (feature-gated) | Named `Violation` enums |
| **Receipt Shape** | `WellShaped` | `receipt.rs` | `ReceiptEnvelope` | `bool` | Shape validation | Presence check |
| **Witness Algebra** | `Join`, `WithTop` | `witness.rs` | Two witnesses \| — | Composed witness \| Top | Metadata algebra | Authority ordering |
| **Typestate** | (PhantomData) | `state.rs` + `evidence.rs` | `Evidence<T, State, W>` | `Evidence<T, NextState, W>` | Compile-time FSM | Impossible types |

---

## Glossary

**Witness:** Zero-sized type-level marker naming a standard, paper, grammar, or law.  
**Evidence:** Carrier struct bundling a value with state (Raw, Admitted, etc.) and witness tags.  
**Admission:** First-class verdict that raw evidence passed judgment.  
**Refusal:** First-class verdict that evidence failed, with named law reason.  
**Graduation Candidate:** Typed case that compat value should enter the `wasm4pm` engine.  
**Loss Report:** Accountable record of discarded evidence (projection name, policy, items).  
**Structure-only:** No execution engines, no discovery/conformance/replay, no cryptography.  
**Typestate:** State machine enforced by Rust type system (illegal states not representable).

---

**End of Hook Contracts Document**
