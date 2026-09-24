use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

#[test]
fn certification_claim_requires_grounded_mapping_attestation() {
    let mut boundary = ProcessBoundary::fully_attested(
        ProcessBoundaryKind::ClaimsCertificationCoverage,
        "certification-claim",
    );
    boundary.has_certification_mapping = false;
    assert_eq!(
        boundary.check(),
        Err(vec![StrictViolation::MissingCertificationMapping])
    );
}
