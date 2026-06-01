// COMPILE-FAIL: FormatKind cannot be passed where LossPolicy is required.
//
// Law: projection-policy-type-distinctness
//
// FormatKind (an enum: OcelJson | OcelXml | XesXml | BpmnXml | ...) and
// LossPolicy (an enum: RefuseLoss | AllowNamedProjection | AllowLossWithReport)
// are structurally similar but semantically distinct. Confusing them silently
// converts a format identity into a loss-handling decision — breaking the boundary
// covenant.
//
// This fixture passes a FormatKind where a LossPolicy is required. The type system
// must reject this mismatch.
//
// Expected error: mismatched types — FormatKind is not LossPolicy.

use wasm4pm_compat::formats::FormatKind;
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn requires_loss_policy(_policy: LossPolicy) {}

fn main() {
    let kind = FormatKind::OcelJson;
    // This must fail: FormatKind is not LossPolicy.
    requires_loss_policy(kind);
}
