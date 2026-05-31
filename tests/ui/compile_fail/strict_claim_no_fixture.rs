// COMPILE-FAIL: Strict boundary law — export without round-trip fixture.
// Law: HasRoundTripFixture is only implemented for ExportBoundaryConst<true, true>.
// Expected error: ExportBoundaryConst<true, false> does not implement HasRoundTripFixture.
use wasm4pm_compat::law::{ExportBoundaryConst, enforce_export_round_trip};

fn main() {
    // has_witness=true, has_round_trip=false: missing round-trip fixture.
    enforce_export_round_trip(&ExportBoundaryConst::<true, false>);
}
