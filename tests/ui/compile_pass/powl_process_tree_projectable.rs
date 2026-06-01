// Law: PowlTreeProjectionLaw — ProcessTreeProjectable implements TreeProjectable and passes assert_tree_projectable; ExceedsProcessTree does not
// COMPILE-PASS: POWL projection law — ProcessTreeProjectable marker admits the gate.
use wasm4pm_compat::powl::{assert_tree_projectable, ProcessTreeProjectable};

fn main() {
    let result = assert_tree_projectable(ProcessTreeProjectable);
    assert!(result);
}
