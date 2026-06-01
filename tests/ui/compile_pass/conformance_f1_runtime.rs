// COMPILE-PASS: F1 runtime newtype — proves F1::new() is fallible, accepts
// values in [0,1], and rejects out-of-range values.
//
// Law: conformance verdict shape — F1 carries a score in [0,1]; it does NOT
// compute the harmonic mean from fitness and precision (that is an engine
// concern). Construction is fallible.

use wasm4pm_compat::conformance::F1;

fn main() {
    assert!(F1::new(0.0).is_some());
    assert!(F1::new(1.0).is_some());

    let f1 = F1::new(0.5).unwrap();
    assert_eq!(f1.get(), 0.5);

    assert!(F1::new(f64::NAN).is_none());
    assert!(F1::new(1.01).is_none());
    assert!(F1::new(-0.5).is_none());
}
