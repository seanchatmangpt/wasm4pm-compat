// COMPILE-FAIL: RefuseLoss policy cannot be used with lossy export functions.
//
// Law: projection-policy-consistency
//
// When LossPolicy = RefuseLoss, the projection must refuse any lossy path. An
// attempt to call accept_lossy_ocel_to_xes or accept_lossy_xes_to_oced with a
// LossReport carrying RefuseLoss is a contradiction: either the projection refused
// (in which case there is no report to carry), or it emitted a report (in which
// case it did not refuse, so the policy should not be RefuseLoss).
//
// This fixture creates a LossReport with RefuseLoss policy and attempts to pass
// it to accept_lossy_ocel_to_xes. The type system and contract must reject this
// logical contradiction.
//
// Note: This is a runtime/semantic error that may not be caught by the type system
// alone. A compile_fail fixture serves to document the expected rejection at a
// higher level (via a lint, diagnostic, or runtime check).
//
// Expected error: type or value mismatch — RefuseLoss is incompatible with lossy
// export boundary.

use wasm4pm_compat::formats::{accept_lossy_ocel_to_xes, FormatKind, LossyFormatExport};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn main() {
    // Create a LossReport with RefuseLoss policy — a contradiction for lossy export.
    let report = LossReport::<(), (), Vec<String>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::RefuseLoss,
        vec!["item".to_string()],
    );
    // LossyFormatExport requires a non-refusing policy (AllowNamedProjection or
    // AllowLossWithReport). RefuseLoss + lossy export is a contradiction.
    let export = LossyFormatExport::new(FormatKind::XesXml, b"<log/>".to_vec(), report);

    // This call may be rejected at a higher semantic level (not purely type-checked).
    accept_lossy_ocel_to_xes(export);
}
