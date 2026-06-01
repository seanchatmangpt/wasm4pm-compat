// COMPILE-FAIL: Petri net node law — TransitionNodeMarker cannot satisfy IsPlaceNode.
// Law: IsPlaceNode and IsTransitionNode are sealed distinct traits.
// A transition marker cannot be used where a place marker is required.
use wasm4pm_compat::petri::{IsPlaceNode, TransitionNodeMarker};

fn needs_place<P: IsPlaceNode>(_: P) {}

fn main() {
    // TransitionNodeMarker does not implement IsPlaceNode.
    needs_place(TransitionNodeMarker);
}
