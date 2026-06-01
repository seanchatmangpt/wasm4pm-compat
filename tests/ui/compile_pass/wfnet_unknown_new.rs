#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: WfNetConst::Unknown — the default Unknown soundness state is
// constructible via ::new() and Default.
use wasm4pm_compat::petri::WfNetConst;
use wasm4pm_compat::law::SoundnessState;

fn only_unknown(_: WfNetConst<{ SoundnessState::Unknown }>) {}

fn main() {
    let wf = WfNetConst::<{ SoundnessState::Unknown }>::new();
    only_unknown(wf);

    let wf2: WfNetConst<{ SoundnessState::Unknown }> = Default::default();
    let _ = wf2;
}
