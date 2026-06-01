// COMPILE-FAIL: PowlChoiceUsedAsLoop — PowlChoiceNode cannot be used where PowlLoopNode is required
// Law: POWL operator law — a choice node and a loop node are distinct structural operators.
// Passing a PowlChoiceNode where the type system requires a TypedPowlLoopNode violates the law.
// Expected error: type mismatch — PowlChoiceNode is not TypedPowlLoopNode.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use wasm4pm_compat::powl::{PowlChoiceNode, PowlNodeId, TypedPowlLoopNode};

fn accept_loop(_node: TypedPowlLoopNode<Vec<PowlNodeId>, 2>) {}

fn main() {
    let choice = PowlChoiceNode::new(vec![PowlNodeId(0), PowlNodeId(1)]);
    accept_loop(choice);
}
