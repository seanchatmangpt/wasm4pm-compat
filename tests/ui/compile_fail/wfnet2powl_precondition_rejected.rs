#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-FAIL: wfnet2powl_precondition — SeparableWfNet precondition is
// enforced at the type level; a plain WfNetConst (without separability marker)
// is rejected by any function that requires SeparableWfNet.
//
// Law: Kourani, Park & van der Aalst (2026) Theorem 4.3 — only a *separable*
// WF-net can be converted to a POWL 2.0 model while preserving the process
// language. The type `SeparableWfNet<S>` is the precondition token; a bare
// `WfNetConst<S>` does not satisfy it. Passing a plain WfNetConst to a
// conversion gate that requires SeparableWfNet must be a compile error.
//
// Expected error: mismatched types — expected SeparableWfNet<...>, found WfNetConst<...>
use wasm4pm_compat::law::SoundnessState;
use wasm4pm_compat::petri::{SeparableWfNet, WfNetConst};
use wasm4pm_compat::powl::WfNet2PowlWitness;

/// Structural gate: only a SeparableWfNet satisfies the Theorem 4.3
/// precondition for WF-net → POWL 2.0 conversion.
///
/// This function is structure-only: it does not perform the conversion; it
/// enforces the type-level precondition that the input is separable.
fn wfnet_to_powl_gate<const S: SoundnessState>(
    _separable: SeparableWfNet<S>,
) -> WfNet2PowlWitness {
    WfNet2PowlWitness::new_internal("gate-context")
}

fn main() {
    // A plain WfNetConst has no separability marker.
    let plain = WfNetConst::<{ SoundnessState::Unknown }>::new();
    // ERROR: expected SeparableWfNet<{Unknown}>, found WfNetConst<{Unknown}>.
    // A plain WfNetConst does not satisfy the Theorem 4.3 separability
    // precondition; this must not compile.
    let _w = wfnet_to_powl_gate(plain);
}
