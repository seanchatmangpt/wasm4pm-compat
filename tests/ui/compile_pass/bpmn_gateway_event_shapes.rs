// COMPILE-PASS: BpmnGateway and BpmnEvent enum variants construct lawfully —
// covers bpmn module beyond pool/lane which is already tested.
//
// Law: BPMN 2.0 graph shape — gateway variants annotate how flow diverges or
// converges; event variants annotate start/intermediate/end/boundary roles.
// Both are structure-only graph labels with no execution semantics here.
use wasm4pm_compat::bpmn::{BpmnEvent, BpmnGateway, BpmnNode, BpmnNodeKind};

fn main() {
    // All five gateway variants are constructible.
    let exclusive = BpmnGateway::Exclusive;
    let parallel = BpmnGateway::Parallel;
    let inclusive = BpmnGateway::Inclusive;
    let event_based = BpmnGateway::EventBased;
    let complex = BpmnGateway::Complex;

    // Equality is structural.
    assert_eq!(exclusive, BpmnGateway::Exclusive);
    assert_ne!(exclusive, parallel);

    // All four event variants are constructible.
    let start = BpmnEvent::Start;
    let intermediate = BpmnEvent::Intermediate;
    let end = BpmnEvent::End;
    let boundary = BpmnEvent::Boundary;

    assert_eq!(start, BpmnEvent::Start);
    assert_ne!(start, end);

    // BpmnNode wraps gateway and event kinds with an id.
    let gw_node = BpmnNode::gateway("gw1", BpmnGateway::Inclusive);
    assert_eq!(gw_node.id(), "gw1");
    assert!(matches!(gw_node.kind(), BpmnNodeKind::Gateway(BpmnGateway::Inclusive)));

    let ev_start = BpmnNode::event("start", BpmnEvent::Start);
    assert_eq!(ev_start.id(), "start");
    assert!(matches!(ev_start.kind(), BpmnNodeKind::Event(BpmnEvent::Start)));

    let ev_intermediate = BpmnNode::event("mid", BpmnEvent::Intermediate);
    assert!(matches!(ev_intermediate.kind(), BpmnNodeKind::Event(BpmnEvent::Intermediate)));

    let ev_boundary = BpmnNode::event("b1", BpmnEvent::Boundary);
    assert!(matches!(ev_boundary.kind(), BpmnNodeKind::Event(BpmnEvent::Boundary)));

    let gw_parallel = BpmnNode::gateway("and-split", BpmnGateway::Parallel);
    assert!(matches!(gw_parallel.kind(), BpmnNodeKind::Gateway(BpmnGateway::Parallel)));

    let gw_event_based = BpmnNode::gateway("eb", BpmnGateway::EventBased);
    assert!(matches!(gw_event_based.kind(), BpmnNodeKind::Gateway(BpmnGateway::EventBased)));

    let gw_complex = BpmnNode::gateway("cx", BpmnGateway::Complex);
    assert!(matches!(gw_complex.kind(), BpmnNodeKind::Gateway(BpmnGateway::Complex)));

    // Use the variants to avoid dead-code warnings.
    let _ = (exclusive, parallel, inclusive, event_based, complex);
    let _ = (start, intermediate, end, boundary);
}
