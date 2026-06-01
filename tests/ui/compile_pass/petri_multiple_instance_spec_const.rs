#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: MultipleInstanceSpecConstBoundsLaw — MultipleInstanceSpecConst<MIN,MAX> requires MIN <= MAX at compile time; lawful bounds (1 <= 4) compile and prove the constraint is open

// COMPILE-PASS: MultipleInstanceSpecConst — the YAWL nofi bounds 1 ≤ MIN ≤ MAX
// are enforced at compile time. Lawful values construct; this fixture proves the
// lawful path is open.
use wasm4pm_compat::petri::MultipleInstanceSpecConst;

fn main() {
    // MIN=1, MAX=1: tight bound, still lawful
    let s1: MultipleInstanceSpecConst<1, 1> = MultipleInstanceSpecConst::new();
    assert_eq!(s1.min(), 1);
    assert_eq!(s1.max(), 1);

    // MIN=2, MAX=10: normal range
    let s2: MultipleInstanceSpecConst<2, 10> = MultipleInstanceSpecConst::new();
    assert_eq!(s2.min(), 2);
    assert_eq!(s2.max(), 10);
}
