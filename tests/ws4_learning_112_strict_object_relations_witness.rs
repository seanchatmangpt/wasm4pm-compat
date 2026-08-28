use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

#[test]
fn object_relation_emission_boundary_requires_witness() {
    let mut boundary = ProcessBoundary::fully_attested(
        ProcessBoundaryKind::EmitsObjectRelations,
        "relations-out",
    );
    boundary.has_witness = false;
    assert_eq!(boundary.check(), Err(vec![StrictViolation::MissingWitness]));
}
