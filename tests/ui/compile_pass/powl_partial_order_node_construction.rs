// COMPILE-PASS: POWL partial order node construction with OrderEdge precedence.
//
// Law: Kourani et al. (2026) §3 — a POWL partial order P(M⁺, ≺) is a DAG of
// child nodes connected by strict precedence edges. OrderEdge carries that law.
use wasm4pm_compat::powl::{OrderEdge, Powl, PowlNode, PowlNodeId, PowlNodeKind, PartialOrder};
use core::marker::PhantomData;

fn main() {
    let a = PowlNodeId(0);
    let b = PowlNodeId(1);
    let c = PowlNodeId(2);

    // Precedence: a → b, a → c (diamond partial order).
    let edges = vec![
        OrderEdge::new(a, b),
        OrderEdge::new(a, c),
    ];

    let po_node: PowlNode<PartialOrder> = PowlNode {
        id: PowlNodeId(3),
        kind: PowlNodeKind::PartialOrder(vec![a, b, c]),
        witness: PhantomData,
    };

    assert_eq!(po_node.id, PowlNodeId(3));
    assert_eq!(edges.len(), 2);
    assert_eq!(edges[0].from, a);
    assert_eq!(edges[1].to, c);
}
