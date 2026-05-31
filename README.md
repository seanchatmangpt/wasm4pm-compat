# wasm4pm-compat

> **Start with compatibility. Graduate to execution.**

A **minimal, paper-complete, feature-capped** Rust crate that defines the
*structure* of process-mining evidence — and the *laws* of how that evidence
crosses boundaries. It is the on-ramp to the [`wasm4pm`] execution engine.

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

## Feature model

The public feature surface is **exactly three**. Features control *capability
stages*, not *canon knowledge* — the base profile already knows every shape.

| Feature    | Default | What it adds                                                       |
|------------|:-------:|--------------------------------------------------------------------|
| `formats`  |   yes   | import/export contracts, round-trip claims, loss-policy surfaces    |
| `strict`   |   no    | opt-in boundary judgment: strict admission/refusal declaration      |
| `wasm4pm`  |   no    | graduation bridge traits toward the `wasm4pm` execution engine      |

There are **no per-format flags** (no `ocel`/`xes`/`bpmn`/`petri`/`powl`/…).
**Nightly is not a feature**: experimental type-level structure lives only
behind the custom cfg `#[cfg(wasm4pm_compat_nightly)]`, never a Cargo feature.
Stable Rust builds by default, and the crate is `#![forbid(unsafe_code)]`.

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

See `examples/` for runnable adoption walkthroughs.

---

## Verification commands

The crate must pass the full matrix on **stable** Rust:

```bash
# Default profile (formats only).
cargo build
cargo test
cargo doc --no-deps

# Minimal canon: no features at all.
cargo build  --no-default-features
cargo test   --no-default-features

# Each optional capability stage.
cargo build  --no-default-features --features formats
cargo build  --no-default-features --features strict
cargo build  --no-default-features --features wasm4pm

# Everything on.
cargo build  --all-features
cargo test   --all-features
cargo doc    --all-features --no-deps

# Lints.
cargo clippy --all-features -- -D warnings
cargo fmt    --check

# Nightly foundry (cfg, NOT a feature) — only when iterating on experiments.
RUSTFLAGS="--cfg wasm4pm_compat_nightly" cargo build
```

---

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at
your option.

[`wasm4pm`]: https://github.com/wasm4pm
