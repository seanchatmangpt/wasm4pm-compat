// COMPILE-FAIL: ComplianceConstraintWitness binding rejects wrong constraint type.
//
// Law: De Santis et al. (2026) — a function that accepts a
// PredictionProblem<ComplianceTarget> (compliance-constrained slot) must reject
// any non-compliance witness at compile time. Here, an OutcomeLabel-witnessed
// problem is passed into a compliance-expecting binding.
//
// The type law: ComplianceTarget ≠ OutcomeLabel as phantom parameters on
// PredictionProblem<T>. Supplying the wrong witness in a compliance-expecting
// slot must be a compile error — the binding rejects the wrong constraint type.
//
// Expected error: mismatched types — PredictionProblem<OutcomeLabel> is not
// PredictionProblem<ComplianceTarget>.

use wasm4pm_compat::prediction::{
    ComplianceTarget, OutcomeLabel, PredictionProblem, PredictionTarget,
};

/// Simulates a compliance-constrained monitor slot.
/// Only a PredictionProblem<ComplianceTarget> is admissible here.
fn accepts_compliance_only(_p: PredictionProblem<ComplianceTarget>) {}

fn main() {
    let outcome_problem = PredictionProblem::<OutcomeLabel>::new(
        vec!["register".into(), "approve".into()],
        PredictionTarget::OutcomeLabel,
    );
    // OutcomeLabel ≠ ComplianceTarget — supplying the wrong witness in a
    // compliance-constrained slot must fail at compile time.
    accepts_compliance_only(outcome_problem);
}
