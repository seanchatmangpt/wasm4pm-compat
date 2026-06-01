#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// Law: SimplicityBoundsLaw — SimplicityConst<NUM,DEN> requires NUM <= DEN; simplicity scores above 1.0 violate Between01 at compile time

// COMPILE-FAIL: SimplicityConst bounds law — simplicity must be in [0, 1].
// Law: Between01<NUM, DEN> requires NUM <= DEN; 3/2 > 1 violates this.
// Expected error: the where-bound Require<{ 3 <= 2 }>: IsTrue is not satisfied.
use wasm4pm_compat::conformance::SimplicityConst;

fn main() {
    // 3/2 = 1.5 > 1.0: out of range.
    let _: SimplicityConst<3, 2> = SimplicityConst::new();
}
