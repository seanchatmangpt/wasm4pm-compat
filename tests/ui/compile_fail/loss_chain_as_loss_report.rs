// COMPILE-FAIL: Loss chain law — LossChain cannot be passed where LossReport is required.
// Law: LossChain (a sequence of named loss steps) and LossReport<From,To,Items>
// (a single projection's loss record) are distinct structural types.
use wasm4pm_compat::loss::{LossChain, LossReport};

fn requires_loss_report(_r: LossReport<(), (), Vec<String>>) {}

fn main() {
    let chain = LossChain::new();
    // This must fail: LossChain is not LossReport<(),(),Vec<String>>.
    requires_loss_report(chain);
}
