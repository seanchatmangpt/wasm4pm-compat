#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Metric bounds law — FitnessConst<3,2> violates NUM <= DEN.
// Law: Between01 bound; 3/2 = 1.5 > 1 is an illegal fitness value at the type level.
use wasm4pm_compat::conformance::FitnessConst;

fn main() {
    // 3/2 = 1.5 > 1.0: illegal fitness value.
    let _: FitnessConst<3, 2> = FitnessConst::new();
}
