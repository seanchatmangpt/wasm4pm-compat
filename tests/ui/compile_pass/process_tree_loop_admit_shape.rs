// COMPILE-PASS: ProcessTree with Loop operator admits shape — lawful Loop(do, redo).
//
// Law: Loop requires exactly 2 children: the do-body and the redo branch.
// A loop with two children passes admit_shape() without refusal.
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeNodeId, ProcessTreeOperator,
};

fn main() {
    let mut t = ProcessTree::new();
    t.nodes.push(ProcessTreeNode::Activity("do_body".into()));
    t.nodes.push(ProcessTreeNode::Activity("redo_branch".into()));
    t.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Loop,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1)],
    });
    t.root = Some(ProcessTreeNodeId(2));
    assert_eq!(t.admit_shape(), Ok(()));
}
