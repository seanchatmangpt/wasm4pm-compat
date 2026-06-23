//! Consuming witness for the `families_match_simd` batch-family-check seam.
//!
//! The SIMD family check is emitted capability; a doctest shows one case, but the
//! bitmask semantics (which bit maps to which lane) and the all-match / none-match
//! / mixed cases are the load-bearing behavior a real consumer (e.g. a graduation
//! gate verifying a witness set is single-family) depends on. This test makes
//! those semantics load-bearing.

use wasm4pm_compat::nightly_foundry::families_match_simd;
use wasm4pm_compat::witness::WitnessFamily::{Paper, Standard};

#[test]
fn all_match_sets_every_bit() {
    let all_paper = [Paper; 8];
    assert_eq!(families_match_simd(all_paper, Paper), 0b1111_1111u8);
}

#[test]
fn none_match_sets_no_bit() {
    let all_paper = [Paper; 8];
    assert_eq!(families_match_simd(all_paper, Standard), 0);
}

#[test]
fn mixed_sets_exactly_the_matching_lanes() {
    // Lanes 1 and 4 are Standard; the rest Paper. Matching Paper must clear
    // exactly bits 1 and 4 (to_bitmask: bit i ↔ lane i, LSB = lane 0).
    let mixed = [Paper, Standard, Paper, Paper, Standard, Paper, Paper, Paper];
    assert_eq!(families_match_simd(mixed, Paper), 0b1110_1101u8);
    // And the complementary query selects exactly the two Standard lanes.
    assert_eq!(families_match_simd(mixed, Standard), 0b0001_0010u8);
}
