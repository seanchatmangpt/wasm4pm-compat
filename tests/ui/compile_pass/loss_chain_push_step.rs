// Law: LossChainPushStepLaw — LossChain::push records a NamedLoss step and the chain is no longer lossless after the push
// COMPILE-PASS: LossChain::push — proves a single NamedLoss step can be recorded

use wasm4pm_compat::loss::{LossChain, NamedLoss, ProjectionName};

fn main() {
    let mut chain = LossChain::new();
    chain.push(NamedLoss::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        "DroppedObjectTypeLinks",
    ));
    assert_eq!(chain.len(), 1);
    assert!(!chain.is_lossless());
    assert!(!chain.is_empty());
}
