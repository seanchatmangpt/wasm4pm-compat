#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-FAIL: Non-separable WF-net cannot be projected to POWL without named refusal
//
// Law: Kourani, Park & van der Aalst (2026) Theorem 4.3 — only a *separable*
// WF-net can be converted to a POWL 2.0 model while preserving the process
// language. The `SeparableWfNet<S>` wrapper is the sole type-level receipt of
// separability; it is non-forgeable because its `_seal` field is private.
//
// A non-separable WF-net cannot be smuggled into the POWL conversion path by
// constructing a forged `SeparableWfNet` via struct-literal syntax.
// Attempting to do so must be a compile error.
//
// This fixture seals the law that the separability precondition is
// non-forgeable: any code path that tries to bypass `declare_separable` and
// construct `SeparableWfNet` directly is rejected at the type level —
// equivalent to "non-separable WF-net refused without named refusal".
//
// Why this differs from wfnet2powl_precondition_rejected.rs:
// That fixture passes the *wrong type* to a gate. This fixture attempts to
// *forge* the required type — a more fundamental violation of the law.
//
// Expected error: field `_seal` of struct `SeparableWfNet` is private
use wasm4pm_compat::law::SoundnessState;
use wasm4pm_compat::petri::{SeparableWfNet, WfNetConst};

fn main() {
    let net = WfNetConst::<{ SoundnessState::Unknown }>::new();
    // Attempt to forge SeparableWfNet by constructing a struct literal.
    // This must fail: `_seal` is a private field of `SeparableWfNet`.
    // Without going through `declare_separable`, a non-separable WF-net
    // cannot be presented as separable — the named refusal is that
    // separability was never established.
    let _forged: SeparableWfNet<{ SoundnessState::Unknown }> = SeparableWfNet {
        net,
        _seal: todo!(),
    };
}
