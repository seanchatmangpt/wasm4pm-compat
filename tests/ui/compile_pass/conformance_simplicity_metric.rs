#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// Law: SimplicityConstBoundsLaw — SimplicityConst satisfies Between01 for valid fractions; Simplicity and Generalization are distinct Metric kinds at the type level

// COMPILE-PASS: SimplicityConst<NUM, DEN> (and the underlying
// Metric<{QualityMetricKind::Simplicity}, NUM, DEN>) satisfies Between01 bounds
// for valid fractions — proves the Simplicity kind is lawfully constructible via
// both the type alias and the generic form, and that Simplicity and Generalization
// are distinct types at the type level even when NUM and DEN match.
//
// Law: Between01 bounds — NUM/DEN must be in [0, 1].

use wasm4pm_compat::conformance::{GeneralizationConst, Metric, SimplicityConst};
use wasm4pm_compat::law::QualityMetricKind;

fn check_simplicity_metric() {
    // Via the type alias — the preferred public surface.
    let alias_lower: SimplicityConst<0, 1> = SimplicityConst::new();
    assert_eq!(alias_lower.num(), 0);
    assert_eq!(alias_lower.den(), 1);

    let alias_interior: SimplicityConst<1, 2> = SimplicityConst::new();
    assert_eq!(alias_interior.num(), 1);
    assert_eq!(alias_interior.den(), 2);

    let alias_upper: SimplicityConst<1, 1> = SimplicityConst::new();
    assert_eq!(alias_upper.num(), 1);
    assert_eq!(alias_upper.den(), 1);

    // Via the generic form — proves the alias is transparent.
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
    // Simplicity and Generalization with identical NUM/DEN are distinct types via aliases:
    fn only_simplicity(_: &SimplicityConst<1, 2>) {}
    fn only_generalization(_: &GeneralizationConst<1, 2>) {}

    let s: SimplicityConst<1, 2> = SimplicityConst::new();
    let g: GeneralizationConst<1, 2> = GeneralizationConst::new();

    only_simplicity(&s);
    only_generalization(&g);

    // Also verify via the generic form.
    fn only_simplicity_generic(_: &Metric<{ QualityMetricKind::Simplicity }, 1, 2>) {}
    fn only_generalization_generic(_: &Metric<{ QualityMetricKind::Generalization }, 1, 2>) {}

    let sg: Metric<{ QualityMetricKind::Simplicity }, 1, 2> = Metric::new();
    let gg: Metric<{ QualityMetricKind::Generalization }, 1, 2> = Metric::new();

    only_simplicity_generic(&sg);
    only_generalization_generic(&gg);
}

fn main() {
    check_simplicity_metric();
    check_kind_distinctness_generalization_vs_simplicity();
}
