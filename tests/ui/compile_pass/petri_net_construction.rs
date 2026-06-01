// Law: PetriNetConstructionLaw — a structurally valid PetriNet (places, transitions, arcs, marking) validates successfully; structure-only with no firing semantics
// COMPILE-PASS: PetriNet — a structurally valid Petri net (places, transitions,
// arcs, marking) validates successfully. Structure-only.
use wasm4pm_compat::petri::{PetriNet, Place, Transition, Arc, Marking};

fn main() {
    let net = PetriNet::new(
        [Place::new("p0"), Place::new("p1")],
        [Transition::new("t0", "fire")],
        [
            Arc::place_to_transition("p0", "t0"),
            Arc::transition_to_place("t0", "p1"),
        ],
        Marking::new([("p0".to_string(), 1)]),
    );
    assert!(net.validate().is_ok());
    assert_eq!(net.places().len(), 2);
    assert_eq!(net.transitions().len(), 1);
    assert_eq!(net.arcs().len(), 2);
}
