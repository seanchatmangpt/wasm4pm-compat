// COMPILE-PASS: ProcessTreeNodeId has total ordering — usable as map key.
//
// Law: ProcessTreeNodeId is repr(transparent) over usize; it derives Ord and
// Hash. This fixture proves ordering and equality hold without any allocation.
use wasm4pm_compat::process_tree::ProcessTreeNodeId;

fn main() {
    let a = ProcessTreeNodeId(0);
    let b = ProcessTreeNodeId(1);
    let c = ProcessTreeNodeId(1);

    assert!(a < b);
    assert_eq!(b, c);
    assert!(b >= a);
    assert_eq!(a.0, 0);
}
