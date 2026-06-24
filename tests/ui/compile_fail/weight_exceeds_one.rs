#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use wasm4pm_compat::multiperspective::PerspectiveWeight;

fn main() {
    // COMPILE-FAIL: NUM > DEN fails the Between01 range assertion [0, 1]
    let _: PerspectiveWeight<5, 4> = PerspectiveWeight::new();
}
