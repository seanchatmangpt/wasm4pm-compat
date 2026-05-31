// COMPILE-PASS: BpmnProcess and BpmnEdge construct lawfully with typed nodes
// and edges — covers the full bpmn graph structure carrier.
//
// Law: BPMN 2.0 graph shape — a process is a set of identified nodes connected
// by sequence-flow edges. Structural validation checks: non-empty, unique ids,
// a start event, an end event, and no dangling edges. Execution, soundness
// analysis, and BPMN↔Petri conversion graduate to wasm4pm.
use wasm4pm_compat::bpmn::{
    BpmnEdge, BpmnEvent, BpmnGateway, BpmnNode, BpmnProcess, BpmnRefusal, BpmnTask,
};

fn main() {
    // Minimal well-formed process: Start → Task → End.
    let minimal = BpmnProcess::new(
        [
            BpmnNode::event("s", BpmnEvent::Start),
            BpmnNode::task("t", BpmnTask::new("approve")),
            BpmnNode::event("e", BpmnEvent::End),
        ],
        [BpmnEdge::new("s", "t"), BpmnEdge::new("t", "e")],
    );
    assert!(minimal.validate().is_ok());
    assert_eq!(minimal.nodes().len(), 3);
    assert_eq!(minimal.edges().len(), 2);

    // BpmnEdge accessors.
    let edge = BpmnEdge::new("a", "b");
    assert_eq!(edge.source(), "a");
    assert_eq!(edge.target(), "b");

    // A more complex process with a parallel split.
    let parallel = BpmnProcess::new(
        [
            BpmnNode::event("start", BpmnEvent::Start),
            BpmnNode::gateway("and-split", BpmnGateway::Parallel),
            BpmnNode::task("task-a", BpmnTask::new("review")),
            BpmnNode::task("task-b", BpmnTask::new("notify")),
            BpmnNode::gateway("and-join", BpmnGateway::Parallel),
            BpmnNode::event("end", BpmnEvent::End),
        ],
        [
            BpmnEdge::new("start", "and-split"),
            BpmnEdge::new("and-split", "task-a"),
            BpmnEdge::new("and-split", "task-b"),
            BpmnEdge::new("task-a", "and-join"),
            BpmnEdge::new("task-b", "and-join"),
            BpmnEdge::new("and-join", "end"),
        ],
    );
    assert!(parallel.validate().is_ok());

    // Missing start event is refused with the named law.
    let no_start = BpmnProcess::new(
        [
            BpmnNode::task("t", BpmnTask::new("x")),
            BpmnNode::event("e", BpmnEvent::End),
        ],
        [BpmnEdge::new("t", "e")],
    );
    assert_eq!(no_start.validate(), Err(BpmnRefusal::MissingStartEvent));

    // Missing end event is refused with the named law.
    let no_end = BpmnProcess::new(
        [
            BpmnNode::event("s", BpmnEvent::Start),
            BpmnNode::task("t", BpmnTask::new("x")),
        ],
        [BpmnEdge::new("s", "t")],
    );
    assert_eq!(no_end.validate(), Err(BpmnRefusal::MissingEndEvent));

    // Dangling edge is refused with the named law.
    let dangling = BpmnProcess::new(
        [
            BpmnNode::event("s", BpmnEvent::Start),
            BpmnNode::event("e", BpmnEvent::End),
        ],
        [BpmnEdge::new("s", "ghost")],
    );
    assert_eq!(dangling.validate(), Err(BpmnRefusal::DanglingEdge));

    // A process with no edges but valid start + end is well-formed at shape level.
    let disconnected_ok = BpmnProcess::new(
        [
            BpmnNode::event("s", BpmnEvent::Start),
            BpmnNode::event("e", BpmnEvent::End),
        ],
        [],
    );
    // No dangling edges — shape is valid (connectivity check is an engine concern).
    assert!(disconnected_ok.validate().is_ok());
}
