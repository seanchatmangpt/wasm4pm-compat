// Law: RiskScoreTargetLaw — PredictionProblem<RiskScore> is lawfully constructible; RiskScore is a distinct prediction target witness from NextActivity and RemainingTime
// COMPILE-PASS: Risk target law — proves PredictionProblem<RiskScore> is lawfully constructible
// with the RiskScore witness and PredictionTarget::Risk variant.

use wasm4pm_compat::prediction::{PredictionProblem, PredictionTarget, RiskScore};

fn check() {
    let problem = PredictionProblem::<RiskScore>::new(
        vec!["open".into(), "escalate".into()],
        PredictionTarget::Risk,
    );
    assert_eq!(problem.target, PredictionTarget::Risk);
    assert_eq!(problem.prefix_len(), 2);
    assert_eq!(problem.horizon, None);

    // Risk problems may also carry a finite horizon.
    let with_horizon =
        PredictionProblem::<RiskScore>::new(vec!["trigger".into()], PredictionTarget::Risk)
            .with_horizon(1);
    assert_eq!(with_horizon.horizon, Some(1));
}

fn main() {
    check();
}
