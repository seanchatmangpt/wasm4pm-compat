// Law: SoundnessClaimedMarkerLaw — SoundnessClaimed is a zero-sized uninhabited marker; distinct from SoundnessUnknown and SoundnessWitnessed at the type level
// COMPILE-PASS: SoundnessClaimed — the uninhabited enum marker compiles as a
// PhantomData type parameter, distinct from SoundnessUnknown and SoundnessWitnessed.
use core::marker::PhantomData;
use wasm4pm_compat::petri::SoundnessClaimed;

struct NetShape<S>(PhantomData<S>);

fn main() {
    let _: NetShape<SoundnessClaimed>;
    let _phantom: PhantomData<SoundnessClaimed> = PhantomData;
    let _ = _phantom;
}
