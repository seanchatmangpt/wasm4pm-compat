#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-FAIL: SeparableWfNet separability precondition — a plain WfNetConst
// is not accepted where SeparableWfNet is required.
//
// Law: Kourani, Park & van der Aalst (2026) Definition 4.1 — separability is a
// distinct precondition. The type SeparableWfNet<S> is the type-level receipt
// that separability has been declared. A bare WfNetConst<S> does not carry this
// receipt; the type system rejects it at any gate that enforces the separability
// precondition.
//
// Why this fixture: the pass fixture (separable_wfnet_marker.rs) proves the
// lawful path is open — a SeparableWfNet wrapping a WfNetConst is accepted.
// This fixture proves the negative: a plain WfNetConst, which has no
// separability marker, is NOT accepted where SeparableWfNet is required.
//
// Expected error: mismatched types — expected SeparableWfNet<_>, found WfNetConst<_>
use wasm4pm_compat::law::SoundnessState;
use wasm4pm_compat::petri::{SeparableWfNet, WfNetConst};

/// Separability gate: only SeparableWfNet satisfies Definition 4.1.
///
/// This mirrors the `only_separable` function in the pass fixture.
fn only_separable<const S: SoundnessState>(_: &SeparableWfNet<S>) {}

fn main() {
    // A plain WfNetConst has no separability marker — it is not a SeparableWfNet.
    let plain = WfNetConst::<{ SoundnessState::Unknown }>::new();
    // ERROR: expected &SeparableWfNet<_>, found &WfNetConst<{Unknown}>.
    // A plain WfNetConst does not satisfy Definition 4.1; this must not compile.
    only_separable(&plain);
}
