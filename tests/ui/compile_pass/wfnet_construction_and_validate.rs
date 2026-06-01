// COMPILE-PASS: WfNet — a structurally valid WF-net (initial + final marking,
// arcs reference declared nodes) passes structural validation. Structure-only.
use wasm4pm_compat::petri::{WfNet, PetriNet, Place, Transition, Arc, Marking};

fn main() {
    let net = PetriNet::new(
        [Place::new("src"), Place::new("snk")],
        [Transition::new("t", "a")],
        [
            Arc::place_to_transition("src", "t"),
            Arc::transition_to_place("t", "snk"),
        ],
        Marking::new([("src".to_string(), 1)]),
    );
    let wf = WfNet::new(net, Marking::new([("snk".to_string(), 1)]));
    assert!(wf.validate().is_ok());

    // final_marking accessor
    assert!(wf.final_marking().is_some());
    // underlying net accessor
    assert_eq!(wf.net().places().len(), 2);
}
