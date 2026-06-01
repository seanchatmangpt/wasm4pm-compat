// COMPILE-FAIL: DFG endpoint law — DfgTargetMarker cannot satisfy IsDfgSource.
// Law: IsDfgSource and IsDfgTarget are sealed distinct traits.
// A DFG target marker must not be confused with a DFG source marker.
use wasm4pm_compat::dfg::{DfgTargetMarker, IsDfgSource};

fn needs_source<S: IsDfgSource>(_: S) {}

fn main() {
    // DfgTargetMarker does not implement IsDfgSource.
    needs_source(DfgTargetMarker);
}
