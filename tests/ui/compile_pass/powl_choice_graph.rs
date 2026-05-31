// Compile-pass fixture: ChoiceGraph variant and ChoiceGraphEdge can be
// constructed; ChoiceGraphEdge and OrderEdge are distinct types.
//
// Law: Kourani et al. (2026) Definition 3.6 — choice graph γ = (N, E) where
// N = X ∪ {▷, □} with a unique start and end node.

use wasm4pm_compat::powl::{
    ChoiceGraphEdge, OrderEdge, Powl, PowlNode, PowlNodeId, PowlNodeKind,
};

fn main() {
    // Start node (▷ = id 0), two decision nodes (1, 2), end node (□ = 3).
    let start = PowlNodeId(0);
    let x1 = PowlNodeId(1);
    let x2 = PowlNodeId(2);
    let end = PowlNodeId(3);

    // Edges: start→x1, start→x2, x1→end, x2→end.
    let edges = vec![
        ChoiceGraphEdge::new(start, x1),
        ChoiceGraphEdge::new(start, x2),
        ChoiceGraphEdge::new(x1, end),
        ChoiceGraphEdge::new(x2, end),
    ];

    let cg_node = PowlNode::new(
        PowlNodeId(10),
        PowlNodeKind::ChoiceGraph {
            nodes: vec![start, x1, x2, end],
            edges,
        },
    );

    // ChoiceGraph variant is distinct from Choice (flat list).
    assert!(matches!(cg_node.kind, PowlNodeKind::ChoiceGraph { .. }));

    // An OrderEdge and a ChoiceGraphEdge carry the same fields but are
    // distinct types — a function accepting one does not accept the other.
    let order_edge = OrderEdge::new(PowlNodeId(0), PowlNodeId(1));
    let choice_edge = ChoiceGraphEdge::new(PowlNodeId(0), PowlNodeId(1));
    // Same structural fields, distinct types.
    assert_eq!(order_edge.from, choice_edge.from);
    assert_eq!(order_edge.to, choice_edge.to);

    // Build a Powl model containing the ChoiceGraph node.
    let mut powl = Powl::new();
    powl.nodes.push(PowlNode::new(PowlNodeId(0), PowlNodeKind::Atom("a".into())));
    powl.nodes.push(cg_node);
    assert_eq!(powl.node_count(), 2);
}
