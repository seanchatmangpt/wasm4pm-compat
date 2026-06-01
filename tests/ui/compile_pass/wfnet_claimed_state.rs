#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: WfNetConst::Claimed — claim_sound() advances the Unknown state
// to Claimed at the type level without any runtime check.
use wasm4pm_compat::petri::WfNetConst;
use wasm4pm_compat::law::SoundnessState;

fn only_claimed(_: WfNetConst<{ SoundnessState::Claimed }>) {}

fn main() {
    let unknown = WfNetConst::<{ SoundnessState::Unknown }>::new();
    let claimed = unknown.claim_sound();
    only_claimed(claimed);
}
