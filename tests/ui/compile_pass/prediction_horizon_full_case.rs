// COMPILE-PASS: PredictionHorizon::FullCase — proves unbounded horizon constructs
// and is the Default variant.
//
// Law: prediction horizon shape — FullCase means the prediction spans the entire
// remaining case with no fixed event or time bound.

use wasm4pm_compat::prediction::PredictionHorizon;

fn main() {
    let h = PredictionHorizon::FullCase;
    assert!(matches!(h, PredictionHorizon::FullCase));
    assert_eq!(PredictionHorizon::default(), PredictionHorizon::FullCase);
    assert_eq!(format!("{h}"), "full-case");
}
