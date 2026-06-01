// COMPILE-PASS: FormatEnvelope witness PhantomData — proves the witness type
// parameter W is zero-cost; FormatEnvelope<W1> and FormatEnvelope<W2> are
// different types at the type level but identical at runtime.
//
// Law: FormatEnvelopeWitnessLaw — the W type parameter in FormatEnvelope<W>
// threads witness family identity through the boundary at zero runtime cost.
use core::marker::PhantomData;
use wasm4pm_compat::formats::{FormatEnvelope, FormatKind};

struct OcelWitness;
struct XesWitness;

fn main() {
    // Two envelopes with different witness families are distinct types.
    let ocel_env = FormatEnvelope::<OcelWitness>::new(FormatKind::OcelJson, b"{}".to_vec());
    let xes_env = FormatEnvelope::<XesWitness>::new(FormatKind::XesXml, b"<log/>".to_vec());

    // Each carries its own bytes.
    assert_eq!(ocel_env.kind, FormatKind::OcelJson);
    assert_eq!(xes_env.kind, FormatKind::XesXml);

    // PhantomData field is accessible and zero-sized.
    let _: PhantomData<OcelWitness> = ocel_env.witness;
    let _: PhantomData<XesWitness> = xes_env.witness;
}
