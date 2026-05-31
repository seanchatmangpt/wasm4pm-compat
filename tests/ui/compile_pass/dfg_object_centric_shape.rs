// COMPILE-PASS: ObjectCentricDfg constructs lawfully — covers the object-centric
// directly-follows graph surface absent from existing dfg_shape fixture.
//
// Law: object-centric DFG — one DFG per object type, not a single flat graph.
// ObjectCentricDfg is structure-only; DFG discovery from an OCEL log graduates
// to wasm4pm. This fixture proves the per-type DFG carrier is constructible.
use wasm4pm_compat::dfg::{Dfg, DfgEdge, DfgEdgeFull, DfgFrequency, DfgNode, ObjectCentricDfg};

fn main() {
    // An empty ObjectCentricDfg is valid.
    let empty = ObjectCentricDfg::new();
    assert!(empty.per_type.is_empty());
    assert!(empty.get("order").is_none());

    // Register DFGs for two object types: "order" and "item".
    let order_dfg = Dfg::new(
        [DfgNode::new("place-order"), DfgNode::new("pay"), DfgNode::new("ship")],
        [
            DfgEdge::new("place-order", "pay", 10),
            DfgEdge::new("pay", "ship", 9),
        ],
    );
    let item_dfg = Dfg::new(
        [DfgNode::new("pick"), DfgNode::new("pack")],
        [DfgEdge::new("pick", "pack", 7)],
    );

    let oc = ObjectCentricDfg::new()
        .with_type_dfg("order", order_dfg)
        .with_type_dfg("item", item_dfg);

    assert_eq!(oc.per_type.len(), 2);

    // Look up by object type.
    let found_order = oc.get("order");
    assert!(found_order.is_some());
    let found_order_dfg = found_order.unwrap();
    assert!(found_order_dfg.validate().is_ok());
    assert_eq!(found_order_dfg.nodes().len(), 3);
    assert_eq!(found_order_dfg.edges().len(), 2);

    let found_item = oc.get("item");
    assert!(found_item.is_some());
    assert!(found_item.unwrap().validate().is_ok());

    assert!(oc.get("invoice").is_none());

    // Object types iterator.
    let types: Vec<&str> = oc.object_types().collect();
    assert_eq!(types, vec!["order", "item"]);

    // DfgEdgeFull carries frequency + optional duration — covers the annotated edge surface.
    let full_edge = DfgEdgeFull::new("place-order", "pay", 10);
    assert_eq!(full_edge.frequency(), DfgFrequency(10));
    assert!(full_edge.duration_ns().is_none());

    let full_edge_timed = DfgEdgeFull::new("pay", "ship", 9).with_duration_ns(500_000);
    assert_eq!(full_edge_timed.frequency(), DfgFrequency(9));
    assert!(full_edge_timed.duration_ns().is_some());
}
