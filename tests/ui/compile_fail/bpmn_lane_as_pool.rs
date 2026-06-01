// COMPILE-FAIL: BPMN structural law — BpmnLane cannot be passed where BpmnPool is required.
// Law: BpmnLane and BpmnPool are distinct structural types. A swim lane cannot be
// confused with a pool container. The type system enforces this at compile time.
use wasm4pm_compat::bpmn::{BpmnLane, BpmnPool};

fn requires_pool(_pool: BpmnPool) {}

fn main() {
    let lane = BpmnLane::new("l1", "Operations", ["t1"]);
    // This must fail: BpmnLane is not BpmnPool.
    requires_pool(lane);
}
