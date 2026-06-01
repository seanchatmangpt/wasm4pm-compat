// COMPILE-PASS: OCPQ CardinalityBoundConst compile-time MIN<=MAX law.
//
// Law: OCPQ Section 4 — a cardinality predicate requires MIN <= MAX. The
// CardinalityBoundConst<MIN, MAX> type enforces this at compile time via
// Require<{ MIN <= MAX }>: IsTrue. Construction is only possible for lawful bounds.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use wasm4pm_compat::ocpq::CardinalityBoundConst;

fn main() {
    let b = CardinalityBoundConst::<1, 5>::new();
    assert_eq!(b.min(), 1);
    assert_eq!(b.max(), 5);

    // Degenerate equal bounds are lawful.
    let b2 = CardinalityBoundConst::<3, 3>::new();
    assert_eq!(b2.min(), 3);
    assert_eq!(b2.max(), 3);

    // Zero-based bounds are lawful.
    let b3 = CardinalityBoundConst::<0, 100>::new();
    assert_eq!(b3.min(), 0);
    assert_eq!(b3.max(), 100);
}
