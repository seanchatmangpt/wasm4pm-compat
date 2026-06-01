// COMPILE-PASS: ChoiceGraphEdge and OrderEdge are structurally distinct types.
//
// Law: Kourani et al. (2026) Definition 3.6 — a ChoiceGraphEdge is a directed
// transition in a choice graph (decision/cyclic logic); an OrderEdge is a
// precedence constraint inside a partial order (scheduling logic). They carry
// identical fields but are not interchangeable at the call site.
use wasm4pm_compat::powl::{ChoiceGraphEdge, OrderEdge, PowlNodeId};

fn takes_order(_: OrderEdge) {}
fn takes_choice(_: ChoiceGraphEdge) {}

fn main() {
    let oe = OrderEdge::new(PowlNodeId(0), PowlNodeId(1));
    let ce = ChoiceGraphEdge::new(PowlNodeId(0), PowlNodeId(1));

    // Same field values, but distinct call sites.
    takes_order(oe);
    takes_choice(ce);

    // Structural fields are readable and equal.
    assert_eq!(oe.from, ce.from);
    assert_eq!(oe.to, ce.to);
}
