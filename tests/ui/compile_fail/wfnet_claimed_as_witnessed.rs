// COMPILE-FAIL: WF-net soundness law — WfNetConst<{SoundnessState::Claimed}> cannot be passed
// where WfNetConst<{SoundnessState::Witnessed}> is required.
// Law: Claimed soundness is not witnessed soundness. The engine must supply a SoundnessProof.
use wasm4pm_compat::law::SoundnessState;
use wasm4pm_compat::petri::WfNetConst;

fn requires_witnessed(_net: WfNetConst<{ SoundnessState::Witnessed }>) {}

fn main() {
    let unknown = WfNetConst::<{ SoundnessState::Unknown }>::new();
    let claimed = unknown.claim_sound();
    // This must fail: Claimed is not Witnessed.
    requires_witnessed(claimed);
}
