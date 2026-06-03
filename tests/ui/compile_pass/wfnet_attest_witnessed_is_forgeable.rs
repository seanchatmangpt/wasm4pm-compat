//! Law: WfNetAttestWitnessedForgeabilityGap — WfNet<SoundnessClaimed>.attest_witnessed() is callable without proof.
//! Structural defect: This is a weaker guarantee than WfNetConst<{SoundnessState::Claimed}>.witness_soundness(proof).
//! Compile-PASS (intentional): This demonstrates the WEAKER legacy API that CAN be called without a witness.
//! This fixture documents the forgeability gap in the legacy WfNet<S> API.
//!
//! Note: This is NOT a compile-fail fixture. It PASSES compilation to prove the method exists and is callable.
//! The defect is that there is no proof requirement — the API is forgeable.

use wasm4pm_compat::petri::{WfNet, SoundnessClaimed, PetriNet, Marking};
use std::collections::HashMap;

fn main() {
    // STRUCTURAL LAW VIOLATION (undocumented): WfNet<S>::attest_witnessed() has no proof requirement
    // PROOF: This code compiles, showing we can call attest_witnessed() without any SoundnessProof
    let net = PetriNet::new(
        vec![],
        vec![],
        vec![],
        Marking::default(),
    );

    let mut claimed: WfNet<SoundnessClaimed> = WfNet::with_soundness_claim(net, Some(Marking::default()));

    // This should NOT compile if soundness were properly enforced
    // But it does — showing the forgeability gap
    let _witnessed = claimed.attest_witnessed();
}
