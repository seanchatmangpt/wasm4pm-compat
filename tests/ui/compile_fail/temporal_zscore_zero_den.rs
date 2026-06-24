#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features, dead_code)]

use wasm4pm_compat::temporal::ZScoreConst;

fn main() {
    // DEN is 0 -> violates DEN > 0 (should fail compilation)
    let _ = ZScoreConst::<5, 0>::new();
}
