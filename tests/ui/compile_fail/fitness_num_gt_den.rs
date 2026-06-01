#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Between01 — FitnessConst<3,2> rejected because NUM > DEN (3/2 > 1)
// Law: Require<{ NUM <= DEN }>: IsTrue fails when NUM=3, DEN=2
// Expected error: mismatched types — expected `false`, found `true`
use wasm4pm_compat::conformance::FitnessConst;

fn main() {
    // 3/2 = 1.5 > 1.0: violates Between01 law
    let _: FitnessConst<3, 2> = FitnessConst::new();
}
