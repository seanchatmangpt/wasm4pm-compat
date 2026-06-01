// COMPILE-PASS: TransitionNodeMarker — zero-sized transition-kind marker compiles
// and satisfies the IsTransitionNode sealed trait bound.
use wasm4pm_compat::petri::{TransitionNodeMarker, IsTransitionNode};

fn transition_slot<T: IsTransitionNode>(_: T) {}

fn main() {
    let m = TransitionNodeMarker;
    transition_slot(m);
    let _ = TransitionNodeMarker::default();
}
