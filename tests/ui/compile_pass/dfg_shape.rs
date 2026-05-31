// COMPILE-PASS: DFG shape — a directly-follows graph compiles with nodes, edges,
// and weights.
//
// Law: DFG is structure-only; discovery from an event log is an engine and
// graduates to wasm4pm. DfgRefusal::DiscoveryRequired enforces this boundary
// at admission time; this fixture proves the structural shape compiles.
use wasm4pm_compat::dfg::{Dfg, DfgEdge, DfgNode, DfgWeight};

fn main() {
    // Build a simple DFG: register → review → approve
    let dfg = Dfg::new(
        [
            DfgNode::new("register"),
            DfgNode::new("review"),
            DfgNode::new("approve"),
        ],
        [
            DfgEdge::new("register", "review", 42u64),
            DfgEdge::new("review", "approve", 38u64),
        ],
    );

    assert!(dfg.validate().is_ok());
    assert_eq!(dfg.nodes().len(), 3);
    assert_eq!(dfg.edges().len(), 2);

    // DfgWeight is a transparent u64 wrapper
    let w = DfgWeight(100);
    assert_eq!(w.count(), 100);

    // A DFG with no edges but a single node is valid at the shape level.
    let single_node_dfg = Dfg::new([DfgNode::new("a")], []);
    assert!(single_node_dfg.validate().is_ok());
}
