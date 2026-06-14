# API Tour — wasm4pm-compat

This document is a guided walk through the most-used API surfaces. Each section
has working code snippets. All snippets use the `prelude` or explicit module paths
— import whichever fits your style.

---

## 1. The Evidence lifecycle

Evidence moves through a strictly-ordered, type-enforced lifecycle. Each stage
is an empty enum used as a `PhantomData` tag inside `Evidence<T, State, W>`.

```
Raw -> Parsed -> Admitted -> { Projected | Exportable | Receipted }
  |                ^
  +-- refuse ------+---> Refused   (terminal; carries a named law)
```

Stage types live in `wasm4pm_compat::state`:

| Stage | Meaning |
|-------|---------|
| `Raw` | Untrusted input, just received |
| `Parsed` | Format-decoder accepted the shape; not yet judged |
| `Admitted` | Passed an `Admit` gate against a named `Witness` |
| `Refused` | Terminal; a specific named law was violated |
| `Projected` | Result of a named lossy projection |
| `Exportable` | Cleared to leave the crate as an external value |
| `Receipted` | Wrapped in a provenance receipt; ready for `wasm4pm` |

### Building Raw evidence

```rust
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::witness::Ocel20;

// Only Raw is freely constructable.
let raw = Evidence::<_, _, Ocel20>::raw("some-bytes");
assert_eq!(raw.value, "some-bytes");
```

### Fast-reject before parsing

```rust
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::witness::Xes1849;

let raw: Evidence<&[u8], _, Xes1849> = Evidence::raw(b"".as_ref());
let refused = raw.refuse();  // Raw -> Refused, before even parsing
let _ = refused.as_refused_value();
```

### Advancing Raw -> Parsed -> Refused

```rust
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::witness::Xes1849;

// Parsed but structurally inadmissible (e.g. no traces).
let refused = Evidence::<_, _, Xes1849>::raw(vec![])
    .into_parsed()
    .into_refused();
let _ = refused.into_refused_value();
```

### The only path to Admitted: an `Admit` impl

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

    fn admit(raw: Evidence<String, Raw, Ocel20>)
        -> Result<Admission<String, Ocel20>, Refusal<&'static str, Ocel20>>
    {
        if raw.value.is_empty() {
            Err(Refusal::new("EmptyLog"))
        } else {
            Ok(Admission::new(raw.value))
        }
    }
}

let ev = NonEmptyLog::admit(Evidence::raw("log-data".to_string()))
    .unwrap()
    .into_evidence();
assert_eq!(ev.into_inner(), "log-data");
```

`Evidence::sealed` (the `Admitted` constructor) is `pub(crate)`. External code
reaches `Admitted` only through `Admission::into_evidence()`, which is only
returned by an `Admit::admit` implementation. This is the one-way door.

### From Admitted to the terminal stages

```rust
use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::witness::Ocel20;

// Path A: Admitted -> Projected -> Receipted
let receipted_via_proj = Admission::<_, Ocel20>::new("v")
    .into_evidence()
    .into_projected()
    .into_receipted();

// Path B: Admitted -> Exportable -> Receipted
let receipted_via_export = Admission::<_, Ocel20>::new("v")
    .into_evidence()
    .into_exportable()
    .into_receipted();

// Path C: Admitted -> Receipted (direct)
let receipted_direct = Admission::<_, Ocel20>::new("v")
    .into_evidence()
    .into_receipted();

assert_eq!(receipted_via_proj.value, "v");
assert_eq!(receipted_via_export.value, "v");
assert_eq!(receipted_direct.value, "v");
```

### Builder ergonomics: `PowlBuilder`

Construction stays terse without weakening the type law. `PowlBuilder` is a fluent arena
builder for POWL models; its terminal `build()` is *checked* and returns a named
`PowlRefusal` on malformed input (use `build_unchecked()` only when the shape is already
known lawful).

```rust
use wasm4pm_compat::powl::{PowlBuilder, PowlRefusal};

// A lawful choice with two branches builds.
let model = PowlBuilder::new()
    .atom("a")
    .atom("b")
    .choice("c", &["a", "b"])
    .build();
assert!(model.is_ok());

// A choice with a single branch is refused — by name, not by panic.
let bad = PowlBuilder::new()
    .atom("a")
    .choice("c", &["a"])
    .build();
assert!(matches!(bad, Err(PowlRefusal::InvalidChoiceArity { .. })));
```

Other builder surfaces follow the same shape across the canon: `Event::new(…)`,
`Trace::from_events(…)`, `EventLog::from_traces(…)`, and the OCEL link/change builders.

---

## 2. The Witness marker system

A witness is a zero-sized empty enum that names a standard, paper, API grammar,
Rust law, or internal bridge. It is the type-level authority a value is admitted,
projected, or graduated against.

### Why witnesses exist

`Admission<T, Ocel20>` and `Admission<T, Xes1849>` are different types. You
cannot pass an OCEL admission where an XES admission is expected. The compiler
enforces the boundary at zero runtime cost.

### The `Witness` trait

```rust
use wasm4pm_compat::witness::{Witness, WitnessFamily, Ocel20};

assert_eq!(Ocel20::KEY, "ocel-2.0");
assert_eq!(Ocel20::TITLE, "OCEL 2.0");
assert_eq!(Ocel20::YEAR, Some(2023));
assert_eq!(Ocel20::FAMILY, WitnessFamily::Standard);
```

### Witness families

| Family | Examples |
|--------|---------|
| `Standard` | `Ocel20`, `Xes1849`, `XesLifecycleExt`, `OcelObjectType` |
| `Paper` | `WfNetSoundnessPaper`, `PowlPaper`, `InductiveMiner`, `AlphaMiner` |
| `ApiGrammar` | `Pm4pyApiGrammar`, `PmaxConsumerGrammar` |
| `RustLaw` | `RustTypestateLaw` |
| `InternalBridge` | `Wasm4pmBridge` |

### All canonical witnesses (as of this writing)

`Ocel20`, `Xes1849`, `Pm4pyApiGrammar`, `PmaxConsumerGrammar`, `PowlPaper`,
`ObjectCentricPetriNetPaper`, `WfNetSoundnessPaper`, `OcpqPaper`, `DeclareFamily`,
`PredictiveMonitoringFamily`, `ReceiptFamily`, `RustTypestateLaw`, `Wasm4pmBridge`,
`YawlPaper`, `SeparableWfNetPaper`, `WorkflowPatternsPaper`, `InductiveMiner`,
`DeclareConstraints`, `AlignmentPaper`, `OcPetriNets`, `LogSkeleton`, `AlphaMiner`,
`XesLifecycleExt`, `XesConceptExt`, `OcelObjectType`, `OcelEventType`,
`OcelAttributeType`, `WfNet2Powl`, `DivergenceWitness`, `ConvergenceWitness`.

### Using sub-witnesses for fine-grained authority

```rust
use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::witness::{Ocel20, OcelObjectType};

// Tag the overall log against the full OCEL 2.0 standard.
let _log_admission: Admission<String, Ocel20> = Admission::new("full-log".to_string());

// Tag a specific object-type declaration against the object-type namespace authority.
let _type_admission: Admission<String, OcelObjectType> = Admission::new("order".to_string());
```

---

## 3. The LossPolicy chain

Some translations between process-evidence shapes cannot be lossless. Flattening
an OCEL log to XES drops all object types except the chosen case notion. This
module makes that loss accountable.

### The three policies

```rust
use wasm4pm_compat::loss::LossPolicy;

// Refuse any projection that would drop evidence.
let _ = LossPolicy::RefuseLoss;

// Allow loss, but only under an explicitly named projection.
let _ = LossPolicy::AllowNamedProjection;

// Allow loss and require a LossReport enumerating what was dropped.
let _ = LossPolicy::AllowLossWithReport;
```

### The `Project` trait

```rust
use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};

enum OcelShape {}
enum XesShape {}

struct Flatten { dropped: Vec<&'static str> }

impl Project for Flatten {
    type From = OcelShape;
    type To = XesShape;
    type Lost = Vec<&'static str>;
    type Reason = &'static str;

    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<OcelShape, XesShape, Vec<&'static str>>, &'static str> {
        if !self.dropped.is_empty() && policy.is_refusing() {
            return Err("FlatteningLoss");
        }
        Ok(LossReport::new(
            ProjectionName("ocel-flatten-to-xes:by-order"),
            policy,
            self.dropped,
        ))
    }
}

// Refused path
let refused = Flatten { dropped: vec!["item"] }.project(LossPolicy::RefuseLoss);
assert_eq!(refused, Err("FlatteningLoss"));

// Reporting path
let report = Flatten { dropped: vec!["item"] }
    .project(LossPolicy::AllowLossWithReport)
    .unwrap();
assert!(!report.is_lossless());
assert_eq!(report.lost, vec!["item"]);
let summary = report.summary("DroppedObjectTypeLinks");
assert_eq!(summary.projection().as_str(), "ocel-flatten-to-xes:by-order");
```

### Compile-time loss categories with `NamedLossConst`

When the loss category is known at compile time, bake it into the type:

```rust
use wasm4pm_compat::loss::NamedLossConst;

type DroppedLinks = NamedLossConst<"DroppedObjectTypeLinks">;
type FlattenedRel = NamedLossConst<"FlattenedMultiObjectRelation">;

// Two distinct loss categories are two distinct types.
assert_eq!(DroppedLinks::NAME, "DroppedObjectTypeLinks");
assert_ne!(DroppedLinks::NAME, FlattenedRel::NAME);
```

### Multi-step loss accounting with `LossChain`

```rust
use wasm4pm_compat::loss::{LossChain, NamedLoss, ProjectionName};

let mut chain = LossChain::new();
chain.push(NamedLoss::new(
    ProjectionName("ocel-flatten-to-xes:by-order"),
    "DroppedObjectTypeLinks",
));
chain.push(NamedLoss::new(
    ProjectionName("xes-to-dfg:aggregate"),
    "FlattenedTimestamps",
));
assert_eq!(chain.len(), 2);
assert!(!chain.is_lossless());
```

---

## 4. The receipt pattern

Receipts are provenance-bearing evidence envelopes. They carry a witness label,
a content digest, and a replay hint. They are **structure only** — the crate
never hashes, signs, or verifies; it carries and validates the form.

### `ReceiptShape` — the minimal form

```rust
use wasm4pm_compat::receipt::{Digest, ReplayHint, ReceiptShape};

let r = ReceiptShape::new(
    "discovery-run",
    Digest::new("blake3:abc123"),
    ReplayHint::new("rerun:plan#1"),
);
assert!(r.is_well_shaped());
```

### `ReceiptEnvelope` — adds a subject field

```rust
use wasm4pm_compat::receipt::{Digest, ReplayHint, ReceiptEnvelope, ReceiptRefusal};

// Safe constructor: refuses with a named law if any field is empty.
let ok = ReceiptEnvelope::try_from_parts(
    "case-42",
    "discovery-run",
    Digest::new("blake3:abc123"),
    ReplayHint::new("wasm4pm://intake/case-42"),
).unwrap();
assert!(ok.is_well_shaped());

// Named refusal: empty subject.
let bad = ReceiptEnvelope::try_from_parts(
    "",
    "discovery-run",
    Digest::new("blake3:abc123"),
    ReplayHint::new("wasm4pm://intake/case-42"),
);
assert_eq!(bad, Err(ReceiptRefusal::MissingSubject));
```

### `ReceiptChain` — multi-step provenance

```rust
use wasm4pm_compat::receipt::{
    Digest, ReplayHint, ReceiptChain, ReceiptEnvelope, ReceiptRefusal,
};

let root = ReceiptEnvelope::new("root", "w", Digest::new("d0"), ReplayHint::new("h0"));
let mut chain = ReceiptChain::try_new("run-001", vec![root]).unwrap();

let step = ReceiptEnvelope::new("step-1", "w", Digest::new("d1"), ReplayHint::new("h1"));
chain.extend_with(step).unwrap();
assert_eq!(chain.len(), 2);
assert_eq!(chain.root().subject, "root");
assert_eq!(chain.tip().subject, "step-1");

// Empty chain is refused with a named law.
assert_eq!(
    ReceiptChain::try_new("bad", vec![]),
    Err(ReceiptRefusal::EmptyChain)
);
```

### `ReceiptChainConst<N>` — fixed-arity, stack-allocated

```rust
use wasm4pm_compat::receipt::{
    Digest, ReplayHint, ReceiptChainConst, ReceiptEnvelope,
};

let a = ReceiptEnvelope::new("root", "w", Digest::new("d0"), ReplayHint::new("h0"));
let b = ReceiptEnvelope::new("step", "w", Digest::new("d1"), ReplayHint::new("h1"));
let chain = ReceiptChainConst::try_new("run-001", [a, b]).unwrap();
// Arity is a compile-time constant:
assert_eq!(chain.arity(), 2);
```

### Chain-sealing admission (`SealingAdmit` seam, v26.6.14)

The `SealingAdmit` trait is a companion seam *beside* `Admit` — it threads a
runtime [`RuntimeSeal`] (a recomputed BLAKE3 chain digest) into the verdict
atomically. The consumer supplies the BLAKE3 fold; `compat` proves shape + chain.

Running example: `cargo run --example sealing_admit_chain`

```rust,no_run
use wasm4pm_compat::admission::{recompute_and_match, RuntimeSeal, SealedAdmission};
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::AffidavitReceiptChain;

let claimed = Digest::new("blake3:abc");
// Consumer supplies the fold; compat proves the match.
let proof = recompute_and_match("events", &claimed, |_e| Digest::new("blake3:abc")).unwrap();
let seal = RuntimeSeal::from_verified_chain(claimed, proof);
let sealed: SealedAdmission<&str, AffidavitReceiptChain> =
    SealedAdmission::seal("payload", seal);
// The only bridge to Admitted state for a chain-sealed witness.
let _ = sealed.into_evidence().into_receipted();
```

Tampered claim: `recompute_and_match` returns `Err(Refusal<ChainHashMismatch,_>)` —
a named law violation, not a panic. See `examples/sealing_admit_chain.rs` for the
full consumer pattern with structural and chain-seal refusals.

### Sealing admitted evidence as Receipted

```rust
use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::witness::Ocel20;

let receipted = Admission::<_, Ocel20>::new("my-log")
    .into_evidence()
    .into_receipted();
assert_eq!(receipted.value, "my-log");
```

---

## 5. Process model types overview

All process model types are structure only. They carry shape; they do not execute.

### `eventlog` — case-centric event log

Core types: `Event`, `Trace`, `EventLog`, `EventStream`.

```rust
use wasm4pm_compat::eventlog::{Event, EventLog, Trace};

let trace = Trace::new("case-1", [Event::new("start").at_ns(0)]);
let log = EventLog::from_traces([trace]);
assert_eq!(log.trace_count(), 1);
assert!(log.validate().is_ok());
```

### `ocel` — object-centric event log

Core types: `Object`, `OcelEvent`, `EventObjectLink`, `ObjectObjectLink`,
`ObjectChange`, `OcelLog`.

```rust
use wasm4pm_compat::ocel::{Object, OcelEvent, OcelLog};

let log = OcelLog::new(
    [Object::new("o1", "order")],
    [OcelEvent::new("e1", "create").at_ns(0)],
    [], [], [],
);
assert!(log.validate().is_ok());
```

### `petri` — Petri net / WF-net shape

Core types: `PetriNet`, `Place`, `Transition`, `Arc`. For typed WF-nets with
compile-time soundness claims see `nightly_foundry::petri_law::WfNetConst`.

### `powl` — Partially Ordered Workflow Language

Core types: `PowlNode`, `PowlOperator`, `PowlModel`. The `TreeProjectable` sealed
trait enforces that only lawful POWL node types can enter a tree projection.

### `process_tree` — process tree shape

Core types: `ProcessTreeNode`, `TreeOperator`. `TypedLoopNode<ARITY>` uses
`Require<{ ARITY == 2 }>: IsTrue` to enforce binary loops at compile time.

### `dfg` — Directly-Follows Graph

Core types: `DfgNode`, `DfgEdge`, `DirectlyFollowsGraph`.

### `declare` — Declare constraint model

Core types: `DeclareConstraint`, `DeclareModel`, `ConstraintTemplate`.
See `witness::DeclareFamily` and `witness::DeclareConstraints` for the two
authority layers.

### `ocpq` — Object-Centric Process Querying

Core types: `OcpqQuery`, `OcpqPredicate`. See `witness::OcpqPaper`.

### `bpmn` — BPMN shape

Core types: `BpmnProcess`, `BpmnFlow`, `BpmnGateway`, `BpmnTask`.

### `conformance` — conformance verdict shape

Core types: `ConformanceVerdict`, `Metric<KIND, NUM, DEN>`.

`Metric<KIND, NUM, DEN>` uses `Between01<NUM, DEN>` from `law` to ensure the
metric value is provably in `[0, 1]` at the type level:

```rust
// This requires nightly feature flags in the caller.
// use wasm4pm_compat::conformance::Metric;
// use wasm4pm_compat::law::MetricKind;
// type Fitness = Metric<{ MetricKind::Fitness }, 9, 10>; // 0.9
```

### `prediction` — prediction problem shape

Core types: `PredictionProblem`, `PredictionTarget`. Structure only; no predictor.

### `causal_net` — causal net shape

Core types: `CausalNet`, `CausalNode`, `InputBinding`, `OutputBinding`.
Covers Heuristics Miner output (Weijters & Ribeiro 2011).
