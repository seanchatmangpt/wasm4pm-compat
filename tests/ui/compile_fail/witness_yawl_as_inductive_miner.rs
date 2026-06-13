// COMPILE-FAIL: Witness confusion law — Evidence<T, Admitted, YawlPaper> cannot be used
// where Evidence<T, Admitted, InductiveMiner> is required.
// Law: YawlPaper (YAWL language) and InductiveMiner (discovery algorithm) are distinct witnesses.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Admitted;
use wasm4pm_compat::witness::{InductiveMiner, YawlPaper};

fn requires_inductive_miner_evidence(_: Evidence<String, Admitted, InductiveMiner>) {}

fn _test(yawl_ev: Evidence<String, Admitted, YawlPaper>) {
    // This must fail: YawlPaper witness is not InductiveMiner.
    requires_inductive_miner_evidence(yawl_ev);
}

fn main() {}
