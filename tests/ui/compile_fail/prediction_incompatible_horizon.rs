#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features, dead_code)]

use wasm4pm_compat::prediction::{
    enforce_admissible_horizon, PredictionHorizonConst, NextActivity
};

fn main() {
    // NextActivity is NOT admissible under FullCase (should fail compilation)
    enforce_admissible_horizon::<NextActivity, { PredictionHorizonConst::FullCase }>();
}
