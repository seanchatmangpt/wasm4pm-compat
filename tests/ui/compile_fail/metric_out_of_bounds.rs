#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
// Law: MetricBoundsLaw — FitnessConst<NUM,DEN> requires NUM <= DEN; a metric above 1.0 violates Between01 at compile time

// COMPILE-FAIL: Metric bounds law — fitness/precision/F1 must be in [0, 1].
// Law: Between01<NUM, DEN> requires NUM <= DEN; 2/1 > 1 violates this.
// Expected error: the where-bound Require<{ 2 <= 1 }>: IsTrue is not satisfied.
use wasm4pm_compat::conformance::FitnessConst;

fn main() {
    // 2/1 = 2.0 > 1.0: out of range.
    let _: FitnessConst<2, 1> = FitnessConst::new();
}
