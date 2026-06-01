// Law: NextActivityTargetLaw — PredictionProblem<NextActivity> is lawfully constructible; NextActivity is a distinct prediction target witness from RemainingTime and RiskScore
// COMPILE-PASS: NextActivity target law — proves PredictionProblem<NextActivity> is lawfully constructible
// with the NextActivity witness and PredictionTarget::NextActivity variant.

use wasm4pm_compat::prediction::{NextActivity, PredictionProblem, PredictionTarget};

fn check() {
    let problem = PredictionProblem::<NextActivity>::new(
        vec!["register".into(), "review".into()],
        PredictionTarget::NextActivity,
    );
    assert_eq!(problem.target, PredictionTarget::NextActivity);
    assert_eq!(problem.prefix_len(), 2);
    assert_eq!(problem.horizon, None);

    // Builder: finite look-ahead horizon is lawful.
    let with_horizon = PredictionProblem::<NextActivity>::new(
        vec!["start".into()],
        PredictionTarget::NextActivity,
    )
    .with_horizon(5);
    assert_eq!(with_horizon.horizon, Some(5));
}

fn main() {
    check();
}
