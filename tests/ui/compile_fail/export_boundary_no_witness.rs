// COMPILE-FAIL: Export boundary law — ExportBoundaryConst<false, true> lacks witness.
// Law: HasRoundTripFixture is only implemented for ExportBoundaryConst<true, true>.
// An export boundary without a witness cannot satisfy the round-trip fixture law.
use wasm4pm_compat::law::{ExportBoundaryConst, enforce_export_round_trip};

fn main() {
    // has_witness=false, has_round_trip=true: still missing the witness requirement.
    enforce_export_round_trip(&ExportBoundaryConst::<false, true>);
}
