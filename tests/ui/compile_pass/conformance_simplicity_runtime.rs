// COMPILE-PASS: Simplicity runtime newtype — proves Simplicity::new() is
// fallible, accepts values in [0,1], and rejects out-of-range values.
//
// Law: conformance verdict shape — Simplicity carries a score in [0,1]; it does
// NOT compute structural parsimony (engine concern).

use wasm4pm_compat::conformance::Simplicity;

fn main() {
    assert!(Simplicity::new(0.0).is_some());
    assert!(Simplicity::new(1.0).is_some());
    assert!(Simplicity::new(0.6).is_some());

    let s = Simplicity::new(0.6).unwrap();
    assert_eq!(s.get(), 0.6);

    assert!(Simplicity::new(-0.5).is_none());
    assert!(Simplicity::new(1.5).is_none());
    assert!(Simplicity::new(f64::NAN).is_none());
}
