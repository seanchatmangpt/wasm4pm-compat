// Law: MissingFinalMarkingRefusalLaw — PetriRefusal::MissingFinalMarking is the named refusal for a WfNet with an empty final marking; it is pattern-matchable as a named law, not a bare error
// COMPILE-PASS: MissingFinalMarking — PetriRefusal variant is constructible and
// pattern-matchable as a named refusal reason on a WF-net boundary.
//
// Proves that:
//   1. A WfNet with an empty final marking is refused.
//   2. The refusal carries PetriRefusal::MissingFinalMarking — the named law.
//   3. The variant is the only accepted reason (not a catch-all).
use wasm4pm_compat::petri::{WfNet, PetriNet, Place, Transition, Arc, Marking, PetriRefusal};

fn check() {
    // Build a structurally valid PetriNet with a non-empty initial marking
    // but supply an empty final marking — the missing-final-marking law fires.
    let net = PetriNet::new(
        [Place::new("src"), Place::new("snk")],
        [Transition::new("t", "a")],
        [Arc::place_to_transition("src", "t"), Arc::transition_to_place("t", "snk")],
        Marking::new([("src".to_string(), 1)]),
    );
    let wf = WfNet::new(net, Marking::empty());

    // The validate boundary must refuse with the exact named law.
    let refusal = wf.validate().unwrap_err();
    assert_eq!(refusal, PetriRefusal::MissingFinalMarking);

    // Named law is auditable via Display.
    let display = format!("{refusal}");
    assert!(display.contains("MissingFinalMarking"));
}

fn main() {
    check();
}
