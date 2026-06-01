#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// Law: GeneralizationBoundsLaw — GeneralizationConst<NUM,DEN> requires NUM <= DEN; scores above 1.0 are rejected at compile time

// COMPILE-FAIL: GeneralizationConst bounds law — generalization must be in [0, 1].
// Law: Between01<NUM, DEN> requires NUM <= DEN; 2/1 > 1 violates this.
// Expected error: the where-bound Require<{ 2 <= 1 }>: IsTrue is not satisfied.
use wasm4pm_compat::conformance::GeneralizationConst;

fn main() {
    // 2/1 = 2.0 > 1.0: out of range.
    let _: GeneralizationConst<2, 1> = GeneralizationConst::new();
}
