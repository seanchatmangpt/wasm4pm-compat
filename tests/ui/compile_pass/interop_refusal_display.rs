// COMPILE-PASS: InteropRefusal Display — proves all refusal variants format as "interop refusal: <law>"

use wasm4pm_compat::interop::InteropRefusal;

fn main() {
    assert_eq!(
        format!("{}", InteropRefusal::UngroundedArtifact),
        "interop refusal: UngroundedArtifact"
    );
    assert_eq!(
        format!("{}", InteropRefusal::FlatClaimOverObjectCentric),
        "interop refusal: FlatClaimOverObjectCentric"
    );
    assert_eq!(
        format!("{}", InteropRefusal::DimensionShapeMismatch),
        "interop refusal: DimensionShapeMismatch"
    );
}
