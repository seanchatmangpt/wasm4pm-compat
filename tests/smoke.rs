//! Smoke tests for the Process Canon Shape (part A): eventlog, ocel, petri, dfg.
//!
//! These tests construct each structural shape and exercise its `validate`
//! surface — both the admitting (Ok) and the refusing (named-law Err) path.
//! They assert *structure only*; nothing here mines, discovers, or replays.
//!
//! Agent 3b owns `tests/smoke_models.rs` (xes/bpmn/powl/process-tree/declare/
//! ocpq/etc.), so there is no function-name clash with this file.

use wasm4pm_compat::dfg::{Dfg, DfgEdge, DfgNode, DfgRefusal};
use wasm4pm_compat::eventlog::{Event, EventLog, EventLogRefusal, EventStream, Trace};
use wasm4pm_compat::ocel::{
    EventObjectLink, Object, ObjectChange, ObjectObjectLink, OcelEvent, OcelLog, OcelRefusal,
};
use wasm4pm_compat::petri::{
    Arc, Marking, ObjectCentricPetriNet, PetriNet, PetriRefusal, Place, SoundnessClaimed,
    SoundnessWitnessed, Transition, WfNet,
};

#[test]
fn smoke_eventlog() {
    // Build event -> trace -> log and check accessors.
    let trace = Trace::new(
        "case-1",
        [
            Event::new("a").at_ns(1).by("alice").with_lifecycle("start"),
            Event::new("b").at_ns(2),
        ],
    );
    assert_eq!(trace.case_id(), "case-1");
    assert_eq!(trace.len(), 2);
    assert_eq!(trace.events()[0].activity(), "a");
    assert_eq!(trace.events()[0].resource(), Some("alice"));
    assert_eq!(trace.events()[0].lifecycle(), Some("start"));

    let log = EventLog::from_traces([trace]);
    assert_eq!(log.trace_count(), 1);
    assert_eq!(log.event_count(), 2);
    assert!(log.validate().is_ok());

    // Refusal: empty trace is refused by a specific named law.
    let bad = EventLog::from_traces([Trace::from_events([])]);
    assert_eq!(bad.validate(), Err(EventLogRefusal::EmptyTrace));

    // Refusal: non-monotonic timestamps are refused.
    let non_monotonic = Trace::new("c", [Event::new("a").at_ns(9), Event::new("b").at_ns(1)]);
    assert_eq!(
        non_monotonic.validate(),
        Err(EventLogRefusal::NonMonotonicTrace)
    );

    // EventStream is the online sibling.
    let mut stream = EventStream::new();
    assert!(stream.is_empty());
    stream.push(Event::new("x"));
    assert_eq!(stream.len(), 1);
}

#[test]
fn smoke_ocel() {
    // A well-shaped OCEL: objects, an event, E2O + O2O links, a change.
    let log = OcelLog::new(
        [Object::new("ord-1", "order"), Object::new("item-9", "item")],
        [OcelEvent::new("e1", "place_order").at_ns(1)],
        [
            EventObjectLink::new("e1", "ord-1").qualified("places"),
            EventObjectLink::new("e1", "item-9"),
        ],
        [ObjectObjectLink::new("ord-1", "item-9").qualified("contains")],
        [ObjectChange::new("ord-1", "status", "placed")],
    );
    assert_eq!(log.objects().len(), 2);
    assert_eq!(log.events().len(), 1);
    assert_eq!(log.event_object_links().len(), 2);
    assert!(log.validate().is_ok());

    // Refusal: dangling E2O link (object "ghost" undeclared).
    let dangling = OcelLog::new(
        [Object::new("ord-1", "order")],
        [OcelEvent::new("e1", "a")],
        [EventObjectLink::new("e1", "ghost")],
        [],
        [],
    );
    assert_eq!(
        dangling.validate(),
        Err(OcelRefusal::DanglingEventObjectLink)
    );

    // Refusal: an OCEL with no E2O links is empty.
    let no_links = OcelLog::new(
        [Object::new("ord-1", "order")],
        [OcelEvent::new("e1", "a")],
        [],
        [],
        [],
    );
    assert_eq!(no_links.validate(), Err(OcelRefusal::EmptyEventObjectLinks));
}

#[test]
#[allow(deprecated)]
fn smoke_petri() {
    // A small WF-net: src --t--> snk, marked and with a final marking.
    let net = PetriNet::new(
        [Place::new("src"), Place::new("snk")],
        [Transition::new("t", "a")],
        [
            Arc::place_to_transition("src", "t"),
            Arc::transition_to_place("t", "snk"),
        ],
        Marking::new([("src".to_string(), 1)]),
    );
    assert!(net.validate().is_ok());

    let wf = WfNet::new(net.clone(), Marking::new([("snk".to_string(), 1)]));
    assert!(wf.validate().is_ok());
    assert!(wf.final_marking().is_some());

    // Soundness is a typestate CLAIM, never computed here. Walk the markers.
    let claimed: WfNet<SoundnessClaimed> = wf.claim_sound();
    let _witnessed: WfNet<SoundnessWitnessed> = claimed.attest_witnessed();

    // Refusal: a WF-net without a final marking is refused by a named law.
    let no_final = WfNet::new(net, Marking::empty());
    assert_eq!(no_final.validate(), Err(PetriRefusal::MissingFinalMarking));

    // OC-Petri-net: typed arc must name a declared object type.
    let oc_net = PetriNet::new(
        [Place::new("p")],
        [Transition::new("t", "a")],
        [Arc::place_to_transition("p", "t").typed("ghost", false)],
        Marking::empty(),
    );
    let ocpn = ObjectCentricPetriNet::new(oc_net, ["order".to_string()]);
    assert_eq!(ocpn.validate(), Err(PetriRefusal::ObjectTypeNotPreserved));
}

#[test]
fn smoke_dfg() {
    // A two-node DFG with one weighted edge.
    let g = Dfg::new(
        [DfgNode::new("a"), DfgNode::new("b")],
        [DfgEdge::new("a", "b", 4)],
    );
    assert_eq!(g.nodes().len(), 2);
    assert_eq!(g.edges()[0].weight().count(), 4);
    assert!(g.validate().is_ok());

    // Refusal: edge to an undeclared node.
    let dangling = Dfg::new([DfgNode::new("a")], [DfgEdge::new("a", "ghost", 1)]);
    assert_eq!(dangling.validate(), Err(DfgRefusal::DanglingEdge));

    // Refusal: an empty graph.
    let empty = Dfg::new([], []);
    assert_eq!(empty.validate(), Err(DfgRefusal::EmptyGraph));
}
