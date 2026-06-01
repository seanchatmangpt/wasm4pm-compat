// COMPILE-PASS: PlaceNodeMarker — zero-sized place-kind marker compiles and
// satisfies the IsPlaceNode sealed trait bound.
use wasm4pm_compat::petri::{PlaceNodeMarker, IsPlaceNode};

fn place_slot<P: IsPlaceNode>(_: P) {}

fn main() {
    let m = PlaceNodeMarker;
    place_slot(m);
    let _ = PlaceNodeMarker::default();
}
