#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: SeparableWfNet::declare_separable — a WfNetConst in any
// soundness state can be wrapped as separable. The net field is accessible.
// Law: Kourani, Park & van der Aalst (2026) Definition 4.1.
use wasm4pm_compat::petri::{SeparableWfNet, WfNetConst};
use wasm4pm_compat::law::SoundnessState;

fn main() {
    // Unknown soundness → separable
    let unknown = WfNetConst::<{ SoundnessState::Unknown }>::new();
    let sep = SeparableWfNet::declare_separable(unknown);
    let _ = &sep.net;

    // Claimed soundness → separable
    let claimed = WfNetConst::<{ SoundnessState::Unknown }>::new().claim_sound();
    let sep_claimed = SeparableWfNet::declare_separable(claimed);
    let _ = &sep_claimed.net;
}
