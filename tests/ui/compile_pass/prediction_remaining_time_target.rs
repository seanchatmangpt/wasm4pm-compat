// COMPILE-PASS: RemainingTime target law — proves PredictionProblem<RemainingTime> is lawfully constructible
// with the RemainingTime witness and PredictionTarget::RemainingTime variant.

use wasm4pm_compat::prediction::{PredictionProblem, PredictionTarget, RemainingTime};

fn check() {
    let problem = PredictionProblem::<RemainingTime>::new(
        vec!["submit".into(), "validate".into(), "approve".into()],
        PredictionTarget::RemainingTime,
    );
    assert_eq!(problem.target, PredictionTarget::RemainingTime);
    assert_eq!(problem.prefix_len(), 3);
    assert_eq!(problem.horizon, None);

    // A finite horizon is lawful for remaining-time problems too.
    let bounded = PredictionProblem::<RemainingTime>::new(
        vec!["start".into()],
        PredictionTarget::RemainingTime,
    )
    .with_horizon(10);
    assert_eq!(bounded.horizon, Some(10));
}

fn main() {
    check();
}
