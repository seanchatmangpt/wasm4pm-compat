// COMPILE-PASS: OCPQ ChildSetBoundConst compile-time labelled CBS bound.
//
// Law: OCPQ Section 4 CBS(A, n_min, n_max) — labelled child-set bound with
// compile-time MIN<=MAX enforcement and branch label encoded as const parameter.
// ChildSetBoundConst<"items", 1, 5> and ChildSetBoundConst<"lines", 1, 5> are
// different types at compile time. Structure-only; CBS evaluation graduates to wasm4pm.
#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
use wasm4pm_compat::ocpq::ChildSetBoundConst;

fn main() {
    let b = ChildSetBoundConst::<"items", 1, 5>::new();
    assert_eq!(b.branch_label(), "items");
    assert_eq!(b.min(), 1);
    assert_eq!(b.max(), 5);

    let b2 = ChildSetBoundConst::<"lines", 0, 10>::new();
    assert_eq!(b2.branch_label(), "lines");
    assert_eq!(b2.min(), 0);
    assert_eq!(b2.max(), 10);

    // Degenerate equal bounds are lawful.
    let b3 = ChildSetBoundConst::<"events", 2, 2>::new();
    assert_eq!(b3.branch_label(), "events");
    assert_eq!(b3.min(), 2);
    assert_eq!(b3.max(), 2);
}
