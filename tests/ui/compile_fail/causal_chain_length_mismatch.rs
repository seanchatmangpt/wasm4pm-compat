// COMPILE-FAIL: CausalChain length — chains of different lengths are distinct types.
//
// Law: CausalChainLengthLaw — CausalChain<3> and CausalChain<2> are different types.
// A function requiring a 3-link causal chain cannot accept a 2-link chain.
// The LENGTH const parameter is the type-level receipt of chain arity.
use wasm4pm_compat::causality::CausalChain;

fn requires_three_link_chain(_chain: CausalChain<3>) {}

fn main() {
    let chain: CausalChain<2> = CausalChain::new();
    // This must fail: CausalChain<2> is not CausalChain<3>.
    // Different chain lengths are non-interchangeable at the type level.
    requires_three_link_chain(chain);
}
