// COMPILE-PASS: SyncMove alignment marker — proves SyncMove is zero-sized,
// Default, Copy, and usable as a Deviation witness.
//
// Law: alignment deviation shape — SyncMove witnesses that log and model agree
// on a step; it is a zero-cost phantom type marker.

use wasm4pm_compat::conformance::{Deviation, SyncMove};

fn accepts_sync(_d: Deviation<SyncMove>) {}

fn main() {
    let marker = SyncMove;
    let marker2 = marker; // Copy
    assert_eq!(marker, marker2);

    let default_marker = SyncMove::default();
    assert_eq!(default_marker, SyncMove);

    let d = Deviation::<SyncMove>::new(0, "process_order");
    assert_eq!(d.position, 0);
    assert_eq!(d.label, "process_order");

    accepts_sync(d);
}
