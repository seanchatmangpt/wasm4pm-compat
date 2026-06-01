#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// COMPILE-FAIL: OCPQ cardinality law — CardinalityBoundConst<MIN,MAX> requires MIN <= MAX.
// Law: A cardinality bound with MIN > MAX is structurally invalid. The where-bound
// Require<{ MIN <= MAX }>: IsTrue enforces this at compile time.
use wasm4pm_compat::ocpq::CardinalityBoundConst;

fn main() {
    // MIN=10, MAX=3: 10 > 3 violates MIN <= MAX.
    let _: CardinalityBoundConst<10, 3> = CardinalityBoundConst::new();
}
