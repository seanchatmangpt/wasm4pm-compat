// COMPILE-FAIL: Rejects a FilterShape applied to an incompatible Pm4pyShape —
// proves check_filter_shape enforces the pm4py shape law at compile time.
//
// Law: DimensionShapeMismatch — FilterShape::ObjectType may only be applied to
// an object-centric artifact shape (Pm4pyShape::ObjectCentricLog).  Applying
// it to a flat shape (e.g. Pm4pyShape::EventLog, IS_OC = false) violates the
// pm4py shape law and must be rejected by the type system.
//
// The type-law gate `assert_filter_oc_compatible` requires the sealed bound
// `RequiresObjectCentric`, which only `FilterShapeConst<true>` satisfies.
// Passing `FilterShapeConst::<false>` (a flat artifact) is a compile error.
//
// Expected error: the trait `RequiresObjectCentric` is not implemented for
//   `FilterShapeConst<false>`
use wasm4pm_compat::interop::{assert_filter_oc_compatible, FilterShapeConst};

fn main() {
    // A flat Pm4pyShape (IS_OC = false): EventLog, PetriNet, ProcessTree, etc.
    // Applying FilterShape::ObjectType to such a shape violates
    // DimensionShapeMismatch — the flat artifact cannot carry object-type
    // filter dimensions.
    let flat_artifact = FilterShapeConst::<false>;

    // ERROR: FilterShapeConst<false> does not satisfy RequiresObjectCentric.
    // The pm4py shape law enforces this at compile time.
    assert_filter_oc_compatible(&flat_artifact);
}
