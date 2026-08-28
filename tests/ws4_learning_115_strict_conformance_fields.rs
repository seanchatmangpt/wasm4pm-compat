use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

#[test]
fn conformance_claim_requires_conformance_fields() {
    let mut boundary = ProcessBoundary::fully_attested(
        ProcessBoundaryKind::ClaimsConformance,
        "conformance-claim",
    );
    boundary.has_conformance_fields = false;
    assert_eq!(
        boundary.check(),
        Err(vec![StrictViolation::MissingConformanceFields])
    );
}
