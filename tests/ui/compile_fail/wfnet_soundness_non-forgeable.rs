// Law: WfNetSoundnessNonForgeabilityLaw — WfNetConst<Witnessed> cannot be constructed via struct literal.
// The private _seal field (of private type wfnet_seal::WfNetSeal) blocks any attempt to forge soundness.
// The only lawful path is WfNetConst::witness_soundness(proof), where SoundnessProof is only constructible inside the petri module.
// COMPILE-FAIL: Direct struct literal construction of WfNetConst<Witnessed>.
// Expected errors:
// 1. field `_seal` of struct `WfNetConst` is private (E0451)
// 2. type `petri::wfnet_seal::WfNetSeal` is private (type error)

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use wasm4pm_compat::petri::WfNetConst;
use wasm4pm_compat::law::SoundnessState;

fn main() {
    // Attempting to forge WfNetConst<Witnessed> by direct struct construction.
    // This must fail because:
    // 1. _seal is a private field
    // 2. _seal's type wfnet_seal::WfNetSeal is private
    let _forged: WfNetConst<{ SoundnessState::Witnessed }> = WfNetConst {
        _seal: todo!(),
    };
}
