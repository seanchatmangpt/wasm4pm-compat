# Multi-Perspective Law

**Module:** `src/multiperspective.rs`  
**Canon family:** `XES_EVENT_LOG`  
**Paper anchor:** Mannhardt, de Leoni, Reijers & van der Aalst (2016). "Balanced Multi-Perspective Checking of Process Conformance." *Computing*, 98(4), 407–437.

---

## Doctrine

Process mining analysis does not end at control-flow. The Mannhardt et al.
(2016) balanced multi-perspective framework identifies four distinct
perspectives, each contributing a weighted cost to conformance alignment:

1. **Control-flow** — what activities happen and in what order.
2. **Data** — what attribute values are recorded on events and objects.
3. **Resource** — who performs each activity (`org:resource` or equivalent).
4. **Time** — when activities happen; durations, sojourn times, timestamps.

This module carries the structural markers for those four perspectives as
zero-sized types. They are thread-able through generic bounds and `PhantomData`
positions to declare — at the type level — which perspectives a piece of
evidence covers.

---

## Structural Surface

| Type | Purpose |
|---|---|
| `ProcessPerspective` | Runtime enum: `ControlFlow`, `Data`, `Resource`, `Time` |
| `ControlFlowPerspective` | Zero-sized marker for the control-flow perspective |
| `DataPerspective` | Zero-sized marker for the data perspective |
| `ResourcePerspective` | Zero-sized marker for the resource perspective |
| `TimePerspective` | Zero-sized marker for the time perspective |
| `MultiPerspectiveEvidence<T, Perspectives>` | Evidence enriched with a perspective-combination phantom |
| `PerspectiveCombination<A, B>` | Compose two perspectives; nest for three or four |

---

## Perspective Witness Lattice

Four witnesses are registered in `src/witness.rs`:

| Witness | Key | Year |
|---|---|---|
| `ControlFlowPerspectiveWitness` | `"cf-perspective"` | 2016 |
| `DataPerspectiveWitness` | `"data-perspective"` | 2016 |
| `ResourcePerspectiveWitness` | `"resource-perspective"` | 2016 |
| `TimePerspectiveWitness` | `"time-perspective"` | 2016 |

---

## Compile-Time Law Receipts

| Fixture | Purpose |
|---|---|
| `tests/ui/compile_pass/multi_perspective_combination.rs` | CF+Data combination compiles |

---

## Zero-Cost Guarantee

All perspective marker types (`ControlFlowPerspective`, `DataPerspective`,
`ResourcePerspective`, `TimePerspective`) are zero-sized. `PerspectiveCombination<A, B>`
is also zero-sized. `MultiPerspectiveEvidence<T, Perspectives>` has the same
layout as `T` — perspective declarations cost nothing at runtime.

---

## Composing Perspectives

```rust
use wasm4pm_compat::multiperspective::{
    MultiPerspectiveEvidence, PerspectiveCombination,
    ControlFlowPerspective, DataPerspective, ResourcePerspective,
};

// Two perspectives
type CfData = PerspectiveCombination<ControlFlowPerspective, DataPerspective>;

// Three perspectives (nested)
type CfDataResource = PerspectiveCombination<ControlFlowPerspective,
                        PerspectiveCombination<DataPerspective, ResourcePerspective>>;

let ev: MultiPerspectiveEvidence<&str, CfData> =
    MultiPerspectiveEvidence::new("place_order");
```

---

## What This Module is NOT

- Not a conformance checker. Per-perspective alignment cost computation
  and perspective weight tuning graduate to `wasm4pm`.
- Not a data validator. It declares which perspectives are present;
  it does not verify the attribute values.

---

## Graduation

When you need to *compute* per-perspective alignment costs, *tune* perspective
weights, or *run* multi-perspective conformance checking, graduate to `wasm4pm`.
