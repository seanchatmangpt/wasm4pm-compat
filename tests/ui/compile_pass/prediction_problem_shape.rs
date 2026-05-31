// COMPILE-PASS: PredictionProblem<T> constructs with a PredictionTarget and a
// PredictionRefusal variant is nameable — covers prediction module beyond
// ComplianceTarget.
//
// Law: prediction problem shape — a problem statement names a prefix and a
// target; it never predicts. PredictionRefusal names specific structural laws.

use wasm4pm_compat::prediction::{
    ComplianceTarget, DriftSignal, OutcomeLabel, PredictionProblem, PredictionRefusal,
    PredictionTarget, PrefixTrace, RiskScore,
};

fn check_all_targets_constructible() {
    // All PredictionTarget variants are constructible.
    let targets = [
        PredictionTarget::NextActivity,
        PredictionTarget::OutcomeLabel,
        PredictionTarget::RemainingTime,
        PredictionTarget::DriftSignal,
        PredictionTarget::Risk,
        PredictionTarget::ComplianceConstraint,
    ];
    assert_eq!(targets.len(), 6);
}

fn check_prediction_problem_witnesses() {
    // PredictionProblem with various witness markers.
    let outcome = PredictionProblem::<OutcomeLabel>::new(
        vec!["submit".into(), "approve".into()],
        PredictionTarget::OutcomeLabel,
    );
    assert_eq!(outcome.target, PredictionTarget::OutcomeLabel);
    assert_eq!(outcome.prefix_len(), 2);
    assert_eq!(outcome.horizon, None);

    let drift = PredictionProblem::<DriftSignal>::new(
        vec!["a".into(), "b".into(), "c".into()],
        PredictionTarget::DriftSignal,
    );
    assert_eq!(drift.prefix_len(), 3);

    let risk = PredictionProblem::<RiskScore>::new(
        vec!["start".into()],
        PredictionTarget::Risk,
    )
    .with_horizon(5);
    assert_eq!(risk.horizon, Some(5));

    let compliance = PredictionProblem::<ComplianceTarget>::new(
        vec!["register".into()],
        PredictionTarget::ComplianceConstraint,
    );
    assert_eq!(compliance.target, PredictionTarget::ComplianceConstraint);

    let prefix = PredictionProblem::<PrefixTrace>::new(
        vec!["x".into()],
        PredictionTarget::NextActivity,
    );
    assert_eq!(prefix.prefix_len(), 1);
}

fn check_refusal_variants_nameable() {
    // All PredictionRefusal variants are constructible and name their law.
    let laws = [
        PredictionRefusal::MissingPrefix,
        PredictionRefusal::MissingTarget,
        PredictionRefusal::EmptyPrefix,
        PredictionRefusal::TargetUnsupported,
        PredictionRefusal::NonPrefixTrace,
        PredictionRefusal::ConstraintNotNamed,
    ];
    assert_eq!(laws.len(), 6);

    // Display format names the law.
    let s = format!("{}", PredictionRefusal::ConstraintNotNamed);
    assert!(s.contains("ConstraintNotNamed"));
}

fn main() {
    check_all_targets_constructible();
    check_prediction_problem_witnesses();
    check_refusal_variants_nameable();
}
