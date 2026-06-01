// COMPILE-PASS: ProcessTree with Xor operator admits shape — lawful XOR(a, b).
//
// Law: Xor requires >= 2 children; a binary Xor(a, b) satisfies that law
// and passes admit_shape() without refusal.
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeNodeId, ProcessTreeOperator,
};

fn main() {
    let mut t = ProcessTree::new();
    t.nodes.push(ProcessTreeNode::Activity("branch_a".into()));
    t.nodes.push(ProcessTreeNode::Activity("branch_b".into()));
    t.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Xor,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1)],
    });
    t.root = Some(ProcessTreeNodeId(2));
    assert_eq!(t.admit_shape(), Ok(()));
}
