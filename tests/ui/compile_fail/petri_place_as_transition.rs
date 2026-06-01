// COMPILE-FAIL: Petri net structural law — Place cannot be passed where Transition is required.
// Law: Place (a named place node in a Petri net) and Transition (a named transition node)
// are distinct structural types. A place must never be confused with a transition.
use wasm4pm_compat::petri::{Place, Transition};

fn requires_transition(_t: Transition) {}

fn main() {
    let place = Place::new("p0");
    // This must fail: Place is not Transition.
    requires_transition(place);
}
