#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: GeneralizationConst<NUM, DEN> (and the underlying
// Metric<{QualityMetricKind::Generalization}, NUM, DEN>) satisfies Between01 bounds
// for valid fractions — proves the Generalization kind is lawfully constructible
// via both the type alias and the generic form.
//
// Law: Between01 bounds — NUM/DEN must be in [0, 1].

use wasm4pm_compat::conformance::{GeneralizationConst, Metric};
use wasm4pm_compat::law::QualityMetricKind;

fn check_generalization_metric() {
    // Via the type alias — the preferred public surface.
    let alias_lower: GeneralizationConst<0, 1> = GeneralizationConst::new();
    assert_eq!(alias_lower.num(), 0);
    assert_eq!(alias_lower.den(), 1);

    let alias_interior: GeneralizationConst<7, 8> = GeneralizationConst::new();
    assert_eq!(alias_interior.num(), 7);
    assert_eq!(alias_interior.den(), 8);

    let alias_upper: GeneralizationConst<1, 1> = GeneralizationConst::new();
    assert_eq!(alias_upper.num(), 1);
    assert_eq!(alias_upper.den(), 1);

    // Via the generic form — proves the alias is transparent.
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
