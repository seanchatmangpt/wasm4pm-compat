// COMPILE-PASS: Place — the named place node is constructible and its id is
// accessible. Structure-only: no token dynamics.
use wasm4pm_compat::petri::Place;

fn main() {
    let p = Place::new("p0");
    assert_eq!(p.id(), "p0");

    let p2 = Place::new("source-place");
    assert_eq!(p2.id(), "source-place");
}
