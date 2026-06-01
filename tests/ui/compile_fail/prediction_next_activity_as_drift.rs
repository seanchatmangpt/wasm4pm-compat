// COMPILE-FAIL: Prediction target law — PredictionProblem<NextActivity> cannot be passed
// where PredictionProblem<DriftSignal> is required.
// Law: NextActivity and DriftSignal are distinct witness types that make
// PredictionProblem structurally incompatible between target families.
use wasm4pm_compat::prediction::{DriftSignal, NextActivity, PredictionProblem, PredictionTarget};

fn requires_drift_problem(_p: PredictionProblem<DriftSignal>) {}

fn main() {
    let next: PredictionProblem<NextActivity> =
        PredictionProblem::new(vec!["a".to_string()], PredictionTarget::NextActivity);
    // This must fail: PredictionProblem<NextActivity> is not PredictionProblem<DriftSignal>.
    requires_drift_problem(next);
}
