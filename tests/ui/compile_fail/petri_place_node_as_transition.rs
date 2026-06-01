// COMPILE-FAIL: Petri net node law — PlaceNodeMarker cannot satisfy IsTransitionNode.
// Law: IsPlaceNode and IsTransitionNode are sealed distinct traits.
// A place marker cannot be used where a transition marker is required.
use wasm4pm_compat::petri::{IsTransitionNode, PlaceNodeMarker};

fn needs_transition<T: IsTransitionNode>(_: T) {}

fn main() {
    // PlaceNodeMarker does not implement IsTransitionNode.
    needs_transition(PlaceNodeMarker);
}
