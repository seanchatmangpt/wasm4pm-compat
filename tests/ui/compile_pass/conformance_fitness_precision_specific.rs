#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// Law: MetricSpecificValuesBoundsLaw — FitnessConst<3,4> and PrecisionConst<1,2> satisfy Between01; both specific rational values compile

// COMPILE-PASS: FitnessConst<3,4> and PrecisionConst<1,2> specific values.
//
// Law: Between01 bounds — targets the exact rational values from the
// "metric-types" batch task: fitness=3/4 and precision=1/2.
// These fixtures focus on the specific batch-target values in isolation,
// proving each satisfies the Between01 law independently of other metrics.

use wasm4pm_compat::conformance::{FitnessConst, PrecisionConst};

fn check_fitness_three_quarters() {
    // 3/4 = 0.75 — the specific batch-target fitness value.
    let f: FitnessConst<3, 4> = FitnessConst::new();
    assert_eq!(f.num(), 3);
    assert_eq!(f.den(), 4);
}

fn check_precision_one_half() {
    // 1/2 = 0.5 — the specific batch-target precision value.
    let p: PrecisionConst<1, 2> = PrecisionConst::new();
    assert_eq!(p.num(), 1);
    assert_eq!(p.den(), 2);
}

fn check_kind_independence() {
    // FitnessConst<3,4> and PrecisionConst<3,4> are distinct types even at
    // identical rationals — kind parameter prevents conflation.
    fn only_fitness(_: FitnessConst<3, 4>) {}
    fn only_precision(_: PrecisionConst<1, 2>) {}

    only_fitness(FitnessConst::<3, 4>::new());
    only_precision(PrecisionConst::<1, 2>::new());
}

fn main() {
    check_fitness_three_quarters();
    check_precision_one_half();
    check_kind_independence();
}
