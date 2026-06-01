#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: Need9ConditionCellLaw — ConditionCell<8> is the lawful maximum; 8 bits satisfies BITS <= 8 and proves the upper bound is reachable

// COMPILE-PASS: Need9 law — 8 condition bits is the maximum lawful count.
// ConditionCell<8> satisfies BITS <= 8.
use wasm4pm_compat::law::ConditionCell;

fn main() {
    let _: ConditionCell<1> = ConditionCell::new();
    let _: ConditionCell<4> = ConditionCell::new();
    let _: ConditionCell<8> = ConditionCell::new();  // maximum: lawful
}
