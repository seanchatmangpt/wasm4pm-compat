// Nightly features required for Metric<KIND, NUM, DEN> with generic_const_exprs
// and adt_const_params. These are declared at the crate root unconditionally;
// integration tests inherit them via `extern crate` / the test harness.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

//! Integration test: metric bounds.
//!
//! Verifies that `FitnessConst<3,4>::num() == 3`, `den() == 4`, that all five
//! quality-dimension types construct correctly, and that `Between01` arithmetic
//! at the boundaries `{0/1, 1/1}` compiles and returns the right constants.

use wasm4pm_compat::conformance::{
    F1Const, FitnessConst, GeneralizationConst, PrecisionConst, QualityProfile, SimplicityConst,
};
use wasm4pm_compat::law::Between01;

// ── FitnessConst numerator / denominator accessors ───────────────────────────

#[test]
fn fitness_const_3_4_num_and_den() {
    let m: FitnessConst<3, 4> = FitnessConst::new();
    assert_eq!(m.num(), 3);
    assert_eq!(m.den(), 4);
}

#[test]
fn fitness_const_0_1_lower_boundary() {
    let m: FitnessConst<0, 1> = FitnessConst::new();
    assert_eq!(m.num(), 0);
    assert_eq!(m.den(), 1);
}

#[test]
fn fitness_const_1_1_upper_boundary() {
    let m: FitnessConst<1, 1> = FitnessConst::new();
    assert_eq!(m.num(), 1);
    assert_eq!(m.den(), 1);
}

// ── All five quality-dimension types construct correctly ─────────────────────

#[test]
fn all_five_metric_types_construct() {
    let _fitness: FitnessConst<3, 4> = FitnessConst::new();
    let _precision: PrecisionConst<1, 2> = PrecisionConst::new();
    let _f1: F1Const<0, 1> = F1Const::new();
    let _gen: GeneralizationConst<7, 8> = GeneralizationConst::new();
    let _simp: SimplicityConst<1, 1> = SimplicityConst::new();
}

#[test]
fn precision_const_num_den() {
    let m: PrecisionConst<1, 2> = PrecisionConst::new();
    assert_eq!(m.num(), 1);
    assert_eq!(m.den(), 2);
}

#[test]
fn f1_const_zero_value() {
    let m: F1Const<0, 1> = F1Const::new();
    assert_eq!(m.num(), 0);
    assert_eq!(m.den(), 1);
}

#[test]
fn generalization_const_num_den() {
    let m: GeneralizationConst<7, 8> = GeneralizationConst::new();
    assert_eq!(m.num(), 7);
    assert_eq!(m.den(), 8);
}

#[test]
fn simplicity_const_num_den() {
    let m: SimplicityConst<1, 2> = SimplicityConst::new();
    assert_eq!(m.num(), 1);
    assert_eq!(m.den(), 2);
}

// ── Between01 at exact boundaries ────────────────────────────────────────────

#[test]
fn between01_zero_over_one() {
    let _b: Between01<0, 1> = Between01::new();
}

#[test]
fn between01_one_over_one() {
    let _b: Between01<1, 1> = Between01::new();
}

#[test]
fn between01_rational_in_range() {
    let b: Between01<3, 4> = Between01::new();
    assert_eq!(b.num(), 3);
    assert_eq!(b.den(), 4);
}

// ── QualityProfile: five-slot composite ──────────────────────────────────────

#[test]
fn quality_profile_all_ones() {
    let p: QualityProfile<1, 1, 1, 1, 1, 1, 1, 1, 1, 1> = QualityProfile::new();
    assert_eq!(p.fitness.num(), 1);
    assert_eq!(p.fitness.den(), 1);
    assert_eq!(p.precision.num(), 1);
    assert_eq!(p.f1.num(), 1);
    assert_eq!(p.generalization.num(), 1);
    assert_eq!(p.simplicity.num(), 1);
}

#[test]
fn quality_profile_heterogeneous_values() {
    // fitness=0.9, precision=0.8, f1=0.85≈17/20, generalization=0.75=3/4, simplicity=0.95=19/20
    let p: QualityProfile<9, 10, 4, 5, 17, 20, 3, 4, 19, 20> = QualityProfile::new();
    assert_eq!(p.fitness.num(), 9);
    assert_eq!(p.fitness.den(), 10);
    assert_eq!(p.precision.num(), 4);
    assert_eq!(p.precision.den(), 5);
    assert_eq!(p.f1.num(), 17);
    assert_eq!(p.generalization.num(), 3);
    assert_eq!(p.simplicity.num(), 19);
}

#[test]
fn quality_profile_all_zero_over_one() {
    let p: QualityProfile<0, 1, 0, 1, 0, 1, 0, 1, 0, 1> = QualityProfile::new();
    assert_eq!(p.fitness.num(), 0);
    assert_eq!(p.precision.num(), 0);
    assert_eq!(p.f1.num(), 0);
    assert_eq!(p.generalization.num(), 0);
    assert_eq!(p.simplicity.num(), 0);
}
