#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features, dead_code)]

use wasm4pm_compat::prediction::{
    enforce_admissible_horizon, PredictionHorizonConst, NextActivity, OutcomeLabel, RemainingTime, RiskScore, ComplianceTarget
};
use wasm4pm_compat::parity::delta::{
    enforce_prediction_horizon_before_drift, DriftWitness
};
use wasm4pm_compat::witness::Wasm4pmBridge;

fn main() {
    // NextActivity under Events(N)
    enforce_admissible_horizon::<NextActivity, { PredictionHorizonConst::Events(5) }>();

    // OutcomeLabel under FullCase
    enforce_admissible_horizon::<OutcomeLabel, { PredictionHorizonConst::FullCase }>();

    // RemainingTime under FullCase
    enforce_admissible_horizon::<RemainingTime, { PredictionHorizonConst::FullCase }>();

    // RemainingTime under TimeUnits(SECS)
    enforce_admissible_horizon::<RemainingTime, { PredictionHorizonConst::TimeUnits(3600) }>();

    // RiskScore under all three
    enforce_admissible_horizon::<RiskScore, { PredictionHorizonConst::Events(3) }>();
    enforce_admissible_horizon::<RiskScore, { PredictionHorizonConst::FullCase }>();
    enforce_admissible_horizon::<RiskScore, { PredictionHorizonConst::TimeUnits(60) }>();

    // ComplianceTarget under all three
    enforce_admissible_horizon::<ComplianceTarget, { PredictionHorizonConst::Events(10) }>();
    enforce_admissible_horizon::<ComplianceTarget, { PredictionHorizonConst::FullCase }>();
    enforce_admissible_horizon::<ComplianceTarget, { PredictionHorizonConst::TimeUnits(86400) }>();

    // Drift witness construction is lawful with alpha in [0,1]
    let _drift = DriftWitness::<1, 10, 5, Wasm4pmBridge>::new(5);

    // Horizon steps (5) <= Change point (10)
    enforce_prediction_horizon_before_drift::<5, 10>();
    enforce_prediction_horizon_before_drift::<10, 10>();
}
