// Compile-fail fixture: ChoiceGraphEdge and OrderEdge are distinct types.
//
// Law: Kourani et al. (2026) — a ChoiceGraphEdge is a transition inside a
// choice graph; an OrderEdge is a precedence constraint in a partial order.
// They are not interchangeable even though they have the same field layout.

use wasm4pm_compat::powl::{ChoiceGraphEdge, OrderEdge, PowlNodeId};

fn accept_choice_edge(_e: ChoiceGraphEdge) {}

fn main() {
    let order_edge = OrderEdge::new(PowlNodeId(0), PowlNodeId(1));
    // ERROR: expected ChoiceGraphEdge, found OrderEdge (E0308).
    accept_choice_edge(order_edge);
}
