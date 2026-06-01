// COMPILE-FAIL: ExportBoundaryConst<true,false> contradicts the boundary covenant.
//
// Law: MissingRoundTripFixture — the boundary covenant requires that any export
// claiming a witness (`HAS_WITNESS = true`) must also declare a round-trip fixture
// (`HAS_ROUND_TRIP = true`). Claiming a witness while hiding the round-trip fixture
// is a structural contradiction that the type law seals shut.
//
// This fixture proves the contradiction at a different gate than strict_export_no_round_trip:
// it passes ExportBoundaryConst<true, false> where ExportBoundaryConst<true, true>
// is expected — a direct type mismatch, not just a missing trait. The compiler
// sees a concrete type error: expected `ExportBoundaryConst<true, true>`, found
// `ExportBoundaryConst<true, false>`.
//
// Expected error: E0308 — mismatched types: ExportBoundaryConst<true, false> is
// not ExportBoundaryConst<true, true>.
use wasm4pm_compat::law::ExportBoundaryConst;

/// A gate that demands a fully-attested export boundary.
fn accept_attested_boundary(_: &ExportBoundaryConst<true, true>) {}

fn main() {
    // Attempting to pass ExportBoundaryConst<true, false> (witness-true, round-trip-false)
    // where the gate requires ExportBoundaryConst<true, true>.
    // This contradicts the boundary covenant: the round-trip fixture is not optional
    // when a witness is claimed.
    accept_attested_boundary(&ExportBoundaryConst::<true, false>);
}
