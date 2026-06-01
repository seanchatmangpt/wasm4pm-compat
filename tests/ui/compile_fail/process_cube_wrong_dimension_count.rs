// COMPILE-FAIL: ProcessCube dimension count law — wrong DIMS violates compile-time constraint.
//
// Law: ProcessCubeDimensionLaw — CubeCell<2> and CubeCell<3> are distinct types.
// A function that demands a 3-dimensional cell cannot receive a 2-dimensional cell.
// The DIMS const generic is the type-level receipt of the cube's dimensional structure.
use wasm4pm_compat::process_cube::CubeCell;

fn requires_three_dim_cell(_cell: CubeCell<3>) {}

fn main() {
    let cell: CubeCell<2> = CubeCell::new();
    // This must fail: CubeCell<2> is not CubeCell<3>.
    // The DIMS const generic distinguishes dimensionality at the type level.
    requires_three_dim_cell(cell);
}
