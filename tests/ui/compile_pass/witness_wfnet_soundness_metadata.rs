// Law: WfNetSoundnessPaperMetadataLaw — WfNetSoundnessPaper implements Witness with KEY="wfnet-soundness-paper", YEAR=Some(1998), FAMILY=Paper; van der Aalst (1998) soundness criterion
// COMPILE-PASS: WfNetSoundnessPaper witness metadata — proves Witness trait constants

use wasm4pm_compat::witness::{Witness, WitnessFamily, WfNetSoundnessPaper};

fn main() {
    assert_eq!(WfNetSoundnessPaper::KEY, "wfnet-soundness-paper");
    assert_eq!(WfNetSoundnessPaper::YEAR, Some(1998));
    assert_eq!(WfNetSoundnessPaper::FAMILY, WitnessFamily::Paper);
}
