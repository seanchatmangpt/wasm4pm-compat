#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: FitnessConst<NUM, DEN> type alias resolves and satisfies Between01
// bounds for valid fractions — covers conformance metric type alias surface.
//
// Law: Between01 bounds — NUM/DEN must be in [0, 1]. FitnessConst is a type alias
// for Metric<{QualityMetricKind::Fitness}, NUM, DEN>; this fixture proves the alias
// resolves, new() constructs, and num()/den() carry the correct values.

use wasm4pm_compat::conformance::FitnessConst;

fn check_fitness_alias() {
    // Boundary: 0/1 = 0.0 — lawful lower bound.
    let zero: FitnessConst<0, 1> = FitnessConst::new();
    assert_eq!(zero.num(), 0);
    assert_eq!(zero.den(), 1);

    // Interior: 3/4 = 0.75 — lawful.
    let three_quarters: FitnessConst<3, 4> = FitnessConst::new();
    assert_eq!(three_quarters.num(), 3);
    assert_eq!(three_quarters.den(), 4);

    // Boundary: 1/1 = 1.0 — lawful upper bound.
    let perfect: FitnessConst<1, 1> = FitnessConst::new();
    assert_eq!(perfect.num(), 1);
    assert_eq!(perfect.den(), 1);

    // Equal numerator and denominator: 7/7 = 1.0 — lawful.
    let seven_sevenths: FitnessConst<7, 7> = FitnessConst::new();
    assert_eq!(seven_sevenths.num(), 7);
    assert_eq!(seven_sevenths.den(), 7);

    // Default impl produces the same as new().
    let default_metric: FitnessConst<3, 4> = Default::default();
    assert_eq!(default_metric.num(), 3);
    assert_eq!(default_metric.den(), 4);
}

fn main() {
    check_fitness_alias();
}
