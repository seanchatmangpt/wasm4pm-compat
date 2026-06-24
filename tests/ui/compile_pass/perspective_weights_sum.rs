#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use wasm4pm_compat::multiperspective::{PerspectiveWeight, MultiPerspectiveWeightConfig};

fn main() {
    // 1/4 + 1/4 + 1/4 + 1/4 = 1.0 (Sum <= 1.0 is met)
    let config = MultiPerspectiveWeightConfig::<
        1, 4, // Control-Flow
        1, 4, // Data
        1, 4, // Resource
        1, 4, // Time
    >::new();

    assert_eq!(config.cf.to_f64(), 0.25);
    assert_eq!(config.data.to_f64(), 0.25);
    assert_eq!(config.resource.to_f64(), 0.25);
    assert_eq!(config.time.to_f64(), 0.25);

    // 2/5 + 1/5 + 1/10 + 1/10 = 0.4 + 0.2 + 0.1 + 0.1 = 0.8 <= 1.0
    let config2 = MultiPerspectiveWeightConfig::<
        2, 5,
        1, 5,
        1, 10,
        1, 10,
    >::new();
    
    assert_eq!(config2.cf.to_f64(), 0.4);
}
