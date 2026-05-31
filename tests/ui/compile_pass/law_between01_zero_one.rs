#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// COMPILE-PASS: Between01<0,1> and Between01<1,1> — boundary values are lawful.
//
// Law: Between01 bounds — NUM/DEN must satisfy DEN > 0 and NUM <= DEN.
// Proves the closed-interval endpoints compile: zero (0/1) and one (1/1)
// are both lawful. Interior fractions are also confirmed. Boundary-only
// testing is the strongest evidence that the [0,1] law is inclusive.

use wasm4pm_compat::law::Between01;

fn check_lower_boundary() {
    // 0/1 = 0.0 — lawful lower bound (inclusive zero).
    let zero: Between01<0, 1> = Between01::new();
    assert_eq!(zero.num(), 0);
    assert_eq!(zero.den(), 1);
}

fn check_upper_boundary() {
    // 1/1 = 1.0 — lawful upper bound (inclusive one).
    let one: Between01<1, 1> = Between01::new();
    assert_eq!(one.num(), 1);
    assert_eq!(one.den(), 1);
}

fn check_interior_fractions() {
    // 1/2 = 0.5 — mid-point.
    let half: Between01<1, 2> = Between01::new();
    assert_eq!(half.num(), 1);
    assert_eq!(half.den(), 2);

    // 3/4 = 0.75 — interior.
    let three_quarters: Between01<3, 4> = Between01::new();
    assert_eq!(three_quarters.num(), 3);
    assert_eq!(three_quarters.den(), 4);

    // 99/100 = 0.99 — near the upper bound.
    let near_one: Between01<99, 100> = Between01::new();
    assert_eq!(near_one.num(), 99);
    assert_eq!(near_one.den(), 100);

    // Default impl is identical to new().
    let default_val: Between01<3, 4> = Default::default();
    assert_eq!(default_val.num(), 3);
    assert_eq!(default_val.den(), 4);
}

fn main() {
    check_lower_boundary();
    check_upper_boundary();
    check_interior_fractions();
}
