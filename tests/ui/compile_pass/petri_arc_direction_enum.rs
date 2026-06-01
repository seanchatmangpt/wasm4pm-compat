// COMPILE-PASS: ArcDirection — the two arc direction variants are constructible
// and distinguishable. Both are Copy, Eq, and Hash.
use wasm4pm_compat::petri::{ArcDirection, Arc};

fn main() {
    let d1 = ArcDirection::PlaceToTransition;
    let d2 = ArcDirection::TransitionToPlace;
    assert_ne!(d1, d2);

    let arc = Arc::place_to_transition("p", "t");
    assert_eq!(arc.direction(), ArcDirection::PlaceToTransition);

    let arc2 = Arc::transition_to_place("t", "p");
    assert_eq!(arc2.direction(), ArcDirection::TransitionToPlace);
}
