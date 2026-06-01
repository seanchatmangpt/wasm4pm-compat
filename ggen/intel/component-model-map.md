# Component Model + WIT Intelligence Map
## wasm4pm-compat → WebAssembly Component Model Bridge

**Last Updated:** 2026-06-01  
**Status:** Design Research  
**References:** 
- [WIT Reference](https://component-model.bytecodealliance.org/design/wit.html)
- [WIT Specification](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md)
- [Component Model Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md)
- [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen)

---

## Part 1: WIT Type System Mapping to wasm4pm-compat

### Primitive Types (Scalars)

| WIT Type | Rust Type | wasm4pm-compat Use |
|----------|-----------|-------------------|
| `bool` | `bool` | Flags, lifecycle state indicators |
| `u8`–`u64` | `u8`–`u64` | Integer metrics, counts, IDs |
| `s8`–`s64` | `i8`–`i64` | Signed offsets, deltas |
| `f32`, `f64` | `f32`, `f64` | Metrics (fitness, precision), probabilities |
| `string` | `String` | Names, descriptions, diagnostic messages |
| `char` | `char` | Character markers for notation |

### Aggregate Types

#### Records → wasm4pm Structures

WIT `record` maps directly to Rust `struct`. Each named field becomes a Rust field.

**Example:** Event log metadata
```wit
record event-metadata {
  trace-id: string,
  timestamp-ns: u64,
  lifecycle-state: string,
}
```

Maps to:
```rust
#[derive(Clone, Debug)]
struct EventMetadata {
    trace_id: String,
    timestamp_ns: u64,
    lifecycle_state: String,
}
```

**wasm4pm Fields Requiring WIT Records:**
- `EventLog` structure (events, traces, object types, relationships)
- `OcelLog` (events, objects, object-object links, object changes)
- `XesLog` (traces, events, attributes)
- `ProcessBoundary` (kind, name, loss policy)
- `LossReport<From, To, Items>` (source type, target type, lost items)
- `Metric<KIND, NUM, DEN>` (kind, numerator, denominator)
- `StrictViolation` (violation kind, lawful stage name, diagnosis)

---

#### Variants → wasm4pm Enums and Discriminated Types

WIT `variant` is a discriminated union. Each case has a name and optional associated payload.

**Case 1: Enum-like (no payloads) → State Tokens**

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

Maps to Rust enum:
```rust
pub enum LifecycleState {
    Raw,
    Parsed,
    Admitted,
    Refused,
    Projected,
    Exportable,
    Receipted,
}
```

**wasm4pm State Tokens:** Raw, Parsed, Admitted, Refused, Projected, Exportable, Receipted

**Case 2: Discriminated Union with Payloads → Refusal Types**

```wit
variant refusal-reason {
  dangling-event-object-link(string),
  missing-final-marking(string),
  invalid-petri-structure(string),
}
```

Maps to Rust enum with associated data:
```rust
pub enum RefusalReason {
    DanglingEventObjectLink(String),
    MissingFinalMarking(String),
    InvalidPetriStructure(String),
}
```

**wasm4pm Refusal Types in WIT:**
- `RefusalReason` (carries named law as string field)
  - `DanglingEventObjectLink(law: string)`
  - `MissingFinalMarking(law: string)`
  - `InvalidPetriStructure(law: string)`
  - `CircularDependency(law: string)`
  - `HiddenProcessMiningGrowth(law: string)`
  - … (one variant per named law in `src/admission.rs`)

**Case 3: Admission/Refusal Result Encoding**

```wit
variant admission-result {
  admitted(record {
    value: u8,
    witness-id: string,
  }),
  refused(record {
    reason: refusal-reason,
    witness-id: string,
  }),
}
```

Maps to Rust:
```rust
pub enum AdmissionResult {
    Admitted { value: u8, witness_id: String },
    Refused { reason: RefusalReason, witness_id: String },
}
```

Or more idiomatically with `result<T, E>`:
```rust
pub type Admission<T, W> = Result<(T, String), (RefusalReason, String)>;
```

---

#### Results → wasm4pm Error Types

WIT `result<T, E>` is the standard error handling type.

**Forms in WIT:**
- `result<T, E>` — Success value `T` or error `E`
- `result<T>` — Success value `T` or error (unit)
- `result<_, E>` — Success (no value) or error `E`
- `result` — Success or error (both unit)

**wasm4pm Mapping:**

| Rust Type | WIT Result | Semantics |
|-----------|-----------|-----------|
| `Result<Admission<T,W>, Refusal<R,W>>` | `result<admission<T>, refusal<R>>` | Admit or refuse (witness-tagged) |
| `Result<Projected<T>, ProjectionRefusal>` | `result<projected<T>, projection-refusal>` | Project or refuse loss policy |
| `Result<LossReport<F,T>, LossError>` | `result<loss-report<F,T>, loss-error>` | Loss accounting or error |
| `Result<StrictViolation, CheckError>` | `result<strict-violation, check-error>` | Strict check pass (Ok) or violation (Err) |

**Key Principle:** Every result must carry named-law information. No generic "Error" or "InvalidInput" catch-alls.

---

### Lists → wasm4pm Collections

WIT `list<T>` maps to `Vec<T>` in Rust.

**wasm4pm Collections:**
- `list<event>` → `Vec<Event>`
- `list<trace>` → `Vec<Trace>`
- `list<object-type>` → `Vec<ObjectType>`
- `list<refusal-reason>` → `Vec<RefusalReason>` (for multi-error reporting)
- `list<loss-item>` → `Vec<LossItem>` (for loss granularity)
- `list<metric>` → `Vec<Metric>` (conformance metrics)

---

### Options → Optional Fields

WIT `option<T>` maps to `Option<T>`.

**wasm4pm Optional Fields:**
- `option<string>` → `Option<String>` for optional descriptions, diagnostic hints
- `option<loss-report>` → `Option<LossReport>` for conditional loss accounting
- `option<lifecycle-annotation>` → `Option<LifecycleAnnotation>` for optional object lifecycle states

---

### Resources → wasm4pm Type-Law Witnesses

WIT `resource<name>` represents a handle to an entity that cannot be copied, should be passed by reference, and has explicit ownership semantics.

**Design Decision:** Witness markers (`Ocel20`, `Xes1849`, `Declare3`, etc.) should be represented as **resources** in the engine world only, not in the compat world.

**Rationale:**
- Witnesses are non-copyable in the type-law semantics (each witness instance is unique)
- Witnesses carry implicit state (the law definition they represent)
- Witnesses should not escape a component via outer aliases
- Engines (discovery, conformance, replay) maintain witness instances across multiple operations

**WIT Resource Declaration:**

```wit
resource witness-ocel20 {
  constructor() -> witness-ocel20,
  method: func() -> result<bool>,
}

resource witness-xes1849 {
  constructor() -> witness-xes1849,
  method: func() -> result<bool>,
}
```

**Rust Binding (wit-bindgen):**
```rust
pub struct WitnessOcel20 {
    rep: wasmtime::component::Resource<WitnessOcel20>,
}

impl WitnessOcel20 {
    pub fn new() -> Self { /* generated */ }
    pub fn method(&self) -> Result<bool> { /* generated */ }
}
```

---

## Part 2: Function Signatures and Error Representation

### WIT Function Syntax

```wit
interface name {
  function-name: func(param1: type1, param2: type2, ...) -> result-type;
}
```

### Error Representation Strategies

#### Strategy A: Explicit `result<T, E>`

```wit
interface compat:admission {
  admit-event-log: func(raw: event-log) -> result<admitted-event-log, admission-refusal>;
}
```

**Advantages:**
- Clear error type in the signature
- Type-safe error handling
- Composable with other operations

**Disadvantages:**
- Requires detailed refusal type definitions
- Callers must pattern-match on results

#### Strategy B: Named Error Variants

```wit
variant admission-error {
  dangling-event-object-link(detail: string),
  missing-final-marking(detail: string),
  invalid-xes-attribute-type(detail: string),
}

interface compat:admission {
  admit-xes-log: func(raw: xes-log) -> result<admitted-xes-log, admission-error>;
}
```

**Advantages:**
- Human-readable error discriminants
- Each error variant carries contextual data
- Maps directly to wasm4pm `Refusal<R, W>` types

**Recommended for wasm4pm-compat:** Strategy B with witness annotation.

#### Strategy C: Diagnostic Records

```wit
record admission-failure {
  kind: string,           // "dangling-event-object-link"
  lawful-stage: string,   // "admitted"
  object-id: string,
  evidence: string,       // diagnostic detail
}

interface compat:admission {
  admit-event-log: func(raw: event-log) -> result<admitted-event-log, admission-failure>;
}
```

**Recommended for wasm4pm-compat:** Combined with Strategy B for richness.

---

## Part 3: Interface Composition and World Definitions

### Compat World (Type-Law Surface)

```wit
package wasm4pm:compat@1.0.0;

interface types {
  // Type definitions shared across compat world
  record event { /* ... */ }
  record trace { /* ... */ }
  record object-type { /* ... */ }
  enum lifecycle-state { raw, parsed, admitted, refused, ... }
  variant refusal-reason { /* ... */ }
  record loss-report { /* ... */ }
}

interface evidence {
  // Evidence lifecycle operations (import from runtime)
  // These are provided by the wasm4pm execution engine
}

interface admission {
  // Admission and refusal paths
  use types.{event, trace, event-log, refusal-reason};
  
  admit-event-log: func(raw: event-log) -> result<event-log, refusal-reason>;
  admit-ocel-log: func(raw: ocel-log) -> result<ocel-log, refusal-reason>;
  refuse-with-law: func(law-name: string) -> refusal-reason;
}

interface loss {
  // Loss accounting and projection
  use types.{loss-report};
  
  record loss-policy {
    kind: string,  // "refuse", "allow-named", "allow-with-report"
  }
  
  project-ocel-to-xes: func(
    admitted: ocel-log,
    policy: loss-policy
  ) -> result<xes-log, record { reason: refusal-reason, loss: loss-report }>;
}

interface strict {
  // Boundary judgment (optional, gated by wasm4pm feature)
  record boundary {
    kind: string,
    name: string,
    has-witness: bool,
    has-round-trip: bool,
  }
  
  check-strict-boundary: func(boundary: boundary) -> result<bool, string>;
}

interface graduation {
  // Graduation bridge (optional, gated by wasm4pm feature)
  record graduation-candidate {
    kind: string,
    is-grounded: bool,
    reason: option<string>,
  }
  
  graduate-to-wasm4pm: func(
    admitted: event-log
  ) -> result<graduation-candidate, string>;
}

world compat {
  export admission;
  export loss;
  export strict;
  export graduation;
}
```

### Engine World (Execution Surface)

```wit
package wasm4pm:engine@1.0.0;

interface discovery {
  // Process discovery algorithms
  use compat:types.{event-log, process-model};
  
  discover-dfg: func(log: event-log) -> result<process-model, string>;
  discover-petri: func(log: event-log) -> result<process-model, string>;
  discover-bpmn: func(log: event-log) -> result<process-model, string>;
}

interface replay {
  // Token replay and alignment
  use compat:types.{event-log, process-model};
  
  replay-on-petri: func(
    log: event-log,
    model: process-model
  ) -> result<record { fitness: f64, cost: u32 }, string>;
}

interface conformance {
  // Conformance metrics
  use compat:types.{event-log, process-model, metric};
  
  check-conformance: func(
    log: event-log,
    model: process-model
  ) -> result<list<metric>, string>;
}

interface ocpq {
  // Object-centric process querying
  use compat:types.{ocel-log};
  
  query-object-lifecycle: func(
    log: ocel-log,
    object-id: string
  ) -> result<list<record { event-id: string, timestamp: u64 }>, string>;
}

interface receipts {
  // Receipt/proof generation
  use compat:types.{event-log};
  
  generate-receipt: func(log: event-log) -> result<string, string>;
  verify-receipt: func(log: event-log, receipt: string) -> result<bool, string>;
}

world engine {
  import discovery;
  import replay;
  import conformance;
  import ocpq;
  import receipts;
}
```

---

## Part 4: Module Splitting and Imports/Exports

### Compat Component Boundary

**Exports (what compat provides to the runtime):**
- `admission::admit-event-log()`
- `admission::admit-ocel-log()`
- `loss::project-ocel-to-xes()`
- `strict::check-strict-boundary()`
- `graduation::graduate-to-wasm4pm()`

**Imports (what compat requires from the host):**
- `evidence::raw()`
- `evidence::parsed()`
- `evidence::admitted()` (only via `Admit::admit()`)
- Type definitions for witness markers

**Principle:** Compat is **read-only**. It does not instantiate engines or perform discovery/replay. It only transforms types and enforces type law.

### Engine Component Boundary

**Exports (what engine provides to the runtime or other engines):**
- `discovery::discover-dfg()`
- `discovery::discover-petri()`
- `replay::replay-on-petri()`
- `conformance::check-conformance()`
- `receipts::generate-receipt()`

**Imports (what engine requires from compat):**
- `admission::admit-*()` (to validate inputs)
- `loss::project-*()` (to transform formats)
- Type definitions

**Principle:** Engine is **executable**. It takes admitted evidence and produces process models, metrics, and receipts.

---

## Part 5: Witness Representation in WIT

### Non-Resource Witness in Compat World

Witnesses in compat are **type-level only** (zero-cost, compile-time). They do not cross component boundaries; they are phantom types.

```rust
// Rust representation (internal to compat):
pub struct Evidence<T, State, W: Witness> {
    value: T,
    _state: PhantomData<State>,
    _witness: PhantomData<W>,
}

// WIT representation (compat exports):
// Witnesses are NOT exported as resources; they are implicit in the structure name.
record admitted-event-log {
  events: list<event>,
  traces: list<trace>,
  witness-id: string,  // String tag for human readability
}
```

**Why No WIT Resources for Witness in Compat:**
- Witnesses are zero-cost types; encoding them as resources adds unnecessary overhead
- Compat does not maintain witness state across operations
- Witness identity is encoded in type names (`AdmittedEventLogOcel20` vs `AdmittedEventLogXes1849`)

### Resource Witness in Engine World (Optional)

If engines need to maintain witness context across discovery/replay operations:

```wit
resource witness-ocel20 {
  // Constructor and methods
}

interface engine:discovery {
  discover-with-witness: func(
    log: admitted-event-log,
    witness: borrow<witness-ocel20>
  ) -> result<process-model, string>;
}
```

---

## Part 6: wit-bindgen Integration

### Rust Binding Generation

```rust
// In Cargo.toml:
[dependencies]
wit-bindgen = "0.20"

// In lib.rs or main.rs:
wit_bindgen::generate!({
    world: "compat",
    path: "wit/compat.wit",
});

// Auto-generated module structure:
// - `bindings::compat::<interface>::*` — generated functions/types
// - `Guest` trait — for exported functions
// - `Component` wrapper — for world instantiation
```

### Generated Types Example

Given WIT:
```wit
record event-log {
  events: list<event>,
  metadata: event-metadata,
}

interface admission {
  admit-event-log: func(raw: event-log) -> result<event-log, refusal-reason>;
}
```

wit-bindgen generates:
```rust
pub struct EventLog {
    pub events: Vec<Event>,
    pub metadata: EventMetadata,
}

pub enum RefusalReason {
    DanglingEventObjectLink(String),
    MissingFinalMarking(String),
    // ... one per named law
}

pub trait Guest {
    fn admit_event_log(raw: EventLog) -> Result<EventLog, RefusalReason>;
}

impl Guest for CompatComponent {
    fn admit_event_log(raw: EventLog) -> Result<EventLog, RefusalReason> {
        // Implement here
    }
}
```

---

## Part 7: Type-Law Receipt Encoding

### Compile-Fail Fixtures in WIT

The ALIVE gate (`cargo test --test ui_tests`) validates type law through compile-fail trybuild fixtures. In a Component Model world, these fixtures would be **WIT files that fail to generate bindings**.

**Example: Witness Confusion**

```wit
// compile_fail/witness_wrong_type.wit
record admitted-log-xes {
  data: list<u8>,
  witness-id: string,  // Claims XES witness
}

// But we try to admit as OCEL
interface invalid {
  admit-as-ocel: func(log: admitted-log-xes) -> result<ocel-log, string>;
}
```

Expected error (from wit-bindgen validation):
```
error: type mismatch in function signature
  → witness-id mismatch: expected ocel20, found xes1849
```

---

## Part 8: Summary – Design Principles

1. **Types, not logic:** WIT interfaces describe structure and contracts, never computation. Execution logic lives in the engine world, not compat.

2. **Named-law refusals:** Every error type must carry a `refusal-reason` that names a specific law (e.g., `DanglingEventObjectLink`, not `Error`).

3. **Result-centric:** All fallible operations return `result<T, E>`, never `option<T>` for errors.

4. **Witness as string tags in compat, resources in engine:** Type-level witnesses are encoded as string identifiers in WIT; only engines that need to maintain witness context use resource handles.

5. **No memory assumptions:** Avoid arrays of fixed size, pointer arithmetic, or shared memory patterns. Use `list<T>` and `record` exclusively.

6. **World splits:**
   - **Compat world:** Exports admission, loss, strict, graduation. Imports none (pure functions).
   - **Engine world:** Imports compat types/functions. Exports discovery, replay, conformance, receipts.

7. **Zero-cost principle:** If a type exists only to satisfy WIT contracts but carries no runtime value, it should be marked `(zero-copy-unit)` or similar (not yet part of WIT MVP but relevant for optimization).

---

## References

- [WIT Reference - bytecodealliance](https://component-model.bytecodealliance.org/design/wit.html)
- [WIT Specification - GitHub](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md)
- [Component Model Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md)
- [wit-bindgen GitHub](https://github.com/bytecodealliance/wit-bindgen)
- [wasmtime component bindgen docs](https://docs.wasmtime.dev/api/wasmtime/component/macro.bindgen.html)
