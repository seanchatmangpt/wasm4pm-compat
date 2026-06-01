// COMPILE-FAIL: WF-net soundness law — WfNetConst<{SoundnessState::Unknown}> cannot be passed
// where WfNetConst<{SoundnessState::Claimed}> is required.
// Law: An unknown-soundness net has not even been claimed sound, let alone witnessed.
use wasm4pm_compat::law::SoundnessState;
use wasm4pm_compat::petri::WfNetConst;

fn requires_claimed(_net: WfNetConst<{ SoundnessState::Claimed }>) {}

fn main() {
    let unknown = WfNetConst::<{ SoundnessState::Unknown }>::new();
    // This must fail: Unknown is not Claimed.
    requires_claimed(unknown);
}
