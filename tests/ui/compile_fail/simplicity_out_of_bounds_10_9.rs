#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Metric bounds law — SimplicityConst<10,9> violates NUM <= DEN.
// Law: Between01 bound; 10/9 > 1 is an illegal simplicity score at the type level.
use wasm4pm_compat::conformance::SimplicityConst;

fn main() {
    // 10/9 > 1.0: illegal simplicity score.
    let _: SimplicityConst<10, 9> = SimplicityConst::new();
}
