// Law: LossChainDebugLaw — LossChain implements Debug; a multi-step loss trail is inspectable without engine logic
// COMPILE-PASS: LossChain Debug — proves LossChain implements Debug

use wasm4pm_compat::loss::{LossChain, NamedLoss, ProjectionName};

fn main() {
    let mut chain = LossChain::new();
    chain.push(NamedLoss::new(ProjectionName("p"), "A"));
    let s = format!("{:?}", chain);
    assert!(s.contains("LossChain"));
}
