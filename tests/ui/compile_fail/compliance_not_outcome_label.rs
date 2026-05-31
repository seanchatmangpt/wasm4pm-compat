// COMPILE-FAIL: Compliance target law — PredictionProblem<ComplianceTarget>
// cannot be passed where PredictionProblem<OutcomeLabel> is required.
//
// Law: De Santis et al. (2026) — compliance-aware PPM introduces a distinct
// prediction target family. A compliance-constrained prediction problem
// carries a different structural obligation than a categorical outcome label.
// Confusing them at a function boundary must be a compile error.
//
// Expected error: mismatched types — PredictionProblem<ComplianceTarget> is
// not PredictionProblem<OutcomeLabel>.
use wasm4pm_compat::prediction::{
    ComplianceTarget, OutcomeLabel, PredictionProblem, PredictionTarget,
};

fn accepts_outcome_label(_p: PredictionProblem<OutcomeLabel>) {}

fn main() {
    let compliance_problem = PredictionProblem::<ComplianceTarget>::new(
        vec!["register".into(), "review".into()],
        PredictionTarget::ComplianceConstraint,
    );
    // ComplianceTarget ≠ OutcomeLabel — must fail at compile time.
    accepts_outcome_label(compliance_problem);
}
