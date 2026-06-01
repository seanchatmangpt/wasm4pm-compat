#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// Law: F1ScoreBoundsLaw — F1Const boundary values 0/1 and 1/1 both satisfy Between01; the closed interval endpoints compile

// COMPILE-PASS: F1Const<0,1> and F1Const<1,1> — zero and perfect F1 are lawful.
//
// Law: Between01 bounds — F1Const with boundary values 0/1 and 1/1 must compile.
// These are distinct fixtures from the precision/f1_aliases fixture because they
// focus on the F1 kind in isolation at its boundary values, without mixing with
// PrecisionConst comparisons. The F1 score is the harmonic mean of fitness and
// precision; structure-only, never computed here.

use wasm4pm_compat::conformance::F1Const;

fn check_f1_zero() {
    // 0/1 = 0.0 — lawful lower bound for F1.
    let zero: F1Const<0, 1> = F1Const::new();
    assert_eq!(zero.num(), 0);
    assert_eq!(zero.den(), 1);
}

fn check_f1_perfect() {
    // 1/1 = 1.0 — lawful upper bound for F1.
    let perfect: F1Const<1, 1> = F1Const::new();
    assert_eq!(perfect.num(), 1);
    assert_eq!(perfect.den(), 1);
}

fn check_f1_interior() {
    // Interior values — confirms the law applies throughout [0,1], not just ends.
    let half: F1Const<1, 2> = F1Const::new();
    assert_eq!(half.num(), 1);
    assert_eq!(half.den(), 2);

    let three_quarters: F1Const<3, 4> = F1Const::new();
    assert_eq!(three_quarters.num(), 3);

    // Default impl.
    let default_val: F1Const<1, 1> = Default::default();
    assert_eq!(default_val.num(), 1);
}

fn main() {
    check_f1_zero();
    check_f1_perfect();
    check_f1_interior();
}
