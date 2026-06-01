// Law: LossChainStepsSliceLaw — LossChain::steps() returns the ordered slice of all accumulated NamedLoss entries; ordering is preserved
// COMPILE-PASS: LossChain::steps — proves steps() returns an ordered slice of NamedLoss entries

use wasm4pm_compat::loss::{LossChain, NamedLoss, ProjectionName};

fn main() {
    let mut chain = LossChain::new();
    chain.push(NamedLoss::new(ProjectionName("p"), "A"));
    chain.push(NamedLoss::new(ProjectionName("q"), "B"));
    chain.push(NamedLoss::new(ProjectionName("r"), "C"));

    let steps = chain.steps();
    assert_eq!(steps.len(), 3);
    assert_eq!(steps[0].projection().as_str(), "p");
    assert_eq!(steps[1].projection().as_str(), "q");
    assert_eq!(steps[2].projection().as_str(), "r");
    assert_eq!(steps[2].category(), "C");
}
