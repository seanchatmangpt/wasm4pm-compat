// COMPILE-PASS: ConformanceVerdict with mixed deviation types — proves the
// deviation path field accepts untyped Deviation shapes at the collection level.
//
// Law: conformance verdict shape — deviations are collected as untyped
// Deviation<()> at the verdict level; typed witnesses are used at the
// individual Deviation<M> call site.

use wasm4pm_compat::conformance::{
    ConformanceVerdict, Deviation, Fitness, LogOnlyMove, ModelOnlyMove,
};

fn main() {
    let mut v = ConformanceVerdict::new();
    v.fitness = Fitness::new(0.6);

    // Add untyped deviations to the verdict's deviation collection.
    v.deviations.push(Deviation::new(2, "skip_approval"));
    v.deviations.push(Deviation::new(5, "extra_review"));

    assert_eq!(v.deviations.len(), 2);
    assert_eq!(v.deviations[0].position, 2);
    assert_eq!(v.deviations[1].label, "extra_review");
    assert!(!v.is_perfect());

    // Individual typed deviations are constructible separately.
    let log_dev = Deviation::<LogOnlyMove>::new(3, "unexpected");
    let model_dev = Deviation::<ModelOnlyMove>::new(7, "missing");
    assert_eq!(log_dev.position, 3);
    assert_eq!(model_dev.label, "missing");
}
