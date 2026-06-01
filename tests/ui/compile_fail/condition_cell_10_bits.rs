#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Need9 law — ConditionCell<10> violates BITS <= 8.
// Law: ConditionCell requires at most 8 primary condition bits. 10 bits exceed
// the Need9-means-split covenant; the caller must split into two cells.
use wasm4pm_compat::law::ConditionCell;

fn main() {
    // 10 bits: exceeds the 8-bit limit.
    let _: ConditionCell<10> = ConditionCell::new();
}
