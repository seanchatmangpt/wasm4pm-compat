#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: OCPQ child-set bound law — ChildSetBoundConst<LABEL,MIN,MAX> requires MIN <= MAX.
// Law: A CBS predicate with MIN > MAX violates OCPQ Section 4. The where-bound
// Require<{ MIN <= MAX }>: IsTrue enforces this at the type level.
use wasm4pm_compat::ocpq::ChildSetBoundConst;

fn main() {
    // MIN=5, MAX=2: 5 > 2 violates MIN <= MAX.
    let _: ChildSetBoundConst<"items", 5, 2> = ChildSetBoundConst::new();
}
