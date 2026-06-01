// COMPILE-FAIL: BPMN structural law — BpmnGateway cannot be passed where BpmnEvent is required.
// Law: BpmnGateway (routing node: XOR, AND, OR) and BpmnEvent (start, end, intermediate)
// are distinct enum types. A gateway must not be confused with an event node.
use wasm4pm_compat::bpmn::{BpmnEvent, BpmnGateway};

fn requires_bpmn_event(_e: BpmnEvent) {}

fn main() {
    let gateway = BpmnGateway::Exclusive;
    // This must fail: BpmnGateway is not BpmnEvent.
    requires_bpmn_event(gateway);
}
