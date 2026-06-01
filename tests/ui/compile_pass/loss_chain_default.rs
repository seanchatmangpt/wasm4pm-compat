// Law: LossChainDefaultLaw — LossChain::default() produces an empty chain that is_lossless(); Default is the canonical empty-chain constructor
// COMPILE-PASS: LossChain Default — proves Default::default() produces an empty lossless chain

use wasm4pm_compat::loss::LossChain;

fn main() {
    let chain: LossChain = Default::default();
    assert!(chain.is_lossless());
    assert_eq!(chain.len(), 0);
}
