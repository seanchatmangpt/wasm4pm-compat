// COMPILE-PASS: ProcessTree with Or (inclusive OR) operator admits shape.
//
// Law: Or requires >= 2 children — an inclusive choice between one thing is
// trivially degenerate. Binary Or(a, b) passes admit_shape() without refusal.
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeNodeId, ProcessTreeOperator,
};

fn main() {
    let mut t = ProcessTree::new();
    t.nodes.push(ProcessTreeNode::Activity("option_a".into()));
    t.nodes.push(ProcessTreeNode::Activity("option_b".into()));
    t.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Or,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1)],
    });
    t.root = Some(ProcessTreeNodeId(2));
    assert_eq!(t.admit_shape(), Ok(()));
}
