// COMPILE-FAIL: IrreduciblePowlSilentlyProjected — Irreducible cannot satisfy TreeProjectable
// Law: POWL irreducibility law — an Irreducible partial order exceeds any block-structured
// process tree and must not be accepted by a gate requiring TreeProjectable.
// Paper: Kourani et al. (2026) §3 — irreducible partial orders cannot be projected to process trees.
// Expected error: Irreducible does not implement TreeProjectable.
use wasm4pm_compat::powl::{assert_tree_projectable, Irreducible};

fn main() {
    // Irreducible is not in tree_projectable_seal; this must fail at compile time.
    assert_tree_projectable(Irreducible);
}
