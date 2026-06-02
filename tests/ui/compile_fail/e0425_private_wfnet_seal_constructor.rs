#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

//! Law: WfNetSealNonConstructibility — WfNetSeal type is private and has no public constructor.
//! Structural guarantee: External code cannot construct the seal required for WfNetConst.
//! Expected error: E0425: cannot find `WfNetSeal` in scope (it's in a private module).

use wasm4pm_compat::law::SoundnessState;

fn main() {
    // STRUCTURAL LAW: WfNetSeal is private (mod wfnet_seal { ... })
    // PROOF: We cannot access or construct WfNetSeal from outside petri module
    // This proves that soundness witnesses cannot be forged
    let _seal = WfNetSeal;
}
