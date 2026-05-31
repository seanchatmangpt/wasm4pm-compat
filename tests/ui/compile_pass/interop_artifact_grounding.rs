// COMPILE-PASS: ArtifactGrounding<W> constructs with a witness marker and a
// Pm4pyShape — covers the grounded artifact shape for interop boundary.
//
// Law: interop grammar — an artifact claim must reference evidence to ground it.
// An empty evidence_ref is an UngroundedArtifact refusal.

use core::marker::PhantomData;
use wasm4pm_compat::interop::{ArtifactGrounding, InteropRefusal, Pm4pyShape};
use wasm4pm_compat::witness::Ocel20;

fn check_grounded() {
    // Construction with a concrete witness marker (Ocel20).
    let g = ArtifactGrounding::<Ocel20>::new(Pm4pyShape::ObjectCentricLog, "ocel:fixture-1");
    assert_eq!(g.shape, Pm4pyShape::ObjectCentricLog);
    assert_eq!(g.evidence_ref, "ocel:fixture-1");
    assert_eq!(g.witness, PhantomData::<Ocel20>);
    assert!(g.is_grounded());
}

fn check_flat_admission() {
    // Flat artifact: flat shape + evidence → admits lawfully.
    let flat = ArtifactGrounding::<()>::new(Pm4pyShape::EventLog, "blake3:abc");
    assert!(flat.admit_flat().is_ok());

    // Object-centric artifact over flat path → FlatClaimOverObjectCentric refusal.
    let ocel = ArtifactGrounding::<()>::new(Pm4pyShape::ObjectCentricLog, "ocel:fixture-2");
    assert_eq!(
        ocel.admit_flat(),
        Err(InteropRefusal::FlatClaimOverObjectCentric)
    );
}

fn check_ungrounded() {
    // Empty evidence_ref → not grounded.
    let bad = ArtifactGrounding::<()>::new(Pm4pyShape::PetriNet, "");
    assert!(!bad.is_grounded());
    assert_eq!(bad.admit_flat(), Err(InteropRefusal::UngroundedArtifact));
}

fn check_witness_type_level() {
    // ArtifactGrounding<Ocel20> and ArtifactGrounding<()> are distinct types.
    fn only_ocel20(_: &ArtifactGrounding<Ocel20>) {}
    let g = ArtifactGrounding::<Ocel20>::new(Pm4pyShape::EventLog, "ref:123");
    only_ocel20(&g);
}

fn main() {
    check_grounded();
    check_flat_admission();
    check_ungrounded();
    check_witness_type_level();
}
