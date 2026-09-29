// COMPILE-PASS: ProcessTreeRefusal::InvalidArity from Loop with fewer than 2 or more than 3 children.
//
// Law: Loop takes 2 (do, redo; Leemans IM) or 3 (do, redo, exit; pm4py/ProM)
// children. A loop with 4 children is refused as InvalidArity — a named
// structural law.
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeNodeId, ProcessTreeOperator,
    ProcessTreeRefusal,
};

fn main() {
    let mut t = ProcessTree::new();
    t.nodes.push(ProcessTreeNode::Activity("do_body".into()));
    t.nodes.push(ProcessTreeNode::Activity("redo".into()));
    t.nodes.push(ProcessTreeNode::Activity("exit".into()));
    t.nodes.push(ProcessTreeNode::Activity("extra".into()));
    t.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Loop,
        children: vec![
            ProcessTreeNodeId(0),
            ProcessTreeNodeId(1),
            ProcessTreeNodeId(2),
            ProcessTreeNodeId(3),
        ],
    });
    t.root = Some(ProcessTreeNodeId(4));
    assert_eq!(t.admit_shape(), Err(ProcessTreeRefusal::InvalidArity));
}
