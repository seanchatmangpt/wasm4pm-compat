// COMPILE-PASS: ProcessCube dimension typed — CubeCell and ProcessCube const generics
// are lawfully constructed with correct DIMS parameters.
//
// Law: ProcessCubeDimensionLaw — the DIMS const generic distinguishes dimensionality.
use wasm4pm_compat::process_cube::{CubeCell, CubeDimension, CubeProjectionWitness, ProcessCube};

fn requires_two_dim_cell(_cell: CubeCell<2>) {}
fn requires_three_dim_cell(_cell: CubeCell<3>) {}

fn main() {
    let cell2: CubeCell<2> = CubeCell::new();
    let cell3: CubeCell<3> = CubeCell::new();

    // Each cell is accepted by the function demanding its exact dimensionality
    requires_two_dim_cell(cell2);
    requires_three_dim_cell(cell3);

    assert_eq!(CubeCell::<4>::new().dim_count(), 4);

    // CubeDimension name literals are distinct types
    let _resource: CubeDimension<"resource"> = CubeDimension;
    let _time: CubeDimension<"time"> = CubeDimension;

    // Projection witness carries both from and to dims
    let w: CubeProjectionWitness<4, 2> = CubeProjectionWitness::new();
    assert_eq!(w.from_dims(), 4);
    assert_eq!(w.to_dims(), 2);

    // ProcessCube with named dimensions
    struct MyLog;
    let cube: ProcessCube<MyLog, 3> = ProcessCube::new();
    assert_eq!(cube.dimension_count(), 3);
}
