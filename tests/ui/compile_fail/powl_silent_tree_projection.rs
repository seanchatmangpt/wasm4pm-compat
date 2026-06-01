#![feature(adt_const_params)]
#![allow(incomplete_features)]
// Law: PowlTreeProjectionLaw — ExceedsProcessTree does not implement TreeProjectable; POWL partial orders that exceed block structure cannot be silently projected to a process tree (Kourani 2505.07052 §3)

// COMPILE-FAIL: POWL projection law — ExceedsProcessTree cannot be projected.
// Paper: Kourani (2505.07052) §3 — POWL partial orders may exceed block structure.
// Expected error: ExceedsProcessTree does not implement TreeProjectable.
use wasm4pm_compat::powl::{assert_tree_projectable, ExceedsProcessTree};

fn main() {
    assert_tree_projectable(ExceedsProcessTree);
}
