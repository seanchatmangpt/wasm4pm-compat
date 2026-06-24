#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use wasm4pm_compat::multiperspective::PerspectiveWeight;

fn main() {
    // COMPILE-FAIL: DEN == 0 is division by zero, failing DEN > 0 assertion
    let _: PerspectiveWeight<1, 0> = PerspectiveWeight::new();
}
