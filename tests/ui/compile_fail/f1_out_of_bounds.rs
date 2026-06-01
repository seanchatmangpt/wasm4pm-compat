#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Metric bounds law — F1Const<NUM,DEN> requires NUM <= DEN.
// Law: Between01 bound; 2/1 > 1 violates the [0,1] range at the type level.
use wasm4pm_compat::conformance::F1Const;

fn main() {
    // 2/1 = 2.0 > 1.0: out of range for an F1 score.
    let _: F1Const<2, 1> = F1Const::new();
}
