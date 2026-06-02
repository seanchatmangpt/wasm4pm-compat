---
gap_id: GAP_LOSS_TREE
gap_name: Loss Accounting Type-Law Rules
status: CLOSED
closed_date: 2026-06-02
---

# Gap Closure Receipt: GAP_LOSS_TREE

## Summary

The loss accounting type-law surface was manufactured in `src/loss.rs`, enforcing
that every lossy projection between process-evidence shapes must be named, policy-gated,
and receipt-bearing. The three types `LossPolicy`, `LossReport`, and `ProjectionName`
together make loss accountable at the type level: no raw format-to-format laundering
is permitted, and silent structure loss is a compile-time defect, not a runtime
surprise.

## Evidence

Files created or modified to close this gap:

- `src/loss.rs` — `LossPolicy` enum (`RefuseLoss`, `AllowNamedProjection`,
  `AllowLossWithReport`), `LossReport<From, To, Items>` generic receipt type,
  `ProjectionName` static-string newtype with `Display`, `NamedLoss` pairing
  projection identity with loss-category label, `Project` trait as the only
  sanctioned lossy transformation path, `IsEmpty` helper bound for vacuously
  lossless detection via `LossReport::is_lossless`
- `src/diagnostic.rs` — `CompatDiagnostic::LossyProjectionWithoutPolicy` and
  `CompatDiagnostic::HiddenFlattening` diagnostics that name the loss-accounting
  violations
- `examples/ocel_to_xes_projection.rs` — runnable demonstration of the full OCEL
  to XES loss covenant: `ProjectionName`, `LossPolicy::AllowLossWithReport` vs
  `LossPolicy::RefuseLoss`, named `XesExportRefusal`

## Audit Gate

The audit gate confirms this gap is closed by verifying:

1. `cargo build --all-features` compiles without error — all three types are
   present and correctly bounded.
2. `cargo test --all-features --tests` passes — unit tests exercise `LossPolicy`
   guard helpers (`is_refusing`, `is_named`, `is_reporting`) and `LossReport`
   construction paths.
3. `cargo test --test ui_tests -- --ignored` passes the compile-fail fixtures that
   assert lossy projections without a `LossPolicy` are rejected by the type system,
   and the compile-pass fixtures that assert lawful projections (with named policy
   and report) are accepted.
4. `cargo run --example ocel_to_xes_projection` runs without panic, demonstrating
   the full loss covenant end-to-end.

verified: 2026-06-02
