// COMPILE-PASS: ArtifactGrounding OCEL flat claim refused — proves FlatClaimOverObjectCentric law

use wasm4pm_compat::interop::{ArtifactGrounding, InteropRefusal, Pm4pyShape};

fn main() {
    let g = ArtifactGrounding::<()>::new(Pm4pyShape::ObjectCentricLog, "ocel:fixture-1");
    assert!(g.is_grounded());
    let result = g.admit_flat();
    assert_eq!(result, Err(InteropRefusal::FlatClaimOverObjectCentric));
}
