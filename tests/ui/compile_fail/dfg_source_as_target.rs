// COMPILE-FAIL: DFG endpoint law — DfgSourceMarker cannot satisfy IsDfgTarget.
// Law: IsDfgSource and IsDfgTarget are sealed distinct traits.
// A DFG source marker must not be confused with a DFG target marker.
use wasm4pm_compat::dfg::{DfgSourceMarker, IsDfgTarget};

fn needs_target<T: IsDfgTarget>(_: T) {}

fn main() {
    // DfgSourceMarker does not implement IsDfgTarget.
    needs_target(DfgSourceMarker);
}
