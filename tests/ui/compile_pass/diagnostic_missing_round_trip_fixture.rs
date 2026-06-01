// COMPILE-PASS: CompatDiagnostic::MissingRoundTripFixture — proves the variant
// is constructible and used as a verdict for round-trip claims without fixtures.
//
// Law: MissingRoundTripFixtureLaw — a round-trip claim (import then export) must
// be backed by a fixture that actually proves it round-trips; a claim without
// a fixture is not a real claim.
use wasm4pm_compat::diagnostic::CompatDiagnostic;

fn audit_round_trip(has_fixture: bool) -> Vec<CompatDiagnostic> {
    let mut diags = vec![];
    if !has_fixture {
        diags.push(CompatDiagnostic::MissingRoundTripFixture);
    }
    diags
}

fn main() {
    let diags = audit_round_trip(false);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0], CompatDiagnostic::MissingRoundTripFixture);

    // The variant is Copy.
    let d = CompatDiagnostic::MissingRoundTripFixture;
    let d2 = d;
    assert_eq!(d, d2);
}
