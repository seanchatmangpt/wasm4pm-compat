// Law: SoundnessUnknownMarkerLaw — SoundnessUnknown is a zero-sized uninhabited marker for WfNet; it is a valid type-level token representing the default unaudited state
// COMPILE-PASS: SoundnessUnknown — the uninhabited enum marker compiles as a
// PhantomData type parameter on WfNet, proving it is a valid type-level token.
use core::marker::PhantomData;
use wasm4pm_compat::petri::SoundnessUnknown;

struct NetShape<S>(PhantomData<S>);

fn main() {
    let _: NetShape<SoundnessUnknown>;
    let _phantom: PhantomData<SoundnessUnknown> = PhantomData;
    let _ = _phantom;
}
