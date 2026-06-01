#![feature(adt_const_params)]
#![allow(incomplete_features)]
// Law: WfNetQueryTraitLaw — WfNetQuery provides a uniform query surface returning the SoundnessState as a runtime value from any WfNetConst variant

// COMPILE-PASS: WfNetQuery — the shared query surface returns the soundness
// state as a runtime value from any WfNetConst variant.
use wasm4pm_compat::petri::{WfNetConst, WfNetQuery};
use wasm4pm_compat::law::SoundnessState;

fn check_state<W: WfNetQuery>(wf: &W) -> SoundnessState {
    wf.soundness_state()
}

fn main() {
    let unknown = WfNetConst::<{ SoundnessState::Unknown }>::new();
    assert_eq!(check_state(&unknown), SoundnessState::Unknown);

    let claimed = WfNetConst::<{ SoundnessState::Unknown }>::new().claim_sound();
    assert_eq!(check_state(&claimed), SoundnessState::Claimed);
}
