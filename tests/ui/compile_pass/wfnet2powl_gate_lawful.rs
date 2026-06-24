#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-PASS: wfnet2powl_gate_lawful — SeparableWfNet can be converted to POWL
// using the into_powl conversion gate.
//
// Law: Kourani, Park & van der Aalst (2026) Theorem 4.3 — separable WF-net
// can be converted to POWL 2.0 preserving language. The conversion gate returning
// WfNet2PowlWitness represents a proof that the model was generated from a separable
// workflow net.

use wasm4pm_compat::petri::{SeparableWfNet, WfNetConst};
use wasm4pm_compat::law::SoundnessState;

fn main() {
    let net = WfNetConst::<{ SoundnessState::Unknown }>::new().claim_sound();
    let sep = SeparableWfNet::declare_separable(net);
    
    // Call the official conversion gate
    let (powl, witness) = sep.into_powl("lawful-workflow-123");
    
    assert_eq!(witness.context, "lawful-workflow-123");
    assert_eq!(powl.node_count(), 0);
}
