// COMPILE-PASS: ExportBoundaryConst<false, false> — structural instantiation
// proves all four combinations of the const bool parameters are constructible.
//
// Law: ExportBoundaryConst<HAS_WITNESS, HAS_ROUND_TRIP> is a plain struct; any
// combination of bools compiles as a type. Only <true, true> satisfies
// HasRoundTripFixture; the others are lawful structs but cannot pass
// enforce_export_round_trip. This fixture proves the FALSE/FALSE form compiles
// as a value — the type-law gate (HasRoundTripFixture) is enforced separately
// by the compile-fail fixture.
use wasm4pm_compat::law::ExportBoundaryConst;

fn main() {
    // All four const-bool combinations are constructible as values.
    let _ff: ExportBoundaryConst<false, false> = ExportBoundaryConst;
    let _ft: ExportBoundaryConst<false, true> = ExportBoundaryConst;
    let _tf: ExportBoundaryConst<true, false> = ExportBoundaryConst;
    let _tt: ExportBoundaryConst<true, true> = ExportBoundaryConst;

    // The FALSE/FALSE form names no witness and no fixture — it is an honest
    // declaration of an unclaimed boundary. It must not compile through
    // enforce_export_round_trip (that is the compile-fail receipt).
    // This fixture proves the struct itself is well-formed.
    let _ = _ff;
    let _ = _ft;
    let _ = _tf;
}
