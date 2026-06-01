// COMPILE-PASS: MissingFinalMarkingError — the first-class error type for the
// missing-final-marking law is constructible, Display-formattable, and converts
// to PetriRefusal::MissingFinalMarking via From.
use wasm4pm_compat::petri::{MissingFinalMarkingError, PetriRefusal};

fn main() {
    let e = MissingFinalMarkingError;
    let display = format!("{e}");
    assert!(display.contains("MissingFinalMarking"));

    let r: PetriRefusal = e.into();
    assert_eq!(r, PetriRefusal::MissingFinalMarking);
}
