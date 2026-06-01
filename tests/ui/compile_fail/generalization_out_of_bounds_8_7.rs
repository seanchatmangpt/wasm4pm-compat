#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Metric bounds law — GeneralizationConst<8,7> violates NUM <= DEN.
// Law: Between01 bound; 8/7 > 1 is an illegal generalization score at the type level.
use wasm4pm_compat::conformance::GeneralizationConst;

fn main() {
    // 8/7 > 1.0: illegal generalization score.
    let _: GeneralizationConst<8, 7> = GeneralizationConst::new();
}
