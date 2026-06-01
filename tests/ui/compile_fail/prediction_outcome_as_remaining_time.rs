// COMPILE-FAIL: Prediction target law — PredictionProblem<OutcomeLabel> cannot be passed
// where PredictionProblem<RemainingTime> is required.
// Law: The type parameter T in PredictionProblem<T> makes outcome-label problems
// and remaining-time problems structurally distinct. Wrong-target confusion is a compile error.
use wasm4pm_compat::prediction::{OutcomeLabel, PredictionProblem, PredictionTarget, RemainingTime};

fn requires_remaining_time_problem(_p: PredictionProblem<RemainingTime>) {}

fn main() {
    let outcome: PredictionProblem<OutcomeLabel> =
        PredictionProblem::new(vec!["a".to_string()], PredictionTarget::OutcomeLabel);
    // This must fail: PredictionProblem<OutcomeLabel> is not PredictionProblem<RemainingTime>.
    requires_remaining_time_problem(outcome);
}
