#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// Law: GeneralizationNineTenthsLaw — GeneralizationConst<9,10> = 0.9 satisfies Between01; batch-target value compiles

// COMPILE-PASS: GeneralizationConst<9,10> — 0.9 generalization score is lawful.
//
// Law: Between01 bounds — NUM/DEN must be in [0, 1].
// Specifically targets the 9/10 value from the "metric-types" batch.
// Generalization measures how well a discovered model covers unseen traces
// (van der Aalst 2016, §9). This fixture proves the specific 9/10 rational
// satisfies the Between01 bound and constructs.

use wasm4pm_compat::conformance::GeneralizationConst;

fn check_generalization_nine_tenths() {
    let g: GeneralizationConst<9, 10> = GeneralizationConst::new();
    assert_eq!(g.num(), 9);
    assert_eq!(g.den(), 10);
}

fn check_generalization_adjacent_values() {
    // Confirm nearby values also lawful — not a boundary-only coincidence.
    let eight_tenths: GeneralizationConst<8, 10> = GeneralizationConst::new();
    assert_eq!(eight_tenths.num(), 8);

    let ten_tenths: GeneralizationConst<10, 10> = GeneralizationConst::new();
    assert_eq!(ten_tenths.num(), 10);
    assert_eq!(ten_tenths.den(), 10);

    // Default impl.
    let default_val: GeneralizationConst<9, 10> = Default::default();
    assert_eq!(default_val.num(), 9);
}

fn main() {
    check_generalization_nine_tenths();
    check_generalization_adjacent_values();
}
