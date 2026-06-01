// COMPILE-PASS: ProcessTreeRefusal::BelowMinimumArity from admit_shape.
//
// Law: Xor, Parallel, Sequence, and Or all require >= 2 children. A Xor node
// with only one child is refused as BelowMinimumArity — not a generic error.
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeNodeId, ProcessTreeOperator,
    ProcessTreeRefusal,
};

fn main() {
    let mut t = ProcessTree::new();
    t.nodes.push(ProcessTreeNode::Activity("only_branch".into()));
    t.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Xor,
        children: vec![ProcessTreeNodeId(0)],
    });
    t.root = Some(ProcessTreeNodeId(1));
    assert_eq!(t.admit_shape(), Err(ProcessTreeRefusal::BelowMinimumArity));
}
