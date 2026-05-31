// COMPILE-PASS: Strict export boundary — an ExportBoundaryConst with both
// witness and round-trip fixture compiles and satisfies HasRoundTripFixture.
//
// Law: strict boundary covenant — ExportBoundaryConst<true, true> is the
// only instantiation that satisfies HasRoundTripFixture. This is the
// complement to the compile-fail fixture (strict_claim_no_fixture.rs) which
// proves ExportBoundaryConst<true, false> is rejected.
//
// The pair (pass + fail) constitutes the type-law receipt for the strict
// export boundary: the lawful path is open, the unlawful path is sealed.
use wasm4pm_compat::law::{ExportBoundaryConst, enforce_export_round_trip};

fn main() {
    // has_witness=true, has_round_trip=true: fully attested — must compile.
    enforce_export_round_trip(&ExportBoundaryConst::<true, true>);

    // The type is distinct from the failing form: ExportBoundaryConst<true, false>
    // does NOT satisfy HasRoundTripFixture; ExportBoundaryConst<true, true> does.
    let _boundary: ExportBoundaryConst<true, true> = ExportBoundaryConst;
}
