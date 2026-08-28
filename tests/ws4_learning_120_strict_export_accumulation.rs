use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

#[test]
fn export_boundary_accumulates_independent_contract_violations() {
    let mut boundary = ProcessBoundary::fully_attested(ProcessBoundaryKind::ExportsFormat, "bad-export");
    boundary.exports_raw_evidence = true;
    boundary.has_witness = false;
    boundary.has_round_trip_fixture = false;
    boundary.has_loss_policy = false;
    boundary.has_refusal_path = false;

    assert_eq!(
        boundary.check(),
        Err(vec![
            StrictViolation::RawEvidenceExported,
            StrictViolation::MissingWitness,
            StrictViolation::MissingRoundTripFixture,
            StrictViolation::MissingLossPolicy,
            StrictViolation::MissingRefusalPath,
        ])
    );
}
