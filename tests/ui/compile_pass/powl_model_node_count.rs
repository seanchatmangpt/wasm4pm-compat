// COMPILE-PASS: Powl model node_count tracks pushed nodes correctly.
//
// Law: Powl is a complete model shape — nodes, edges, root. node_count() is
// a pure structural query; it does not execute or replay. This fixture proves
// the shape is mutable and trackable.
use wasm4pm_compat::powl::{Powl, PowlNode, PowlNodeId, PowlNodeKind};

fn main() {
    let mut p = Powl::new();
    assert_eq!(p.node_count(), 0);
    assert!(p.root.is_none());

    p.nodes.push(PowlNode::new(PowlNodeId(0), PowlNodeKind::Atom("a".into())));
    p.nodes.push(PowlNode::new(PowlNodeId(1), PowlNodeKind::Atom("b".into())));
    p.nodes.push(PowlNode::new(
        PowlNodeId(2),
        PowlNodeKind::PartialOrder(vec![PowlNodeId(0), PowlNodeId(1)]),
    ));
    p.root = Some(PowlNodeId(2));

    assert_eq!(p.node_count(), 3);
    assert_eq!(p.root, Some(PowlNodeId(2)));
}
