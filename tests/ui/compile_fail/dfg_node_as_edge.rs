// COMPILE-FAIL: DFG structural law — DfgNode cannot be passed where DfgEdge is required.
// Law: DfgNode (an activity node) and DfgEdge (a directed frequency edge between activities)
// are distinct structural types. A node must not be confused with an edge.
use wasm4pm_compat::dfg::{DfgEdge, DfgNode};

fn requires_dfg_edge(_e: DfgEdge) {}

fn main() {
    let node = DfgNode::new("place_order");
    // This must fail: DfgNode is not DfgEdge.
    requires_dfg_edge(node);
}
