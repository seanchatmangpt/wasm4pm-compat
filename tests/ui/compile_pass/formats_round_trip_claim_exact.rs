// COMPILE-PASS: RoundTripClaim::exact — proves an exact (lossless) round-trip
// claim constructs with allows_lossy = false and is_named() reflects fixture presence.
//
// Law: RoundTripClaimExactLaw — an exact round-trip claim asserts byte-or-shape-
// exact fidelity; it must name the fixture under which the claim is discharged.
use wasm4pm_compat::formats::{FormatKind, RoundTripClaim};

fn main() {
    let claim = RoundTripClaim::exact(FormatKind::OcelJson, "p2p-tiny");
    assert_eq!(claim.format, FormatKind::OcelJson);
    assert_eq!(claim.fixture, "p2p-tiny");
    assert!(!claim.allows_lossy);
    assert!(claim.is_named());

    // A blank fixture is not a real claim.
    let unnamed = RoundTripClaim::exact(FormatKind::XesXml, "   ");
    assert!(!unnamed.is_named());

    // Claims are PartialEq.
    let c2 = RoundTripClaim::exact(FormatKind::OcelJson, "p2p-tiny");
    assert_eq!(claim, c2);
}
