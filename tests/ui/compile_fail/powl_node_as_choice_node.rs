// COMPILE-FAIL: POWL structural law — PowlNode cannot be passed where PowlChoiceNode is required.
// Law: PowlNode (a generic POWL node with a kind) and PowlChoiceNode (a structured exclusive
// choice with branch list) are distinct types. A generic node is not a choice node.
use wasm4pm_compat::powl::{PowlChoiceNode, PowlNode, PowlNodeId, PowlNodeKind};

fn requires_choice_node(_c: PowlChoiceNode) {}

fn main() {
    let node = PowlNode::new(PowlNodeId(0), PowlNodeKind::Xor);
    // This must fail: PowlNode is not PowlChoiceNode.
    requires_choice_node(node);
}
