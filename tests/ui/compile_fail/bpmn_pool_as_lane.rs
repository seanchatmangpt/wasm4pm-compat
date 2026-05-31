// COMPILE-FAIL: BpmnPoolLaneConfusionLaw — a BpmnPool cannot be passed where
// a BpmnLane is required.
//
// Law: BPMN 2.0 — a pool is the top-level participant container; a lane is a
// swimlane subdivision *inside* a pool. They are structurally distinct types.
// A BpmnPool holds a process and a Vec<BpmnLane>; a BpmnLane holds a set of
// node-id assignments. Passing a BpmnPool in place of a BpmnLane must be
// rejected by the type system.
//
// Expected error: mismatched types — expected BpmnLane, found BpmnPool.
use wasm4pm_compat::bpmn::{BpmnEdge, BpmnEvent, BpmnLane, BpmnNode, BpmnPool, BpmnProcess};

fn register_lane(_lane: BpmnLane) {}

fn main() {
    let process = BpmnProcess::new(
        [
            BpmnNode::event("s", BpmnEvent::Start),
            BpmnNode::event("e", BpmnEvent::End),
        ],
        [BpmnEdge::new("s", "e")],
    );
    // A BpmnPool is a participant container, not a lane.
    let pool = BpmnPool::new("p1", "Claims", process, []);
    // ERROR: BpmnPool is not BpmnLane — the pool/lane boundary must not be blurred.
    register_lane(pool);
}
