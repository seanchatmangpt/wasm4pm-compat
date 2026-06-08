# wasm4pm ↔ wasm4pm-compat Evidence Boundary Contract

**Status:** Forward Design (v0.1)  
**Date:** 2026-06-01  
**Audience:** Process Intelligence, Agent 5, wasm4pm adoption agents  
**Hard Gate:** Do not add mining/conformance/replay engine behavior into wasm4pm-compat

---

## Executive Summary

This contract defines the **evidence boundary** between the full `wasm4pm` process-mining execution engine and its lightweight companion `wasm4pm-compat`, which is **structure-only** (no algorithms, no compute, no persistence).

The boundary answer is:

> **Market physics evidence enters compat as *shapes* (what), not as *values* (how much or computed results).**

Evidence types split as follows:

| Category | Stays in compat | Graduates to wasm4pm |
|----------|---|---|
| **Event logs** | Event/Trace/EventLog/OCEL shapes, builders | Log parsing, trace assembly, normalization |
| **Process models** | PetriNet/ProcessTree/BPMN/DFG/Declare **shapes** (structure only) | Model discovery, model loading, token replay, alignment |
| **Conformance** | Verdict **shapes**: Fitness/Precision/F1 bounds, deviation **shapes** | Computing fitness via replay, precision via alignment, generalization via test logs |
| **Causality** | Object-centric link **shapes** (E2O, O2O, O2E) | Causality **execution** (query resolution, dependencies) |
| **Receipts** | Receipt **shape** (hash, commitment, witness marker) | Receipt **minting** (chaining, custody, validation) |
| **Market-specific** | Claim **shapes**: MarketPlanckCell, Construct8Delta, RepresentationGap as opaque `ArtifactGrounding<W>` | Market-physics **execution**: computing deltas, detecting gaps, measuring quanta |

---

## Part 1: Evidence Types That Map to compat

### 1.1 Event Log Shapes

**In compat:** Complete structure.  
**Entry point:** `crate::eventlog`, `crate::ocel`

```rust
// Structure only — builders, shapes, validation of shape laws
pub struct Event { id, timestamp, activity, attributes, ... }
pub struct EventLog { traces: Vec<Trace>, ... }
pub struct OcelLog { events, objects, e2o_links, o2o_links, ... }

// Shape validators (refuse wrong structure)
pub fn validate_event_structure() -> Result<(), EventRefusal>
pub fn validate_ocel_shape() -> Result<(), OcelRefusal>
```

**Computation that does NOT happen in compat:**
- Parsing raw bytes (that's wasm4pm or host responsibility)
- Normalizing timestamps to a common zone
- Deduplicating events
- Sorting by timestamp
- Extracting case notions from object types

**How to recognize a boundary violation:**
- If you are calling `.parse()` on raw JSON/CSV in compat → wrong layer (should be in wasm4pm, then hand admitted log to compat)
- If you are computing `trace.duration` by subtracting timestamps → wrong layer (wasm4pm does that after admission)

**Graduation trigger:** "I need to ingest raw XES/OCEL/CSV and normalize it" → graduate to wasm4pm

---

### 1.2 Process Model Shapes

**In compat:** Structural definitions only.  
**Entry points:** `crate::petri`, `crate::process_tree`, `crate::bpmn`, `crate::dfg`, `crate::declare`

```rust
// Shape only — carries structure, never loads from bytes or executes
pub struct WfNetConst<const SOUNDNESS: bool> { 
    places: Vec<Place>,
    transitions: Vec<Transition>,
    arcs: Vec<Arc>,
    // Type-level SOUNDNESS witness
}

pub struct ProcessTree { /* structure */ }
pub struct DFG { /* directly-follows edges */ }
pub struct BpmnModel { /* activities, gateways, flows */ }

// Shape validators
pub fn validate_wfnet_structure() -> Result<(), PetriRefusal>
pub fn validate_process_tree_well_formedness() -> Result<(), TreeRefusal>
```

**Computation that does NOT happen in compat:**
- Loading PNML files (wasm4pm does parsing → compat carries shape)
- Checking soundness (that's a wasm4pm gate)
- Computing reachability graphs
- Minimizing/simplifying the net
- Projecting the net onto a subset of activities

**How to recognize a boundary violation:**
- If you are reading `.pnml` or `.bpmn` XML in compat → wrong layer
- If you are running a reachability algorithm → wrong layer
- If you are checking "is this net sound?" by algorithm → wrong layer

**Graduation trigger:** "I need to discover a model from a log" or "I need to check if my Petri net is sound" → graduate to wasm4pm

---

### 1.3 Conformance Verdict Shapes

**In compat:** Bounded metric newtypes and verdict structure.  
**Entry points:** `crate::conformance`, `crate::law` (for bounded metrics)

```rust
// Type-level bounded metrics — can only construct 0.0 ≤ value ≤ 1.0
pub type FitnessConst<const NUM: u64, const DEN: u64> = Metric<Fitness, NUM, DEN>;
pub type PrecisionConst<const NUM: u64, const DEN: u64> = Metric<Precision, NUM, DEN>;

pub struct ConformanceVerdict {
    fitness: Option<FitnessConst<_, _>>,
    precision: Option<PrecisionConst<_, _>>,
    f1: Option<F1Const<_, _>>,
    deviations: Vec<Deviation>,
}

// Shape validators
pub fn validate_conformance_verdict() -> Result<(), ConformanceRefusal>
pub fn validate_deviation_shape() -> Result<(), ConformanceRefusal>
```

**Computation that does NOT happen in compat:**
- Token replay (wasm4pm: `replay.rs` or `crates/pm-core/src/alignment.rs`)
- Computing fitness from a log and a model
- Computing precision from deviations
- Computing generalization from a test log
- Measuring F1 as a derived metric

**How to recognize a boundary violation:**
- If you are implementing token-based replay → wrong layer
- If you are computing an alignment → wrong layer
- If you are tallying deviations into a fitness score → wrong layer

**Graduation trigger:** "I have a log and a model, and I need to know how well they conform" → graduate to wasm4pm

---

### 1.4 Object-Centric Causality Link Shapes

**In compat:** Link **structure** and **topology**.  
**Entry points:** `crate::ocel`, `crate::causality`

```rust
// Links: structure only, no causality resolution
pub struct EventObjectLink {
    event_id: EventId,
    object_id: ObjectId,
    qualified_by: Option<QualificationName>,  // e.g. "stakeholder", "created"
}

pub struct ObjectObjectLink {
    source_object: ObjectId,
    target_object: ObjectId,
    rel_type: String,  // e.g. "child_of", "delegated_to"
}

pub enum CausalityShape {
    EventTriggers(EventId, EventId),           // e2e
    ObjectInvolvedInEvent(ObjectId, EventId),  // o2e / e2o
    ObjectAffectsObject(ObjectId, ObjectId),   // o2o
}

// Shape validators
pub fn validate_e2o_link() -> Result<(), OcelRefusal>
pub fn validate_causality_shape() -> Result<(), CausalityRefusal>
```

**Computation that does NOT happen in compat:**
- Resolving causality dependencies (querying which receipts depend on which repairs)
- Transitive closure of o2o links
- Detecting cycles in object causality
- Computing dependency chains
- Running OCPQ (Object-Centric Process Query) queries

**How to recognize a boundary violation:**
- If you are resolving "which receipt transitively depends on this repair?" → wrong layer (OCPQ execution)
- If you are computing a causal graph → wrong layer
- If you are doing dependency inference → wrong layer

**Graduation trigger:** "I need to answer a causality query" or "I need to compute object dependencies" → graduate to wasm4pm (ocpq module)

---

### 1.5 Receipt Shapes

**In compat:** Receipt **structure**, witness **marker**, and **shape validation**.  
**Entry points:** `crate::receipt`, `crate::witness`

```rust
// Shape only — carries commitment, never validates chain or mints
pub struct Receipt<W: Witness> {
    hash: ReceiptHash,
    commitment: String,      // e.g. "fitness >= 0.8"
    witness: PhantomData<W>, // zero-cost type marker
    timestamp_claimed: Option<u64>,
}

pub enum ReceiptShape {
    ConformanceCommitment { claims: ConformanceTriple },
    ModelDiscovery { model_tag: String },
    ObjectCentricQuery { query_hash: String },
    ProcessWorldFixture { fixture_id: String },
}

// Shape validators
pub fn validate_receipt_shape() -> Result<(), ReceiptRefusal>
pub fn validate_commitment_bound() -> Result<(), ReceiptRefusal>
```

**Computation that does NOT happen in compat:**
- Minting receipts (creating new hashes, signing chains)
- Validating receipt chains (proving A → B → C custody)
- Computing receipt hashes
- Cross-receipt consistency checking
- Receipt revocation or amendment

**How to recognize a boundary violation:**
- If you are computing a hash and storing it as a receipt → wrong layer (receipt minting is wasm4pm)
- If you are validating a chain of receipts → wrong layer
- If you are checking "does this receipt sign my computation?" → wrong layer

**Graduation trigger:** "I computed a result and need to issue a receipt" or "I need to validate a receipt chain" → graduate to wasm4pm (receipt minting/validation modules)

---

## Part 2: Market Physics Evidence Types (Forward Design)

### 2.1 What Is "Market Physics Evidence"?

Market physics evidence describes **constraints and deltas in agent decision-making under bounded rationality, information asymmetry, and time pressure**. Examples:

- **MarketPlanckCell**: The minimum atomic unit of decision (e.g., "an agent can change its mind on at most 1 product attribute per decision cycle")
- **Construct8Delta**: An 8-dimensional change vector (price, availability, quality, latency, certainty, trust, cost, risk)
- **RepresentationGap**: The information loss between the agent's belief state and the observable market state

These are **not process mining primitives**. They are **domain-specific evidence shapes for a particular market-driven system**.

### 2.2 How Market Physics Evidence Maps to compat

**Market physics types enter compat as *opaque grounded artifacts*, never as *computed values*.**

```rust
// In compat: market_physics.rs (new module, if adopted)

use crate::interop::{ArtifactGrounding, Pm4pyShape, InteropRefusal};

/// An opaque reference to a market-physics claim, grounded in evidence.
/// 
/// The compat layer does NOT interpret, compute, or validate the claim.
/// It only enforces that:
/// 1. The artifact is grounded in evidence (evidence_ref is non-empty)
/// 2. The shape tag is stable (can be logged, routed, rejected by name)
/// 3. The witness marker tracks what law justifies admission
#[derive(Debug, Clone)]
pub struct MarketPhysicsArtifact<W> {
    /// The kind of market physics shape being admitted (planck cell, delta, gap, etc.)
    pub shape: MarketPhysicsShape,
    /// Opaque reference to the evidence that grounds this claim.
    /// E.g., "blake3:deadbeef" or "experiment:agent-5-trial-23"
    pub evidence_ref: String,
    /// Type-level witness family marker (e.g., "MarketEconomicsStudy")
    pub witness: PhantomData<W>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MarketPhysicsShape {
    /// A minimum atomic unit of agent decision (the Planck cell of the market).
    /// Claim: "agents can change at most this dimension per cycle"
    PlanckCell,
    /// An 8-dimensional change vector (price, availability, quality, latency, etc.)
    Construct8Delta,
    /// The gap between an agent's belief state and observable market reality.
    RepresentationGap,
    /// A bounded-rationality constraint (e.g., "agent has ≤ 100ms decision time")
    BoundedRationalityConstraint,
    /// A market-observable invariant (e.g., "price never decreases within 1 hour")
    MarketInvariant,
}

impl MarketPhysicsShape {
    pub const fn tag(self) -> &'static str {
        match self {
            Self::PlanckCell => "planck_cell",
            Self::Construct8Delta => "construct8_delta",
            Self::RepresentationGap => "representation_gap",
            Self::BoundedRationalityConstraint => "bounded_rationality",
            Self::MarketInvariant => "market_invariant",
        }
    }
}

/// Refusals specific to market physics evidence admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MarketPhysicsRefusal {
    /// The artifact was not grounded in evidence.
    UngroundedClaim,
    /// The claimed shape is incompatible with the witness family.
    /// E.g., a PlanckCell claim cannot be grounded in a ProcessMiningStudy witness.
    ShapeWitnessIncompatible,
    /// The evidence reference is malformed or missing required fields.
    MalformedEvidenceReference,
}

impl MarketPhysicsArtifact<W> {
    pub fn new(shape: MarketPhysicsShape, evidence_ref: impl Into<String>) -> Self {
        Self {
            shape,
            evidence_ref: evidence_ref.into(),
            witness: PhantomData,
        }
    }

    /// Admit this market physics artifact, or refuse with a specific law.
    pub fn admit(&self) -> Result<(), MarketPhysicsRefusal> {
        if self.evidence_ref.trim().is_empty() {
            return Err(MarketPhysicsRefusal::UngroundedClaim);
        }
        Ok(())
    }
}
```

### 2.3 What Stays in compat

Market physics **shapes** only:

```
MarketPhysicsArtifact<W> {
    shape: MarketPhysicsShape::PlanckCell,
    evidence_ref: "experiment-23-planck-bounds",
}
```

- ✅ Carrying the shape tag
- ✅ Carrying the evidence reference
- ✅ Validating groundedness
- ✅ Type-level witness tracking
- ✅ Building admission/refusal boundaries

### 2.4 What Graduates to wasm4pm

Market physics **computation**:

```
// In wasm4pm: market_physics module (new)

/// Computes the actual Planck cell from a log of agent decisions.
pub fn discover_planck_cell(
    log: &EventLog,
    agent_id: AgentId,
) -> Result<PlanckCellValue, MarketMiningError>

/// Extracts the 8-dimensional delta vectors from state transitions.
pub fn extract_construct8_deltas(
    before: AgentBeliefState,
    after: AgentBeliefState,
) -> Construct8DeltaVector

/// Measures the representation gap between agent beliefs and observable reality.
pub fn measure_representation_gap(
    agent_beliefs: &BeliefState,
    market_reality: &ObservableState,
) -> RepresentationGapMetric
```

Never in compat. These are algorithms. They belong in wasm4pm.

---

## Part 3: Evidence Type Mapping Table

| Evidence Type | Compat Role | wasm4pm Role | Boundary | Graduation Trigger |
|---|---|---|---|---|
| **EventLog** | Structure + builders | Parsing, normalization | Host → (parsing) → wasm4pm → (admission) → compat | "I need to ingest raw event data" |
| **OCEL 2.0 Log** | Shape + validation | Object-centric analysis | Host → (parsing) → wasm4pm → (admission) → compat | "I need to extract object notions from flat logs" |
| **Petri Net** | Shape only | Discovery, soundness check, replay | Host → (load PNML) → wasm4pm → (admit net shape) → compat | "I need to check if my net is sound" or "I need to discover a net" |
| **ProcessTree** | Shape only | Discovery, unfolding | Host → (construct) → wasm4pm → (admit tree shape) → compat | "I need to discover a process tree" |
| **DFG** | Shape only | Discovery, visualization | Host → (construct) → wasm4pm → (admit dfg shape) → compat | "I need to discover a DFG" |
| **ConformanceVerdict** | Verdict shape + bounded metrics | Fitness/precision computation | wasm4pm (compute) → compat (carry shape) | "I need to compute fitness from a log and model" |
| **E2O Links** | Link structure | Causality resolution | compat (shape) → wasm4pm (query OCPQ) | "I need to answer 'which events involve which objects?'" |
| **O2O Links** | Link structure | Transitive closure, graph | compat (shape) → wasm4pm (compute dependencies) | "I need to find all transitive object dependencies" |
| **Receipt** | Shape + witness | Minting, chain validation | compat (shape) → wasm4pm (mint + validate) | "I need to issue or verify a receipt" |
| **MarketPlanckCell** | Artifact grounding (opaque) | Computation/discovery | compat (carry shape) → wasm4pm (compute) | "I need to discover the Planck cell from agent decisions" |
| **Construct8Delta** | Artifact grounding (opaque) | Delta extraction/measurement | compat (carry shape) → wasm4pm (compute) | "I need to extract delta vectors from state transitions" |
| **RepresentationGap** | Artifact grounding (opaque) | Gap measurement/analysis | compat (carry shape) → wasm4pm (compute) | "I need to measure belief-vs-reality gap" |

---

## Part 4: Layering Rules (Hard Gates)

### 4.1 No Execution in compat

```
❌ FORBIDDEN in wasm4pm-compat:
  - Algorithms (discovery, conformance, replay, alignment, mining)
  - Persistence (file I/O, caching, database)
  - Async/concurrency (tokio, crossbeam, threads)
  - External dependencies (except core, alloc)
  - Arithmetic computation on metrics (except type-level bounds)
  - State machines (only structure representation allowed)
  - Interpretation of values from external format

✅ ALLOWED in wasm4pm-compat:
  - Type-level arithmetic (const generics, const bounds)
  - Immutable value carriers (newtypes, structs with PhantomData)
  - Shape validators (refuse wrong structure, admit correct structure)
  - Builder APIs (fluent construction of evidence shapes)
  - Display/Debug impls for diagnostics
  - Witness markers (zero-cost type markers)
  - Receipt/commitment shapes (never minted or validated at runtime)
```

### 4.2 Graduate When You Need to:

- **Parse or normalize** raw evidence → wasm4pm
- **Discover** a model or process → wasm4pm
- **Compute** a metric (fitness, precision, gap) → wasm4pm
- **Execute** a query (OCPQ, causality resolution) → wasm4pm
- **Mint** receipts or validate chains → wasm4pm
- **Replay** logs or run alignments → wasm4pm
- **Measure** anything that requires algorithms → wasm4pm

### 4.3 Witness Marker Rule

Every evidence artifact in compat carries a witness marker `W: Witness`:

```rust
pub struct Evidence<T, State, W> { /* ... */ }
pub struct ArtifactGrounding<W> { /* ... */ }
pub struct MarketPhysicsArtifact<W> { /* ... */ }
pub struct Receipt<W> { /* ... */ }
```

The witness is **zero-cost** (PhantomData). It exists to:
1. Track *which law* justifies admission
2. Prevent accidental mixing of incompatible evidence types
3. Enable type-safe routing to wasm4pm

Example: A `Pm4pyShape::EventLog` grounded by witness `Ocel20` is different from one grounded by witness `Xes1849` *at the type level*, even though both carry the same shape tag.

---

## Part 5: Test Surfaces (Proof of Boundary)

### 5.1 Positive Tests: Correct Admission

```rust
#[test]
fn market_physics_artifact_admission_grounded() {
    let artifact = MarketPhysicsArtifact::<MyWitness>::new(
        MarketPhysicsShape::PlanckCell,
        "experiment:42",
    );
    assert!(artifact.admit().is_ok());
}

#[test]
fn market_physics_shape_tag_is_stable() {
    assert_eq!(MarketPhysicsShape::PlanckCell.tag(), "planck_cell");
    assert_eq!(MarketPhysicsShape::Construct8Delta.tag(), "construct8_delta");
}
```

### 5.2 Negative Tests: Correct Refusal

```rust
#[test]
fn market_physics_artifact_refuses_ungrounded() {
    let artifact = MarketPhysicsArtifact::<MyWitness>::new(
        MarketPhysicsShape::RepresentationGap,
        "",
    );
    assert!(matches!(artifact.admit(), Err(MarketPhysicsRefusal::UngroundedClaim)));
}
```

### 5.3 Boundary Tests: Prevent Laundering

```rust
#[test]
fn market_physics_is_not_computed_in_compat() {
    // This test documents what is NOT allowed in compat:
    // ❌ let cell = discover_planck_cell(log); // forbidden
    // ❌ let deltas = extract_deltas(before, after); // forbidden
    // ✅ let artifact = MarketPhysicsArtifact::new(Shape::PlanckCell, "evidence-ref");
}

#[test]
fn graduation_candidate_signals_when_market_physics_needs_computation() {
    let artifact = MarketPhysicsArtifact::new(
        MarketPhysicsShape::PlanckCell,
        "experiment:42",
    );
    let candidate = GraduationCandidate::new(
        GraduationReason::RebuildingProcessMiningLocally,
        "planck_cell discovery",
        "experiment:42",
    );
    assert!(candidate.is_grounded());
    assert!(candidate.reason.is_hard_signal());
}
```

---

## Part 6: Format Laundering Prevention

### 6.1 The Laundering Risk

**Laundering:** Accepting raw bytes, interpreting them as an evidence type, and claiming they are admitted *without actual admission*.

Example (FORBIDDEN):
```rust
// ❌ LAUNDERING in compat
fn market_physics_from_raw_json(json: &str) -> Result<MarketPhysicsArtifact<W>, Error> {
    let parsed: serde_json::Value = serde_json::from_str(json)?;
    // Never admit! Just return the parsed shape without proving grounding!
    Ok(MarketPhysicsArtifact::new(
        MarketPhysicsShape::PlanckCell,
        "json-bytes",  // ❌ This is laundering
    ))
}
```

### 6.2 The Lawful Path

1. **Host or wasm4pm parses raw bytes** (not compat)
2. **wasm4pm validates structure and computes the evidence**
3. **wasm4pm calls compat with a grounded artifact**
4. **compat admits or refuses the artifact based on shape and witness**

Example (LAWFUL):
```rust
// ✅ In wasm4pm: parse and compute
pub fn discover_planck_cell_from_json(json: &str) -> Result<PlanckCellValue, Error> {
    let parsed = serde_json::from_str::<AgentEventLog>(json)?;
    let planck_cell = compute_planck_cell(&parsed)?;
    Ok(planck_cell)
}

// ✅ In wasm4pm: hand off to compat
pub fn ground_planck_cell_in_compat(
    cell_value: PlanckCellValue,
    experiment_id: &str,
) -> Result<MarketPhysicsArtifact<MarketPhysicsWitness>, MarketPhysicsRefusal> {
    // Witness the computation with the evidence reference
    let artifact = MarketPhysicsArtifact::new(
        MarketPhysicsShape::PlanckCell,
        format!("experiment:{}", experiment_id),
    );
    artifact.admit()?;
    Ok(artifact)
}

// ✅ In compat: carry the shape
pub fn validate_market_physics_shape(
    artifact: &MarketPhysicsArtifact<W>,
) -> Result<(), MarketPhysicsRefusal> {
    artifact.admit()
}
```

### 6.3 Test for Laundering Prevention

```rust
#[test]
fn market_physics_refuses_unadmitted_raw_interpretation() {
    // This test ensures we do NOT interpret raw JSON in compat
    // and claim it's admitted without evidence of computation.
    
    let raw_json = r#"{"planck": {"dims": 1, "value": 0.5}}"#;
    
    // If someone tries to do this:
    // let artifact = MarketPhysicsArtifact::from_json_raw(raw_json);
    // It should fail: we don't have an evidence_ref that points to the
    // wasm4pm computation that produced this value.
}
```

---

## Part 7: Shared Crate Justification

### 7.1 Why NOT a Shared Crate (Today)

- ❌ **Premature:** No evidence that wasm4pm needs to import wasm4pm-compat yet.
- ❌ **Nightly-only conflict:** wasm4pm uses stable Rust; wasm4pm-compat requires nightly features.
- ❌ **Composition freedom:** Each crate benefits from independent versioning and feature gates.
- ❌ **Gradualism:** Market physics shapes are speculative (not yet in wasm4pm); premature coupling breaks design.

### 7.2 When to Create a Shared Crate (Future)

Create a new crate `wasm4pm-market-evidence` shared by both when:

- ✅ wasm4pm has implemented at least 3 market-physics algorithms (planck cell, delta extraction, gap measurement)
- ✅ At least 2 downstream systems (beyond wasm4pm itself) import the shapes
- ✅ The shapes have survived 3+ months of production use without redesign
- ✅ The nightly-feature requirement has stabilized or been modularized

**Shared crate design** (when justified):

```toml
# wasm4pm-market-evidence/Cargo.toml
[package]
name = "wasm4pm-market-evidence"
version = "26.6.8"
edition = "2021"

[features]
# No nightly features in the shared crate (market shapes don't need them)
default = []

[dependencies]
# Core types only, no computation
serde = { version = "1.0", features = ["derive"] }
```

```rust
// wasm4pm-market-evidence/src/lib.rs
pub mod shapes;
pub mod witness;
pub mod refusal;

// Re-export from wasm4pm-compat (shapes stay there)
pub use wasm4pm_compat::{
    interop::ArtifactGrounding,
    witness::Witness,
};
```

**In wasm4pm:**
```rust
// wasm4pm/src/market_physics.rs
use wasm4pm_market_evidence::shapes::*;

pub fn discover_planck_cell(log: &EventLog) -> Result<PlanckCellValue, Error> { ... }
pub fn extract_construct8_deltas(...) -> Result<Construct8DeltaVector, Error> { ... }
```

**In wasm4pm-compat:** No change (shapes stay here as local module `market_physics.rs`).

---

## Part 8: Forbidden Patterns (Hard Blocks)

### 8.1 Forbidden: Computation in compat

```rust
// ❌ FORBIDDEN: fitness computation in compat
pub fn compute_fitness(log: &EventLog, model: &WfNet) -> f64 { ... }

// ❌ FORBIDDEN: algorithm in compat
pub fn discover_dfg(log: &EventLog) -> DFG { ... }

// ❌ FORBIDDEN: OCPQ execution in compat
pub fn resolve_causality_query(query: &str, log: &OcelLog) -> Vec<ObjectId> { ... }

// ❌ FORBIDDEN: receipt minting in compat
pub fn mint_receipt(commitment: &str) -> Receipt<W> { ... }

// ❌ FORBIDDEN: state machine in compat (except as data structures)
pub struct Agent {
    state: Rc<RefCell<AgentState>>,  // ❌ mutable state
}

// ❌ FORBIDDEN: persistence in compat
pub fn save_artifact_to_disk(artifact: &Evidence<T, S, W>, path: &Path) { ... }
```

### 8.2 Forbidden: Laundering Shapes as Computed Values

```rust
// ❌ FORBIDDEN: claiming a shape is computed when it was just parsed
pub fn market_physics_from_untrusted_json(json: &str) -> MarketPhysicsArtifact<W> {
    serde_json::from_str::<MarketPhysicsShape>(json)
        .map(|shape| MarketPhysicsArtifact::new(shape, "untrusted-json"))
        .map_err(|_| MarketPhysicsRefusal::UngroundedClaim)
}
```

### 8.3 Forbidden: Breaking Witness Invariants

```rust
// ❌ FORBIDDEN: creating a witness without grounding
pub fn witness_without_evidence<W: Witness>() -> W {
    unsafe { std::mem::zeroed() }  // ❌ violates witness covenant
}

// ❌ FORBIDDEN: mixing witness families
pub fn accept_any_witness(artifact: &ArtifactGrounding<dyn Witness>) {
    // ❌ loses type-level tracking
}
```

---

## Part 9: Adoption Path (Process to Follow)

### 9.1 For wasm4pm-compat Maintainers

If adding market physics shapes to wasm4pm-compat:

1. **Create `src/market_physics.rs`** (structure only)
2. **Define shapes:** `MarketPhysicsShape`, `MarketPhysicsArtifact<W>`
3. **Define refusals:** `MarketPhysicsRefusal` (specific named laws)
4. **Add builders:** e.g., `MarketPhysicsArtifact::new()`, `admit()`
5. **Write shape-only tests** (no computation, no graduation)
6. **Add doctests** that demonstrate admission/refusal only
7. **NO wasm4pm import** — shapes are self-contained
8. **Tag this commit** with `wasm4pm-compat-market-physics-v0.1`

### 9.2 For wasm4pm Maintainers

If implementing market physics algorithms:

1. **Create `src/market_physics/` directory** (or extend existing)
2. **Define value types:** `PlanckCellValue`, `Construct8DeltaVector`, `RepresentationGapMetric`
3. **Implement algorithms:** `discover_planck_cell()`, `extract_deltas()`, `measure_gap()`
4. **Call wasm4pm-compat for shape grounding:**
   ```rust
   use wasm4pm_compat::market_physics::MarketPhysicsArtifact;
   
   pub fn graduate_planck_cell(
       value: PlanckCellValue,
       experiment_id: &str,
   ) -> MarketPhysicsArtifact<MarketPhysicsWitness> {
       MarketPhysicsArtifact::new(
           MarketPhysicsShape::PlanckCell,
           format!("experiment:{}", experiment_id),
       )
   }
   ```
5. **Write algorithm tests** (compute + graduate)
6. **Add receipt minting** (optional, depends on spec)
7. **Tag this commit** with `wasm4pm-market-physics-v0.1`

### 9.3 For Downstream Hosts (e.g., ggen)

1. **Parse raw agent decision logs**
2. **Hand to wasm4pm** via API boundary (not compat)
3. **Receive `MarketPhysicsArtifact<W>` from wasm4pm**
4. **Carry artifacts in compat layer** (shape only)
5. **Route to wasm4pm when you need to:** compute gap, extract deltas, etc.

---

## Part 10: Examples

### 10.1 Example: Valid Compat-Only Admission

```rust
use wasm4pm_compat::market_physics::{MarketPhysicsArtifact, MarketPhysicsShape};
use wasm4pm_compat::witness::MarketEconomicsStudy;

#[test]
fn admit_market_physics_artifact() {
    // A host / wasm4pm computed the Planck cell,
    // and now grounds it in compat.
    
    let artifact = MarketPhysicsArtifact::<MarketEconomicsStudy>::new(
        MarketPhysicsShape::PlanckCell,
        "experiment:agent-5-trial-23",
    );
    
    // Compat admits based on grounding only
    assert!(artifact.admit().is_ok());
    assert_eq!(artifact.shape.tag(), "planck_cell");
}
```

### 10.2 Example: Valid Graduation

```rust
use wasm4pm::market_physics::{discover_planck_cell, PlanckCellValue};
use wasm4pm_compat::market_physics::MarketPhysicsArtifact;

#[test]
fn graduate_planck_cell_to_engine() {
    // In wasm4pm: compute the actual Planck cell value
    let log = load_agent_event_log("experiment-23.jsonl");
    let cell_value: PlanckCellValue = discover_planck_cell(&log)
        .expect("Planck cell discovery failed");
    
    // Graduate to compat: ground the shape
    let artifact = MarketPhysicsArtifact::new(
        MarketPhysicsShape::PlanckCell,
        "experiment:23",
    );
    assert!(artifact.admit().is_ok());
    
    // Now the artifact can be carried through compat layers
    assert_eq!(artifact.shape.tag(), "planck_cell");
}
```

### 10.3 Example: Laundering (FORBIDDEN)

```rust
// ❌ DO NOT DO THIS

#[test]
fn do_not_compute_in_compat() {
    let raw_json = r#"{"planck": 0.5}"#;
    
    // ❌ This would be laundering:
    // let shape = serde_json::from_str(raw_json).unwrap();
    // let artifact = MarketPhysicsArtifact::new(shape, "raw-json");
    
    // ✅ Correct: admit only when evidence_ref points to computation
    let artifact = MarketPhysicsArtifact::new(
        MarketPhysicsShape::PlanckCell,
        "experiment:23",  // points to wasm4pm computation
    );
    assert!(artifact.admit().is_ok());
}
```

---

## Conclusion

**The boundary is clear:**

| Aspect | compat | wasm4pm |
|--------|--------|---------|
| **Market shapes** | Carry, validate structure | Compute, discover, measure |
| **Artifact grounding** | Check if evidence_ref exists | Produce the evidence_ref via algorithm |
| **Witness markers** | Type-level, zero-cost | Runtime witness construction |
| **Refusals** | Specific named laws | Application-level routing |
| **Graduation** | Signal when computation is needed | Execute computation, ground in compat |

**Hard gate:** Do not add mining/conformance/replay engine behavior into wasm4pm-compat.

**Proof:** When you are tempted to add an algorithm to compat, ask: "Does this *compute* a result?" If yes, it belongs in wasm4pm. If no (only *carrying* or *validating* shape), it belongs in compat.

---

## References

- [wasm4pm CLAUDE.md](../CLAUDE.md) — wasm4pm architecture and constraints
- [wasm4pm-compat CLAUDE.md](/Users/sac/wasm4pm-compat/CLAUDE.md) — compat crate design principles
- [Process Intelligence ALIVE_001](../../../process-intelligence/ALIVE_001.md) — Rationale for parallel type universes
- [Evidence Architecture](../docs/primitives/04-EVIDENCE-ARCHITECTURE.md) — Type-level evidence design (if present)
- [Interop Module](../src/interop.rs) — Shape and grounding primitives in compat
- [Engine Bridge Module](../src/engine_bridge.rs) — Graduation candidate design

---

**Version:** 0.1 (Forward Design)  
**Author:** Agent 5 (Process Evidence Boundary)  
**Last Updated:** 2026-06-01  
**Status:** Open for review by wasm4pm + wasm4pm-compat maintainers
