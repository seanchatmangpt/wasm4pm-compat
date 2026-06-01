// COMPILE-PASS: ConformanceVerdict Default — proves ConformanceVerdict::default()
// produces an empty verdict with all Option fields None and no deviations.
//
// Law: conformance verdict shape — the default verdict is empty; all scores are
// absent and the deviation path is empty. This is the zero-knowledge starting
// point before any engine populates the verdict.

use wasm4pm_compat::conformance::ConformanceVerdict;

fn main() {
    let v = ConformanceVerdict::default();
    assert!(v.fitness.is_none());
    assert!(v.precision.is_none());
    assert!(v.f1.is_none());
    assert!(v.generalization.is_none());
    assert!(v.simplicity.is_none());
    assert!(v.deviations.is_empty());
    assert!(!v.is_perfect());
}
