// COMPILE-PASS: ProcessTree::admit_shape() structural law surface.
//
// Law: block-structured process tree shapes must satisfy:
// - non-empty tree must have a root,
// - Silent nodes carry no children,
// - Loop nodes have exactly 2 children,
// - Sequence/Xor/Parallel/Or have at least 2 children.
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeNodeId, ProcessTreeOperator, ProcessTreeRefusal,
};

fn main() {
    // Empty tree: no root needed, passes.
    let empty = ProcessTree::new();
    assert_eq!(empty.admit_shape(), Ok(()));

    // Well-formed Sequence(a, b): passes.
    let mut seq = ProcessTree::new();
    seq.nodes.push(ProcessTreeNode::Activity("a".into()));
    seq.nodes.push(ProcessTreeNode::Activity("b".into()));
    seq.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Sequence,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1)],
    });
    seq.root = Some(ProcessTreeNodeId(2));
    assert_eq!(seq.admit_shape(), Ok(()));

    // Well-formed Loop(do, redo): passes.
    let mut lp = ProcessTree::new();
    lp.nodes.push(ProcessTreeNode::Activity("do_body".into()));
    lp.nodes.push(ProcessTreeNode::Activity("redo".into()));
    lp.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Loop,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1)],
    });
    lp.root = Some(ProcessTreeNodeId(2));
    assert_eq!(lp.admit_shape(), Ok(()));

    // Non-empty without root: MissingRoot.
    let mut no_root = ProcessTree::new();
    no_root.nodes.push(ProcessTreeNode::Activity("a".into()));
    assert_eq!(no_root.admit_shape(), Err(ProcessTreeRefusal::MissingRoot));

    // Silent node with children: TauLeafWithChildren.
    let mut tau = ProcessTree::new();
    tau.nodes.push(ProcessTreeNode::Activity("child".into()));
    tau.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Silent,
        children: vec![ProcessTreeNodeId(0)],
    });
    tau.root = Some(ProcessTreeNodeId(1));
    assert_eq!(tau.admit_shape(), Err(ProcessTreeRefusal::TauLeafWithChildren));

    // Loop with 1 child: InvalidArity.
    let mut bad_loop = ProcessTree::new();
    bad_loop.nodes.push(ProcessTreeNode::Activity("only".into()));
    bad_loop.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Loop,
        children: vec![ProcessTreeNodeId(0)],
    });
    bad_loop.root = Some(ProcessTreeNodeId(1));
    assert_eq!(bad_loop.admit_shape(), Err(ProcessTreeRefusal::InvalidArity));

    // XOR with 1 child: BelowMinimumArity.
    let mut bad_xor = ProcessTree::new();
    bad_xor.nodes.push(ProcessTreeNode::Activity("only".into()));
    bad_xor.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Xor,
        children: vec![ProcessTreeNodeId(0)],
    });
    bad_xor.root = Some(ProcessTreeNodeId(1));
    assert_eq!(bad_xor.admit_shape(), Err(ProcessTreeRefusal::BelowMinimumArity));

    // Dangling node reference.
    let mut dangling = ProcessTree::new();
    dangling.nodes.push(ProcessTreeNode::Activity("a".into()));
    dangling.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Sequence,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(99)],
    });
    dangling.root = Some(ProcessTreeNodeId(1));
    assert_eq!(dangling.admit_shape(), Err(ProcessTreeRefusal::DanglingNodeReference));
}
