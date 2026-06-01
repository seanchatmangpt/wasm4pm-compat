#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Between01 — GeneralizationConst<8,7> rejected because NUM > DEN (8/7 > 1)
// Law: Require<{ NUM <= DEN }>: IsTrue fails when NUM=8, DEN=7
// Expected error: mismatched types — expected `false`, found `true`
use wasm4pm_compat::conformance::GeneralizationConst;

fn main() {
    // 8/7 > 1.0: violates Between01 law
    let _: GeneralizationConst<8, 7> = GeneralizationConst::new();
}
