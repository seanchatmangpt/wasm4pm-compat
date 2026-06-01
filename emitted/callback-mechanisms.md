# Callback & Event Mechanism Discovery

**Scope:** wasm4pm-compat Rust nightly-first process-evidence crate  
**Date:** 2026-06-01  
**Pattern Search:** `fn(&`, `Box<dyn`, `Arc<Mutex<dyn`, `emit(`, `subscribe(`, `on(`, `.listen(`, closures, observer patterns, RxJS/async streams

---

## Executive Summary

**wasm4pm-compat contains NO traditional callback mechanisms** — no function pointers, no closures, no trait objects, no async streams, no observer pattern, no event emitter, no publish-subscribe system.

**What it DOES have:**
- **Typestate callbacks** — zero-cost type-level state transitions using `PhantomData` markers
- **Trait-based polymorphism** — sealed traits guarding type-law enforcement
- **Builder pattern** — fluent APIs with immutable state chaining
- **Result-based control flow** — `Admission<T,W> / Refusal<R,W>` as first-class verdicts
- **Iterator adaptation** — `IntoIterator` for lazy log traversal
- **Witness markers** — compile-time proof carriers (not runtime callbacks)

This is intentional: the crate is **structure-only**, forbidding runtime logic (`#![forbid(unsafe_code)]`). All callback-like behavior is achieved through **type-level dispatch and trait bounds**, not function pointers or closures.

---

## Pattern Categories Found

### 1. Typestate Callbacks (Zero-Cost State Transitions)

**What:** `PhantomData` markers encoding compile-time state machine transitions. State changes are encoded in the *type*, not stored at runtime.

**Location:** `src/workflow.rs`, `src/evidence.rs`, `src/state.rs`

**Example:**
```rust
// From src/workflow.rs — parallel workflow cancellation as a typestate
pub struct BranchToken<T, S: BranchState> {
    pub _task: PhantomData<T>,
    pub _state: PhantomData<S>,  // <- zero-cost state tag
}

impl<T> BranchToken<T, Pending> {
    pub fn start(self) -> BranchToken<T, Running> {
        // Type changed: Pending → Running; no callback; no runtime cost
        BranchToken { _task: PhantomData, _state: PhantomData }
    }
}

// ParallelWorkflow<A, B, Running, Running>
pub fn cancel_b_from_a(self) -> ParallelWorkflow<A, B, Completed, Canceled> {
    // Fires cancellation: consuming Running branch and returning Canceled
    // No callback invoked; type system enforces Branch B cannot be completed
}
```

**Callback analogy:** A traditional event system would use `on_cancel(callback)`. Here, the *type itself* prevents illegal transitions; the compiler enforces them at compile time.

**Key insight:** This is a **type law callback** — the type system is the observer.

---

### 2. Admission/Refusal as Verdict Callbacks

**What:** The `Admit` trait is the single public boundary for judging evidence. It is called **exactly once** when evidence crosses into the compat layer.

**Location:** `src/admission.rs`

**Structure:**
```rust
/// The ONLY sanctioned way to turn Raw → Admitted evidence
pub trait Admit {
    type Raw;
    type Admitted;
    type Reason;  // <- specific named law (not a catch-all)
    type Witness;

    fn admit(
        raw: Evidence<Self::Raw, Raw, Self::Witness>
    ) -> Result<
        Admission<Self::Admitted, Self::Witness>,
        Refusal<Self::Reason, Self::Witness>
    >;
}
```

**Callback analogy:** Like a validation hook that fires once at the boundary. Instead of a callback, it's a trait impl that the type system enforces must exist.

**Both outcomes are first-class:**
- `Admission<T, W>` — accepted value
- `Refusal<R, W>` — rejected with named reason `R` (e.g., `MissingFinalMarking`, `DanglingEventObjectLink`)

**No catch-all:** Every refusal must carry a specific law. `InvalidInput` is forbidden.

---

### 3. Builder Pattern (Fluent API with Immutable Chaining)

**What:** Chainable `with_*` methods that accumulate state without callbacks. Each call returns `Self`, allowing composition.

**Location:** `src/eventlog.rs`, `src/dfg.rs`, `src/ocel.rs`, `src/xes.rs`, `src/petri.rs`

**Example:**
```rust
// From src/eventlog.rs — Event builder
impl Event {
    pub fn new(activity: impl Into<String>) -> Self { /* ... */ }
    
    pub fn by(mut self, resource: impl Into<String>) -> Self {
        self.resource = Some(resource.into());
        self  // <- immutable chaining; no callback
    }
    
    pub fn with_lifecycle(mut self, transition: impl Into<String>) -> Self {
        self.lifecycle = Some(transition.into());
        self  // <- fluent continuation
    }
}

// Usage: no callbacks, just method chaining
let event = Event::new("place_order")
    .by("resource_1")
    .with_lifecycle("complete");
```

**Callback analogy:** Similar to RxJS `.pipe()` chaining, but without observables. Each method is a pure transformation; composition happens through type-level trait bounds, not runtime callbacks.

---

### 4. Trait-Based Polymorphism & Sealed Traits

**What:** Trait objects (`dyn T`) and sealed traits guard type-law enforcement. Sealed traits prevent external implementations.

**Location:** `src/petri.rs`, `src/dfg.rs`, `src/law.rs`, `src/powl.rs`, `src/ids.rs`, `src/strict.rs`

**Pattern:**
```rust
// From src/petri.rs — sealed arc trait
mod arc_seal {
    pub(crate) trait Sealed {}
    impl<P, T, W> Sealed for super::PlaceToTransitionArc<P, T, W> {}
    impl<T, P, W> Sealed for super::TransitionToPlaceArc<T, P, W> {}
}

pub trait IsValidArc: arc_seal::Sealed {}  // <- can only impl inside this module

// From src/dfg.rs — sealed endpoint traits
mod dfg_endpoint_seal {
    pub trait SourceSeal {}
    pub trait TargetSeal {}
}

pub trait IsDfgSource: dfg_endpoint_seal::SourceSeal {}
pub trait IsDfgTarget: dfg_endpoint_seal::TargetSeal {}
```

**Callback analogy:** Sealing is like a capability/permission check — only types we trust can pass. The type system is the enforcer; no runtime dispatch needed.

**Single trait object found:**
```rust
// From src/receipt.rs
/// A caller that holds a `dyn WellShaped` (or `T: WellShaped`) can check
pub trait WellShaped { /* ... */ }
```
But this is still structure-only; it's a shape marker, not a callback target.

---

### 5. Iterator Adaptation (Lazy Traversal)

**What:** `IntoIterator` and `Iterator` impls allow logs/nets to be traversed without callbacks. Lazy evaluation through iteration.

**Location:** `src/eventlog.rs`, `src/dfg.rs`

**Example:**
```rust
// From src/eventlog.rs
impl IntoIterator for EventLog {
    type Item = Trace;
    type IntoIter = std::vec::IntoIter<Trace>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.traces.into_iter()
    }
}

// From src/eventlog.rs — reference iterator
impl<'a> IntoIterator for &'a EventLog {
    type Item = &'a Trace;
    type IntoIter = std::slice::Iter<'a, Trace>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.traces.iter()
    }
}

// Usage: no callbacks, just iteration
for trace in &log {
    // process trace
}
```

**Callback analogy:** Like a pull-based event stream (similar to backpressure in RxJS) rather than push-based callbacks. The caller controls the pace.

---

### 6. Witness Markers (Compile-Time Proof Carriers)

**What:** `Witness` trait and marker types encode *which* law/standard was applied. They are zero-sized types used to distinguish admissions.

**Location:** `src/witness.rs`

**Structure:**
```rust
/// The witness family — each names a law, paper, or standard
pub trait Witness {
    const KEY: &'static str;      // "ocel20", "xes1849", etc.
    const TITLE: &'static str;    // human-readable
    const YEAR: u16;              // authoritatively tagged
    const FAMILY: WitnessFamily;  // classification
}

// Marker types — zero-sized
pub struct Ocel20;        // witness for OCEL 2.0 standard
pub struct Xes1849;       // witness for XES 1849-2016
pub struct WfNetSoundnessPaper;  // witness for soundness law
```

**Usage example:**
```rust
// Admission<T, Ocel20> is a DIFFERENT TYPE from Admission<T, Xes1849>
// This prevents mixing standards at the type level.
let ocel_admission: Admission<OcelLog, Ocel20> = /* ... */;
let xes_admission: Admission<XesLog, Xes1849> = /* ... */;
// These cannot be confused; the type system enforces correct standard.
```

**Callback analogy:** Not a callback, but a compile-time *proof tag*. The witness proves which law was applied; the compiler ensures correct law usage.

---

### 7. Result-Based Control Flow

**What:** `Result<Admission<T,W>, Refusal<R,W>>` replaces callback chains. Both paths are typed and composable.

**Count:** 189 Result-based returns across the codebase.

**Example:**
```rust
// From src/interop.rs
pub fn admit_flat(&self) -> Result<(), InteropRefusal> {
    if !self.is_grounded() {
        return Err(InteropRefusal::UngroundedArtifact);
    }
    if self.shape.is_object_centric() {
        return Err(InteropRefusal::FlatClaimOverObjectCentric);
    }
    Ok(())
}
```

**Callback analogy:** Instead of `.on_success(callback).on_error(error_callback)`, Result types allow composition via `?` operator. The error path is as first-class as the success path.

---

### 8. Graduation Bridge (Explicit Boundary Crossing)

**What:** `GraduateToWasm4pm` trait signals when evidence should leave compat and enter the execution engine. It produces a `GraduationCandidate` (not a callback; a structured witness).

**Location:** `src/engine_bridge.rs`

**Pattern:**
```rust
pub enum GraduationReason {
    NeedsDiscovery,
    NeedsConformanceExecution,
    NeedsReplay,
    NeedsReceipts,
    NeedsBenchmarkGate,
    NeedsObjectCentricQueryExecution,
    RebuildingProcessMiningLocally,
}

pub struct GraduationCandidate {
    pub reason: GraduationReason,
    pub subject: String,
    pub evidence_ref: String,  // <- grounds the case
}

pub trait GraduateToWasm4pm {
    fn candidate(&self) -> GraduationCandidate;
}
```

**Callback analogy:** Like a *checkpoint* or *gate callback* that says "I need the real engine now." But it's structure-only; producing a candidate doesn't execute graduation, it just names the case.

---

### 9. Streaming Context Tags (Online vs. Offline)

**What:** `ContextualEvidence<T, Context>` tags evidence as online (live stream) or offline (batch log) using `PhantomData`. Type system prevents mixing contexts.

**Location:** `src/streaming.rs`

**Pattern:**
```rust
pub struct OnlineMonitoringContext;
pub struct OfflineAnalysisContext;

pub struct ContextualEvidence<T, Context> {
    pub inner: T,
    _ctx: PhantomData<Context>,  // <- zero-cost tag
}

impl<T> ContextualEvidence<T, OfflineAnalysisContext> {
    pub fn offline(inner: T) -> Self { /* ... */ }
}

impl<T> ContextualEvidence<T, OnlineMonitoringContext> {
    pub fn online(inner: T) -> Self { /* ... */ }
}

// Type safety: OnlineEvidence<T> != OfflineEvidence<T>
// A function demanding offline context cannot receive online evidence
pub fn offline_conformance_check(ev: OfflineEvidence<Log>) { /* ... */ }
```

**Callback analogy:** Not a callback, but a *context guard*. The type system prevents accidentally mixing streaming and batch evidence.

---

### 10. Loss Policy & Projection Gate

**What:** `LossPolicy` and `Project` trait enforce that lossy transformations declare their loss *before* loss occurs. No silent transformation.

**Location:** `src/loss.rs`

**Patterns:**
```rust
pub enum LossPolicy {
    RefuseLoss,                              // <- reject if loss would occur
    AllowNamedProjection(ProjectionName),   // <- named, auditable loss
    AllowLossWithReport,                     // <- loss must be reported
}

pub trait Project {
    type From;
    type To;
    type Lost;
    type Reason;  // <- why loss occurred
    
    fn project(
        from: Self::From,
        policy: LossPolicy,
    ) -> Result<
        LossReport<Self::From, Self::To, Self::Lost>,
        Self::Reason
    >;
}
```

**Callback analogy:** Like a *before-hook* that fires before loss is committed. The policy decides whether to allow it; the report documents what was lost.

---

## Trait Implementations & Dispatch

**Total trait impl blocks:** 243  
**Key trait families:**

| Trait Family | Count | Purpose |
|---|---|---|
| Sealed traits | 8+ | Type-law enforcement, capability control |
| `From<_> / Into<_>` | 20+ | Conversions (builders, unwrapping) |
| `IntoIterator` | 5+ | Log/net traversal without callbacks |
| `Witness` family | 60+ | Standard/law markers (witness impls) |
| `Display / Debug` | 30+ | Introspection (not callbacks) |
| Custom logic traits | `Admit`, `Project`, `GraduateToWasm4pm`, `StrictCheck` | Boundary enforcement |

**No trait objects for callbacks:** The single `dyn WellShaped` is a shape marker, not a dispatch target.

---

## Async/Concurrency

**Async patterns:** NONE  
**Tokio channels:** NONE  
**RxJS-style streams:** NONE  
**std::sync types:** NONE  

**Reason:** Structure-only crate. No runtime concurrency model. Graduation to `wasm4pm` handles execution.

---

## Event Log Traversal (Process-Specific Iteration)

**What:** Event logs are traversed via `IntoIterator`, not callbacks.

**Location:** `src/eventlog.rs`

**Example:**
```rust
pub struct EventLog {
    traces: Vec<Trace>,
}

// Ownership-taking iteration
impl IntoIterator for EventLog {
    type IntoIter = std::vec::IntoIter<Trace>;
    fn into_iter(self) -> Self::IntoIter {
        self.traces.into_iter()
    }
}

// Borrowed iteration
impl<'a> IntoIterator for &'a EventLog {
    type IntoIter = std::slice::Iter<'a, Trace>;
    fn into_iter(self) -> Self::IntoIter {
        self.traces.iter()
    }
}

// Usage
let log = EventLog::from_traces([trace1, trace2]);
for trace in log {
    // Pull-based, not push-based
}
```

---

## Summary: Callback Mechanisms in wasm4pm-compat

| Mechanism | Found? | Type | Location |
|---|:---:|---|---|
| Function pointers | NO | N/A | N/A |
| Closures (`Fn`, `FnMut`, `FnOnce`) | NO | N/A | N/A |
| Trait objects (`dyn FnMut(...)`) | NO | N/A | N/A |
| Async/tokio channels | NO | N/A | N/A |
| Observer pattern | NO | N/A | N/A |
| Event emitter | NO | N/A | N/A |
| Publish-subscribe | NO | N/A | N/A |
| RxJS-style streams | NO | N/A | N/A |
| **Typestate transitions** | **YES** | Type-level state machine | `src/workflow.rs`, `src/evidence.rs` |
| **Trait-based dispatch** | **YES** | Sealed traits, polymorphism | 243 impl blocks across codebase |
| **Builder pattern** | **YES** | Fluent API chaining | `src/eventlog.rs`, `src/ocel.rs`, `src/petri.rs`, etc. |
| **Admission/Refusal verdicts** | **YES** | First-class outcomes | `src/admission.rs` |
| **Iterator adaptation** | **YES** | Lazy traversal | `src/eventlog.rs`, `src/dfg.rs` |
| **Witness markers** | **YES** | Compile-time proof tags | `src/witness.rs` |
| **Graduation boundary** | **YES** | Explicit stage exit | `src/engine_bridge.rs` |

---

## Design Philosophy

**wasm4pm-compat deliberately avoids traditional callbacks** because:

1. **Structure-only mandate:** No runtime logic. All behavior is encoded in the type system.
2. **Zero-cost abstraction:** `PhantomData` markers and type parameters have zero runtime cost.
3. **Compile-time safety:** Illegal state transitions and law violations are caught at compile time, not runtime.
4. **Refusal is first-class:** Errors/rejections are full-fledged types, not exceptions or callback hooks.
5. **Graduation path:** When *execution* is needed, evidence graduates to `wasm4pm`. Compat stays pure.

The crate trades the flexibility of runtime callbacks for **type safety and compile-time guarantees**. This is intentional and aligns with the "structure-only" covenant.

---

## Relevant Documentation

- `src/lib.rs` — overview and feature model
- `src/admission.rs` — boundary verdict semantics
- `src/workflow.rs` — typestate state machine examples
- `src/engine_bridge.rs` — graduation boundary
- `src/loss.rs` — loss policy and projection gates
- `src/witness.rs` — witness families and proof carriers
- `docs/REFUSAL_LAW.md` — specific refusal reasons

---

**Conclusion:** wasm4pm-compat contains **zero traditional callback mechanisms**. All "callback-like" behavior is achieved through **type-level dispatch, trait bounds, and state markers** — strategies that align with the crate's structure-only, zero-cost philosophy.
