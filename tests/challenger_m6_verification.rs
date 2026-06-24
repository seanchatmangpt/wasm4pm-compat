#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use wasm4pm_compat::ocpq::{NormedSimilarityScore, SimilarityScore};
use wasm4pm_compat::parity::delta::enforce_prediction_horizon_before_drift;
use wasm4pm_compat::temporal::{ActivityPair, Seconds, TemporalProfile, TimeDelta};

#[test]
fn verify_m6_timedelta_instantiation() {
    let td = TimeDelta::<Seconds>::new(42.5);
    assert_eq!(td.0, 42.5);
}

#[test]
fn verify_m6_temporal_profile_unit_compatibility() {
    let pair = ActivityPair::<String, String>::new();
    let profile = TemporalProfile::new(
        TimeDelta::<Seconds>::new(10.0),
        TimeDelta::<Seconds>::new(2.0),
        pair,
    );
    assert_eq!(profile.avg.0, 10.0);
    assert_eq!(profile.std.0, 2.0);
}

// Helper to assert that two values have the exact same type
fn assert_same_type<T>(_: T, _: T) {}

#[test]
fn verify_m6_similarity_score_limits() {
    // 0/1: minimum boundary score (0.0)
    let score_zero = SimilarityScore::<0, 1>::new();
    assert_eq!(score_zero.score.num(), 0);
    assert_eq!(score_zero.score.den(), 1);

    // 1/1: maximum boundary score (1.0)
    let score_one = SimilarityScore::<1, 1>::new();
    assert_eq!(score_one.score.num(), 1);
    assert_eq!(score_one.score.den(), 1);

    // 5/10: mid-range score (0.5)
    let score_mid = SimilarityScore::<5, 10>::new();
    assert_eq!(score_mid.score.num(), 5);
    assert_eq!(score_mid.score.den(), 10);

    // Verify GCD normalization of bounds compiles and produces identical type
    let score_norm = NormedSimilarityScore::<2, 4>::new();
    let score_base = SimilarityScore::<1, 2>::new();
    assert_same_type(score_norm, score_base);
}

#[test]
fn verify_m6_prediction_horizon_boundaries() {
    // Horizon is equal to the change point (limit boundary)
    enforce_prediction_horizon_before_drift::<10, 10>();

    // Horizon is strictly less than the change point (safe zone)
    enforce_prediction_horizon_before_drift::<5, 10>();
}
