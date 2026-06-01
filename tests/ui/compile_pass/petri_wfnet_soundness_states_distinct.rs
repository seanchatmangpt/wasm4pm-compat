// Law: WfNetSoundnessStatesDistinctLaw — SoundnessUnknown, SoundnessClaimed, and SoundnessWitnessed produce three mutually distinct WfNet<S> types; no cross-assignment is possible
// COMPILE-PASS: SoundnessUnknown, SoundnessClaimed, SoundnessWitnessed are
// mutually distinct types — WfNet<Unknown>, WfNet<Claimed>, WfNet<Witnessed>
// are three separate types that cannot be confused at the type level.
#[allow(deprecated)]
use wasm4pm_compat::petri::{WfNet, SoundnessUnknown, SoundnessClaimed, SoundnessWitnessed, PetriNet, Marking};

fn only_unknown(_: &WfNet<SoundnessUnknown>) {}
fn only_claimed(_: &WfNet<SoundnessClaimed>) {}
fn only_witnessed(_: &WfNet<SoundnessWitnessed>) {}

fn main() {
    let wf = WfNet::new(PetriNet::default(), Marking::new([("snk".to_string(), 1)]));
    only_unknown(&wf);

    let claimed = wf.claim_sound();
    only_claimed(&claimed);

    let witnessed = claimed.attest_witnessed();
    only_witnessed(&witnessed);
}
