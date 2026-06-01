// COMPILE-PASS: process cube dimensional shapes — proves the typed shapes from
// the Process Cube framework (van der Aalst 2013) compile correctly.
//
// Law: The process cube module is structure-only. CubeSlice, CubeCell,
// ProcessCube, CubeDimension, CubeDimensionKind, CubeProjectionWitness, and
// CellComparison are typed shapes; the cube computation (slicing, dicing,
// cross-cell comparison) graduates to wasm4pm.
use core::marker::PhantomData;
use wasm4pm_compat::process_cube::{
    CellComparison, CubeDimension, CubeDimensionKind, CubeProjectionWitness, CubeSlice,
    ProcessCube,
};

struct MyLog;

fn main() {
    // CubeDimension — distinct types by name const param
    let _resource_dim: CubeDimension<"resource"> = CubeDimension;
    let _time_dim: CubeDimension<"time"> = CubeDimension;

    // CubeSlice — binds a dimension to a value
    let slice: CubeSlice<CubeDimension<"resource">, &str> = CubeSlice {
        dimension: PhantomData,
        value: "Alice",
    };
    assert_eq!(slice.value, "Alice");

    // CubeCell — intersection of N dimension slices
    let cell_2d = wasm4pm_compat::process_cube::CubeCell::<2>::new();
    assert_eq!(cell_2d.dim_count(), 2);

    let cell_3d = wasm4pm_compat::process_cube::CubeCell::<3>::default();
    assert_eq!(cell_3d.dim_count(), 3);

    // ProcessCube — typed metamodel for a log type and dimension count
    let cube: ProcessCube<MyLog, 3> = ProcessCube::new();
    assert_eq!(cube.dimension_count(), 3);

    let cube_default: ProcessCube<MyLog, 2> = ProcessCube::default();
    assert_eq!(cube_default.dimension_count(), 2);

    // CubeProjectionWitness — receipt of a projection step
    let witness: CubeProjectionWitness<4, 2> = CubeProjectionWitness::new();
    assert_eq!(witness.from_dims(), 4);
    assert_eq!(witness.to_dims(), 2);

    // CubeDimensionKind — all variants and Display
    let kinds = [
        CubeDimensionKind::Activity,
        CubeDimensionKind::Resource,
        CubeDimensionKind::Time,
        CubeDimensionKind::DataAttribute,
        CubeDimensionKind::ObjectType,
        CubeDimensionKind::CaseAttribute,
    ];
    let displays: Vec<_> = kinds.iter().map(|k| format!("{}", k)).collect();
    assert_eq!(displays[0], "activity");
    assert_eq!(displays[1], "resource");
    assert_eq!(displays[2], "time");
    assert_eq!(displays[3], "data-attribute");
    assert_eq!(displays[4], "object-type");
    assert_eq!(displays[5], "case-attribute");

    // CellComparison — structural cross-cell comparison shape
    let cmp = CellComparison::<2> {
        cell_a: wasm4pm_compat::process_cube::CubeCell::<2>::new(),
        cell_b: wasm4pm_compat::process_cube::CubeCell::<2>::new(),
    };
    assert_eq!(cmp.cell_a.dim_count(), 2);
    assert_eq!(cmp.cell_b.dim_count(), 2);
}
