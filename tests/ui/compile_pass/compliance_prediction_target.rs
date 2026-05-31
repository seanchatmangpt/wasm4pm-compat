// Compile-pass fixture: PredictionProblem<ComplianceTarget> with
// PredictionTarget::ComplianceConstraint can be constructed.
//
// Law: De Santis et al. (2026) — compliance-aware PPM introduces a distinct
// prediction target shape: "does this prefix comply with named constraint C?"
// A ComplianceTarget-witnessed problem is structurally distinct from an
// OutcomeLabel-witnessed problem.

use wasm4pm_compat::prediction::{
    ComplianceTarget, PredictionProblem, PredictionTarget,
};

fn main() {
    // A compliance-aware prediction problem: does this prefix satisfy rule C?
    let problem = PredictionProblem::<ComplianceTarget>::new(
        vec!["register".into(), "review".into()],
        PredictionTarget::ComplianceConstraint,
    );

    assert_eq!(problem.target, PredictionTarget::ComplianceConstraint);
    assert_eq!(problem.prefix_len(), 2);
    assert_eq!(problem.horizon, None);

    // ComplianceConstraint is a distinct variant from OutcomeLabel.
    assert_ne!(problem.target, PredictionTarget::OutcomeLabel);

    // The witness type distinguishes ComplianceTarget from OutcomeLabel
    // at the call site — this is the structural law being proved.
    fn accepts_compliance(_p: PredictionProblem<ComplianceTarget>) {}
    accepts_compliance(problem);

    // With_horizon works on compliance problems too.
    let bounded = PredictionProblem::<ComplianceTarget>::new(
        vec!["submit".into()],
        PredictionTarget::ComplianceConstraint,
    )
    .with_horizon(10);
    assert_eq!(bounded.horizon, Some(10));
}
