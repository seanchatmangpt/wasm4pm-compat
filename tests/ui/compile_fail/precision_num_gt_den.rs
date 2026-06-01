#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Between01 — PrecisionConst<5,3> rejected because NUM > DEN (5/3 > 1)
// Law: Require<{ NUM <= DEN }>: IsTrue fails when NUM=5, DEN=3
// Expected error: mismatched types — expected `false`, found `true`
use wasm4pm_compat::conformance::PrecisionConst;

fn main() {
    // 5/3 > 1.0: violates Between01 law
    let _: PrecisionConst<5, 3> = PrecisionConst::new();
}
