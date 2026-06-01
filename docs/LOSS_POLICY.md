# The Loss Policy

> No raw format-to-format laundering. Lossy projection requires a **named
> projection** + a **`LossPolicy`** + a **`LossReport`** + a refusal path.

Some translations between process-evidence shapes cannot be lossless. When a
transformation must discard real evidence, `wasm4pm-compat` makes the loss
**accountable** rather than silent. This document walks the canonical case —
flattening an object-centric log (OCEL) to a classic single-case log (XES) — and
shows how the `loss` module (`src/loss.rs`) governs it.

---

## The canonical loss: OCEL → XES flattening

An **OCEL** event can be linked to *many* objects of *many* object types
(an `order`, several `item`s, an `invoice`, …). A classic **XES** log has exactly
one case notion per trace. To flatten OCEL into XES you must:

1. pick **one** object type to act as the case (say, `order`), and
2. **discard** every event-to-object link to the other types (`item`, `invoice`).

Those discarded links are genuine evidence. They cannot quietly disappear — a
downstream reader of the XES log would never know an object-centric structure
ever existed. The loss law exists precisely to keep that disappearance on the
record.

---

## The three policies

`LossPolicy` forces the caller to decide **before** any loss happens:

| Policy                    | Meaning                                                                 |
|---------------------------|-------------------------------------------------------------------------|
| `RefuseLoss`              | Loss is not tolerated. A projection that would drop evidence **refuses** with a named reason (e.g. `FlatteningLoss`). |
| `AllowNamedProjection`    | Loss is permitted, but only via an explicitly **named** projection (a `ProjectionName`). Items need not be enumerated. |
| `AllowLossWithReport`     | Loss is permitted **and** must be **reported**: a `LossReport` enumerating the discarded items is produced alongside the result. |

There is no fourth option — there is no "lose silently".

---

## The named projection

A `ProjectionName` is a `&'static str` newtype that makes a transformation
*recognizable*:

```rust
ProjectionName("ocel-flatten-to-xes:by-case")
```

Two runs of the same named projection mean the same thing. The name encodes both
*what* is happening (flatten OCEL to XES) and *how the case is chosen* (`by-case`).
Because it is `&'static str`, the name lives in the binary and cannot be confused
with arbitrary user input.

---

## The loss report

A `LossReport<From, To, Items>` is the receipt of a lossy projection:

```rust
pub struct LossReport<From, To, Items> {
    pub projection: ProjectionName, // which named projection ran
    pub policy: LossPolicy,         // under what policy
    pub lost: Items,                // exactly what was discarded
    // From / To are zero-sized PhantomData shape tags
}
```

The `From` and `To` parameters tag the shapes the projection bridged, so a report
for an OCEL→XES flatten cannot be mistaken for one between different shapes. The
`lost` field is the concrete, inspectable record of discarded evidence.

---

## The `Project` law in action

`Project` is the **only** sanctioned lossy transformation. It honors the supplied
`LossPolicy`: under `RefuseLoss` it returns a named reason instead of losing
anything; otherwise it returns a `LossReport`.

```rust
use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};

enum OcelShape {}
enum XesShape {}

#[derive(Debug, PartialEq, Eq)]
enum FlattenRefusal { FlatteningLoss }

struct OcelFlatten { object_types: Vec<&'static str>, case_type: &'static str }

impl Project for OcelFlatten {
    type From = OcelShape;
    type To = XesShape;
    type Lost = Vec<&'static str>;
    type Reason = FlattenRefusal;

    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason> {
        let dropped: Vec<&'static str> = self
            .object_types.iter().copied()
            .filter(|t| *t != self.case_type)
            .collect();
        if !dropped.is_empty() && policy == LossPolicy::RefuseLoss {
            return Err(FlattenRefusal::FlatteningLoss);     // refusal path
        }
        Ok(LossReport::new(
            ProjectionName("ocel-flatten-to-xes:by-case"),
            policy,
            dropped,                                         // on the record
        ))
    }
}
```

The two outcomes that matter:

```rust
// RefuseLoss: dropping "item" links is refused with a NAMED law.
let refused = OcelFlatten { object_types: vec!["order", "item"], case_type: "order" }
    .project(LossPolicy::RefuseLoss);
assert_eq!(refused.err(), Some(FlattenRefusal::FlatteningLoss));

// AllowLossWithReport: the loss is allowed and itemized.
let report = OcelFlatten { object_types: vec!["order", "item", "invoice"], case_type: "order" }
    .project(LossPolicy::AllowLossWithReport)
    .unwrap();
assert_eq!(report.lost, vec!["item", "invoice"]);
```

`tests/loss_projection.rs` exercises exactly these paths.

---

## Why this is non-negotiable

The flow is strictly:

```text
external  ──admit──▶  typed admitted compat  ──project (named + policy + report)──▶  external / wasm4pm
```

A raw OCEL byte stream may **not** be rewritten directly into an XES byte stream.
It must be admitted into a typed compat value first, and any flattening must go
through `Project` under a `LossPolicy`. Two diagnostics enforce this:

- `CompatDiagnostic::LossyProjectionWithoutPolicy` — a lossy transformation that
  is not governed by a `LossPolicy`.
- `CompatDiagnostic::HiddenFlattening` — structure discarded without a
  `LossReport` to itemize it.

The principle, stated plainly: **a compatibility layer that loses evidence in
secret is worse than no layer, because it provides false assurance.** The loss
law turns every unavoidable loss into a named, policied, reported, refusable
event — so the loss travels on the record, never off it.

---

## `ProjectionName` and `LossReport` are mandatory, not optional

Two types enforce accountability beyond the policy decision:

### `ProjectionName`

```rust
pub struct ProjectionName(pub &'static str);
```

A newtype over `&'static str`. Not a `String`, not a `&str` — the static lifetime
and newtype wrapper mean:

1. The name lives in the binary, not in user-supplied runtime data.
2. A bare `&str` does not satisfy the type — `ProjectionName("name")` is required.
3. Two runs of the same projection have the same `ProjectionName`, making the
   operation recognizable and auditable.

The compile-fail fixture `projection_name_bare_str.rs` proves that a raw `&str`
is rejected where `ProjectionName` is required.

### `LossReport<From, To, Items>`

```rust
pub struct LossReport<From, To, Items> {
    pub projection: ProjectionName,
    pub policy: LossPolicy,
    pub lost: Items,
    // From / To are zero-sized PhantomData shape tags
}
```

Key properties:

- `From` and `To` type parameters tag the shapes bridged. A `LossReport<OcelShape, XesShape, _>`
  cannot be confused with a `LossReport<XesShape, OcedShape, _>`.
- `lost` holds the concrete discarded items — an inspectable record, not a count.
- `is_lossless()` is only available when `Items: IsEmpty`, enforced at compile time.
  (See `loss_report_is_lossless_bound.rs` for the compile-fail receipt.)
- `AllowLossWithReport` path that returns `()` instead of a `LossReport` is
  rejected by the compiler (see `loss_without_report_on_allow_path.rs`).

### The three-type lock

A lawful lossy projection requires all three in sequence:

```
LossPolicy → ProjectionName → LossReport
```

Omitting any one of the three is a compile error, not a runtime warning.
