#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: Metric<{QualityMetricKind::Generalization}, NUM, DEN> satisfies Between01
// bounds for valid fractions — proves the Generalization kind is lawfully constructible.
//
// Law: Between01 bounds — NUM/DEN must be in [0, 1]. There is no GeneralizationConst
// type alias (only FitnessConst/PrecisionConst/F1Const exist); this fixture proves
// that the Generalization variant of QualityMetricKind is supported by the generic
// Metric<KIND, NUM, DEN> type and that Between01 enforcement applies uniformly.

use wasm4pm_compat::conformance::Metric;
use wasm4pm_compat::law::QualityMetricKind;

fn check_generalization_metric() {
    // Boundary: 0/1 = 0.0 — lawful lower bound.
    let zero: Metric<{ QualityMetricKind::Generalization }, 0, 1> = Metric::new();
    assert_eq!(zero.num(), 0);
    assert_eq!(zero.den(), 1);

    // Interior: 3/5 = 0.6 — lawful.
    let three_fifths: Metric<{ QualityMetricKind::Generalization }, 3, 5> = Metric::new();
    assert_eq!(three_fifths.num(), 3);
    assert_eq!(three_fifths.den(), 5);

    // Boundary: 1/1 = 1.0 — lawful upper bound.
    let perfect: Metric<{ QualityMetricKind::Generalization }, 1, 1> = Metric::new();
    assert_eq!(perfect.num(), 1);
    assert_eq!(perfect.den(), 1);

    // Equal numerator and denominator: 8/8 = 1.0 — lawful.
    let eight_eighths: Metric<{ QualityMetricKind::Generalization }, 8, 8> = Metric::new();
    assert_eq!(eight_eighths.num(), 8);
    assert_eq!(eight_eighths.den(), 8);

    // Default impl produces the same as new().
    let default_metric: Metric<{ QualityMetricKind::Generalization }, 3, 5> = Default::default();
    assert_eq!(default_metric.num(), 3);
    assert_eq!(default_metric.den(), 5);
}

fn main() {
    check_generalization_metric();
}
