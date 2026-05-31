// COMPILE-PASS: RoundTripClaim constructs with required FormatEnvelope contract
//
// Proves that:
//   1. RoundTripClaim::exact constructs with a FormatKind and fixture name.
//   2. RoundTripClaim::lossy_tolerant constructs with allows_lossy = true.
//   3. is_named() returns true for a non-blank fixture name.
//   4. is_named() returns false for a blank/whitespace fixture name.
//   5. PartialEq and Clone are implemented.
//   6. The claim is structure only — it names what must be tested, not the test itself.

use wasm4pm_compat::formats::{FormatKind, RoundTripClaim};

fn main() {
    // Exact (lossless) round-trip claim.
    let exact = RoundTripClaim::exact(FormatKind::OcelJson, "p2p-tiny");
    assert_eq!(exact.format, FormatKind::OcelJson);
    assert_eq!(exact.fixture, "p2p-tiny");
    assert!(!exact.allows_lossy);
    assert!(exact.is_named());

    // Lossy-tolerant round-trip claim (e.g. whitespace/ordering normalization).
    let tolerant = RoundTripClaim::lossy_tolerant(FormatKind::XesXml, "running-example");
    assert_eq!(tolerant.format, FormatKind::XesXml);
    assert_eq!(tolerant.fixture, "running-example");
    assert!(tolerant.allows_lossy);
    assert!(tolerant.is_named());

    // An unnamed claim (blank fixture) is not a real claim.
    let unnamed = RoundTripClaim::exact(FormatKind::PowlJson, "  ");
    assert!(!unnamed.is_named());

    // Multiple format variants are usable in claims.
    let bpmn_claim = RoundTripClaim::exact(FormatKind::BpmnXml, "simple-sequence");
    assert_eq!(bpmn_claim.format, FormatKind::BpmnXml);

    let petri_claim = RoundTripClaim::lossy_tolerant(FormatKind::PetriPnml, "wfnet-example");
    assert_eq!(petri_claim.format, FormatKind::PetriPnml);
    assert!(petri_claim.allows_lossy);

    // Clone is implemented.
    let cloned = exact.clone();
    assert_eq!(exact, cloned);

    // PartialEq distinguishes claims by format and fixture.
    let other = RoundTripClaim::exact(FormatKind::OcelJson, "different-fixture");
    assert_ne!(exact, other);
}
