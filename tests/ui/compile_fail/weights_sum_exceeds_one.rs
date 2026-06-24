#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use wasm4pm_compat::multiperspective::MultiPerspectiveWeightConfig;

fn main() {
    // COMPILE-FAIL: 1/2 + 1/2 + 1/4 + 0/1 = 1.25 > 1.0 (Sum exceeds 1.0)
    let _ = MultiPerspectiveWeightConfig::<
        1, 2, // 0.5
        1, 2, // 0.5
        1, 4, // 0.25
        0, 1, // 0.0
    >::new();
}
