// COMPILE-PASS: WfNet::claim_sound — Unknown → Claimed is a type-level re-tag.
// WfNet::attest_witnessed — Claimed → Witnessed is a type-level re-tag.
// Both transitions compile and produce distinct types.
use wasm4pm_compat::petri::{WfNet, PetriNet, Marking, SoundnessUnknown, SoundnessClaimed, SoundnessWitnessed};

fn main() {
    let wf: WfNet<SoundnessUnknown> = WfNet::new(PetriNet::default(), Marking::new([("snk".to_string(), 1)]));
    let claimed: WfNet<SoundnessClaimed> = wf.claim_sound();
    let witnessed: WfNet<SoundnessWitnessed> = claimed.attest_witnessed();
    // The witnessed net still has final marking accessible.
    assert!(witnessed.final_marking().is_some());
}
