//! Consuming witness for the `NormedBetween01` GCD-normalization law.
//!
//! The law's whole claim is that equivalent ratios collapse to the SAME TYPE:
//! `NormedBetween01<3, 6>` and `NormedBetween01<1, 2>` are not merely equal
//! values — they are the identical type, because both reduce through `gcd` to
//! `Between01<1, 2>`. Without a consuming test, that type-identity claim is
//! dormant. This test makes it load-bearing: the assignments below compile ONLY
//! if the types are genuinely identical (a mismatch is a compile error, failing
//! the build), and the const `gcd` is exercised directly.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use wasm4pm_compat::law::{gcd, NormedBetween01};

#[test]
fn equivalent_ratios_are_the_same_type() {
    // 3/6 and 1/2 must be the SAME type. If they were distinct types, this
    // assignment would not compile — so compilation IS the type-identity witness.
    let three_sixths: NormedBetween01<3, 6> = Default::default();
    let one_half: NormedBetween01<1, 2> = three_sixths;
    let _ = one_half;

    // And the reverse direction, plus a third equivalent ratio (4/8 -> 1/2).
    let four_eighths: NormedBetween01<4, 8> = Default::default();
    let _also_one_half: NormedBetween01<1, 2> = four_eighths;
}

#[test]
fn gcd_reduces_as_the_law_requires() {
    // The const fn underlying the normalization, witnessed at runtime.
    assert_eq!(gcd(3, 6), 3);
    assert_eq!(gcd(4, 8), 4);
    assert_eq!(gcd(1, 2), 1);
    assert_eq!(gcd(7, 5), 1); // coprime
    assert_eq!(gcd(0, 9), 9); // gcd(0, n) == n
}
