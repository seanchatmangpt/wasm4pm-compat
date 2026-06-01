#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: BipartiteArcConst<TransitionToPlace> — the post-incidence arc
// direction is a distinct type from PlaceToTransition at compile time.
use wasm4pm_compat::petri::BipartiteArcConst;
use wasm4pm_compat::law::ArcDirectionConst;

fn main() {
    // new(place_id, transition_id, weight) — same parameter order for both directions
    let post = BipartiteArcConst::<{ ArcDirectionConst::TransitionToPlace }, u32>::new("p1", "t1", 2);
    assert_eq!(post.place_id(), "p1");
    assert_eq!(post.transition_id(), "t1");
    assert_eq!(post.weight(), 2u32);
    assert_eq!(post.direction(), ArcDirectionConst::TransitionToPlace);
}
