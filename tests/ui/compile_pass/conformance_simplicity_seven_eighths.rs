#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// Law: SimplicitySevenEighthsLaw — SimplicityConst<7,8> = 0.875 satisfies Between01; batch-target value compiles

// COMPILE-PASS: SimplicityConst<7,8> — 0.875 simplicity score is lawful.
//
// Law: Between01 bounds — NUM/DEN must be in [0, 1].
// Specifically targets the 7/8 value from the "metric-types" batch.
// Simplicity measures structural parsimony of a process model (van der Aalst 2016, §9).
// This fixture proves the specific 7/8 rational satisfies Between01 and constructs.

use wasm4pm_compat::conformance::SimplicityConst;

fn check_simplicity_seven_eighths() {
    let s: SimplicityConst<7, 8> = SimplicityConst::new();
    assert_eq!(s.num(), 7);
    assert_eq!(s.den(), 8);
}

fn check_simplicity_adjacent_values() {
    // Confirm nearby values also lawful.
    let six_eighths: SimplicityConst<6, 8> = SimplicityConst::new();
    assert_eq!(six_eighths.num(), 6);

    let eight_eighths: SimplicityConst<8, 8> = SimplicityConst::new();
    assert_eq!(eight_eighths.num(), 8);
    assert_eq!(eight_eighths.den(), 8);

    // Default impl.
    let default_val: SimplicityConst<7, 8> = Default::default();
    assert_eq!(default_val.num(), 7);
}

fn main() {
    check_simplicity_seven_eighths();
    check_simplicity_adjacent_values();
}
