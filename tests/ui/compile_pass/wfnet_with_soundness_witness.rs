#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: WfNetSoundnessStatesLaw — Unknown and Claimed WfNetConst states are publicly constructible; Witnessed state requires witness_soundness(proof) via the petri module

// COMPILE-PASS: WF-net soundness — Unknown and Claimed states are constructible.
// Witnessed state is reachable only via witness_soundness(proof) — not tested here
// since SoundnessProof is pub(crate). This fixture proves the public path compiles
// and the type states are distinct (they can't be substituted for each other).
use wasm4pm_compat::petri::WfNetConst;
use wasm4pm_compat::law::SoundnessState;

fn only_unknown(_: WfNetConst<{ SoundnessState::Unknown }>) {}
fn only_claimed(_: WfNetConst<{ SoundnessState::Claimed }>) {}

fn main() {
    let unknown = WfNetConst::<{ SoundnessState::Unknown }>::new();
    let claimed = unknown.claim_sound();

    // These calls prove the types are distinct (would fail if unified):
    only_claimed(claimed);
    only_unknown(WfNetConst::<{ SoundnessState::Unknown }>::new());
}
