// Law: Pm4pyShapeObjectCentricDistinctionLaw — only Pm4pyShape::ObjectCentricLog is object-centric; EventLog and PetriNet are flat shapes
// COMPILE-PASS: Pm4pyShape::is_object_centric — proves only ObjectCentricLog returns true

use wasm4pm_compat::interop::Pm4pyShape;

fn main() {
    assert!(Pm4pyShape::ObjectCentricLog.is_object_centric());
    assert!(!Pm4pyShape::EventLog.is_object_centric());
    assert!(!Pm4pyShape::PetriNet.is_object_centric());
    assert!(!Pm4pyShape::Bpmn.is_object_centric());
}
