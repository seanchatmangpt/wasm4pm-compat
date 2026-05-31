// COMPILE-PASS: Pm4pyShape and FilterShape variants are constructible and
// check_filter_shape accepts lawful combinations — covers pm4py interop surface.
//
// Law: DimensionShapeMismatch — FilterShape::ObjectType over a flat Pm4pyShape
// is refused at runtime; all other combinations are structurally admissible.

use wasm4pm_compat::interop::{
    check_filter_shape, FilterShape, FilterShapeConst, Pm4pyShape,
    assert_filter_oc_compatible,
};

fn check_pm4py_shapes() {
    // All Pm4pyShape variants are constructible.
    let shapes = [
        Pm4pyShape::EventLog,
        Pm4pyShape::ObjectCentricLog,
        Pm4pyShape::PetriNet,
        Pm4pyShape::ProcessTree,
        Pm4pyShape::Bpmn,
        Pm4pyShape::DirectlyFollowsGraph,
        Pm4pyShape::Declare,
    ];
    assert_eq!(shapes.len(), 7);

    // Tag and is_object_centric produce stable values.
    assert_eq!(Pm4pyShape::EventLog.tag(), "event_log");
    assert_eq!(Pm4pyShape::ObjectCentricLog.tag(), "ocel");
    assert!(!Pm4pyShape::EventLog.is_object_centric());
    assert!(Pm4pyShape::ObjectCentricLog.is_object_centric());
}

fn check_filter_shapes() {
    // All FilterShape variants are constructible.
    let _activity = FilterShape::Activity;
    let _timeframe = FilterShape::Timeframe;
    let _variant = FilterShape::Variant;
    let _attribute = FilterShape::Attribute;
    let _object_type = FilterShape::ObjectType;
}

fn check_filter_shape_law() {
    // Lawful combinations: non-object-type filter over any shape.
    assert!(check_filter_shape(Pm4pyShape::EventLog, FilterShape::Activity).is_ok());
    assert!(check_filter_shape(Pm4pyShape::EventLog, FilterShape::Timeframe).is_ok());
    assert!(check_filter_shape(Pm4pyShape::EventLog, FilterShape::Variant).is_ok());
    assert!(check_filter_shape(Pm4pyShape::EventLog, FilterShape::Attribute).is_ok());

    // Object-centric filter over object-centric shape is lawful.
    assert!(check_filter_shape(Pm4pyShape::ObjectCentricLog, FilterShape::ObjectType).is_ok());

    // Object-centric filter over flat shape is refused.
    assert!(check_filter_shape(Pm4pyShape::EventLog, FilterShape::ObjectType).is_err());
}

fn check_const_filter_shape_law() {
    // FilterShapeConst<true> (object-centric) satisfies RequiresObjectCentric.
    assert_filter_oc_compatible(&FilterShapeConst::<true>);
}

fn main() {
    check_pm4py_shapes();
    check_filter_shapes();
    check_filter_shape_law();
    check_const_filter_shape_law();
}
