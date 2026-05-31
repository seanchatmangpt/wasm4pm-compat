// Compile-pass fixture: BpmnPool and BpmnLane can be constructed and validated.
//
// Law: BPMN 2.0 (Real-Life BPMN 4th ed.) — Pool is the top-level container
// for a process participant; Lane is a swimlane subdividing a pool for
// organisational roles.

use wasm4pm_compat::bpmn::{
    BpmnEdge, BpmnEvent, BpmnLane, BpmnNode, BpmnPool, BpmnProcess, BpmnTask,
};

fn main() {
    // Build a simple process: Start → approve → End.
    let process = BpmnProcess::new(
        [
            BpmnNode::event("s", BpmnEvent::Start),
            BpmnNode::task("t", BpmnTask::new("approve")),
            BpmnNode::event("e", BpmnEvent::End),
        ],
        [BpmnEdge::new("s", "t"), BpmnEdge::new("t", "e")],
    );

    // Build a lane that claims the task node.
    let lane = BpmnLane::new("lane-ops", "Operations", ["t"]);
    assert_eq!(lane.id(), "lane-ops");
    assert_eq!(lane.name(), "Operations");
    assert_eq!(lane.node_ids().len(), 1);

    // Build a pool wrapping the process and the lane.
    let pool = BpmnPool::new("pool-1", "Claims", process, [lane]);
    assert_eq!(pool.id(), "pool-1");
    assert_eq!(pool.name(), "Claims");
    assert_eq!(pool.lanes().len(), 1);

    // The pool validates: process is well-formed, lane references declared nodes.
    assert!(pool.validate().is_ok());

    // A pool with no lanes is also valid.
    let process2 = BpmnProcess::new(
        [
            BpmnNode::event("s2", BpmnEvent::Start),
            BpmnNode::event("e2", BpmnEvent::End),
        ],
        [BpmnEdge::new("s2", "e2")],
    );
    let pool_no_lanes = BpmnPool::new("pool-2", "Empty", process2, []);
    assert!(pool_no_lanes.validate().is_ok());
}
