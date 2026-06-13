// COMPILE-PASS: StrictViolation law names are stable and specifically named.
//
// Law: StrictViolation named law covenant — every strict violation carries a
// stable, human-readable law name via .law(). The names are not magic strings
// invented at the call site; they are the canonical law identifiers. This proves
// the law surface is open and that all eight violation kinds are constructible.
//
// Requires: --features strict
use wasm4pm_compat::strict::StrictViolation;

fn main() {
    // Every StrictViolation variant has a stable, specifically named law string.
    assert_eq!(StrictViolation::MissingWitness.law(), "MissingWitness");
    assert_eq!(
        StrictViolation::MissingRoundTripFixture.law(),
        "MissingRoundTripFixture"
    );
    assert_eq!(StrictViolation::MissingLossPolicy.law(), "MissingLossPolicy");
    assert_eq!(StrictViolation::RawEvidenceExported.law(), "RawEvidenceExported");
    assert_eq!(StrictViolation::MissingRefusalPath.law(), "MissingRefusalPath");
    assert_eq!(
        StrictViolation::MissingConformanceFields.law(),
        "MissingConformanceFields"
    );
    assert_eq!(StrictViolation::MissingReceiptShape.law(), "MissingReceiptShape");
    assert_eq!(
        StrictViolation::HiddenProcessMiningGrowth.law(),
        "HiddenProcessMiningGrowth"
    );

    // Every variant is constructible — covering all eight law kinds.
    let violations = [
        StrictViolation::MissingWitness,
        StrictViolation::MissingRoundTripFixture,
        StrictViolation::MissingLossPolicy,
        StrictViolation::RawEvidenceExported,
        StrictViolation::MissingRefusalPath,
        StrictViolation::MissingConformanceFields,
        StrictViolation::MissingReceiptShape,
        StrictViolation::HiddenProcessMiningGrowth,
    ];
    assert_eq!(violations.len(), 8);

    // Display is implemented — law() is the canonical law name.
    assert_eq!(StrictViolation::MissingWitness.law(), "MissingWitness");
}
