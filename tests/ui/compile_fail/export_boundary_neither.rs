// COMPILE-FAIL: Export boundary law — ExportBoundaryConst<false, false> satisfies no law.
// Law: HasRoundTripFixture requires both HAS_WITNESS=true and HAS_ROUND_TRIP=true.
// A bare export with no witness and no round-trip fixture is the most defective declaration.
use wasm4pm_compat::law::{ExportBoundaryConst, enforce_export_round_trip};

fn main() {
    // has_witness=false, has_round_trip=false: missing both.
    enforce_export_round_trip(&ExportBoundaryConst::<false, false>);
}
