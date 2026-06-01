// COMPILE-PASS: ProcessTree with Parallel (AND) operator admits shape.
//
// Law: Parallel requires >= 2 children; a binary And(a, b) satisfies that law
// and passes admit_shape() without refusal.
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeNodeId, ProcessTreeOperator,
};

fn main() {
    let mut t = ProcessTree::new();
    t.nodes.push(ProcessTreeNode::Activity("task_a".into()));
    t.nodes.push(ProcessTreeNode::Activity("task_b".into()));
    t.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Parallel,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1)],
    });
    t.root = Some(ProcessTreeNodeId(2));
    assert_eq!(t.admit_shape(), Ok(()));
}
