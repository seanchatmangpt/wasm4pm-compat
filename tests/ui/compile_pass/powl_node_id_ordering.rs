// COMPILE-PASS: PowlNodeId has total ordering — usable as map key.
//
// Law: PowlNodeId is repr(transparent) over usize; it derives Ord and Hash.
// This fixture proves ordering holds without any allocation.
use wasm4pm_compat::powl::PowlNodeId;

fn main() {
    let a = PowlNodeId(0);
    let b = PowlNodeId(2);
    let c = PowlNodeId(2);

    assert!(a < b);
    assert_eq!(b, c);
    assert!(b > a);
    assert_eq!(a.0, 0);
}
