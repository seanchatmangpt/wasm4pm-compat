#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: PowlCompositionDepthLaw — PowlComposition<_,8> satisfies Require<{DEPTH<=MAX_POWL_DEPTH}>: IsTrue; proves the POWL composition-depth law is open at the ceiling (Kourani et al. 2026 §3)

// COMPILE-PASS: PowlComposition<_, 8> — POWL composition-depth law.
//
// Law: Kourani et al. (2026) §3 — POWL composition nesting depth is bounded by
// MAX_POWL_DEPTH (8). PowlComposition<_, 8> satisfies Require<{ DEPTH <= MAX_POWL_DEPTH }>: IsTrue.
use wasm4pm_compat::powl::PowlComposition;

fn main() {
    let c: PowlComposition<[&str; 1], 8> = PowlComposition::new(["atom"]);
    assert_eq!(c.inner[0], "atom");
}
