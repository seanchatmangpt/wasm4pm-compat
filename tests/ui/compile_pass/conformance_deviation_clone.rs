// COMPILE-PASS: Deviation<M> clone and eq — proves Deviation<M> is Clone, PartialEq,
// and that cloned deviations compare equal.
//
// Law: alignment deviation shape — Deviation<M> is a value type; clone produces
// an equal copy, and eq compares position and label.

use wasm4pm_compat::conformance::{Deviation, SyncMove};

fn main() {
    let d = Deviation::<SyncMove>::new(2, "review_case");
    let d2 = d.clone();
    assert_eq!(d, d2);
    assert_eq!(d2.position, 2);
    assert_eq!(d2.label, "review_case");

    // Different positions are not equal.
    let d3 = Deviation::<SyncMove>::new(99, "review_case");
    assert_ne!(d, d3);
}
