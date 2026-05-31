// COMPILE-PASS: BpmnGateway::Exclusive — proves lawful construction of an
// exclusive (XOR) gateway node and its embedding in a valid BpmnProcess graph.
//
// Law: BPMN 2.0 graph shape — a gateway node is an identified vertex annotated
// with a divergence/convergence kind. Exclusive means one outgoing branch is
// taken. This fixture proves the type is constructible and embeds correctly
// in a well-formed process graph.

use wasm4pm_compat::bpmn::{
    BpmnEdge, BpmnEvent, BpmnGateway, BpmnNode, BpmnNodeKind, BpmnProcess, BpmnTask,
};

fn main() {
    // BpmnGateway::Exclusive is the XOR gateway kind.
    let gw = BpmnGateway::Exclusive;
    assert_eq!(gw, BpmnGateway::Exclusive);

    // A gateway node wraps the kind with an id.
    let node = BpmnNode::gateway("g1", BpmnGateway::Exclusive);
    assert_eq!(node.id(), "g1");
    assert!(matches!(node.kind(), BpmnNodeKind::Gateway(BpmnGateway::Exclusive)));

    // All five gateway variants are constructible.
    let _parallel = BpmnGateway::Parallel;
    let _inclusive = BpmnGateway::Inclusive;
    let _event_based = BpmnGateway::EventBased;
    let _complex = BpmnGateway::Complex;

    // Embed an exclusive gateway in a well-formed process:
    // Start → XOR-split → branch A → End
    //                   → branch B → End  (two end events allowed)
    let process = BpmnProcess::new(
        [
            BpmnNode::event("start", BpmnEvent::Start),
            BpmnNode::gateway("xor", BpmnGateway::Exclusive),
            BpmnNode::task("task-a", BpmnTask::new("approve")),
            BpmnNode::task("task-b", BpmnTask::new("reject")),
            BpmnNode::event("end-a", BpmnEvent::End),
            BpmnNode::event("end-b", BpmnEvent::End),
        ],
        [
            BpmnEdge::new("start", "xor"),
            BpmnEdge::new("xor", "task-a"),
            BpmnEdge::new("xor", "task-b"),
            BpmnEdge::new("task-a", "end-a"),
            BpmnEdge::new("task-b", "end-b"),
        ],
    );

    // Graph-shape validation succeeds: nodes declared, start/end present,
    // all edges connect declared nodes.
    assert!(process.validate().is_ok());

    // The gateway node can be located by id in the process.
    let found = process.nodes().iter().find(|n| n.id() == "xor");
    assert!(found.is_some());
    assert!(matches!(
        found.unwrap().kind(),
        BpmnNodeKind::Gateway(BpmnGateway::Exclusive)
    ));
}
