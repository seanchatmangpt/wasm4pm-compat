#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// COMPILE-FAIL: yawl_multi_instance_bounds — YAWL Definition 1 nofi invariant violated.
// Law: 1 ≤ MIN ≤ MAX (compile-time). MultipleInstanceSpecConst<5, 2> has MIN > MAX,
// which violates Require<{ 5 <= 2 }>: IsTrue.
// Expected error: the where-bound `Require<{ MIN <= MAX }>: IsTrue` is not satisfied.
use wasm4pm_compat::petri::MultipleInstanceSpecConst;

fn main() {
    // min=5, max=2: 5 > 2 violates the YAWL nofi invariant 1 ≤ min ≤ max.
    // Require<{ 5 <= 2 }>: IsTrue is false — this must not compile.
    let _: MultipleInstanceSpecConst<5, 2> = MultipleInstanceSpecConst::new();
}
