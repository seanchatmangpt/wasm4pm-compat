#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features, dead_code)]

use wasm4pm_compat::temporal::{
    TemporalProfile, TimeDelta, Seconds, ActivityPair, ZScoreConst
};

fn main() {
    let pair = ActivityPair::<String, String>::new();
    // Consistent units (Seconds)
    let _ = TemporalProfile::new(
        TimeDelta::<Seconds>::new(1.0),
        TimeDelta::<Seconds>::new(0.5),
        pair,
    );

    // Lawful ZScoreConst with non-zero denominator
    let z = ZScoreConst::<5, 2>::new();
    assert_eq!(z.to_f64(), 2.5);
}
