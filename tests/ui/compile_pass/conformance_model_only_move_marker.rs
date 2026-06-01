// COMPILE-PASS: ModelOnlyMove alignment marker — proves ModelOnlyMove is
// zero-sized, Default, Copy, and usable as a Deviation witness.
//
// Law: alignment deviation shape — ModelOnlyMove witnesses that the model
// required a step the log did not show (a skip / missing activity).

use wasm4pm_compat::conformance::{Deviation, ModelOnlyMove};

fn accepts_model_only(_d: Deviation<ModelOnlyMove>) {}

fn main() {
    let marker = ModelOnlyMove;
    let marker2 = marker; // Copy
    assert_eq!(marker, marker2);
    assert_eq!(ModelOnlyMove::default(), ModelOnlyMove);

    let d = Deviation::<ModelOnlyMove>::new(6, "mandatory_sign_off");
    assert_eq!(d.position, 6);
    assert_eq!(d.label, "mandatory_sign_off");

    accepts_model_only(d);
}
