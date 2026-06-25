use wasm4pm_compat::powl::{
    ChoiceGraphEdge, OrderEdge, Powl, PowlBuilder, PowlNode, PowlNodeId, PowlNodeKind,
};

#[test]
fn test_powl_node_id_serde() {
    let id = PowlNodeId(42);
    let serialized = serde_json::to_string(&id).unwrap();
    assert_eq!(serialized, "42");

    let deserialized: PowlNodeId = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, id);
}

#[test]
fn test_choice_graph_edge_serde() {
    let edge = ChoiceGraphEdge::new(PowlNodeId(1), PowlNodeId(2));
    let serialized = serde_json::to_string(&edge).unwrap();
    let deserialized: ChoiceGraphEdge = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, edge);
    assert_eq!(deserialized.from, PowlNodeId(1));
    assert_eq!(deserialized.to, PowlNodeId(2));
}

#[test]
fn test_order_edge_serde() {
    let edge = OrderEdge::new(PowlNodeId(3), PowlNodeId(4));
    let serialized = serde_json::to_string(&edge).unwrap();
    let deserialized: OrderEdge = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, edge);
    assert_eq!(deserialized.from, PowlNodeId(3));
    assert_eq!(deserialized.to, PowlNodeId(4));
}

#[test]
fn test_powl_node_kind_serde() {
    let kinds = vec![
        PowlNodeKind::Start,
        PowlNodeKind::End,
        PowlNodeKind::Atom("activity_a".to_string()),
        PowlNodeKind::Silent,
        PowlNodeKind::PartialOrder(vec![PowlNodeId(1), PowlNodeId(2)]),
        PowlNodeKind::ChoiceGraph {
            nodes: vec![PowlNodeId(1), PowlNodeId(2)],
            edges: vec![ChoiceGraphEdge::new(PowlNodeId(1), PowlNodeId(2))],
        },
    ];

    for kind in kinds {
        let serialized = serde_json::to_string(&kind).unwrap();
        let deserialized: PowlNodeKind = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, kind);
    }
}

#[test]
fn test_powl_node_serde() {
    let node = PowlNode::new(PowlNodeId(10), PowlNodeKind::Atom("test".to_string()));
    let serialized = serde_json::to_string(&node).unwrap();
    let deserialized: PowlNode = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, node);
}

#[test]
fn test_powl_full_model_serde() {
    let powl = PowlBuilder::new()
        .atom("START")
        .atom("END")
        .atom("task_a")
        .atom("task_b")
        .partial_order("concurrent_production", &["task_a", "task_b"], &[])
        .atom("review")
        .atom("finalize")
        .choice_graph(
            "top_level",
            &[
                "START",
                "concurrent_production",
                "review",
                "finalize",
                "END",
            ],
            &[
                ("START", "concurrent_production"),
                ("concurrent_production", "review"),
                ("review", "finalize"),
                ("review", "concurrent_production"),
                ("finalize", "END"),
            ],
        )
        .root("top_level")
        .build()
        .unwrap();

    let serialized = serde_json::to_string(&powl).unwrap();
    let deserialized: Powl = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.node_count(), powl.node_count());
    assert_eq!(deserialized.root, powl.root);
}
