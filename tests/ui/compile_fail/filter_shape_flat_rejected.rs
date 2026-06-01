// COMPILE-FAIL: DimensionShapeMismatch law — FilterShapeConst<false> cannot satisfy RequiresObjectCentric.
// Law: An ObjectType filter requires an object-centric artifact shape. Applying it to
// a flat (case-centric) shape violates the pm4py DimensionShapeMismatch law.
use wasm4pm_compat::interop::{assert_filter_oc_compatible, FilterShapeConst};

fn main() {
    // IS_OC=false: flat artifact shape; ObjectType filter is rejected.
    assert_filter_oc_compatible(&FilterShapeConst::<false>);
}
