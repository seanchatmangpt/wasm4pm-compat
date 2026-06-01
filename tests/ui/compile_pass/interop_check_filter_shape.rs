// Law: DimensionShapeMismatchLaw — FilterShape::ObjectType on a flat Pm4pyShape returns InteropRefusal::DimensionShapeMismatch; Activity filters on flat shapes are permitted
// COMPILE-PASS: check_filter_shape — proves DimensionShapeMismatch for ObjectType filter on flat shape

use wasm4pm_compat::interop::{
    check_filter_shape, FilterShape, InteropRefusal, Pm4pyShape,
};

fn main() {
    // Activity filter on flat log: valid.
    assert!(check_filter_shape(Pm4pyShape::EventLog, FilterShape::Activity).is_ok());

    // ObjectType filter on object-centric log: valid.
    assert!(check_filter_shape(Pm4pyShape::ObjectCentricLog, FilterShape::ObjectType).is_ok());

    // ObjectType filter on flat log: DimensionShapeMismatch.
    assert_eq!(
        check_filter_shape(Pm4pyShape::EventLog, FilterShape::ObjectType),
        Err(InteropRefusal::DimensionShapeMismatch)
    );
}
