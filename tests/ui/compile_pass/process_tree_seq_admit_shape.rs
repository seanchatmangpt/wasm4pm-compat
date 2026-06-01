// COMPILE-PASS: ProcessTree with Sequence operator admits shape — lawful Seq(a, b, c).
//
// Law: Sequence requires >= 2 children; a three-step sequence satisfies that
// law and passes admit_shape() without refusal.
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeNodeId, ProcessTreeOperator,
};

fn main() {
    let mut t = ProcessTree::new();
    t.nodes.push(ProcessTreeNode::Activity("register".into()));
    t.nodes.push(ProcessTreeNode::Activity("approve".into()));
    t.nodes.push(ProcessTreeNode::Activity("close".into()));
    t.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Sequence,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1), ProcessTreeNodeId(2)],
    });
    t.root = Some(ProcessTreeNodeId(3));
    assert_eq!(t.admit_shape(), Ok(()));
}
