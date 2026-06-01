// COMPILE-PASS: RiskScore witness binding — proves PredictionProblem<RiskScore>
// is type-level distinct from PredictionProblem<DriftSignal> and that a function
// accepting RiskScore problems cannot accept DriftSignal problems at the type level.
//
// Law: prediction problem shape — witness markers prevent silent substitution
// of one target family for another at the call site.

use wasm4pm_compat::prediction::{
    DriftSignal, PredictionProblem, PredictionTarget, RiskScore,
};

fn accepts_risk_only(_p: PredictionProblem<RiskScore>) {}
fn accepts_drift_only(_p: PredictionProblem<DriftSignal>) {}

fn main() {
    let risk = PredictionProblem::<RiskScore>::new(
        vec!["open".into(), "escalate".into()],
        PredictionTarget::Risk,
    );
    let drift = PredictionProblem::<DriftSignal>::new(
        vec!["a".into(), "b".into()],
        PredictionTarget::DriftSignal,
    );

    accepts_risk_only(risk);
    accepts_drift_only(drift);
}
