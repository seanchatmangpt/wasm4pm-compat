#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: BipartiteArcConst<PlaceToTransition> — the pre-incidence arc
// direction is encoded as a const-generic parameter, producing a distinct type
// from the post-incidence direction.
use wasm4pm_compat::petri::BipartiteArcConst;
use wasm4pm_compat::law::ArcDirectionConst;

fn main() {
    let pre = BipartiteArcConst::<{ ArcDirectionConst::PlaceToTransition }, u8>::new("p0", "t0", 1);
    assert_eq!(pre.place_id(), "p0");
    assert_eq!(pre.transition_id(), "t0");
    assert_eq!(pre.weight(), 1u8);
    assert_eq!(pre.direction(), ArcDirectionConst::PlaceToTransition);
}
