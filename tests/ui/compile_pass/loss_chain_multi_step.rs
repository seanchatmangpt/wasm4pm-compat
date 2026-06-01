// Law: LossChainMultiStepLaw — a multi-step projection pipeline accumulates all named losses in order; no step is silently dropped
// COMPILE-PASS: LossChain multi-step — proves a two-step pipeline accumulates both named losses

use wasm4pm_compat::loss::{LossChain, NamedLoss, ProjectionName};

fn main() {
    let mut chain = LossChain::new();
    chain.push(NamedLoss::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        "DroppedObjectTypeLinks",
    ));
    chain.push(NamedLoss::new(
        ProjectionName("xes-to-dfg:aggregate"),
        "FlattenedTimestamps",
    ));
    assert_eq!(chain.len(), 2);
    assert!(!chain.is_lossless());
    assert_eq!(chain.steps()[0].category(), "DroppedObjectTypeLinks");
    assert_eq!(chain.steps()[1].category(), "FlattenedTimestamps");
}
