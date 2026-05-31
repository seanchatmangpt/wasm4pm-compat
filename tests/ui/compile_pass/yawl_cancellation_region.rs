// Compile-pass fixture: CancellationRegion can be constructed and attached as
// an optional field on a PetriNet-bearing structure.
//
// Law: YAWL Definition 1 rem: T ⇸ P(T ∪ C \ {i,o}) — each task optionally
// names a cancellation region (a named set of node ids). The shape is
// structure-only; token removal execution graduates to wasm4pm.

use wasm4pm_compat::petri::{CancellationRegion, PetriNet, Place, Transition, Arc, Marking};

fn main() {
    // Construct a cancellation region naming two nodes.
    let region = CancellationRegion::new(["p1", "t2"]);
    assert_eq!(region.members().len(), 2);
    assert_eq!(region.members()[0], "p1");

    // An empty cancellation region is valid (the task has no cancellation set).
    let empty = CancellationRegion::default();
    assert!(empty.members().is_empty());

    // Construct a basic PetriNet and associate a cancellation region with it
    // as application-level metadata (petri.rs models the shape; enforcement
    // of which nodes belong to the region graduates to wasm4pm).
    let net = PetriNet::new(
        [Place::new("p1"), Place::new("p2")],
        [Transition::new("t1", "approve"), Transition::new("t2", "cancel")],
        [
            Arc::place_to_transition("p1", "t1"),
            Arc::transition_to_place("t1", "p2"),
            Arc::place_to_transition("p1", "t2"),
        ],
        Marking::new([("p1".to_string(), 1)]),
    );
    assert!(net.validate().is_ok());

    // A task in the net can be given a cancellation region covering p2.
    let _task_cancellation = CancellationRegion::new(["p2"]);
}
