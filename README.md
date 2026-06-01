# wasm4pm-compat

![nightly-only](https://img.shields.io/badge/toolchain-nightly--only-orange)
![no-unsafe](https://img.shields.io/badge/unsafe-forbid%28unsafe__code%29-red)
![structure-only](https://img.shields.io/badge/scope-structure--only-blue)

> **Nightly Rust required. Applications conform upward to future type law.**

> *Start with compatibility. Graduate to execution.*

A **nightly-only, paper-complete, feature-capped** Rust crate that defines the
*structure* of process-mining evidence — and the *laws* of how that evidence
crosses boundaries. It is the on-ramp to the [`wasm4pm`] execution engine.

**This crate requires nightly Rust.** The `rust-toolchain.toml` pins to nightly.
The crate root declares `#![feature(generic_const_exprs, adt_const_params,
const_trait_impl, min_specialization, portable_simd)]`. There is no stable
build target and no MSRV. Applications must conform upward to the type law,
not the other way around.

---

## What this crate **is**

- A **structure-only standard** for process evidence. It knows the full canon:
  events, traces, logs, **OCEL**, **XES**, **BPMN**, **Petri nets**, **WF-nets**,
  **OC-Petri-nets**, **POWL**, **process trees**, **Declare**, **OC-Declare**,
  **OCPQ**, **DFG**, **conformance verdicts**, **prediction problems**, and
  **receipt-shaped evidence**.
- A **boundary layer**. External formats are *admitted* into typed compat
  values, then *exported* back out — or *graduated* to `wasm4pm`.
- A crate where **refusal is first-class**. Every serious surface refuses with a
  *specific named law* (e.g. `DanglingEventObjectLink`, `FlatteningLoss`,
  `MissingFinalMarking`, `UnsoundWfNet`) — never a bare `InvalidInput`.
- Built from **small, transparent, strongly-named types**: `PhantomData`
  witness/state markers and zero-cost `#[repr(transparent)]` ID wrappers.

## What this crate is **NOT**

- **Not** a lite `wasm4pm`. It contains **no engines** — no discovery, no
  conformance checking, no replay, no alignment, no optimization, no
  visualization.
- **Not** a raw format-to-format laundromat. There is no path from one external
  format directly to another. The only legal route is:

  ```text
  external  →  typed admitted compat  →  external  |  wasm4pm
  ```

- **Not** a place where loss is silent. Lossy projection always requires a
  named projection **plus** a loss policy, a loss report, and a refusal path.

---

## Key Concepts

### Evidence lifecycle

The central invariant is a typed, one-way lifecycle enforced by the type system:

```text
Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ {Projected | Exportable | Receipted}
  │                                  ▲
  └────────────── refuse ────────────┴──▶ Refused  (terminal; carries a named law)
```

`Evidence<T, State, W>` is the universal carrier. `State` and `W` are
`PhantomData` tags — zero-cost at runtime. `Evidence<T, Raw, W>` and
`Evidence<T, Admitted, W>` are **different types**. A function demanding
admitted evidence cannot be called with raw evidence.

### Witness markers

A witness (e.g. `Ocel20`, `Xes1849`, `WfNetSoundnessPaper`) is a zero-sized
empty enum that names *which authority* a piece of evidence answers to.
`Admission<T, Ocel20>` and `Admission<T, Xes1849>` are different types — the
type system prevents silent confusion between standards.

### LossPolicy

Lossy projection requires a `LossPolicy` decided **before** any loss occurs:

| Variant | Meaning |
|---|---|
| `RefuseLoss` | Loss is not tolerated; projection must refuse |
| `AllowNamedProjection` | Loss is permitted under a named `ProjectionName` |
| `AllowLossWithReport` | Loss is permitted and a `LossReport` enumerating dropped items is required |

### ALIVE gate

`cargo test --test ui_tests -- --ignored` runs trybuild fixtures:

- **compile-fail fixtures** — each must fail for the *intended named law*, not
  accidentally. A fixture that fails for the wrong reason is not a valid
  type-law receipt.
- **compile-pass fixtures** — each must compile successfully, proving the lawful
  path is open.

The ALIVE gate is the certification that the type law is structurally sound.

---

## Quick Example

The full `Raw → Admitted` path, using the `Admit` trait:

```rust,ignore
use wasm4pm_compat::admission::{Admit, Admission, Refusal};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Raw;
use wasm4pm_compat::witness::Ocel20;

/// Admit a bool that represents "has at least one object link".
enum LinkedOcel {}

impl Admit for LinkedOcel {
    type Raw = bool;
    type Admitted = bool;
    type Reason = &'static str;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<bool, Raw, Ocel20>,
    ) -> Result<Admission<bool, Ocel20>, Refusal<&'static str, Ocel20>> {
        if raw.value {
            Ok(Admission::new(true))
        } else {
            Err(Refusal::new("DanglingEventObjectLink"))
        }
    }
}

// Raw evidence enters at the boundary.
let raw: Evidence<bool, Raw, Ocel20> = Evidence::raw(true);

// The only way to reach Admitted is through Admit::admit.
let admitted = LinkedOcel::admit(raw).unwrap().into_evidence();

// Admitted evidence can now proceed to Projected, Exportable, or Receipted.
let exportable = admitted.into_exportable();
assert_eq!(exportable.value, true);
```

For runnable examples, see the `examples/` directory.

---

## What's In The Box

Always-on modules (the canon — present with `--no-default-features`):

| Module | What it provides |
|---|---|
| `law` | Compile-time law kernel: `Assert`/`IsTrue`/`Require` bounds, `ConditionCell<BITS>`, `Between01<NUM,DEN>`, `ConstParamTy` enum set |
| `evidence` | `Evidence<T, State, W>` — the universal lifecycle carrier |
| `state` | Lifecycle stage tokens: `Raw`, `Parsed`, `Admitted`, `Refused`, `Projected`, `Exportable`, `Receipted` |
| `witness` | `Witness` trait and all named authority markers (`Ocel20`, `Xes1849`, `WfNetSoundnessPaper`, …) |
| `admission` | `Admit` trait, `Admission<T,W>`, `Refusal<R,W>` — the only `Raw → Admitted` path |
| `loss` | `Project` trait, `LossPolicy`, `LossReport`, `ProjectionName`, `NamedLoss` |
| `eventlog` | `Event`, `Trace`, `EventLog`, `EventStream` — case-centric shapes with builder APIs |
| `ocel` | `OcelLog`, `OcelEvent`, `Object`, `EventObjectLink`, `ObjectObjectLink`, `ObjectChange` |
| `xes` | XES interchange shape |
| `bpmn` | BPMN model shape |
| `petri` | Petri net and WF-net shapes, `WfNetConst<SOUNDNESS>` |
| `powl` | POWL shape, `TreeProjectable` sealed trait |
| `process_tree` | Process tree shape, `TypedLoopNode<ARITY>` with compile-time arity law |
| `declare` | Declare and OC-Declare constraint shapes |
| `ocpq` | Object-centric process query shape |
| `dfg` | Directly-follows graph (DFG) shape |
| `conformance` | Conformance verdict shape, `Metric<KIND, NUM, DEN>` with `Between01` bounds |
| `prediction` | Prediction problem shape |
| `receipt` | Receipt-shaped provenance envelope |
| `ids` | Zero-cost `#[repr(transparent)]` identifier wrappers |
| `diagnostic` | Named diagnostic codes for all boundary violations |
| `interop` | Import/export/round-trip plumbing traits, `Pm4pyShape` |
| `causal_net` | Causal net structural shape (Heuristics Miner output) |
| `prelude` | Re-exports of the most-needed shapes and laws |
| `nightly_foundry` | Always-on staging area: `petri_law`, `powl_law`, `evidence_law`, `token_law` |

Feature-gated modules:

| Module | Feature | What it adds |
|---|---|---|
| `formats` | `formats` (default on) | `ImportFormat`, `ExportFormat`, `FormatExport`, `FormatKind`, `RoundTripClaim`, `LossyFormatExport` |
| `strict` | `strict` | `ProcessBoundary`, `StrictCheck`, `StrictViolation`, `ExportBoundaryConst<HAS_WITNESS, HAS_ROUND_TRIP>` |
| `graduation` | `wasm4pm` | `GraduateToWasm4pm`, `GraduationCandidate`, `GraduationReason` |

---

## Not In The Box

These capabilities belong in `wasm4pm`, not here:

| Capability | Why it graduates |
|---|---|
| Process discovery (Alpha, Inductive, Heuristic miners) | Requires log replay and causal matrix computation |
| Conformance checking (alignments, token replay) | Requires model + log execution |
| Performance/variant mining | Requires aggregation over full event data |
| Log-model fitness/precision scores | Engine computation over the admitted log |
| WF-net soundness verification | Reachability analysis — engine work |
| POWL language-equivalence proof | Tree projection and language containment |
| OCPQ query execution | Object graph traversal |
| Predictive monitoring | ML inference over event sequences |

The graduation path: admit your evidence here, then hand the `GraduationCandidate`
(via the `wasm4pm` feature's bridge traits) to the engine.

---

## Test Surfaces

| Surface | Purpose | Command | Cadence |
|---|---|---|---|
| Unit + integration tests | Fast behavior checks | `cargo test --all-features --tests` | Sub-second after first build; run every change |
| ALIVE gate (trybuild) | Type-law receipts — compile-fail and compile-pass fixtures | `cargo test --test ui_tests -- --ignored` | Explicit opt-in; ~4 min cold |
| Documentation audit | Verify every public doctest compiles | `cargo test --doc --all-features` | Explicit opt-in; slow on nightly |

**Rule:** Doctests teach usage. Trybuild proves law.

Doctests are disabled in the default test run (`doctest = false` in `Cargo.toml`).
Every doctest touching `generic_const_exprs` or `adt_const_params` types triggers
a separate nightly `rustc` invocation — 200+ such invocations make `cargo test`
take minutes. Doc examples are still rendered by `cargo doc`.

---

## Feature model

The public feature surface is **exactly three**. Features control *capability
stages*, not *canon knowledge* — the base profile already knows every shape.

| Feature    | Default | What it adds                                                       |
|------------|:-------:|--------------------------------------------------------------------|
| `formats`  |   yes   | import/export contracts, round-trip claims, loss-policy surfaces    |
| `strict`   |   no    | opt-in boundary judgment: strict admission/refusal declaration      |
| `wasm4pm`  |   no    | graduation bridge traits toward the `wasm4pm` execution engine      |

There are **no per-format flags** (no `ocel`/`xes`/`bpmn`/`petri`/`powl`/…).
**Nightly is not a Cargo feature**: nightly toolchain is the requirement. The
crate is `#![forbid(unsafe_code)]` and has zero runtime dependencies.

---

## Base profile knows the canon

Disabling every optional feature does **not** make the crate forget any shape.
With `--no-default-features`, the always-on modules still define the complete
canon of process-evidence structure. Features add *what you may do at the
boundary*, not *what the crate knows*.

## Format covenant

Importers and exporters are **contracts**, not converters. An import produces a
typed, admitted compat value; an export consumes one. A round-trip claim is a
structured assertion that `export(import(x))` preserves the relevant structure —
or that it cannot, in which case the loss is *named and reported*, never hidden.

## Refusal law

Refusal is a value, not a panic. Each refusal carries a **specific named law**
identifying *which* structural obligation was violated. Bare `InvalidInput` is
forbidden. This is what makes admission auditable.

## Loss law

A lossy projection must declare its `ProjectionName`, obey a `LossPolicy`, and
emit a `LossReport`. If the policy forbids the loss, the projection refuses.
There is no way to drop structure quietly.

## Graduation path

When you need to *run* something — discover a model, check conformance, replay a
log — you graduate. With the `wasm4pm` feature, bridge traits hand your typed
compat evidence to the execution engine. The compat crate stays structure-only;
the engine does the work.

---

## Examples

```rust,ignore
use wasm4pm_compat::prelude::*;

let event = Event::new("place_order");
let trace = Trace::from_events([event]);
let log = EventLog::from_traces([trace]);
assert_eq!(log.trace_count(), 1);
```

Runnable examples in `examples/`:

| Example | Feature | What it demonstrates |
|---|---|---|
| `basic_eventlog` | (none) | Build `Event`/`Trace`/`EventLog` with builder API, validate structure, use `EventStream` |
| `basic_ocel` | (none) | Build `OcelLog` with E2O and O2O links, object changes, validate structural integrity |
| `ocel_to_xes_projection` | `formats` | Full OCEL → XES loss-covenant flow: `ProjectionName`, `LossPolicy`, `LossReport`, named refusal |
| `strict_boundary_claim` | `strict` | Declare a `ProcessBoundary`, run `StrictCheck`, observe named `StrictViolation` law codes |
| `graduation_candidate` | `wasm4pm` | Implement `GraduateToWasm4pm`, produce a `GraduationCandidate`, verify grounded/ungrounded |

---

## Verification commands

**Nightly toolchain required** (`rust-toolchain.toml` pins it; bare `cargo` is nightly).

```bash
# Dev loop — sub-second after initial build.
cargo build --all-features
cargo test  --all-features --tests
cargo clippy --all-features -- -D warnings
cargo fmt --check

# ALIVE gate — type-law receipts (explicit opt-in, ~4 min cold).
cargo test --test ui_tests -- --ignored

# Documentation audit (explicit opt-in).
cargo test --doc --all-features
```

---

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at
your option.

[`wasm4pm`]: https://github.com/wasm4pm
