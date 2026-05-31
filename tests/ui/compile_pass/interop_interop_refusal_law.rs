// COMPILE-PASS: InteropRefusal::law — proves all refusal variants have stable named law strings

use wasm4pm_compat::interop::InteropRefusal;

fn main() {
    assert_eq!(InteropRefusal::UngroundedArtifact.law(), "UngroundedArtifact");
    assert_eq!(InteropRefusal::FlatClaimOverObjectCentric.law(), "FlatClaimOverObjectCentric");
    assert_eq!(InteropRefusal::VacuousConformanceClaim.law(), "VacuousConformanceClaim");
    assert_eq!(InteropRefusal::DimensionShapeMismatch.law(), "DimensionShapeMismatch");
    assert_eq!(
        InteropRefusal::UnadmittedRawInterpretation.law(),
        "UnadmittedRawInterpretation"
    );
}
