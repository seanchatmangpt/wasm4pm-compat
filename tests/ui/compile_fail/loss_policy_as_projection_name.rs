// COMPILE-FAIL: Loss type law — LossPolicy cannot be passed where ProjectionName is required.
// Law: LossPolicy (an enum: RefuseLoss | AllowNamedProjection | AllowLossWithReport)
// and ProjectionName (a &'static str newtype) are distinct types. A policy tag
// must not be confused with the projection's name label.
use wasm4pm_compat::loss::{LossPolicy, ProjectionName};

fn requires_projection_name(_n: ProjectionName) {}

fn main() {
    let policy = LossPolicy::RefuseLoss;
    // This must fail: LossPolicy is not ProjectionName.
    requires_projection_name(policy);
}
