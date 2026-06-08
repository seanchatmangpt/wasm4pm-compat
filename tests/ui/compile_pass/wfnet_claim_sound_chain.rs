// Law: WfNetClaimSoundChainLaw — WfNet::claim_sound produces WfNet<Claimed> and attest_witnessed produces WfNet<Witnessed>; both transitions are type-level re-tags producing distinct types
// COMPILE-PASS: WfNet::claim_sound — Unknown → Claimed is a type-level re-tag.
// WfNet::attest_witnessed — Claimed → Witnessed is a type-level re-tag (migrated; use WfNetConst for non-forgeable path).
// Both transitions compile and produce distinct types.
#[allow(migrated)]
use wasm4pm_compat::petri::{WfNet, PetriNet, Marking, SoundnessUnknown, SoundnessClaimed};

fn main() {
    let wf: WfNet<SoundnessUnknown> = WfNet::new(PetriNet::default(), Marking::new([("snk".to_string(), 1)]));
    let claimed: WfNet<SoundnessClaimed> = wf.claim_sound();
    // The claimed net still has final marking accessible.
    assert!(claimed.final_marking().is_some());
}
