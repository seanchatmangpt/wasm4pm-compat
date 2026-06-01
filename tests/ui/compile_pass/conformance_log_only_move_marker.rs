// COMPILE-PASS: LogOnlyMove alignment marker — proves LogOnlyMove is zero-sized,
// Default, Copy, and usable as a Deviation witness.
//
// Law: alignment deviation shape — LogOnlyMove witnesses that the log had a step
// the model could not match (an insertion relative to the model).

use wasm4pm_compat::conformance::{Deviation, LogOnlyMove};

fn accepts_log_only(_d: Deviation<LogOnlyMove>) {}

fn main() {
    let marker = LogOnlyMove;
    let marker2 = marker; // Copy
    assert_eq!(marker, marker2);
    assert_eq!(LogOnlyMove::default(), LogOnlyMove);

    let d = Deviation::<LogOnlyMove>::new(4, "unexpected_cancel");
    assert_eq!(d.position, 4);

    accepts_log_only(d);
}
