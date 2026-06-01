#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Metric bounds law — PrecisionConst<NUM,DEN> requires NUM <= DEN.
// Law: Between01 bound; 5/3 > 1 violates the [0,1] range at the type level.
use wasm4pm_compat::conformance::PrecisionConst;

fn main() {
    // 5/3 > 1.0: out of range for a precision score.
    let _: PrecisionConst<5, 3> = PrecisionConst::new();
}
