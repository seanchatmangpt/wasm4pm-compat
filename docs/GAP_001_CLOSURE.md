# GAP_001 Closure: wasm4pm-compat Integration Strategy

**Date:** 2026-06-01  
**Status:** Authorization  
**Sealed By:** Process Intelligence ALIVE_001  

## Executive Summary

GAP_001 is the parallel-universe problem: `wasm4pm-compat` and `wasm4pm` maintain two separate type universes with no bridge. This document closes the gap by establishing:

1. **Import Strategy** — what type law compat exports for wasm4pm to import
2. **Witness Bridging** — how compat witness markers travel into the engine  
3. **Refusal Law Alignment** — which refusal shapes wasm4pm must respect
4. **Receipt Covenant** — how compat receipts become engine receipts
5. **Graduation Boundaries** — where structure ends and execution begins

## The Problem

**Current state:**
- `wasm4pm-compat` is structure-only (no algorithms, no execution)
- `wasm4pm` is execution-only (no structure law, raw types)
- They do not depend on each other
- A process form (e.g., OCEL log) has no typed path from compat admission to engine execution
- Refusal shapes are incompatible: compat uses named laws; wasm4pm uses string error messages

**Authorization:** The authorization to build this bridge lives in the Process Intelligence research program (ALIVE_001), which established that compat is the *type layer* and wasm4pm is the *execution layer*. They must not merge; they must be **coupled by explicit trait boundaries**.

## 1. Import Strategy: What wasm4pm Imports from wasm4pm-compat

### 1.1 Core Type Shapes (Zero-Cost Re-exports)

`wasm4pm` must re-export the following zero-cost shape types from compat, never redefine them:

| Shape | compat Module | Purpose | Why Re-export |
|-------|---------------|---------|---------------|
| `Evidence<T, State, W>` | `evidence` | Universal carrier | Single source of truth; all wasm4pm algorithms work on Evidence |
| `EventLog`, `Trace`, `Event` | `eventlog` | Base log structure | Canonical representation; no encoding variation |
| `OcelLog` | `ocel` | Object-centric logs | Canonical OCEL shape; wasm4pm must preserve it |
| `PetriNet`, `WfNet`, `WfNetConst<SOUNDNESS>` | `petri` | Petri net model | Type-law soundness carried as const generic |
| `ProcessTree`, `TreeProjectable` | `process_tree` | Process tree model | Sealed trait ensures only lawful projections |
| `DeclareModel`, `DeclareConstraint` | `declare` | Declare constraints | Canonical constraint shape |
| `DFG` (with `ActivityId`, `CaseId`, etc.) | `dfg`, `ids` | Directly-follows graphs | Typed ID wrappers prevent string mixing |
| `Between01<NUM, DEN>` | `law` | Fractional metrics | Compile-time fraction bounds |
| `Metric<KIND, NUM, DEN>` | `conformance` | Fitness/precision/etc. | Bounded 0-1 metrics with named kind |
| `State` tokens (`Raw`, `Parsed`, `Admitted`, `Refused`, `Projected`, `Exportable`, `Receipted`) | `state` | Typestate markers | Core lifecycle; no re-implementation |

**Implementation:** Add to `wasm4pm/Cargo.toml`:
```toml
[dependencies]
wasm4pm-compat = { path = "../wasm4pm-compat", features = ["wasm4pm"] }
```

**Policy:** If a type appears in both crates and they differ, the compat version is canonical. Redefining it in wasm4pm is a refusal trigger.

### 1.2 Witness Markers (Not Implementation)

`wasm4pm` must **import** all witness markers from compat but must **never** implement or "upgrade" them:

```rust
// In wasm4pm — this is correct:
use wasm4pm_compat::witness::{Witness, WitnessFamily};
use wasm4pm_compat::witness::{Ocel20, Xes1849, WfNetSoundnessPaper, /* ... */};

// This is WRONG — do not do this:
pub struct Ocel20 {} // FORBIDDEN: compat owns witness definitions
impl Witness for Ocel20 {} // FORBIDDEN: duplicate authority
```

The only wasm4pm-owned witness is the **ReplayAuthority** (already defined in `wasm4pm/src/replay/mod.rs`), which wraps compat witnesses for engine execution.

**Witness flow:**
```
compat: Evidence<Log, Admitted, Ocel20>
         ↓ (graduation)
wasm4pm: Evidence<Log, Admitted, Ocel20> + ReplayAuthority::execute(...)
```

The `Ocel20` witness travels *unchanged* into the engine.

### 1.3 Admission and Refusal Shapes

`wasm4pm` must support the following admission/refusal pattern from compat:

```rust
// From compat:
pub trait Admit {
    type Raw;
    type Admitted;
    type Reason;          // Named law, not a string
    type Witness;
    fn admit(raw: Evidence<Self::Raw, Raw, Self::Witness>)
        -> Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>>;
}

// wasm4pm extends this with execution:
pub trait ExecutionAdmit: Admit {
    type ExecutionError;   // Computation failures during real replay/conformance
    fn admit_and_execute(
        raw: Evidence<Self::Raw, Raw, Self::Witness>
    ) -> Result<
        (Admission<Self::Admitted, Self::Witness>, ExecutionContext),
        Either<Refusal<Self::Reason, Self::Witness>, ExecutionAdmit::ExecutionError>
    >;
}
```

The key: wasm4pm **layers execution on top** of compat's structure, never replaces it.

## 2. Witness Bridging: How Compat Witnesses Carry into Engine Execution

### 2.1 The Witness Journey

A compat witness names an *authority* (e.g., "OCEL 2.0", "WF-net Soundness"). When evidence graduates to wasm4pm, the witness is the **proof carrier**:

```
Stage 1 (compat):
  Evidence<OcelLog, Admitted, Ocel20>
  ↑ Witness says: "admitted against OCEL 2.0 standard"
  ↑ No execution happens; structure is checked only

Stage 2 (wasm4pm):
  GraduationCandidate { reason: NeedsObjectCentricQueryExecution, witness: Ocel20 }
  ↑ Graduation signals "I need to *execute* something against this standard"

Stage 3 (engine execution):
  ExecutionReceipt<ConformanceResult, Ocel20>
  ↑ Receipt carries the same witness so auditors can verify:
  ↑ "This result was computed respecting the Ocel20 law, not re-interpreted"
```

### 2.2 Witness Preservation Rules

When a compat value graduates to wasm4pm:

1. **Never drop the witness.** If compat receives `Evidence<Log, Admitted, Ocel20>`, wasm4pm must preserve `Ocel20` in the execution context.

2. **Never mix witnesses.** An OCEL log admitted against `Ocel20` cannot be executed under `Xes1849` without explicit re-admission (which compat provides, wasm4pm does not).

3. **Witness travels in metadata, not payload.** The witness is a `PhantomData` tag; it costs zero bytes but enforces type safety at compile time.

4. **Graduate-time witness export:** When exporting a receipt back to compat:
   ```rust
   pub struct ExecutionReceipt<T, W> {
       result: T,
       witness: PhantomData<W>,  // Same witness as input
       proof_chain: ProvenanceChain,
   }
   ```

### 2.3 Witness Metadata for Auditing

Compat defines witness metadata that wasm4pm engines must expose in diagnostics:

```rust
use wasm4pm_compat::witness::{Witness, WitnessFamily, Ocel20};

// In wasm4pm diagnostics:
fn audit_trail(log: &Evidence<OcelLog, Admitted, Ocel20>) -> String {
    format!(
        "Executed against {} ({}); family={:?}; year={}",
        Ocel20::TITLE,      // "OCEL 2.0"
        Ocel20::KEY,        // "ocel-2.0"
        Ocel20::FAMILY,     // WitnessFamily::Standard
        Ocel20::YEAR.unwrap_or(0),
    )
}
```

**Audit rule:** Every wasm4pm receipt must carry a human-readable trail naming the witness that governed execution.

## 3. Refusal Law Alignment: Which Shapes wasm4pm Must Respect

### 3.1 Two-Layer Refusal Model

Compat and wasm4pm have **different refusal taxonomies** because they operate at different layers:

**Layer 1: compat (structural refusal)**
```rust
enum OcelAdmissionRefusal {
    DanglingEventObjectLink,      // OCEL law: no orphan events
    MissingObjectIdentifier,       // OCEL law: objects need IDs
    CyclicObjectObjectLink,        // OCEL law: no object-dependency cycles
}

pub struct Refusal<R, W> {
    reason: R,                     // Named law, never a string
    witness: PhantomData<W>,
}
```

**Layer 2: wasm4pm (execution refusal)**
```rust
enum ExecutionFailure {
    StructuralRefusal(Box<dyn core::fmt::Debug>),  // Re-box compat Refusal<R, W>
    ComputationFailed(String),                      // Algorithm did not converge
    BudgetExceeded(String),                         // Latency/memory limit hit
    InternalStateError(String),                     // Invariant violated
}
```

### 3.2 Refusal Bridge Rule

When compat refuses, wasm4pm **must not retry or "fix" it**:

```rust
// WRONG:
match compat_admit(log) {
    Ok(admitted) => run_algorithm(admitted),
    Err(refusal) => {
        // Trying to work around a refusal is misrepresenting the law
        log.force_fix_ocel_links()  // FORBIDDEN
        run_algorithm(log_anyway)    // FORBIDDEN
    }
}

// CORRECT:
match compat_admit(log) {
    Ok(admitted) => run_algorithm(admitted),
    Err(refusal) => {
        log_refusal(refusal);
        return Err(ExecutionFailure::StructuralRefusal(refusal));
    }
}
```

The compat refusal is *not an error to recover from*. It is a *law boundary crossed*. If the law forbids it, wasm4pm cannot execute it.

### 3.3 Named Refusal Requirement

wasm4pm must **never use catch-all error strings** when compat provides named refusal reasons:

**WRONG:**
```rust
pub enum ParseError {
    Invalid(String),  // Too vague
}

pub fn parse_ocel(json: &str) -> Result<OcelLog, ParseError> {
    // ...
    Err(ParseError::Invalid("bad OCEL link format".into()))  // String, not law name
}
```

**CORRECT:**
```rust
pub enum OcelParseRefusal {
    DanglingEventObjectLink,       // Named law from compat
    MissingObjectIdentifier,
    CyclicObjectObjectLink,
}

pub fn parse_and_admit_ocel(json: &str)
    -> Result<
        Admission<OcelLog, Ocel20>,
        Refusal<OcelParseRefusal, Ocel20>
    >
{
    let raw = parse_json(json)?;
    compat_ocel_admit(raw)  // Use compat's Admit impl
}
```

**Audit rule:** Every ExecutionFailure that originates from compat must carry the compat Refusal as a named witness, never flatten it to a string.

## 4. Receipt Covenant: How Compat Receipts Become Engine Receipts

### 4.1 Receipt Shape Inheritance

compat defines receipt shapes; wasm4pm extends them with execution proof:

**In compat:**
```rust
pub struct ReceiptEnvelope<T, W> {
    payload: T,
    witness: PhantomData<W>,
    provenance: ProvenanceChain,
}
```

**In wasm4pm:**
```rust
pub struct ExecutionReceipt<T, W> {
    // Inherit from compat:
    payload: T,
    witness: PhantomData<W>,
    provenance: ProvenanceChain,  // Extended with execution chain
    
    // Add wasm4pm evidence:
    algorithm: AlgorithmTag,
    execution_time_ns: u64,
    version: String,
    proof_gate: Option<ProofGateResult>,
}

impl<T, W> From<ReceiptEnvelope<T, W>> for ExecutionReceipt<T, W> {
    fn from(env: ReceiptEnvelope<T, W>) -> Self {
        ExecutionReceipt {
            payload: env.payload,
            witness: env.witness,
            provenance: env.provenance,  // compat provenance carries forward
            algorithm: AlgorithmTag::Unknown,
            execution_time_ns: 0,
            version: VERSION.to_string(),
            proof_gate: None,
        }
    }
}
```

The compat receipt is **immutable foundation**; wasm4pm *builds on top* of it, never replaces it.

### 4.2 Receipt Chain Semantics

A receipt chain must prove:
1. **Compat admission chain** — structural checks that passed to admit the evidence
2. **Execution chain** — algorithms that ran, parameters used, time elapsed
3. **Refusal chain** — specific named laws broken (if refused at any stage)

```
Receipt {
    compat_provenance: {
        event: "admitted_ocel",
        witness: "ocel-2.0",
        checks: ["no_dangling_links", "valid_object_ids"],
    },
    execution_provenance: {
        algorithm: "alpha_miner",
        parameters: { min_sup: 0.3 },
        gates: [
            { name: "budget_check", passed: true, duration_ns: 1_000_000 },
            { name: "fitness_check", passed: true, fitness: 0.98 },
        ],
    },
    hash: "blake3:<digest>",
}
```

**Audit rule:** A receipt is valid only if:
- It carries the compat admission witness
- Every execution step is annotated with its algorithm and parameters
- The final hash covers both compat and execution provenances

### 4.3 Round-Trip Claim: Compat ↔ Engine ↔ Compat

compat exports a `RoundTripClaim` trait; wasm4pm must honor it:

```rust
// In compat:
pub trait RoundTripClaim<T, W> {
    fn claim_lossless_encode(&self) -> Result<Vec<u8>, Self::Reason>;
    fn claim_lossless_decode(bytes: &[u8]) -> Result<T, Self::Reason>;
}

// In wasm4pm:
pub trait RoundTripAttestation<T, W>: RoundTripClaim<T, W> {
    fn attest_round_trip(&self) -> Result<AttestationReceipt<T, W>, ExecutionFailure>;
}

// Usage:
let log: Evidence<OcelLog, Admitted, Ocel20> = admitted_log;
let encoded = log.claim_lossless_encode()?;
let _decoded: OcelLog = OcelLog::claim_lossless_decode(&encoded)?;
// If both succeed, the Receipt carries a "round_trip" proof gate
```

## 5. Graduation Boundaries: Where Structure Ends and Execution Begins

### 5.1 The Covenant

```
wasm4pm-compat boundary
              |
              ↓ (owned by compat)
    Evidence<T, State, W>  ← Type law, witnesses, lifecycle
    Admit<Reason>          ← Structural refusal
    RoundTripClaim         ← Shape preservation claims
              |
    ──────────────────────────────────────────────
              |
              ↓ (owned by wasm4pm)
    ExecutionAdmit         ← Add execution context
    ReplayAuthority        ← Token-based execution
    ConformanceEngine      ← Discovery & checking
    OptimizationPass       ← Model transformation
              |
```

### 5.2 What Stays in compat

- All type definitions (Evidence, Admit, Witness, State, Refusal)
- All shape/structure modules (ocel, petri, eventlog, etc.)
- All admission/refusal laws
- All zero-cost type transformations
- All witness definitions and metadata

**Compat exports, wasm4pm imports.** No cycles.

### 5.3 What Moves to wasm4pm

- All algorithms (discovery, conformance, replay, alignment)
- All proof gates (fitness checks, budget checks, gate results)
- All execution contexts (state machines, token carriers, simulation frames)
- All engine-owned witnesses (ReplayAuthority, DiscoveryAuthority, etc.)
- All optimizations and heuristics

**wasm4pm runs; compat reasons.** No execution in compat.

### 5.4 Graduation Trigger Points

A compat value graduates to wasm4pm when:

1. **Needs discovery** — `GraduationReason::NeedsDiscovery`
   - Graduation payload: `Evidence<EventLog, Admitted, Xes1849>` + `DiscoveryParams`
   - Engine produces: `GraduationResult::Model(Evidence<PetriNet, Admitted, Xes1849>)`

2. **Needs conformance** — `GraduationReason::NeedsConformanceExecution`
   - Graduation payload: `(Evidence<EventLog, Admitted, W>, Evidence<Model, Admitted, W>)`
   - Engine produces: `Evidence<ConformanceResult, Receipted, W>` with proof gates

3. **Needs replay** — `GraduationReason::NeedsReplay`
   - Graduation payload: `(Evidence<EventLog, Admitted, W>, Evidence<Model, Admitted, W>)`
   - Engine produces: `Evidence<ReplayTrace, Receipted, W>` with step-by-step evidence

4. **Needs receipts** — `GraduationReason::NeedsReceipts`
   - Graduation payload: `Evidence<T, Admitted, W>` with no existing receipt
   - Engine produces: `Evidence<T, Receipted, W>` with ProvenanceChain

5. **Needs OCPQ execution** — `GraduationReason::NeedsObjectCentricQueryExecution`
   - Graduation payload: `(Evidence<OcelLog, Admitted, Ocel20>, OcpqQuery)`
   - Engine produces: `Evidence<QueryResult, Receipted, Ocel20>`

For each trigger, **the witness must be preserved**. A result that arrives back from wasm4pm must carry the same witness that left compat.

## 6. Implementation Roadmap

### Phase 1: Import Core Types (Immediate)
- [ ] Add `wasm4pm-compat` dependency to `wasm4pm/Cargo.toml` with `wasm4pm` feature
- [ ] Replace all hand-rolled type definitions in `wasm4pm-types` with re-exports from compat
- [ ] Update `wasm4pm-types/src/lib.rs` to re-export `Evidence<T, State, W>`, `Witness`, ID types, etc.
- [ ] Add integration tests: verify types are identical across crate boundary

### Phase 2: Bridge Admission and Refusal (Week 2)
- [ ] Implement `ExecutionAdmit` trait in `wasm4pm/src/admission_bridge.rs`
- [ ] Create `wasm4pm::error::ExecutionFailure` that wraps compat `Refusal<R, W>`
- [ ] Add conversion: `impl From<Refusal<R, W>> for ExecutionFailure`
- [ ] Document refusal preservation rules in `docs/REFUSAL_COVENANT.md`

### Phase 3: Witness Bridging (Week 3)
- [ ] Create `wasm4pm::witness_bridge.rs` with re-exports
- [ ] Define `ReplayAuthority` as the *only* wasm4pm-owned witness
- [ ] Add auditing trait: `impl WitnessAuditor for ExecutionReceipt<T, W>`
- [ ] Test: verify witness metadata appears in logs and receipts

### Phase 4: Receipt Extension (Week 4)
- [ ] Extend `compat::ReceiptEnvelope` → `wasm4pm::ExecutionReceipt`
- [ ] Implement `RoundTripAttestation` trait
- [ ] Add proof gates: `fitness_check`, `budget_check`, `convergence_check`
- [ ] Test: round-trip encode/decode with receipt attestation

### Phase 5: Graduation Bridge (Week 5)
- [ ] Implement `GraduationAdapter<T, W>` that wraps compat's `GraduateToWasm4pm`
- [ ] Add `execute_graduated(candidate: GraduationCandidate) -> Result<ExecutionReceipt, ExecutionFailure>`
- [ ] Route each `GraduationReason` to the appropriate wasm4pm engine
- [ ] Test: E2E from compat admission to wasm4pm execution and back

### Phase 6: Integration Tests (Week 6)
- [ ] Write `tests/compat_integration/` suite:
   - OCEL → discovery → PetriNet → conformance → receipt
   - EventLog → replay → token trace → receipt
   - Cross-witness refusal (try to execute OCEL under Xes1849 — should refuse)
- [ ] Add benchmark: measure zero-cost overhead of type law
- [ ] Acceptance criteria: all receipts carry preserved witnesses, no string errors

## 7. Governance and Audit

### 7.1 Code Review Criteria

Every change at the compat/wasm4pm boundary must pass:

1. **Type preservation** — witnesses unchanged, states unchanged
2. **No re-implementation** — shared types come from compat, never redefined
3. **Refusal preservation** — no string error messages where compat provides named laws
4. **Witness auditing** — every receipt carries human-readable witness metadata

### 7.2 Test Coverage

Required test cases:

| Test | Location | Trigger | Expected |
|------|----------|---------|----------|
| `test_witness_preserved_through_graduation` | `tests/compat_integration/witness.rs` | Graduate OCEL evidence | `ExecutionReceipt` carries `Ocel20` |
| `test_refusal_blocks_execution` | `tests/compat_integration/refusal.rs` | Try to execute refused log | `ExecutionFailure::StructuralRefusal` |
| `test_receipt_round_trip` | `tests/compat_integration/receipt.rs` | Encode → execute → decode | All data recoverable, witness preserved |
| `test_cross_witness_rejection` | `tests/compat_integration/witness.rs` | Execute under wrong witness | Immediate refusal, no algorithm run |

### 7.3 Deprecation Path

When re-exporting compat types, mark the old wasm4pm definitions as `#[deprecated]`:

```rust
// In wasm4pm-types (deprecated):
#[deprecated(
    since = "0.5.0",
    note = "use wasm4pm_compat::evidence::Evidence instead"
)]
pub struct Evidence<T, S> { /* old impl */ }

// In wasm4pm (new):
pub use wasm4pm_compat::evidence::Evidence;
```

Migration period: 2 releases. Then remove the old impl.

## 8. Related Documents

- **REFUSAL_LAW.md** (in compat) — Detailed refusal naming requirements
- **PAPERLAW_CROWN_ALIVE_004.md** (in compat) — Compile-fail/pass receipts certifying type law
- **WASM4PM_ARCHITECTURE.md** (to be written) — Full engine layer design

## Closure Conditions

GAP_001 is closed when:

1. ✓ wasm4pm-types imports all core type shapes from wasm4pm-compat
2. ✓ All witness markers flow unchanged from compat to receipts
3. ✓ No execution logic in compat; all execution in wasm4pm
4. ✓ Refusal shapes preserved across boundary
5. ✓ Integration tests pass: compat admission → wasm4pm execution → engine receipt → back to compat
6. ✓ Witness metadata exposed in all diagnostics and receipts
7. ✓ Commit message: `"docs: GAP_001 closure plan—compat/wasm4pm type bridge"`

---

**Authorization trail:**
- Process Intelligence ALIVE_001 (established compat as type layer, wasm4pm as execution layer)
- PAPERLAW_CROWN_ALIVE_004 (sealed compat type law with 98 papers, 602 receipts)
- GAP_001 authorization: Bridge design, witness preservation, refusal alignment, receipt covenant
