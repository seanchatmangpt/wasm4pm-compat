// Law: LossChainExtendLaw — LossChain::extend merges two sub-pipeline chains into one ordered trail; cumulative loss accounting is correct
// COMPILE-PASS: LossChain::extend — proves two sub-pipeline chains merge into one

use wasm4pm_compat::loss::{LossChain, NamedLoss, ProjectionName};

fn main() {
    let mut a = LossChain::new();
    a.push(NamedLoss::new(ProjectionName("p"), "StepA"));

    let mut b = LossChain::new();
    b.push(NamedLoss::new(ProjectionName("q"), "StepB"));

    a.extend(b);
    assert_eq!(a.len(), 2);
    assert_eq!(a.steps()[0].category(), "StepA");
    assert_eq!(a.steps()[1].category(), "StepB");
}
