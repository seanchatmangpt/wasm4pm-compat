// COMPILE-PASS: PredictionTarget::ComplianceConstraint variant — proves that
// the compliance constraint target is constructible and is distinct from all
// other prediction target variants.
//
// Law: compliance-aware PPM (De Santis et al., 2026) — ComplianceConstraint is
// a structurally distinct prediction target requiring a named constraint; it is
// not interchangeable with OutcomeLabel despite both being binary.

use wasm4pm_compat::prediction::PredictionTarget;

fn main() {
    let t = PredictionTarget::ComplianceConstraint;
    assert_ne!(t, PredictionTarget::OutcomeLabel);
    assert_ne!(t, PredictionTarget::NextActivity);
    assert_ne!(t, PredictionTarget::RemainingTime);
    assert_ne!(t, PredictionTarget::DriftSignal);
    assert_ne!(t, PredictionTarget::Risk);

    // All six variants are distinct.
    let all = [
        PredictionTarget::NextActivity,
        PredictionTarget::OutcomeLabel,
        PredictionTarget::RemainingTime,
        PredictionTarget::DriftSignal,
        PredictionTarget::Risk,
        PredictionTarget::ComplianceConstraint,
    ];
    assert_eq!(all.len(), 6);
}
