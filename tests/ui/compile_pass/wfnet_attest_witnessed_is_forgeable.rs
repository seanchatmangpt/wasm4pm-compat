//! Law: WfNetAttestWitnessedForgeabilityGap — WfNet<SoundnessClaimed> is constructible without proof.
//! Structural defect: This is a weaker guarantee than WfNetConst<{SoundnessState::Claimed}>.witness_soundness(proof).
//! Compile-PASS (intentional): This demonstrates the WEAKER legacy API that CAN be called without a witness.
//! This fixture documents the forgeability gap in the legacy WfNet<S> API.
//!
//! Note: This is NOT a compile-fail fixture. It PASSES compilation to prove the method exists and is callable.
//! The defect is that there is no proof requirement — the API is forgeable.

use wasm4pm_compat::petri::{WfNet, SoundnessClaimed, PetriNet, Marking};

fn main() {
    // STRUCTURAL LAW VIOLATION (documented): WfNet<S>::claim_sound() has no proof requirement.
    // PROOF: This code compiles, showing we can obtain WfNet<SoundnessClaimed> without any SoundnessProof.
    let net = PetriNet::new(
        vec![],
        vec![],
        vec![],
        Marking::default(),
    );

    // claim_sound() freely advances the typestate without a proof token — forgeability gap.
    let _claimed: WfNet<SoundnessClaimed> = WfNet::new(net, Marking::default()).claim_sound();
}
