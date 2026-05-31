#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-FAIL: WfNet2PowlSourceLaw — a plain PetriNet (non-WF-net) cannot
// be passed through the WF-net to POWL conversion gate.
//
// Law: Kourani, Park & van der Aalst (2026) Theorem 4.3 — the WF-net to POWL
// 2.0 conversion requires a *separable WF-net*, not a bare PetriNet. A plain
// PetriNet has neither the WF-net soundness marker (WfNetConst) nor the
// separability marker (SeparableWfNet). Passing it to the conversion gate
// must be rejected by the type system because PetriNet ≠ SeparableWfNet.
//
// Expected error: mismatched types — expected SeparableWfNet<_>, found PetriNet.
use wasm4pm_compat::law::SoundnessState;
use wasm4pm_compat::petri::{Marking, PetriNet, SeparableWfNet};
use wasm4pm_compat::powl::WfNet2PowlWitness;

/// Structural gate: only a SeparableWfNet satisfies the Theorem 4.3
/// precondition for WF-net → POWL 2.0 conversion.
fn wfnet_to_powl_gate<const S: SoundnessState>(
    _separable: SeparableWfNet<S>,
) -> WfNet2PowlWitness {
    WfNet2PowlWitness::new_internal("gate-context")
}

fn main() {
    // A bare PetriNet — has no WF-net shape, no soundness marker, no
    // separability. It is the wrong source type entirely.
    let net = PetriNet::new([], [], [], Marking::empty());
    // ERROR: PetriNet is not SeparableWfNet — a non-WF-net type cannot enter
    // the WfNet2POWL conversion path.
    let _w = wfnet_to_powl_gate(net);
}
