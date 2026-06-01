// COMPILE-FAIL: WF-net / Petri structural law — WfNetConst cannot be passed where Place is required.
// Law: WfNetConst (the whole workflow net with soundness state) and Place (a single place node)
// are structurally distinct. A workflow net must not be confused with a place.
use wasm4pm_compat::law::SoundnessState;
use wasm4pm_compat::petri::{Place, WfNetConst};

fn requires_place(_p: Place) {}

fn main() {
    let wfnet = WfNetConst::<{ SoundnessState::Unknown }>::new();
    // This must fail: WfNetConst is not Place.
    requires_place(wfnet);
}
