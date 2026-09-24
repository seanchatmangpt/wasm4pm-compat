use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

#[test]
fn import_boundary_requires_round_trip_fixture() {
    let mut boundary =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::ImportsFormat, "ocel-in");
    boundary.has_round_trip_fixture = false;
    assert_eq!(
        boundary.check(),
        Err(vec![StrictViolation::MissingRoundTripFixture])
    );
}
