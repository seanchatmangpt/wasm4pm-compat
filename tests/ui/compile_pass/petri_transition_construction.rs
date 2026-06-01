// Law: TransitionConstructionLaw — Transition is constructible with id and label; labeled and silent transitions are distinct shapes; structure-only with no firing semantics
// COMPILE-PASS: Transition — labeled and silent transitions are constructible.
// Structure-only: no firing semantics.
use wasm4pm_compat::petri::Transition;

fn main() {
    let t = Transition::new("t0", "approve");
    assert_eq!(t.id(), "t0");
    assert_eq!(t.label(), "approve");
    assert!(!t.is_silent());

    let silent = Transition::silent("t1");
    assert_eq!(silent.id(), "t1");
    assert!(silent.is_silent());
}
