// COMPILE-FAIL: POWL law — ExceedsProcessTree cannot satisfy TreeProjectable.
// Law: TreeProjectable is sealed to ProcessTreeProjectable only.
// A POWL that exceeds process-tree structure cannot be declared tree-projectable.
use wasm4pm_compat::powl::{assert_tree_projectable, ExceedsProcessTree};

fn main() {
    // ExceedsProcessTree does NOT implement TreeProjectable.
    assert_tree_projectable(ExceedsProcessTree);
}
