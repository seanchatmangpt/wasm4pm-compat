// COMPILE-PASS: Generalization runtime newtype — proves Generalization::new() is
// fallible, accepts values in [0,1], and rejects out-of-range values.
//
// Law: conformance verdict shape — Generalization carries a score in [0,1]; it
// does NOT compute generalization from a log and model (engine concern).

use wasm4pm_compat::conformance::Generalization;

fn main() {
    assert!(Generalization::new(0.9).is_some());
    assert!(Generalization::new(0.0).is_some());
    assert!(Generalization::new(1.0).is_some());

    let g = Generalization::new(0.9).unwrap();
    assert_eq!(g.get(), 0.9);

    assert!(Generalization::new(1.1).is_none());
    assert!(Generalization::new(f64::INFINITY).is_none());
    assert!(Generalization::new(f64::NEG_INFINITY).is_none());
}
