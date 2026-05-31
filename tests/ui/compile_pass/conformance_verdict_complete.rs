#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// COMPILE-PASS: Metric bounds law — valid fractional scores compile.
// FitnessConst<3, 4> = 0.75 ∈ [0,1], PrecisionConst<1, 2> = 0.5 ∈ [0,1].
use wasm4pm_compat::conformance::{FitnessConst, PrecisionConst, F1Const};

fn main() {
    let f: FitnessConst<3, 4>    = FitnessConst::new();   // 0.75: lawful
    let p: PrecisionConst<1, 2>  = PrecisionConst::new(); // 0.5: lawful
    let f1: F1Const<0, 1>        = F1Const::new();        // 0.0: lawful (boundary)
    let f2: FitnessConst<1, 1>   = FitnessConst::new();   // 1.0: lawful (boundary)

    assert_eq!(f.num(), 3);
    assert_eq!(f.den(), 4);
    assert_eq!(p.num(), 1);
    assert_eq!(p.den(), 2);
    let _ = (f1, f2);
}
