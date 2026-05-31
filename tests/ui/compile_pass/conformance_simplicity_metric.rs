#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: Metric<{QualityMetricKind::Simplicity}, NUM, DEN> satisfies Between01
// bounds for valid fractions — proves the Simplicity kind is lawfully constructible.
//
// Law: Between01 bounds — NUM/DEN must be in [0, 1]. There is no SimplicityConst
// type alias (only FitnessConst/PrecisionConst/F1Const exist); this fixture proves
// that the Simplicity variant of QualityMetricKind is supported by the generic
// Metric<KIND, NUM, DEN> type, that Between01 enforcement applies uniformly across
// all QualityMetricKind variants, and that Simplicity and Generalization metrics
// are distinct types even when NUM and DEN match.

use wasm4pm_compat::conformance::Metric;
use wasm4pm_compat::law::QualityMetricKind;

fn check_simplicity_metric() {
    // Boundary: 0/1 = 0.0 — lawful lower bound.
    let zero: Metric<{ QualityMetricKind::Simplicity }, 0, 1> = Metric::new();
    assert_eq!(zero.num(), 0);
    assert_eq!(zero.den(), 1);

    // Interior: 2/3 ≈ 0.667 — lawful.
    let two_thirds: Metric<{ QualityMetricKind::Simplicity }, 2, 3> = Metric::new();
    assert_eq!(two_thirds.num(), 2);
    assert_eq!(two_thirds.den(), 3);

    // Boundary: 1/1 = 1.0 — lawful upper bound.
    let perfect: Metric<{ QualityMetricKind::Simplicity }, 1, 1> = Metric::new();
    assert_eq!(perfect.num(), 1);
    assert_eq!(perfect.den(), 1);

    // Large denominator: 99/100 = 0.99 — lawful.
    let near_perfect: Metric<{ QualityMetricKind::Simplicity }, 99, 100> = Metric::new();
    assert_eq!(near_perfect.num(), 99);
    assert_eq!(near_perfect.den(), 100);

    // Default impl produces the same as new().
    let default_metric: Metric<{ QualityMetricKind::Simplicity }, 2, 3> = Default::default();
    assert_eq!(default_metric.num(), 2);
    assert_eq!(default_metric.den(), 3);
}

fn check_kind_distinctness_generalization_vs_simplicity() {
    // Simplicity and Generalization with identical NUM/DEN are distinct types:
    // each function below accepts only its own kind.
    fn only_simplicity(_: &Metric<{ QualityMetricKind::Simplicity }, 1, 2>) {}
    fn only_generalization(_: &Metric<{ QualityMetricKind::Generalization }, 1, 2>) {}

    let s: Metric<{ QualityMetricKind::Simplicity }, 1, 2> = Metric::new();
    let g: Metric<{ QualityMetricKind::Generalization }, 1, 2> = Metric::new();

    only_simplicity(&s);
    only_generalization(&g);
}

fn main() {
    check_simplicity_metric();
    check_kind_distinctness_generalization_vs_simplicity();
}
