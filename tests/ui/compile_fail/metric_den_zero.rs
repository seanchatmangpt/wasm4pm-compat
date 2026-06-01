#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Between01 — FitnessConst<1,0> rejected because DEN=0 violates Require<{ DEN > 0 }>
// Law: Require<{ DEN > 0 }>: IsTrue fails when DEN=0 (division by zero in metric)
// Expected error: mismatched types — expected `false`, found `true`
use wasm4pm_compat::conformance::FitnessConst;

fn main() {
    // DEN=0: zero denominator violates Between01 law
    let _: FitnessConst<1, 0> = FitnessConst::new();
}
