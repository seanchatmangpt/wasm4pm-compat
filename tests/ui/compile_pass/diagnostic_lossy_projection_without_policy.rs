// COMPILE-PASS: CompatDiagnostic::LossyProjectionWithoutPolicy — proves the
// variant is constructible and used as a verdict for ungoverned projections.
//
// Law: LossyProjectionWithoutPolicyLaw — any lossy projection must be governed
// by a LossPolicy; a projection that drops evidence without a named policy is
// refused with this diagnostic.
use wasm4pm_compat::diagnostic::CompatDiagnostic;

fn audit_loss_policy(has_policy: bool) -> Option<CompatDiagnostic> {
    if !has_policy {
        Some(CompatDiagnostic::LossyProjectionWithoutPolicy)
    } else {
        None
    }
}

fn main() {
    let diag = audit_loss_policy(false);
    assert_eq!(diag, Some(CompatDiagnostic::LossyProjectionWithoutPolicy));

    let clean = audit_loss_policy(true);
    assert!(clean.is_none());

    // Debug is implemented.
    let _ = format!("{:?}", CompatDiagnostic::LossyProjectionWithoutPolicy);
}
