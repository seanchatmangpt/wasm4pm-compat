# Quick Start — wasm4pm-compat

## What it is

`wasm4pm-compat` is a **structure-only, nightly-only, process-evidence type foundry**.

It knows the full canon of process-evidence shapes — events, traces, logs, OCEL, XES,
BPMN, Petri nets, WF-nets, OC-Petri-nets, POWL, process trees, Declare, OC-Declare,
OCPQ, DFG, conformance verdicts, prediction problems, and receipt-shaped evidence — and
represents them as small, strongly-named, transparent types with witness markers and
typestate wrappers.

The central contract: **start with compatibility, graduate to execution.** This crate
carries and validates the shape of evidence. The `wasm4pm` engine acts on it.

## Minimum setup

### rust-toolchain.toml

This crate requires nightly unconditionally. Add a toolchain file to your project root
if you do not already have one:

```toml
[toolchain]
channel = "nightly"
```

### Cargo.toml dependency

```toml
[dependencies]
wasm4pm-compat = "26.6.13"
# Optional: enable strict boundary judgment
# wasm4pm-compat = { version = "26.6.13", features = ["strict"] }
# Optional: enable the wasm4pm graduation bridge
# wasm4pm-compat = { version = "26.6.13", features = ["wasm4pm"] }
```

The `formats` feature is on by default. It adds import/export contracts, round-trip
claims, and loss surfaces. See [DESIGN_DECISIONS.md](DESIGN_DECISIONS.md) for why there
are exactly three features.

### Nightly feature flags in your crate root

If your crate uses `generic_const_exprs` types from this crate (e.g. `ConditionCell`,
`Between01`, `WfNetConst`), you must declare the same nightly features in your own
`lib.rs` or `main.rs`:

```rust
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
```

For most adopters that only use the shape types, no extra feature flags are needed in
your crate root.

## First five things you can do

### 1. Build and validate a case-centric event log

```rust
use wasm4pm_compat::eventlog::{Event, EventLog, Trace};

let trace = Trace::new("case-001", [
    Event::new("place_order").at_ns(1_000).by("web"),
    Event::new("approve").at_ns(2_000).by("alice"),
    Event::new("ship").at_ns(3_000).by("warehouse"),
]);
let log = EventLog::from_traces([trace]);
assert!(log.validate().is_ok());
```

### 2. Build and validate an object-centric event log (OCEL)

```rust
use wasm4pm_compat::ocel::{EventObjectLink, Object, OcelEvent, OcelLog};

let objects = [Object::new("ord-1", "order"), Object::new("item-9", "item")];
let events = [OcelEvent::new("e1", "place_order").at_ns(1_000)];
let e2o = [EventObjectLink::new("e1", "ord-1").qualified("places")];
let log = OcelLog::new(objects, events, e2o, [], []);
assert!(log.validate().is_ok());
```

### 3. Admit evidence through the boundary

```rust
use wasm4pm_compat::admission::{Admit, Admission, Refusal};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Raw;
use wasm4pm_compat::witness::Ocel20;

enum NonEmptyLog {}

impl Admit for NonEmptyLog {
    type Raw = String;
    type Admitted = String;
    type Reason = &'static str;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<String, Raw, Ocel20>,
    ) -> Result<Admission<String, Ocel20>, Refusal<&'static str, Ocel20>> {
        if raw.value.is_empty() {
            Err(Refusal::new("EmptyLog"))
        } else {
            Ok(Admission::new(raw.value))
        }
    }
}

let admitted = NonEmptyLog::admit(Evidence::raw("my-log".to_string())).unwrap();
let ev = admitted.into_evidence();
assert_eq!(ev.into_inner(), "my-log");
```

### 4. Use a loss policy for a lossy projection

```rust
use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};

enum OcelShape {}
enum XesShape {}

struct FlattenOcel { dropped_types: Vec<&'static str> }

impl Project for FlattenOcel {
    type From = OcelShape;
    type To = XesShape;
    type Lost = Vec<&'static str>;
    type Reason = &'static str;

    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<OcelShape, XesShape, Vec<&'static str>>, &'static str> {
        if !self.dropped_types.is_empty() && policy.is_refusing() {
            return Err("FlatteningLoss");
        }
        Ok(LossReport::new(
            ProjectionName("ocel-flatten-to-xes:by-order"),
            policy,
            self.dropped_types,
        ))
    }
}

let proj = FlattenOcel { dropped_types: vec!["item", "delivery"] };
let report = proj.project(LossPolicy::AllowLossWithReport).unwrap();
assert_eq!(report.lost, vec!["item", "delivery"]);
```

### 5. Construct a receipt envelope

```rust
use wasm4pm_compat::receipt::{Digest, ReplayHint, ReceiptEnvelope};

let envelope = ReceiptEnvelope::try_from_parts(
    "case-42",
    "discovery-run",
    Digest::new("blake3:abc123"),
    ReplayHint::new("wasm4pm://intake/case-42"),
).unwrap();
assert!(envelope.is_well_shaped());
```

## Annotated Evidence lifecycle example

The central invariant is a typed, one-way lifecycle enforced by the type system:

```
Raw -> Parsed -> Admitted -> { Projected | Exportable | Receipted }
  |                ^
  +-- refuse ------+---------> Refused  (terminal; carries a named law)
```

Here is a complete walk from raw input to a receipted value:

```rust
use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::receipt::{Digest, ReplayHint, ReceiptEnvelope};
use wasm4pm_compat::witness::Ocel20;

// Step 1 — receive untrusted input; tag it Raw.
let raw: Evidence<String, _, Ocel20> = Evidence::raw("ocel-bytes".to_string());

// Step 2 — format decoder accepted the bytes; advance to Parsed.
let parsed = raw.into_parsed();

// Step 3 — admission gate (named law check); advance to Admitted.
// In practice this comes from an Admit impl. Here we simulate it.
let admitted = Admission::<_, Ocel20>::new(parsed.value).into_evidence();

// Step 4a — exit via a named lossy projection to Projected, then Exportable.
let projected = admitted.into_projected();
let _exportable = projected.into_exportable();

// Step 4b — or advance directly to Receipted for hand-off to wasm4pm.
let admitted2 = Admission::<_, Ocel20>::new("ocel-bytes-2".to_string()).into_evidence();
let receipted = admitted2.into_receipted();
assert_eq!(receipted.value, "ocel-bytes-2");
```

Because `Evidence<T, Raw, W>` and `Evidence<T, Admitted, W>` are distinct types,
a function that accepts admitted evidence will not compile if you pass raw evidence.
The law is enforced at compile time, at zero runtime cost.

## Common patterns

### Guard on witness type

Use the witness type parameter to prevent cross-standard confusion:

```rust
use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::witness::{Ocel20, Xes1849};

// These are different types — they cannot be substituted for each other.
let _ocel_admitted: Admission<String, Ocel20> = Admission::new("ocel".to_string());
let _xes_admitted: Admission<String, Xes1849> = Admission::new("xes".to_string());
```

### Named refusal reason

Every refusal must carry a specific reason — never a bare "InvalidInput":

```rust
use wasm4pm_compat::admission::Refusal;
use wasm4pm_compat::witness::WfNetSoundnessPaper;

enum WfNetLaw { MissingFinalMarking, UnsoundNet }

let r = Refusal::<_, WfNetSoundnessPaper>::new(WfNetLaw::MissingFinalMarking);
assert!(matches!(r.reason, WfNetLaw::MissingFinalMarking));
```

### Check a witness key at runtime

```rust
use wasm4pm_compat::witness::{Witness, Ocel20};

assert_eq!(Ocel20::KEY, "ocel-2.0");
assert_eq!(Ocel20::TITLE, "OCEL 2.0");
assert_eq!(Ocel20::YEAR, Some(2023));
```

## What it is NOT

- **Not an execution engine.** No process discovery, no conformance checking,
  no token replay, no alignment computation, no visualization. Those belong in
  `wasm4pm`.

- **Not a data-laundering layer.** You cannot pass raw bytes directly to an
  export format. Every value must be admitted through a named boundary, and every
  lossy projection must declare a `LossPolicy` and emit a `LossReport`.

- **Not a lite build of wasm4pm.** It is a compatibility surface: the agreed shape
  of evidence before any engine processes it.

- **Not stable-Rust compatible.** The type law (`generic_const_exprs`,
  `adt_const_params`, `min_specialization`) requires nightly unconditionally.
  There is no stable build target and no MSRV.

- **Not extensible with extra Cargo features.** The public feature surface is
  exactly three (`formats`, `strict`, `wasm4pm`). Adding per-format flags would
  break the contract; opening issues or PRs for new features is welcomed, but the
  three-feature cap is a non-negotiable design invariant.
