// COMPILE-PASS: SoundnessWitnessed — the uninhabited enum marker compiles as a
// PhantomData type parameter, distinct from the other two soundness tokens.
use core::marker::PhantomData;
use wasm4pm_compat::petri::SoundnessWitnessed;

struct NetShape<S>(PhantomData<S>);

fn main() {
    let _: NetShape<SoundnessWitnessed>;
    let _phantom: PhantomData<SoundnessWitnessed> = PhantomData;
    let _ = _phantom;
}
