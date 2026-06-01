// Law: FilterShapeVariantsLaw — all FilterShape variants (Activity, Timeframe, ObjectType, Attribute) are constructible; the closed set is exhaustive
// COMPILE-PASS: FilterShape all variants — proves all FilterShape variants construct

use wasm4pm_compat::interop::FilterShape;

fn main() {
    let shapes = [
        FilterShape::Activity,
        FilterShape::Timeframe,
        FilterShape::Variant,
        FilterShape::Attribute,
        FilterShape::ObjectType,
    ];
    assert_eq!(shapes.len(), 5);
}
