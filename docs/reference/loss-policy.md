# Reference: Loss Policy & Report Specifications

This document defines the technical specifications for the `LossPolicy` configurations and the resulting `LossReport` shapes in `wasm4pm-compat` version `26.6.9`.

---

## 1. The `LossPolicy` Enum

The `LossPolicy` enum (compiled under the `formats` feature flag in `src/loss.rs`) governs how data-loss is handled during structural projections.

```rust
pub enum LossPolicy {
    /// Prevent the projection from executing if any structural data is dropped.
    RefuseLoss,
    /// Allow the projection ONLY if the target query shape has been statically registered.
    AllowNamedProjection,
    /// Allow the projection to proceed but write a detailed record of discarded elements.
    AllowLossWithReport,
}
```

---

## 2. Policy Behaviors & Result Mapping

| Selected Policy | Condition (No Loss) | Condition (With Loss) | Return Result Type |
| :--- | :--- | :--- | :--- |
| **`RefuseLoss`** | Success | Rejection | `Err(ProjectionError::FlatteningLoss)` |
| **`AllowNamedProjection`** | Success | Success (If named) | `Ok((Evidence<To, Projected, W>, LossReport))` |
| **`AllowLossWithReport`** | Success | Success | `Ok((Evidence<To, Projected, W>, LossReport))` |

---

## 3. The `LossReport` Struct Layout

When a projection allows data-loss, the method returns a `LossReport` describing the dropped items.

```rust
pub struct LossReport<From, To, Items> {
    /// The unique identifier of the projection mapping.
    pub projection: ProjectionName,
    /// The policy chosen to authorize this projection.
    pub policy: LossPolicy,
    /// The collection of discarded elements (e.g., event-to-object links or object types).
    pub lost: Items,
    /// Compile-time marker of source type.
    from: PhantomData<From>,
    /// Compile-time marker of target type.
    to: PhantomData<To>,
}
```

### Key Fields

- **`projection`**: `ProjectionName` (newtype wrapper for a static string, e.g. `ProjectionName("ocel-to-xes-case-customer")`).
- **`policy`**: The matching `LossPolicy` variant authorizing the transition.
- **`lost`**: A generic collection (typically `Vec<String>` or a specialized list type) cataloging the dropped attributes, relations, or nodes.
