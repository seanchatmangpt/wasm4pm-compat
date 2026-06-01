// COMPILE-PASS: HasRoundTripFixture bound — only ExportBoundaryConst<true, true>
// satisfies the sealed HasRoundTripFixture bound and can pass the type-law gate.
//
// Law: strict export boundary covenant — a boundary that declares both a witness
// (HAS_WITNESS=true) and a round-trip fixture (HAS_ROUND_TRIP=true) is the only
// instantiation that can be handed to enforce_export_round_trip. The sealed trait
// makes this a compile-time law: no other combination compiles through the gate.
//
// This fixture proves the lawful path (true, true) is open, as the complement
// to the compile-fail fixture that proves (true, false) is sealed.
use wasm4pm_compat::law::{ExportBoundaryConst, HasRoundTripFixture, enforce_export_round_trip};

/// A function requiring the HasRoundTripFixture bound — proves the bound is
/// satisfiable only by ExportBoundaryConst<true, true>.
fn require_fixture<B: HasRoundTripFixture>(_boundary: &B) {
    // The bound is satisfied — the boundary has both witness and fixture.
}

fn main() {
    // Only <true, true> satisfies HasRoundTripFixture.
    let full = ExportBoundaryConst::<true, true>;
    require_fixture(&full);
    enforce_export_round_trip(&full);

    // Calling with an explicit type annotation — proves the type is distinct.
    let typed: ExportBoundaryConst<true, true> = ExportBoundaryConst;
    enforce_export_round_trip(&typed);
}
