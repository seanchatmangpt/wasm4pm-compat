#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features, dead_code)]

use wasm4pm_compat::temporal::{TemporalProfile, TimeDelta, Seconds, Milliseconds, ActivityPair};

fn main() {
    let pair = ActivityPair::<String, String>::new();
    // avg is Seconds, std is Milliseconds -> violates Unit type unification (should fail compilation)
    let _ = TemporalProfile::new(
        TimeDelta::<Seconds>::new(1.0),
        TimeDelta::<Milliseconds>::new(500.0),
        pair,
    );
}
