#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: SeparableWfNet marker — a separability claim can be attached
// to a WfNetConst, and SeparableWfNet carries the soundness state forward.
//
// Law: Kourani, Park & van der Aalst (2026) Definition 4.1 — only a separable
// WF-net can be converted to POWL 2.0 without language loss. This marker
// expresses that precondition at the type level.
use wasm4pm_compat::law::SoundnessState;
use wasm4pm_compat::petri::{SeparableWfNet, WfNetConst};

fn only_separable<const S: SoundnessState>(_: &SeparableWfNet<S>) {}

fn main() {
    // A WfNet in Unknown soundness state can be wrapped as separable.
    let wfnet = WfNetConst::<{ SoundnessState::Unknown }>::new();
    let sep = SeparableWfNet::declare_separable(wfnet);

    only_separable(&sep);

    // The underlying net is still accessible.
    let _ = &sep.net;

    // Claimed soundness also works.
    let claimed_net = WfNetConst::<{ SoundnessState::Unknown }>::new().claim_sound();
    let sep_claimed = SeparableWfNet::declare_separable(claimed_net);
    only_separable(&sep_claimed);
}
