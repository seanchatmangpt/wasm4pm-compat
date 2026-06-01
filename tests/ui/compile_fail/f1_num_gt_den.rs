#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Between01 — F1Const<2,1> rejected because NUM > DEN (2/1 > 1)
// Law: Require<{ NUM <= DEN }>: IsTrue fails when NUM=2, DEN=1
// Expected error: mismatched types — expected `false`, found `true`
use wasm4pm_compat::conformance::F1Const;

fn main() {
    // 2/1 = 2.0 > 1.0: violates Between01 law
    let _: F1Const<2, 1> = F1Const::new();
}
