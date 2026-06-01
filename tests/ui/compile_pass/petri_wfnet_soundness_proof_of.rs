// COMPILE-PASS: WfNetSoundnessProofOf — the phantom-typed proof carrier
// compiles as a type-level annotation. Only constructible inside petri or
// via the wasm4pm bridge; this fixture proves it is a valid public type.
use core::marker::PhantomData;
use wasm4pm_compat::petri::WfNetSoundnessProofOf;

struct OrderFulfillmentNet;

fn main() {
    // The proof is a type-level phantom; it can appear in type position.
    let _: PhantomData<WfNetSoundnessProofOf<OrderFulfillmentNet>> = PhantomData;
}
