// COMPILE-PASS: ProcessTreeRefusal::InvalidArity from Loop with != 2 children.
//
// Law: Loop requires exactly 2 children (do-body + redo). A loop with 3
// children is refused as InvalidArity — a named structural law.
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeNodeId, ProcessTreeOperator,
    ProcessTreeRefusal,
};

fn main() {
    let mut t = ProcessTree::new();
    t.nodes.push(ProcessTreeNode::Activity("do_body".into()));
    t.nodes.push(ProcessTreeNode::Activity("redo".into()));
    t.nodes.push(ProcessTreeNode::Activity("extra".into()));
    t.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Loop,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1), ProcessTreeNodeId(2)],
    });
    t.root = Some(ProcessTreeNodeId(3));
    assert_eq!(t.admit_shape(), Err(ProcessTreeRefusal::InvalidArity));
}
