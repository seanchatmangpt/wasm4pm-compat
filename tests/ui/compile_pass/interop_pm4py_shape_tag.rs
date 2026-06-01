// Law: Pm4pyShapeStableTagLaw — all Pm4pyShape variants return stable machine-readable tag strings via .tag(); tags are not bare error labels
// COMPILE-PASS: Pm4pyShape::tag — proves all Pm4pyShape variants have stable machine-readable tags

use wasm4pm_compat::interop::Pm4pyShape;

fn main() {
    assert_eq!(Pm4pyShape::EventLog.tag(), "event_log");
    assert_eq!(Pm4pyShape::ObjectCentricLog.tag(), "ocel");
    assert_eq!(Pm4pyShape::PetriNet.tag(), "petri_net");
    assert_eq!(Pm4pyShape::ProcessTree.tag(), "process_tree");
    assert_eq!(Pm4pyShape::Bpmn.tag(), "bpmn");
    assert_eq!(Pm4pyShape::DirectlyFollowsGraph.tag(), "dfg");
    assert_eq!(Pm4pyShape::Declare.tag(), "declare");
}
