// COMPILE-FAIL: Petri net structural law — Transition cannot be passed where Place is required.
// Law: Transition (a named transition node) and Place (a named place node) are distinct structural types.
// A transition must never be confused with a place.
use wasm4pm_compat::petri::{Place, Transition};

fn requires_place(_p: Place) {}

fn main() {
    let t = Transition::new("t0");
    // This must fail: Transition is not Place.
    requires_place(t);
}
