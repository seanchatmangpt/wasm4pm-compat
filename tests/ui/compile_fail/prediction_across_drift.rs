#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features, dead_code)]

use wasm4pm_compat::parity::delta::enforce_prediction_horizon_before_drift;

fn main() {
    // Horizon steps (12) > Change point (10), violating HORIZON_STEPS <= CHANGE_POINT (should fail compilation)
    enforce_prediction_horizon_before_drift::<12, 10>();
}
