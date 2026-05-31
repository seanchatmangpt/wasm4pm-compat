#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-FAIL: Need9 law — ConditionCell<9> violates BITS <= 8.
// Paper: Blue River Dam covenant — "Need9 means split."
// Expected error: the where-bound `Require<{ 9 <= 8 }>: IsTrue` is not satisfied.
use wasm4pm_compat::law::ConditionCell;

fn main() {
    let _: ConditionCell<9> = ConditionCell::new();
}
