// COMPILE-PASS: ProcessBoundary with witness attestation — a boundary that
// declares has_witness=true compiles and satisfies the witness obligation.
//
// Law: MissingWitness — any boundary that emits, imports, exports, or claims
// a receipt owes a type-level witness. Attesting has_witness=true is the
// structure-only declaration that a witness threads through the boundary.
// This fixture proves the attestation is constructible and checkable.
//
// Requires: --features strict
use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

fn main() {
    // A boundary with has_witness=true and all other obligations met passes check.
    let b = ProcessBoundary::fully_attested(ProcessBoundaryKind::ImportsFormat, "ocel-in");
    assert!(b.has_witness, "fully_attested sets has_witness=true");
    assert!(b.check().is_ok());

    // A boundary with has_witness=false (missing witness) names MissingWitness.
    let mut no_witness =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::ImportsFormat, "ocel-in");
    no_witness.has_witness = false;
    let violations = no_witness.check().unwrap_err();
    assert!(
        violations.contains(&StrictViolation::MissingWitness),
        "missing witness must be named MissingWitness"
    );

    // A boundary for EmitsObjectRelations also owes a witness.
    let mut no_witness_oc =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::EmitsObjectRelations, "oc-emitter");
    no_witness_oc.has_witness = false;
    let v2 = no_witness_oc.check().unwrap_err();
    assert!(v2.contains(&StrictViolation::MissingWitness));

    // Strict mode collects ALL violations, not just the first.
    let mut multi =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::ExportsFormat, "xes-out");
    multi.has_witness = false;
    multi.has_round_trip_fixture = false;
    multi.has_loss_policy = false;
    let v3 = multi.check().unwrap_err();
    assert!(v3.contains(&StrictViolation::MissingWitness));
    assert!(v3.contains(&StrictViolation::MissingRoundTripFixture));
    assert!(v3.contains(&StrictViolation::MissingLossPolicy));
    assert!(v3.len() >= 3);
}
