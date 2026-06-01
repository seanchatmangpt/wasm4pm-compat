#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Between01 — SimplicityConst<10,9> rejected because NUM > DEN (10/9 > 1)
// Law: Require<{ NUM <= DEN }>: IsTrue fails when NUM=10, DEN=9
// Expected error: mismatched types — expected `false`, found `true`
use wasm4pm_compat::conformance::SimplicityConst;

fn main() {
    // 10/9 > 1.0: violates Between01 law
    let _: SimplicityConst<10, 9> = SimplicityConst::new();
}
