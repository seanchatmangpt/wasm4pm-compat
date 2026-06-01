// COMPILE-FAIL: POWL structural law — OrderEdge cannot be passed where ChoiceGraphEdge is required.
// Law: OrderEdge (a strict partial-order precedence edge a ≺ b) and ChoiceGraphEdge
// (an edge in a choice graph) are distinct structural types. Confusing them is a compile error.
use wasm4pm_compat::powl::{ChoiceGraphEdge, OrderEdge, PowlNodeId};

fn requires_choice_edge(_e: ChoiceGraphEdge) {}

fn main() {
    let order = OrderEdge::new(PowlNodeId(0), PowlNodeId(1));
    // This must fail: OrderEdge is not ChoiceGraphEdge.
    requires_choice_edge(order);
}
