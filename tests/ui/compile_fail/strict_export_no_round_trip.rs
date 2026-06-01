// COMPILE-FAIL: StrictExportWithoutRoundTrip — ExportBoundaryConst<true,false> rejected.
//
// Law: MissingRoundTripFixture — an export boundary that declares a witness
// but no round-trip fixture violates the strict boundary covenant.
// `enforce_export_round_trip` requires `B: HasRoundTripFixture`, which is
// only implemented for `ExportBoundaryConst<true, true>`.
//
// `ExportBoundaryConst<true, false>` attests: "I have a witness but I lack a
// round-trip fixture." The type law rejects this: a strict export boundary must
// carry both. The absence of `HasRoundTripFixture` on `<true, false>` is the
// named type receipt for the MissingRoundTripFixture law.
//
// Expected error: E0277 — ExportBoundaryConst<true, false> does not implement
// HasRoundTripFixture (trait bound not satisfied).
use wasm4pm_compat::law::{ExportBoundaryConst, enforce_export_round_trip};

fn strict_export_boundary_check<B: wasm4pm_compat::law::HasRoundTripFixture>(_b: &B) {
    enforce_export_round_trip(_b);
}

fn main() {
    // has_witness=true, has_round_trip=false:
    // This export boundary claims a witness but lacks the round-trip fixture.
    // The strict covenant rejects it: MissingRoundTripFixture law is violated.
    strict_export_boundary_check(&ExportBoundaryConst::<true, false>);
}
