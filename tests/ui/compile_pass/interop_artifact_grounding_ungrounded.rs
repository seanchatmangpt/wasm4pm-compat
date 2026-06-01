// Law: UngroundedArtifactLaw — an ArtifactGrounding with an empty evidence_ref is ungrounded; admit_flat() returns InteropRefusal::UngroundedArtifact
// COMPILE-PASS: ArtifactGrounding empty ref — proves empty evidence_ref is ungrounded and refused

use wasm4pm_compat::interop::{ArtifactGrounding, InteropRefusal, Pm4pyShape};

fn main() {
    let bad = ArtifactGrounding::<()>::new(Pm4pyShape::PetriNet, "");
    assert!(!bad.is_grounded());
    let result = bad.admit_flat();
    assert_eq!(result, Err(InteropRefusal::UngroundedArtifact));
}
