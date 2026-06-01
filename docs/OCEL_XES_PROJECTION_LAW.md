# OCEL_XES_PROJECTION_LAW — Object-Centric to Case-Centric Projection

> The canonical lossy transformation: flattening an OCEL 2.0 log to a classic
> XES log requires a named projection, a loss policy, and a loss report.
> There is no silent path.

---

## Why this projection loses evidence

An **OCEL 2.0** event log (IEEE 1849.1-like, Ghahfarokhi et al. 2022) is
**object-centric**: a single event can link to multiple objects of multiple
object types. An event in an order-management process may be linked to:

- one `order` (the case driving the flattening)
- multiple `item`s (the line items)
- one `invoice` (the billing artifact)
- one `shipment` (the logistics object)

A classic **XES** log (IEEE 1849-2023) is **case-centric**: each trace has
exactly one case identifier, and every event belongs to exactly one trace.

To produce a XES log from an OCEL log you must:

1. Pick one object type to act as the case notion (e.g., `order`).
2. For each `order` object, collect the events linked to it — one trace per order.
3. **Discard** every event-to-object link to the non-case object types.

Steps 1 and 2 are structure-preserving. Step 3 is a genuine loss: the
`item`, `invoice`, and `shipment` links vanish from the XES output. A
reader of the XES file cannot know an object-centric structure ever existed.

---

## The type-law path: OCEL → XES

The only sanctioned path is through the `Project` trait with explicit policy.

```
OcelLog (Admitted, Ocel20)
  │
  ▼ Project::project(policy)
  │
  ├─ policy = RefuseLoss
  │      → Err(FlatteningLoss)   ← named refusal, nothing dropped
  │
  └─ policy = AllowLossWithReport
         → Ok(LossReport<OcelShape, XesShape, Vec<DroppedLink>>)
                ↳ XesLog shape + loss record on the same result
```

The `From` = `OcelShape` and `To` = `XesShape` type parameters tag the
report so an OCEL→XES loss report cannot be confused with, say, an
XES→OCED loss report.

---

## Compile-fail receipts

Two fixtures prove the rejection of unlawful paths:

| Fixture | Law sealed |
|---------|-----------|
| `ocel_to_xes_no_loss_report.rs` | OCEL→XES projection without a `LossReport` is rejected — `FormatExport` cannot satisfy `LossyFormatExport` |
| `ocel_log_as_xes_log.rs` | `OcelLog` cannot be passed where `XesLog` is required — the two log types are distinct |

---

## Compile-pass receipts

| Fixture | Law proven |
|---------|-----------|
| `interop_ocel_to_xes_projection.rs` | Lawful OCEL→XES projection with `AllowLossWithReport` and `ProjectionName` compiles |
| `interop_ocel_to_xes_projection_const.rs` | Const-projection surface compiles with the bridge trait |
| `ocel_to_xes_with_named_projection.rs` | Named `ProjectionName("ocel-flatten-to-xes:by-case")` projection compiles |
| `formats_accept_lossy_ocel_to_xes.rs` | `accept_lossy` boundary accepts OCEL→XES with full loss report |

---

## The `ProjectionName` for OCEL→XES

The canonical projection name for this transformation is:

```
ProjectionName("ocel-flatten-to-xes:by-case")
```

The suffix `:by-case` names the case-selection strategy. If a different
case-selection strategy is used (e.g., `:by-item`), it must use a distinct
`ProjectionName` — the name encodes the strategy, not just the formats.

---

## Loss report shape

```rust
LossReport<OcelShape, XesShape, Vec<DroppedEventObjectLink>>
```

- `projection` — the `ProjectionName` naming this transformation
- `policy` — `AllowLossWithReport` (or `AllowNamedProjection` for lightweight mode)
- `lost` — the enumerated `DroppedEventObjectLink` values discarded in this run

Under `AllowNamedProjection`, items need not be enumerated, but the
`ProjectionName` is still required.

---

## The reverse: XES → OCED

XES-to-OCED projection (enriching a flat log toward an object-centric shape) is
also governed by loss law and has its own fixtures:

- `xes_to_oced_without_loss_policy.rs` — rejects projection without policy
- `xes_to_oced_without_projection_name.rs` — rejects bare `&str` instead of `ProjectionName`
- `xes_to_oced_loss_report_rejected.rs` — rejects wrong loss report type

The projection name for XES→OCED uses the pattern:
```
ProjectionName("xes-enrich-to-oced:<strategy>")
```

---

## Why there is no direct format-to-format path

From `src/loss.rs` and the architecture covenant:

```
external  ──admit──▶  typed admitted compat  ──project (named + policy + report)──▶  external / wasm4pm
```

A raw OCEL byte stream may not be rewritten directly into a XES byte stream.
The admission step is not optional — it is the structural gate that gives the
projection something typed to work with, and makes the loss report possible.
