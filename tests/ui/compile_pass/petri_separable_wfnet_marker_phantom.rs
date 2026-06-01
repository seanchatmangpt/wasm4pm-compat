// COMPILE-PASS: SeparableWfNetMarker — the uninhabited phantom marker compiles
// as a type parameter for downstream types that need to carry a separability claim.
// Law: Kourani, Park & van der Aalst (2026) Definition 4.1.
use core::marker::PhantomData;
use wasm4pm_compat::petri::SeparableWfNetMarker;

struct PowlOutput<S>(PhantomData<S>);

fn main() {
    let _: PowlOutput<SeparableWfNetMarker>;
    let _: PhantomData<SeparableWfNetMarker> = PhantomData;
}
