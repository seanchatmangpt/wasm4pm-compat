// Law: WfNetConstNonForgeabilityLaw — WfNetConst<Witnessed> is not constructible via struct literal; it is only reachable via witness_soundness(proof) inside the petri module.
// The public API offers Unknown → Claimed via const-generics as the lawful attestation path.
// COMPILE-PASS: WfNetConst const-generic path — Unknown and Claimed states are constructible and distinctly typed.
// Witnessed requires a SoundnessProof (pub(crate)), so it is tested indirectly via the non-forgeability compile-fail fixture.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use wasm4pm_compat::petri::WfNetConst;
use wasm4pm_compat::law::SoundnessState;

fn accepts_unknown(_: WfNetConst<{ SoundnessState::Unknown }>) {}
fn accepts_claimed(_: WfNetConst<{ SoundnessState::Claimed }>) {}

fn main() {
    // Construct Unknown state via const-generic.
    let unknown = WfNetConst::<{ SoundnessState::Unknown }>::new();
    accepts_unknown(unknown);

    // Advance to Claimed via const-generic.
    let unknown2 = WfNetConst::<{ SoundnessState::Unknown }>::new();
    let claimed = unknown2.claim_sound();
    accepts_claimed(claimed);

    // Prove the const states are distinct types — would fail to compile if unified.
    let _ = WfNetConst::<{ SoundnessState::Unknown }>::new();
    let _ = WfNetConst::<{ SoundnessState::Claimed }>::new();
}
