// COMPILE-PASS: CausalChain typed — chains of different lengths are distinct types
// and each is accepted by its length-specific function.
//
// Law: CausalChainLengthLaw — the LENGTH const parameter enforces causal chain arity.
use wasm4pm_compat::causality::{CausalChain, CausalLink, CausallyOrderedEvidence};

struct PlaceOrder;
struct ConfirmOrder;
struct ShipOrder;

fn requires_two_link_chain(_chain: CausalChain<2>) {}
fn requires_three_link_chain(_chain: CausalChain<3>) {}

fn main() {
    let chain2: CausalChain<2> = CausalChain::new();
    let chain3: CausalChain<3> = CausalChain::new();

    requires_two_link_chain(chain2);
    requires_three_link_chain(chain3);

    assert_eq!(CausalChain::<5>::new().length(), 5);

    // Typed causal link — direction is encoded in the type parameters
    let _link: CausalLink<PlaceOrder, ConfirmOrder> = CausalLink::new();
    let _link2: CausalLink<ConfirmOrder, ShipOrder> = CausalLink::new();

    // Causally-ordered evidence wrapper
    let ev = CausallyOrderedEvidence::new(42u32);
    assert_eq!(ev.inner, 42);
}
