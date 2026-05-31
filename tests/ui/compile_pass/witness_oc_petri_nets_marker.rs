// COMPILE-PASS: OcPetriNets witness marker — proves OcPetriNets is a distinct
// named law that implements Witness with correct metadata, and that it is
// non-interchangeable with ObjectCentricPetriNetPaper at the type level.
//
// Law: Object-centric Petri nets notation authority (2020). OcPetriNets names
// the model class (object types, variable arcs, binding elements); it is
// distinct from ObjectCentricPetriNetPaper, which names the discovery algorithm
// output. An Admission<T, OcPetriNets> is a different type from
// Admission<T, ObjectCentricPetriNetPaper>. This fixture proves the marker
// compiles, carries correct metadata, and belongs to WitnessFamily::Paper.
use wasm4pm_compat::witness::{
    ObjectCentricPetriNetPaper, OcPetriNets, Witness, WitnessFamily,
};

fn accept_witness_key<W: Witness>() -> &'static str {
    W::KEY
}

fn main() {
    // OcPetriNets carries the correct metadata.
    assert_eq!(OcPetriNets::KEY, "oc-petri-nets");
    assert_eq!(
        OcPetriNets::TITLE,
        "Object-Centric Petri Nets (notation)"
    );
    assert_eq!(OcPetriNets::YEAR, Some(2020));
    assert_eq!(OcPetriNets::FAMILY, WitnessFamily::Paper);

    // OcPetriNets satisfies the Witness bound.
    let key = accept_witness_key::<OcPetriNets>();
    assert_eq!(key, "oc-petri-nets");

    // OcPetriNets and ObjectCentricPetriNetPaper share the same year but name
    // orthogonal authorities: model notation vs. discovery algorithm paper.
    assert_ne!(OcPetriNets::KEY, ObjectCentricPetriNetPaper::KEY);
    assert_ne!(OcPetriNets::TITLE, ObjectCentricPetriNetPaper::TITLE);
    assert_eq!(OcPetriNets::YEAR, ObjectCentricPetriNetPaper::YEAR);
    assert_eq!(OcPetriNets::FAMILY, ObjectCentricPetriNetPaper::FAMILY);

    // Non-interchangeability: distinct PhantomData types.
    let _ocpn: std::marker::PhantomData<OcPetriNets> = std::marker::PhantomData;
    let _ocpnp: std::marker::PhantomData<ObjectCentricPetriNetPaper> = std::marker::PhantomData;
}
