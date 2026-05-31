// COMPILE-PASS: ProcessTree, ProcessTreeOperator, and ProcessTreeNode construct
// lawfully — covers the general process tree shape beyond the typed-loop-arity fixture.
//
// Law: block-structured process tree — recursive structure of Sequence, Xor,
// Parallel, Loop, and Silent operators over activity leaves. Structure-only;
// discovery (inductive miner), replay, simplification, and conformance graduate
// to wasm4pm.
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeNodeId, ProcessTreeOperator,
};

fn main() {
    // Empty tree.
    let empty = ProcessTree::new();
    assert!(empty.root.is_none());
    assert_eq!(empty.node_count(), 0);

    // Build a simple Sequence tree: Sequence(a, b).
    // Node 0: activity "a", Node 1: activity "b", Node 2: Sequence([0, 1]).
    let mut tree = ProcessTree::new();
    tree.nodes.push(ProcessTreeNode::Activity("a".into()));
    tree.nodes.push(ProcessTreeNode::Activity("b".into()));
    tree.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Sequence,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1)],
    });
    tree.root = Some(ProcessTreeNodeId(2));

    assert_eq!(tree.node_count(), 3);
    assert_eq!(tree.root, Some(ProcessTreeNodeId(2)));

    // All five operators are constructible.
    let _seq = ProcessTreeOperator::Sequence;
    let _xor = ProcessTreeOperator::Xor;
    let _par = ProcessTreeOperator::Parallel;
    let _loop_op = ProcessTreeOperator::Loop;
    let _silent = ProcessTreeOperator::Silent;

    // Operators are Copy + Eq.
    assert_eq!(ProcessTreeOperator::Xor, ProcessTreeOperator::Xor);
    assert_ne!(ProcessTreeOperator::Sequence, ProcessTreeOperator::Parallel);

    // ProcessTreeNodeId is Copy + Ord.
    let id0 = ProcessTreeNodeId(0);
    let id1 = ProcessTreeNodeId(1);
    assert!(id0 < id1);
    assert_eq!(id0.0, 0);

    // Activity leaf.
    let leaf = ProcessTreeNode::Activity("register".into());
    if let ProcessTreeNode::Activity(ref name) = leaf {
        assert_eq!(name, "register");
    } else {
        panic!("expected Activity");
    }

    // Operator node with XOR and two children.
    let xor_node = ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Xor,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1)],
    };
    if let ProcessTreeNode::Operator { operator, children } = &xor_node {
        assert_eq!(*operator, ProcessTreeOperator::Xor);
        assert_eq!(children.len(), 2);
    } else {
        panic!("expected Operator");
    }

    // Silent node (no children, represents tau).
    let silent_node = ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Silent,
        children: vec![],
    };
    if let ProcessTreeNode::Operator { operator, children } = &silent_node {
        assert_eq!(*operator, ProcessTreeOperator::Silent);
        assert!(children.is_empty());
    }

    // Loop operator with two children: do body + redo.
    let loop_node = ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Loop,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1)],
    };
    if let ProcessTreeNode::Operator { operator, children } = &loop_node {
        assert_eq!(*operator, ProcessTreeOperator::Loop);
        assert_eq!(children.len(), 2);
    }
}
