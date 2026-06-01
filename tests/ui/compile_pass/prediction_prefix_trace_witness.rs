// COMPILE-PASS: PrefixTrace witness — proves PredictionProblem<PrefixTrace> is
// lawfully constructible to mark the input shape (an observed prefix).
//
// Law: prediction problem shape — PrefixTrace is the phantom type witness that
// marks the problem's input kind (a case prefix observed so far) rather than
// the prediction target.

use wasm4pm_compat::prediction::{PredictionProblem, PredictionTarget, PrefixTrace};

fn accepts_prefix_trace(_p: PredictionProblem<PrefixTrace>) {}

fn main() {
    let p = PredictionProblem::<PrefixTrace>::new(
        vec!["register".into(), "validate".into()],
        PredictionTarget::NextActivity,
    );
    assert_eq!(p.prefix_len(), 2);
    assert_eq!(p.horizon, None);

    let with_horizon = PredictionProblem::<PrefixTrace>::new(
        vec!["start".into()],
        PredictionTarget::RemainingTime,
    )
    .with_horizon(20);
    assert_eq!(with_horizon.horizon, Some(20));

    accepts_prefix_trace(p);
}
