// Law: ArcTypedVariableLaw — Arc::typed carries an object-type label and a variable flag; the arc is structure-only with no token dynamics
// COMPILE-PASS: Arc::typed — an OC-Petri-net arc typed by object type with the
// variable flag is constructible and queryable. Structure-only.
use wasm4pm_compat::petri::Arc;

fn main() {
    let arc = Arc::place_to_transition("p0", "t0").typed("order", true);
    assert_eq!(arc.object_type(), Some("order"));
    assert!(arc.is_variable());

    let fixed = Arc::transition_to_place("t0", "p1").typed("item", false);
    assert_eq!(fixed.object_type(), Some("item"));
    assert!(!fixed.is_variable());
}
