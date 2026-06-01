// COMPILE-PASS: ProcessBoundary has_round_trip_fixture flag — import/export
// boundaries with the flag set pass the round-trip obligation.
//
// Law: MissingRoundTripFixture — import and export boundaries must declare a
// named round-trip fixture. has_round_trip_fixture=true satisfies this; false
// triggers the violation. This fixture proves the flag field is constructible,
// readable, and checked by StrictCheck.
//
// Requires: --features strict
use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

fn main() {
    // Import boundary with round-trip fixture set: passes covenant.
    let with_fixture =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::ImportsFormat, "ocel-json-rt");
    assert!(with_fixture.has_round_trip_fixture);
    assert!(with_fixture.check().is_ok());

    // Import boundary without round-trip fixture: MissingRoundTripFixture.
    let mut no_fixture =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::ImportsFormat, "ocel-json");
    no_fixture.has_round_trip_fixture = false;
    let violations = no_fixture.check().unwrap_err();
    assert!(violations.contains(&StrictViolation::MissingRoundTripFixture));

    // Export boundary also requires a round-trip fixture.
    let mut no_rt_export =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::ExportsFormat, "xes-out");
    no_rt_export.has_round_trip_fixture = false;
    let v2 = no_rt_export.check().unwrap_err();
    assert!(v2.contains(&StrictViolation::MissingRoundTripFixture));

    // EmitsEvents does NOT owe a round-trip fixture (only import/export do).
    let mut emit_no_rt =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::EmitsEvents, "events");
    emit_no_rt.has_round_trip_fixture = false;
    // No MissingRoundTripFixture for EmitsEvents (but check may still find other issues).
    if let Err(vs) = emit_no_rt.check() {
        assert!(
            !vs.contains(&StrictViolation::MissingRoundTripFixture),
            "EmitsEvents must not trigger MissingRoundTripFixture"
        );
    }
}
