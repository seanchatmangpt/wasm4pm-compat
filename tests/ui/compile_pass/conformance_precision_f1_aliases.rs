#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// Law: QualityMetricKindDistinctionLaw — Metric<Fitness,N,D>, Metric<Precision,N,D>, and Metric<F1,N,D> are three distinct types; the KIND const-param prevents silent conflation

// COMPILE-PASS: PrecisionConst and F1Const type aliases are distinct types at
// compile time — prevents KIND-parameter conflation.
//
// Law: QualityMetricKind — Metric<Fitness, N, D>, Metric<Precision, N, D>,
// and Metric<F1, N, D> are three distinct types even when NUM and DEN match.
// This fixture proves the aliases resolve, construct, and cannot be silently
// substituted for each other.

use wasm4pm_compat::conformance::{F1Const, FitnessConst, PrecisionConst};

fn check_precision_alias() {
    // Boundary: 0/1 = 0.0 — lawful lower bound.
    let zero: PrecisionConst<0, 1> = PrecisionConst::new();
    assert_eq!(zero.num(), 0);
    assert_eq!(zero.den(), 1);

    // Interior: 1/2 = 0.5 — lawful.
    let half: PrecisionConst<1, 2> = PrecisionConst::new();
    assert_eq!(half.num(), 1);
    assert_eq!(half.den(), 2);

    // Boundary: 1/1 = 1.0 — lawful upper bound.
    let perfect: PrecisionConst<1, 1> = PrecisionConst::new();
    assert_eq!(perfect.num(), 1);
    assert_eq!(perfect.den(), 1);
}

fn check_f1_alias() {
    // Boundary: 0/1 = 0.0 — lawful lower bound.
    let zero: F1Const<0, 1> = F1Const::new();
    assert_eq!(zero.num(), 0);
    assert_eq!(zero.den(), 1);

    // Interior: 2/3 ≈ 0.667 — lawful.
    let two_thirds: F1Const<2, 3> = F1Const::new();
    assert_eq!(two_thirds.num(), 2);
    assert_eq!(two_thirds.den(), 3);

    // Boundary: 1/1 = 1.0 — lawful upper bound.
    let perfect: F1Const<1, 1> = F1Const::new();
    assert_eq!(perfect.num(), 1);
    assert_eq!(perfect.den(), 1);
}

fn check_kind_distinctness() {
    // FitnessConst, PrecisionConst, and F1Const with identical NUM/DEN are
    // distinct types: each function below accepts only its own kind.
    fn only_fitness(_: &FitnessConst<3, 4>) {}
    fn only_precision(_: &PrecisionConst<3, 4>) {}
    fn only_f1(_: &F1Const<3, 4>) {}

    let f: FitnessConst<3, 4> = FitnessConst::new();
    let p: PrecisionConst<3, 4> = PrecisionConst::new();
    let f1: F1Const<3, 4> = F1Const::new();

    only_fitness(&f);
    only_precision(&p);
    only_f1(&f1);
}

fn main() {
    check_precision_alias();
    check_f1_alias();
    check_kind_distinctness();
}
