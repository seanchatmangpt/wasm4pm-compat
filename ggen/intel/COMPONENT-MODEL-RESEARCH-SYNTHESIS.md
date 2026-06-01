# Component Model + WIT Research Synthesis
## wasm4pm-compat Bridge Architecture

**Date:** 2026-06-01  
**Research Effort:** Deep research → Component Model spec + WIT docs + wit-bindgen tooling  
**Output Documents:** 
- `component-model-map.md` — WIT type system and interface design principles (652 lines)
- `wit-surface-ledger.yaml` — Detailed compat world + engine world specification (841 lines)

---

## Executive Summary

This synthesis bridges **WebAssembly Component Model** (WIT + wit-bindgen) with **wasm4pm-compat type-law architecture**. The goal is to expose wasm4pm-compat's admission, loss, strict, and graduation logic as standardized Component Model interfaces, enabling polyglot consumption (Rust, Go, TypeScript, etc.) and WASI runtime compatibility.

### Key Research Findings

1. **WIT Type System Maps Cleanly to wasm4pm-compat**
   - Primitives (bool, u8–u64, f32–f64, string) ↔ scalar fields
   - Records (struct-like) ↔ Evidence, Admission, Refusal types
   - Variants (discriminated unions) ↔ Refusal reasons, State tokens
   - Results (`result<T, E>`) ↔ Success/failure paths
   - Lists (`list<T>`) ↔ Vec<T> (events, traces, losses)
   - Resources ↔ Witness markers (in engine world, not compat)

2. **Error Representation: Named-Law Refusals**
   - Every `result<T, E>` error arm must carry a specific named law
   - WIT variant discriminant encodes the law (e.g., `dangling-event-object-link`, `missing-final-marking`)
   - No catch-all `Error` or `InvalidInput` types
   - Witness tag passed as string field in admitted records

3. **Component Boundary: Compat vs Engine**
   - **Compat world** (export): Pure functions, no state
     - `admission::admit-event-log()`, `admit-ocel-log()`, `admit-xes-log()`
     - `loss::project-ocel-to-xes()`, `project-xes-to-dfg()`
     - `strict::check-strict-boundary()`
     - `graduation::graduate-to-wasm4pm()`
   - **Engine world** (import): Executable algorithms
     - `discovery::discover-dfg()`, `discover-petri()`, `discover-bpmn()`
     - `replay::replay-on-petri()`, `align-on-petri()`
     - `conformance::check-conformance()`
     - `ocpq::query-object-lifecycle()`, `query-object-relations()`
     - `receipts::generate-receipt()`, `verify-receipt()`

4. **Witness Encoding Strategy**
   - In compat world: Witness is **phantom type** (zero-cost), encoded as `witness-id: string` field
   - In engine world: Witness can be **resource handle** if engines need to maintain witness context
   - No resource types in compat (no state, no ownership transfer)

5. **Feature Gating in WIT**
   - `compat.wit` (base): admission only
   - `compat-formats.wit` (formats feature): admission + loss
   - `compat-strict.wit` (strict feature): admission + strict
   - `compat-wasm4pm.wit` (wasm4pm feature): admission + graduation
   - `compat-all.wit` (all features): admission + loss + strict + graduation
   - `engine.wit` (wasm4pm feature): discovery + replay + conformance + ocpq + receipts

6. **wit-bindgen Integration**
   - Macro: `generate!({ world: "compat", path: "wit/compat.wit" })`
   - Generates Rust `struct`, `enum`, and `trait Guest` for each interface
   - Auto-derives field types from WIT records/variants
   - Enables seamless Rust-↔-Component Model binding

---

## Part 1: Type System Mapping

### Primitives

| WIT Type | Rust Type | wasm4pm-compat Context |
|----------|-----------|----------------------|
| `bool` | `bool` | Lifecycle flags, feature gates |
| `u8`–`u64` | `u8`–`u64` | Event counts, timestamps (ns), cost metrics |
| `s8`–`s64` | `i8`–`i64` | Offsets, deltas |
| `f32`, `f64` | `f32`, `f64` | Fitness, precision, generalization scores |
| `string` | `String` | Event IDs, object IDs, law names, descriptions |
| `char` | `char` | Activity notation, state markers |

### Aggregate Types: Records

WIT **records** ↔ Rust **structs**. Zero-overhead representation.

**Examples from wasm4pm-compat:**

1. **EventLog (base structure)**
   ```wit
   record event-log {
     events: list<event>,
     traces: list<trace>,
     metadata: event-log-metadata,
   }
   ```
   Maps to `EventLog` struct in `src/eventlog.rs`.

2. **OcelLog (object-centric structure)**
   ```wit
   record ocel-log {
     events: list<ocel-event>,
     objects: list<ocel-object>,
     e2o-links: list<event-to-object-link>,
     o2o-links: list<object-to-object-link>,
     object-changes: list<object-change>,
   }
   ```
   Maps to `OcelLog` struct in `src/ocel.rs`.

3. **LossReport (loss accounting)**
   ```wit
   record loss-report {
     from-format: string,
     to-format: string,
     items-lost: list<loss-item>,
     summary: string,
   }
   ```
   Maps to `LossReport<From, To, Items>` in `src/loss.rs`.

4. **Metric (conformance measurement)**
   ```wit
   record metric {
     kind: string,
     numerator: u64,
     denominator: u64,
     notes: string,
   }
   ```
   Maps to `Metric<KIND, NUM, DEN>` in `src/conformance.rs`.

### Aggregate Types: Variants (Enums)

WIT **variants** ↔ Rust **enums**. Discriminated unions with optional payloads.

**Case A: Enum-like (no payloads) → State Tokens**

```wit
variant lifecycle-state {
  raw,
  parsed,
  admitted,
  refused,
  projected,
  exportable,
  receipted,
}
```

Maps to `enum LifecycleState` in `src/state.rs`.

**Case B: Discriminated Union with Payloads → Refusal Reasons**

```wit
variant refusal-reason {
  dangling-event-object-link(record { event-id: string, object-id: string, object-type: string }),
  missing-final-marking(record { place-id: string, state-id: string }),
  invalid-petri-structure(record { violation: string, element-id: string }),
  circular-dependency(record { cycle: list<string> }),
  hidden-process-mining-growth(record { discovered-elements: u32, boundary-elements: u32 }),
  invalid-loss-policy(record { transformation: string, policy-required: string }),
  witness-mismatch(record { expected: string, found: string }),
}
```

Maps to:
```rust
pub enum RefusalReason {
    DanglingEventObjectLink { event_id: String, object_id: String, object_type: String },
    MissingFinalMarking { place_id: String, state_id: String },
    // ... etc
}
```

Each variant name is a **named law**. No generic `Error` or `InvalidInput`.

### Result Types

WIT **result<T, E>** ↔ Rust **Result<T, E>**.

**Forms:**

| WIT | Meaning | Rust |
|-----|---------|------|
| `result<T, E>` | Success T or error E | `Result<T, E>` |
| `result<T>` | Success T or error () | `Result<T, ()>` |
| `result<_, E>` | Success () or error E | `Result<(), E>` |
| `result` | Success () or error () | `Result<(), ()>` |

**wasm4pm-compat Applications:**

1. **Admission paths**
   ```wit
   admit-event-log: func(raw: event-log) -> result<event-log, refusal-reason>;
   ```
   Rust: `fn admit_event_log(raw: EventLog) -> Result<EventLog, RefusalReason>`

2. **Loss projection**
   ```wit
   project-ocel-to-xes: func(admitted: ocel-log, policy: loss-policy) ->
     result<record { xes-log: xes-log, report: loss-report }, refusal-reason>;
   ```
   Rust: `fn project_ocel_to_xes(admitted: OcelLog, policy: LossPolicy) -> Result<(XesLog, LossReport), RefusalReason>`

3. **Strict checking**
   ```wit
   check-strict-boundary: func(boundary: process-boundary) -> result<bool, strict-violation>;
   ```
   Rust: `fn check_strict_boundary(boundary: ProcessBoundary) -> Result<bool, StrictViolation>`

### Lists and Collections

WIT **list<T>** ↔ Rust **Vec<T>**.

**Examples:**
- `list<event>` → `Vec<Event>`
- `list<trace>` → `Vec<Trace>`
- `list<object-type>` → `Vec<ObjectType>`
- `list<refusal-reason>` → `Vec<RefusalReason>` (multi-error reports)
- `list<loss-item>` → `Vec<LossItem>`
- `list<metric>` → `Vec<Metric>`

### Options

WIT **option<T>** ↔ Rust **Option<T>**.

**Examples:**
- `option<string>` → `Option<String>` (optional descriptions)
- `option<loss-report>` → `Option<LossReport>` (conditional loss accounting)
- `option<lifecycle-annotation>` → `Option<LifecycleAnnotation>` (optional metadata)

### Resources (Engine World Only)

WIT **resource<name>** represents a handle to external state with explicit ownership.

**Design Decision:** Resources are **NOT** used in the compat world (it's pure, stateless). They are **optional** in the engine world if engines need to maintain witness context:

```wit
resource witness-ocel20 {
  constructor(id: string) -> witness-ocel20,
  method: func(self: borrow<witness-ocel20>) -> result<bool>,
}
```

Rust binding (via wit-bindgen):
```rust
pub struct WitnessOcel20 {
    rep: wasmtime::component::Resource<WitnessOcel20>,
}

impl WitnessOcel20 {
    pub fn new(id: String) -> Self { /* generated */ }
    pub fn method(&self) -> Result<bool, String> { /* generated */ }
}
```

---

## Part 2: Interface Composition and World Definitions

### Compat World Structure

The compat world exports **four interfaces** (the last two optional, gated by features):

#### Interface 1: admission

**Export. Pure functions. No state.**

```wit
interface admission {
  use types.{event-log, ocel-log, xes-log, refusal-reason};
  
  admit-event-log: func(raw: event-log) -> result<event-log, refusal-reason>;
  admit-ocel-log: func(raw: ocel-log) -> result<ocel-log, refusal-reason>;
  admit-xes-log: func(raw: xes-log) -> result<xes-log, refusal-reason>;
}
```

**Functions:**
- `admit-event-log()`: Validate and transition raw → admitted for generic event log
- `admit-ocel-log()`: Validate and transition raw → admitted for OCEL format
- `admit-xes-log()`: Validate and transition raw → admitted for XES format

**Witness Encoding:** Returned record includes `witness-id: string` field (e.g., "ocel20", "xes1849").

#### Interface 2: loss (Feature: formats)

**Export. Pure functions. Lossy transformation with accounting.**

```wit
interface loss {
  use types.{ocel-log, xes-log, loss-report, refusal-reason};
  
  record loss-policy {
    kind: string,  // "refuse" | "allow-named" | "allow-with-report"
  }
  
  project-ocel-to-xes: func(admitted: ocel-log, policy: loss-policy) ->
    result<record { xes-log: xes-log, report: loss-report }, refusal-reason>;
  
  project-xes-to-dfg: func(admitted: xes-log, policy: loss-policy) ->
    result<record { dfg: dfg-model, report: loss-report }, refusal-reason>;
}
```

**Functions:**
- `project-ocel-to-xes()`: Transform OCEL (multi-object) to XES (trace-only) with loss accounting
- `project-xes-to-dfg()`: Summarize XES to directly-follows graph

**Loss Policies:**
- `"refuse"` — Reject if data loss occurs (error if mismatch detected)
- `"allow-named"` — Accept loss, emit detailed LossReport
- `"allow-with-report"` — Unconditional acceptance with audit trail

**Principle:** Every lossy transformation emits a `loss-report` describing what was lost and why.

#### Interface 3: strict (Feature: strict)

**Export. Boundary judgment and attestation.**

```wit
interface strict {
  use types.{process-boundary};
  
  variant strict-violation {
    missing-witness-marker(string),
    missing-loss-policy(string),
    missing-round-trip(string),
    hidden-process-mining-growth(string),
  }
  
  check-strict-boundary: func(boundary: process-boundary) -> result<bool, strict-violation>;
}
```

**Function:** `check-strict-boundary()` validates declared boundary constraints:
1. Has witness marker (proof of law knowledge)
2. Has loss policy (loss is accounted for)
3. Has round-trip definition (bidirectional path attested)

**Returns:**
- `Ok(true)` if all constraints pass
- `Err(strict-violation)` with specific diagnostic if any fail

#### Interface 4: graduation (Feature: wasm4pm)

**Export. Bridge from type law to execution engine.**

```wit
interface graduation {
  use types.{event-log, refusal-reason};
  
  record graduation-candidate {
    kind: string,         // "discovery" | "conformance" | "replay"
    is-grounded: bool,    // Can engine consume this?
    reason: option<string>, // Why ungrounded?
  }
  
  graduate-to-wasm4pm: func(admitted: event-log) -> result<graduation-candidate, refusal-reason>;
}
```

**Function:** `graduate-to-wasm4pm()` signals readiness for engine consumption.
- **Grounded:** Engine can execute discovery, replay, or conformance checks.
- **Ungrounded:** Engine will reject; missing required annotations or metadata.

### Engine World Structure

The engine world imports **five interfaces** (all gated by wasm4pm feature):

#### Interface 1: discovery

**Import. Process discovery algorithms.**

```wit
interface discovery {
  record dfg-model {
    activities: list<string>,
    edges: list<record { from: string, to: string, count: u32 }>,
  }
  record petri-net {
    places: list<string>,
    transitions: list<string>,
    arcs: list<record { from: string, to: string }>,
  }
  record bpmn-model {
    flows: list<string>,
    gateways: list<string>,
    events: list<string>,
  }
  
  discover-dfg: func(admitted: event-log) -> result<dfg-model, string>;
  discover-petri: func(admitted: event-log, method: string) -> result<petri-net, string>;
  discover-bpmn: func(admitted: event-log) -> result<bpmn-model, string>;
}
```

**Algorithms:**
- `discover-dfg()`: Directly-follows graph
- `discover-petri()`: Petri net (alpha, inductive miner, IM[d])
- `discover-bpmn()`: BPMN diagram (process tree → BPMN conversion)

#### Interface 2: replay

**Import. Token replay and alignment.**

```wit
interface replay {
  use discovery.{petri-net};
  
  replay-on-petri: func(admitted: event-log, model: petri-net) ->
    result<record { fitness: f64, moves-on-log: u32, moves-on-model: u32 }, string>;
  
  align-on-petri: func(admitted: event-log, model: petri-net) ->
    result<record { cost: u32, alignment: list<string> }, string>;
}
```

**Algorithms:**
- `replay-on-petri()`: Token replay, fitness computation
- `align-on-petri()`: Optimal alignment (A* search)

#### Interface 3: conformance

**Import. Conformance checking and metrics.**

```wit
interface conformance {
  use discovery.{petri-net, bpmn-model, dfg-model};
  
  check-conformance: func(admitted: event-log, model: process-model) ->
    result<list<metric>, string>;
}
```

**Metrics:**
- Fitness: Trace replayability
- Precision: Model does not allow unobserved behavior
- Generalization: Model captures all observed sequences
- Simplicity: Structural parsimonity

#### Interface 4: ocpq

**Import. Object-centric process querying.**

```wit
interface ocpq {
  use compat:types.{ocel-log};
  
  query-object-lifecycle: func(admitted: ocel-log, object-id: string) ->
    result<list<record { event-id: string, timestamp: u64, activity: string }>, string>;
  
  query-object-relations: func(admitted: ocel-log, object-id: string) ->
    result<list<string>, string>;
}
```

**Queries:**
- `query-object-lifecycle()`: All events affecting a single object
- `query-object-relations()`: Objects related through shared events

#### Interface 5: receipts

**Import. Receipt and proof generation.**

```wit
interface receipts {
  use compat:types.{event-log};
  
  generate-receipt: func(admitted: event-log) -> result<string, string>;
  verify-receipt: func(admitted: event-log, receipt: string) -> result<bool, string>;
}
```

**Functions:**
- `generate-receipt()`: Cryptographic hash or Merkle proof
- `verify-receipt()`: Validation against evidence

---

## Part 3: Witness Representation Strategy

### Compat World: String-Tagged Witness (Zero-Cost)

Witnesses in compat are **phantom types** at the Rust level; in WIT, they are encoded as string fields:

```wit
record admitted-event-log {
  events: list<event>,
  traces: list<trace>,
  metadata: event-log-metadata,
  witness-id: string,  // "ocel20" | "xes1849" | "bpmn20" | ...
}
```

**Rationale:**
- Witnesses are zero-cost in Rust (PhantomData)
- No need to pass witness state across component boundary
- String tag is sufficient for runtime dispatch in engine

**Witness ID Values:**
- `"ocel20"` — Object-Centric Event Log (2020)
- `"xes1849"` — eXtensible Event Stream
- `"bpmn20"` — Business Process Model and Notation (2.0)
- `"petri00"` — Petri Net
- `"powl00"` — Process Specification Language
- `"declare3"` — Declare (3.x)
- `"yawl20"` — Yet Another Workflow Language (2.0)

### Engine World: Optional Resource Witness

If an engine needs to maintain witness context across multiple discovery/replay operations, it can use a **resource handle**:

```wit
resource witness-ocel20 {
  constructor(id: string) -> witness-ocel20,
  law: func(self: borrow<witness-ocel20>) -> result<string>,
}

interface engine:discovery {
  discover-with-witness: func(
    admitted: event-log,
    witness: borrow<witness-ocel20>
  ) -> result<process-model, string>;
}
```

**Why Optional?**
- Simple engines (stateless discovery) don't need it
- Complex engines (multi-pass analysis, incremental learning) benefit from witness handles
- Principle: **Resources only where state and ownership matter**

---

## Part 4: Feature Gating and WIT File Strategy

### Cargo Features in wasm4pm-compat

1. **formats** (default) — Import/export contracts, loss accounting
2. **strict** (opt-in) — Boundary judgment and strict checking
3. **wasm4pm** (opt-in) — Graduation bridge, engine world

### WIT Files by Feature

| File | Features | Exports |
|------|----------|---------|
| `compat.wit` | (none) | admission |
| `compat-formats.wit` | formats | admission, loss |
| `compat-strict.wit` | strict | admission, strict |
| `compat-wasm4pm.wit` | wasm4pm | admission, graduation |
| `compat-all.wit` | all three | admission, loss, strict, graduation |
| `engine.wit` | wasm4pm | discovery, replay, conformance, ocpq, receipts |

### Build Integration (build.rs)

```rust
// build.rs excerpt
use std::env;
use wit_bindgen::generate;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let wit_path = format!("{}/wit", manifest_dir);
    
    let world_name = if cfg!(all(
        feature = "formats",
        feature = "strict",
        feature = "wasm4pm"
    )) {
        "compat-all"
    } else if cfg!(all(feature = "formats", feature = "strict")) {
        // ... more feature combinations
    } else if cfg!(feature = "formats") {
        "compat-formats"
    } else {
        "compat"
    };
    
    generate!({
        world: world_name,
        path: wit_path,
    });
}
```

---

## Part 5: wit-bindgen Integration

### Macro Invocation

```rust
// In src/lib.rs or main.rs
wit_bindgen::generate!({
    world: "compat",
    path: "wit/compat.wit",
});
```

### Generated Bindings Structure

For a compat.wit with `admission` interface:

```rust
// Generated module: crate::bindings::compat::admission
pub struct EventLog {
    pub events: Vec<Event>,
    pub traces: Vec<Trace>,
    pub metadata: EventLogMetadata,
    pub witness_id: String,
}

pub enum RefusalReason {
    DanglingEventObjectLink { event_id: String, object_id: String, object_type: String },
    MissingFinalMarking { place_id: String, state_id: String },
    // ... one per law
}

pub trait Guest {
    fn admit_event_log(raw: EventLog) -> Result<EventLog, RefusalReason>;
    fn admit_ocel_log(raw: OcelLog) -> Result<OcelLog, RefusalReason>;
    fn admit_xes_log(raw: XesLog) -> Result<XesLog, RefusalReason>;
}

// Implement this trait to export compat functions:
pub struct CompatComponent;

impl Guest for CompatComponent {
    fn admit_event_log(raw: EventLog) -> Result<EventLog, RefusalReason> {
        // Bridge to src/admission.rs
        match crate::admission::Admit::admit(&raw) {
            Ok(admitted) => Ok(/* convert to WIT EventLog */),
            Err(refusal) => Err(/* convert to WIT RefusalReason */),
        }
    }
}
```

### Type Conversion Bridge

The wit-bindgen generated types and wasm4pm-compat internal types must be converted:

```rust
// Bridge module: src/wit_bridge.rs (hypothetical)

impl From<crate::admission::Admission<EventLog, Witness>> for wit_bindings::EventLog {
    fn from(admitted: crate::admission::Admission<EventLog, Witness>) -> Self {
        wit_bindings::EventLog {
            events: admitted.value.events.into_iter().map(|e| /* ... */).collect(),
            traces: admitted.value.traces.into_iter().map(|t| /* ... */).collect(),
            metadata: /* ... */,
            witness_id: Witness::KEY.to_string(), // Convert phantom type to string
        }
    }
}

impl From<crate::admission::Refusal<R, W>> for wit_bindings::RefusalReason {
    fn from(refusal: crate::admission::Refusal<R, W>) -> Self {
        match refusal.reason {
            DanglingEventObjectLink { event_id, object_id, object_type } => {
                wit_bindings::RefusalReason::DanglingEventObjectLink {
                    event_id,
                    object_id,
                    object_type,
                }
            }
            // ... one arm per law
        }
    }
}
```

---

## Part 6: Type-Law Receipt (ALIVE Gate) in WIT

The ALIVE gate (`cargo test --test ui_tests`) validates type law through compile-fail trybuild fixtures. In a Component Model world, these fixtures become **WIT validation tests**.

### Compile-Fail WIT Fixtures

**Example 1: Witness Mismatch**

```wit
// tests/ui/wit/compile_fail/witness_mismatch.wit
record admitted-log-for-xes {
  data: list<u8>,
  witness-id: string,  // Claims XES witness
}

interface invalid {
  // Trying to admit as OCEL when witness says XES
  admit-as-ocel: func(log: admitted-log-for-xes) -> result<ocel-log, refusal-reason>;
}
```

**Expected error (from wit-bindgen validation):**
```
error: type mismatch in function signature
  → witness-id claim mismatch: expected "ocel20", found "xes1849"
```

**Example 2: Invalid Loss Policy**

```wit
// tests/ui/wit/compile_fail/invalid_loss_policy.wit
interface invalid {
  project-without-policy: func(
    admitted: ocel-log
    // Missing loss-policy parameter
  ) -> result<xes-log, refusal-reason>;
}
```

**Expected error:**
```
error: missing required parameter: loss-policy
```

### Compile-Pass WIT Fixtures

**Example: Valid Admission and Projection**

```wit
// tests/ui/wit/compile_pass/valid_admission_projection.wit
interface valid {
  use compat:types.{ocel-log, xes-log, loss-report, refusal-reason};
  use compat:loss.{loss-policy};
  
  admit-and-project: func(raw: ocel-log, policy: loss-policy) ->
    result<record { xes-log: xes-log, report: loss-report }, refusal-reason>;
}
```

**Validation:** wit-bindgen generates bindings without error.

---

## Part 7: Implementation Roadmap

### Phase 1: WIT File Generation (2–3 weeks)

**Tasks:**
1. Write `ggen/wit/types.wit` (shared types: event, trace, metric, etc.)
2. Write `ggen/wit/compat.wit` (admission interface)
3. Write `ggen/wit/compat-formats.wit` (admission + loss)
4. Write `ggen/wit/compat-strict.wit` (admission + strict)
5. Write `ggen/wit/compat-wasm4pm.wit` (admission + graduation)
6. Write `ggen/wit/compat-all.wit` (all four interfaces)
7. Write `ggen/wit/engine.wit` (discovery + replay + conformance + ocpq + receipts)

**Deliverables:**
- 7 .wit files (300–500 lines total)
- Component Model-compliant syntax
- All refusal reasons documented

### Phase 2: wit-bindgen Integration (1–2 weeks)

**Tasks:**
1. Add `wit-bindgen = "0.20"` to Cargo.toml
2. Create `build.rs` to invoke wit-bindgen for each feature combination
3. Configure build.rs to detect Cargo features and select appropriate world
4. Verify generated types in `target/wit-gen/` directory

**Deliverables:**
- Functional build.rs
- Generated Rust bindings for all feature combinations
- No build errors

### Phase 3: Compat → WIT Bridge (2–3 weeks)

**Tasks:**
1. Create `src/wit_bridge.rs` module for type conversions
2. Implement `From<Admission<T, W>> → WIT<T>` for each type
3. Implement `From<Refusal<R, W>> → WIT::RefusalReason` for each law
4. Implement `Guest` trait for compat world
5. Test roundtrip: Rust struct → WIT → Rust struct

**Deliverables:**
- Zero-cost conversion bridge
- Lossless type mapping
- Integration tests passing

### Phase 4: Type-Law Receipt Tests (1 week)

**Tasks:**
1. Write WIT compile-fail fixtures (witness mismatch, missing loss policy, etc.)
2. Write WIT compile-pass fixtures (valid admission, projection, strict checks)
3. Add WIT validation to CI pipeline
4. Verify ALIVE gate covers WIT-level law

**Deliverables:**
- 10–15 WIT fixtures (5 compile-fail, 10 compile-pass)
- CI integration
- ALIVE gate updated to include WIT validation

### Phase 5: Engine Integration (4–6 weeks)

**Tasks:**
1. Implement engine world import stubs (discovery, replay, conformance, ocpq, receipts)
2. Link engine imports to compat exports (admission → discovery dependency chain)
3. Implement discovery algorithms (DFG, Petri, BPMN)
4. Implement replay and conformance interfaces
5. Implement OCPQ and receipt generation
6. Test end-to-end: Compat admission → Engine discovery → Conformance metrics

**Deliverables:**
- Engine world bindings
- Executable discovery, replay, conformance, OCPQ, receipts
- Integration tests showing compat → engine flow

---

## Part 8: Key Principles Applied

1. **Type Law Over Strings**
   - Named-law refusals prevent silent failures
   - Witness markers prevent witness confusion
   - WIT variant discriminants encode law names

2. **Pure Compat, Executable Engine**
   - Compat exports pure functions (no state)
   - Engine imports executable algorithms
   - Clear boundary: structure vs. logic

3. **Zero-Cost Abstraction**
   - Witness phantom types compile away
   - WIT records map 1:1 to Rust structs
   - No runtime overhead for type law

4. **Feature Gating in Architecture**
   - Different WIT files for different feature sets
   - build.rs selects world based on Cargo features
   - No feature-gating pollution in WIT semantics

5. **WASI Compatibility**
   - Component Model is WASI standard
   - wit-bindgen supports all languages (Rust, Go, Python, TypeScript, etc.)
   - Cross-language polyglot interop achievable

---

## References

All cited sources with full URLs:

1. [WIT Reference - Component Model](https://component-model.bytecodealliance.org/design/wit.html) — Type system, function signatures, interface composition
2. [WIT Specification (MVP)](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) — Record/variant/result encoding, package structure, world definitions
3. [Component Model Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md) — Resource ownership, component linking, ABI neutrality
4. [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) — Code generation tooling, Rust bindings
5. [wasmtime component bindgen](https://docs.wasmtime.dev/api/wasmtime/component/macro.bindgen.html) — Generated code structure, macro usage
6. [wit-bindgen crates.io](https://crates.io/crates/wit-bindgen) — Package documentation

---

## Output Summary

Two comprehensive documents have been created:

| Document | Lines | Purpose |
|----------|-------|---------|
| `component-model-map.md` | 652 | Deep dive into WIT types, functions, error handling, witness encoding, wit-bindgen integration |
| `wit-surface-ledger.yaml` | 841 | Detailed specification of compat world + engine world interfaces, refusal encoding, feature gating, implementation roadmap |

**Total Research + Output:** ~1,500 lines of architecture documentation.

**Next Actions:**
1. Review WIT file syntax against `component-model-map.md` sections 1–2
2. Create `ggen/wit/*.wit` files following `wit-surface-ledger.yaml` specifications
3. Configure `build.rs` for wit-bindgen integration (Phase 2)
4. Implement type bridges in `src/wit_bridge.rs` (Phase 3)
5. Add WIT validation tests to ALIVE gate (Phase 4)
6. Implement engine world algorithms (Phase 5)
