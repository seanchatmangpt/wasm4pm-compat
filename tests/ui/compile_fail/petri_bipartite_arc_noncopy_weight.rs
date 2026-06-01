#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: BipartiteArcNonCopyWeightLaw — BipartiteArcConst::weight() requires Weight: Copy;
// a non-Copy weight type (String) cannot be used with weight() on BipartiteArcConst.
// Law: Murata (1989) §2 — arc weights are multiplicities (positive integers, Copy types);
// non-Copy weight types violate the arc-weight law surface.
// Expected error: E0277 — the trait bound `String: Copy` is not satisfied.
use wasm4pm_compat::law::ArcDirectionConst;
use wasm4pm_compat::petri::BipartiteArcConst;

fn requires_copy_weight<W: Copy>(
    arc: &BipartiteArcConst<{ ArcDirectionConst::PlaceToTransition }, W>,
) -> W {
    arc.weight()
}

fn main() {
    // String is not Copy — this must fail to satisfy the Weight: Copy bound.
    let arc = BipartiteArcConst::<{ ArcDirectionConst::PlaceToTransition }, String>::new(
        "p0",
        "t0",
        "not-a-weight".to_string(),
    );
    // Calling weight() on a non-Copy type violates the arc-weight law.
    let _ = requires_copy_weight(&arc);
}
