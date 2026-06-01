// COMPILE-PASS: ConformanceVerdict with all five runtime scores populated —
// proves the full verdict shape is constructible with all quality dimensions.
//
// Law: conformance verdict shape — all five van der Aalst quality dimensions
// (fitness, precision, f1, generalization, simplicity) are optional fields that
// carry externally computed scores; the verdict never computes them.

use wasm4pm_compat::conformance::{
    ConformanceVerdict, F1, Fitness, Generalization, Precision, Simplicity,
};

fn main() {
    let mut v = ConformanceVerdict::new();
    v.fitness = Fitness::new(0.9);
    v.precision = Precision::new(0.8);
    v.f1 = F1::new(0.847);
    v.generalization = Generalization::new(0.75);
    v.simplicity = Simplicity::new(0.95);

    assert!(v.fitness.is_some());
    assert!(v.precision.is_some());
    assert!(v.f1.is_some());
    assert!(v.generalization.is_some());
    assert!(v.simplicity.is_some());
    assert!(v.deviations.is_empty());

    assert_eq!(v.fitness.unwrap().get(), 0.9);
    assert_eq!(v.precision.unwrap().get(), 0.8);
    assert_eq!(v.generalization.unwrap().get(), 0.75);
    assert_eq!(v.simplicity.unwrap().get(), 0.95);
}
