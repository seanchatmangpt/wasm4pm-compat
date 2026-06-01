// COMPILE-PASS: Precision runtime newtype — proves Precision::new() is fallible,
// accepts values in [0,1], and rejects out-of-range values.
//
// Law: conformance verdict shape — Precision carries a score in [0,1]; it does
// NOT compute one. Construction is fallible to enforce the unit interval law.

use wasm4pm_compat::conformance::Precision;

fn main() {
    assert!(Precision::new(0.0).is_some());
    assert!(Precision::new(1.0).is_some());

    let p = Precision::new(0.7).unwrap();
    assert_eq!(p.get(), 0.7);

    assert!(Precision::new(2.0).is_none());
    assert!(Precision::new(-0.01).is_none());
    assert!(Precision::new(f64::NAN).is_none());
}
