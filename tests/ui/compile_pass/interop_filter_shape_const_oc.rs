// COMPILE-PASS: FilterShapeConst<true> — proves object-centric filter shape const compiles through gate

use wasm4pm_compat::interop::{assert_filter_oc_compatible, FilterShapeConst};

fn main() {
    assert_filter_oc_compatible(&FilterShapeConst::<true>);
}
