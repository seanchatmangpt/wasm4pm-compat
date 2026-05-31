#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: QualityProfile construction — all five metric slots satisfy
// Between01 bounds simultaneously.
//
// Law: Between01 bounds on all five van der Aalst quality dimensions.
// A QualityProfile is only constructible when every individual metric
// (fitness, precision, F1, generalization, simplicity) satisfies NUM/DEN ∈ [0,1].
// This fixture proves lawful profiles construct via new() and Default.

use wasm4pm_compat::conformance::QualityProfile;

fn check_all_perfect() {
    // All metrics at 1/1 = 1.0 — the perfect profile.
    let p: QualityProfile<1, 1, 1, 1, 1, 1, 1, 1, 1, 1> = QualityProfile::new();
    assert_eq!(p.fitness.num(), 1);
    assert_eq!(p.fitness.den(), 1);
    assert_eq!(p.precision.num(), 1);
    assert_eq!(p.simplicity.den(), 1);
}

fn check_all_zero() {
    // All metrics at 0/1 = 0.0 — the zero profile (lawful lower bound).
    let p: QualityProfile<0, 1, 0, 1, 0, 1, 0, 1, 0, 1> = QualityProfile::new();
    assert_eq!(p.fitness.num(), 0);
    assert_eq!(p.generalization.num(), 0);
    assert_eq!(p.simplicity.num(), 0);
}

fn check_mixed_profile() {
    // fitness=0.9, precision=0.8, f1=17/20, generalization=3/4, simplicity=19/20
    let p: QualityProfile<9, 10, 4, 5, 17, 20, 3, 4, 19, 20> = QualityProfile::new();
    assert_eq!(p.fitness.num(), 9);
    assert_eq!(p.fitness.den(), 10);
    assert_eq!(p.precision.num(), 4);
    assert_eq!(p.precision.den(), 5);
    assert_eq!(p.f1.num(), 17);
    assert_eq!(p.f1.den(), 20);
    assert_eq!(p.generalization.num(), 3);
    assert_eq!(p.generalization.den(), 4);
    assert_eq!(p.simplicity.num(), 19);
    assert_eq!(p.simplicity.den(), 20);
}

fn check_task_target_values() {
    // Covers the specific values from the "metric-types" batch task:
    // fitness=3/4, precision=1/2, f1=0/1, generalization=9/10, simplicity=7/8
    let p: QualityProfile<3, 4, 1, 2, 0, 1, 9, 10, 7, 8> = QualityProfile::new();
    assert_eq!(p.fitness.num(), 3);
    assert_eq!(p.precision.num(), 1);
    assert_eq!(p.f1.num(), 0);
    assert_eq!(p.generalization.num(), 9);
    assert_eq!(p.simplicity.num(), 7);
    assert_eq!(p.simplicity.den(), 8);
}

fn check_default() {
    // Default impl is identical to new() for all-perfect profile.
    let p: QualityProfile<1, 1, 1, 1, 1, 1, 1, 1, 1, 1> = Default::default();
    assert_eq!(p.fitness.num(), 1);
}

fn main() {
    check_all_perfect();
    check_all_zero();
    check_mixed_profile();
    check_task_target_values();
    check_default();
}
