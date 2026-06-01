// COMPILE-FAIL: MultiPerspective evidence context — wrong perspective combination.
//
// Law: PerspectiveCombinationLaw — MultiPerspectiveEvidence with only ControlFlowPerspective
// is a different type from MultiPerspectiveEvidence with a combined CF+Data perspective.
// A function requiring multi-perspective (both CF + Data) must reject single-perspective evidence.
use wasm4pm_compat::multiperspective::{
    ControlFlowPerspective, DataPerspective, MultiPerspectiveEvidence, PerspectiveCombination,
};

fn requires_cf_and_data(
    _ev: MultiPerspectiveEvidence<u32, PerspectiveCombination<ControlFlowPerspective, DataPerspective>>,
) {
}

fn main() {
    let cf_only: MultiPerspectiveEvidence<u32, ControlFlowPerspective> =
        MultiPerspectiveEvidence::new(1u32);
    // This must fail: MultiPerspectiveEvidence<_, ControlFlowPerspective> is not
    // MultiPerspectiveEvidence<_, PerspectiveCombination<ControlFlowPerspective, DataPerspective>>.
    // The Perspectives type parameter enforces full perspective coverage.
    requires_cf_and_data(cf_only);
}
