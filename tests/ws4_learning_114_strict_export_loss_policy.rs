use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

#[test]
fn export_boundary_requires_loss_policy() {
    let mut boundary =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::ExportsFormat, "xes-out");
    boundary.has_loss_policy = false;
    assert_eq!(
        boundary.check(),
        Err(vec![StrictViolation::MissingLossPolicy])
    );
}
