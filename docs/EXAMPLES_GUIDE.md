# Examples Guide — wasm4pm-compat

This document describes each example in the `examples/` directory, explains how to
run them, and provides guidance for writing new examples within the crate's remit.

---

## How to run examples

```bash
# Default feature set (includes `formats`)
cargo run --example <name>

# With optional features
cargo run --example <name> --features strict
cargo run --example <name> --features wasm4pm
cargo run --example <name> --all-features
```

All examples print to stdout. None of them read files, open network connections, or
write state anywhere. They are self-contained demonstrations of structure.

---

## Existing examples

### `basic_eventlog`

**File:** `examples/basic_eventlog.rs`
**Features required:** none (base profile)
**Run:** `cargo run --example basic_eventlog`

Demonstrates constructing the case-centric event-log shape. Builds two traces with
timestamped events and resource annotations, assembles them into an `EventLog`, calls
`validate()` to check structural well-shapedness, and prints a case-by-case activity
sequence.

Also demonstrates `EventStream` — the append-only online sibling of a log, for
streaming evidence that has not yet been assembled into full traces.

**Key types:** `Event`, `Trace`, `EventLog`, `EventStream`

**What it does not show:** discovery, conformance checking, replay. Those graduate to
`wasm4pm`.

---

### `basic_ocel`

**File:** `examples/basic_ocel.rs`
**Features required:** none (base profile)
**Run:** `cargo run --example basic_ocel`

Demonstrates constructing the object-centric event log (OCEL) shape. Builds two
objects (`order`, `item`), one event, event-to-object (E2O) links with qualifiers,
an object-to-object (O2O) link, and an object change (attribute update with timestamp).
Calls `validate()` for structural integrity.

This example highlights that OCEL is first-class in this crate, not "event log plus
extras". The distinction between E2O and O2O links, and the presence of object
changes, are all modeled as dedicated, named types.

**Key types:** `Object`, `OcelEvent`, `EventObjectLink`, `ObjectObjectLink`,
`ObjectChange`, `OcelLog`

**What it does not show:** flattening OCEL to XES (see `ocel_to_xes_projection`),
object-centric process discovery.

---

### `ocel_to_xes_projection`

**File:** `examples/ocel_to_xes_projection.rs`
**Features required:** `formats` (default)
**Run:** `cargo run --example ocel_to_xes_projection`

Demonstrates the OCEL-to-XES projection under the format covenant. Going from an
object-centric log to a flat log is inherently lossy (XES has one case notion; OCEL
has many). The example shows three scenarios:

1. **AllowWithReport** — the projection succeeds; the loss (dropped object types) is
   reported explicitly in a loss-facts list.
2. **ForbidLoss** — the same projection is refused with the named law
   `FlatteningLoss`, because dropping object types is not permitted.
3. **Unnamed target** — the projection is refused before any bytes are produced,
   because no case-notion object type was specified.

**Key types:** `FormatExport`, `FormatKind`, `Pm4pyShape` (from `interop`)

**Key concepts:** `LossPolicy`, named refusal, the "no raw laundering" covenant

---

### `strict_boundary_claim`

**File:** `examples/strict_boundary_claim.rs`
**Features required:** `strict`
**Run:** `cargo run --example strict_boundary_claim --features strict`

Demonstrates the strict-mode boundary judgment surface. Shows three scenarios:

1. A fully-attested export boundary that passes `check()`.
2. A boundary that is missing its loss policy and refusal path — refused with two
   named `StrictViolation`s: `MissingLossPolicy` and `MissingRefusalPath`.
3. A boundary that claims replay capability — refused with
   `HiddenProcessMiningGrowth`, because replay is engine territory.

**Key types:** `ProcessBoundary`, `ProcessBoundaryKind`, `StrictViolation`

**Key concept:** Strict mode judges *declarations*, not data. It never touches an
event log.

---

### `graduation_candidate`

**File:** `examples/graduation_candidate.rs`
**Features required:** `wasm4pm`
**Run:** `cargo run --example graduation_candidate --features wasm4pm`

Demonstrates declaring a graduation candidate — the bridge from `wasm4pm-compat`
to the `wasm4pm` execution engine. Implements `GraduateToWasm4pm` on a host type
that holds an admitted OCEL log awaiting discovery, then inspects the resulting
`GraduationCandidate`.

Shows that an ungrounded candidate (empty `evidence_ref`) is not reviewable — the
engine intake will reject it.

**Key types:** `GraduateToWasm4pm`, `GraduationCandidate`, `GraduationReason`

**Key concept:** The compat layer produces the *case for execution*; the engine
adjudicates it. Holding a `GraduationCandidate` does not run the engine.

---

## How to write a new example

Follow these rules to keep examples within the crate's remit:

### 1. Stay structure-only

Examples must not:
- Perform process discovery.
- Compute conformance metrics.
- Replay tokens.
- Compute alignments.
- Parse real event-log files (use hard-coded in-memory values).
- Open network connections or read environment variables.

If the scenario you want to demonstrate requires execution, the example belongs in
`wasm4pm`, not here. The closing line of your example should be something like:
`"(structure only — X graduates to wasm4pm)"`.

### 2. Put the feature guard at the top

If your example requires a non-default feature, wrap the body in
`#[cfg(feature = "...")]` and add a `#[cfg(not(feature = "..."))]` fallback `main`
that prints an `eprintln!` message with the correct `cargo run` invocation.

```rust
#[cfg(feature = "strict")]
fn main() {
    // ... example body ...
}

#[cfg(not(feature = "strict"))]
fn main() {
    eprintln!(
        "This example needs the `strict` feature.\n\
         Run with: cargo run --example my_example --features strict"
    );
}
```

### 3. Keep examples small and focused

Each example should demonstrate one concept. If you find yourself needing a helper
module inside the example (like `mod demo { ... }`), check whether the example is
trying to do too much.

### 4. Do not duplicate doctest content

If the exact code already appears in a module's doctests, the example does not add
value. Prefer doctests for individual-API demonstration and examples for end-to-end
flows (construction + admission + projection + receipt) that cross multiple modules.

### 5. Add the example to this guide

When your example is merged, add an entry to this file with:
- Filename
- Features required
- `cargo run` invocation
- One-paragraph description
- Key types used
- What it does not show
