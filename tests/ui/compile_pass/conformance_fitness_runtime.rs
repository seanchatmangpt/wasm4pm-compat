// COMPILE-PASS: Fitness runtime newtype — proves Fitness::new() is fallible,
// accepts values in [0,1], and rejects out-of-range values.
//
// Law: conformance verdict shape — Fitness carries a score in [0,1]; it does
// NOT compute one. Construction is fallible to enforce the unit interval law.

use wasm4pm_compat::conformance::Fitness;

fn main() {
    // Boundary values are accepted.
    assert!(Fitness::new(0.0).is_some());
    assert!(Fitness::new(1.0).is_some());

    // Interior value.
    let f = Fitness::new(0.85).unwrap();
    assert_eq!(f.get(), 0.85);

    // Out-of-range values are rejected.
    assert!(Fitness::new(1.5).is_none());
    assert!(Fitness::new(-0.1).is_none());
    assert!(Fitness::new(f64::NAN).is_none());
    assert!(Fitness::new(f64::INFINITY).is_none());
}
