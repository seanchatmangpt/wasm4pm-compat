// COMPILE-PASS: LossChain::new — proves an empty LossChain constructs and reports lossless

use wasm4pm_compat::loss::LossChain;

fn main() {
    let chain = LossChain::new();
    assert!(chain.is_lossless());
    assert!(chain.is_empty());
    assert_eq!(chain.len(), 0);
}
