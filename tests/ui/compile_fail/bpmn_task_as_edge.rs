// COMPILE-FAIL: BPMN structural law — BpmnTask cannot be passed where BpmnEdge is required.
// Law: BpmnTask (activity node) and BpmnEdge (sequence flow) are distinct types.
// An activity node must not be confused with a control-flow edge.
use wasm4pm_compat::bpmn::{BpmnEdge, BpmnTask};

fn requires_edge(_edge: BpmnEdge) {}

fn main() {
    let task = BpmnTask::new("approve");
    // This must fail: BpmnTask is not BpmnEdge.
    requires_edge(task);
}
