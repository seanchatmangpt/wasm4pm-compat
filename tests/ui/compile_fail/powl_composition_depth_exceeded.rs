#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// COMPILE-FAIL: POWL composition-depth law — PowlComposition<_, 9> violates DEPTH <= MAX_POWL_DEPTH.
// Law: Kourani et al. (2026) §3 — POWL composition nesting depth must not exceed MAX_POWL_DEPTH (8).
use wasm4pm_compat::powl::{PowlComposition, MAX_POWL_DEPTH};

fn main() {
    // DEPTH = MAX_POWL_DEPTH + 1 = 9: exceeds the lawful composition-depth ceiling.
    let _: PowlComposition<[&str; 1], { MAX_POWL_DEPTH + 1 }> = PowlComposition::new(["atom"]);
}
