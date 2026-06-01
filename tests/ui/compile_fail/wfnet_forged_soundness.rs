#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: WfNetSoundnessNonForgeabilityLaw — WfNetConst<Witnessed> cannot be constructed via struct literal; the private _seal field prevents forging a soundness witness outside the petri module

// COMPILE-FAIL: WF-net soundness non-forgeability.
// Law: SoundnessState::Witnessed is only reachable via witness_soundness(proof),
// where SoundnessProof is only constructible inside the petri module.
// Expected error: field `_seal` of struct `WfNetConst` is private.
use wasm4pm_compat::petri::WfNetConst;
use wasm4pm_compat::law::SoundnessState;

fn main() {
    // Direct struct-literal construction fails: _seal is a private field.
    let _forged: WfNetConst<{ SoundnessState::Witnessed }> = WfNetConst {
        _seal: todo!(),
    };
}
