// COMPILE-PASS: OutcomeLabel witness — proves PredictionProblem<OutcomeLabel>
// is lawfully constructible and is structurally distinct from ComplianceTarget.
//
// Law: prediction problem shape — OutcomeLabel is the phantom type witness for
// categorical case-outcome prediction; it is structurally distinct from
// ComplianceTarget even though both are binary classification targets.

use wasm4pm_compat::prediction::{OutcomeLabel, PredictionProblem, PredictionTarget};

fn accepts_outcome(_p: PredictionProblem<OutcomeLabel>) {}

fn main() {
    let p = PredictionProblem::<OutcomeLabel>::new(
        vec!["submit".into(), "approve".into()],
        PredictionTarget::OutcomeLabel,
    );
    assert_eq!(p.target, PredictionTarget::OutcomeLabel);
    assert_eq!(p.prefix_len(), 2);

    accepts_outcome(p);
}
