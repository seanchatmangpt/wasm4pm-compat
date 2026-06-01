#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Bipartite arc direction law — BipartiteArcConst<{PlaceToTransition}> cannot be
// passed where BipartiteArcConst<{TransitionToPlace}> is required.
// Law: ArcDirectionConst is a const generic; different directions produce distinct types.
use wasm4pm_compat::law::ArcDirectionConst;
use wasm4pm_compat::petri::BipartiteArcConst;

fn requires_post_arc(
    _arc: BipartiteArcConst<{ ArcDirectionConst::TransitionToPlace }, u8>,
) {
}

fn main() {
    let pre = BipartiteArcConst::<{ ArcDirectionConst::PlaceToTransition }, u8>::new("p0", "t0", 1);
    // This must fail: PlaceToTransition arc is not a TransitionToPlace arc.
    requires_post_arc(pre);
}
