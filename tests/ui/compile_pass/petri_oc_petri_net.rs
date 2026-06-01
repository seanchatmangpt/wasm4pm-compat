// COMPILE-PASS: ObjectCentricPetriNet — an OC-Petri-net with typed arcs
// validates when all arc object types are declared. Structure-only.
use wasm4pm_compat::petri::{ObjectCentricPetriNet, PetriNet, Place, Transition, Arc, Marking};

fn main() {
    let net = PetriNet::new(
        [Place::new("p")],
        [Transition::new("t", "ship")],
        [Arc::place_to_transition("p", "t").typed("order", false)],
        Marking::empty(),
    );
    let ocpn = ObjectCentricPetriNet::new(net, ["order".to_string()]);
    assert!(ocpn.validate().is_ok());
    assert_eq!(ocpn.object_types(), &["order".to_string()]);
}
