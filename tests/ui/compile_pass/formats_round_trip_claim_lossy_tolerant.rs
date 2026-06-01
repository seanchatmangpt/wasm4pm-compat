// COMPILE-PASS: RoundTripClaim::lossy_tolerant — proves a lossy-tolerant
// round-trip claim constructs with allows_lossy = true.
//
// Law: RoundTripClaimLossyLaw — some round trips tolerate normalization loss
// (e.g. whitespace/ordering); such claims must be explicitly marked as lossy-
// tolerant so tests can verify the appropriate weaker equivalence.
use wasm4pm_compat::formats::{FormatKind, RoundTripClaim};

fn main() {
    let claim = RoundTripClaim::lossy_tolerant(FormatKind::XesXml, "running-example");
    assert_eq!(claim.format, FormatKind::XesXml);
    assert!(claim.allows_lossy);
    assert!(claim.is_named());

    // Clone is implemented.
    let cloned = claim.clone();
    assert_eq!(claim, cloned);

    // Distinct from an exact claim on the same fixture.
    let exact = RoundTripClaim::exact(FormatKind::XesXml, "running-example");
    assert_ne!(claim, exact);
}
