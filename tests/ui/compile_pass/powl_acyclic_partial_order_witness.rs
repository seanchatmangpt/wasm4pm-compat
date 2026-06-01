// COMPILE-PASS: POWL acyclicity law — AcyclicPartialOrder satisfies AcyclicWitness.
//
// Law: Kourani et al. (2026) §3 — a POWL partial order P(M⁺, ≺) requires ≺
// to be a strict partial order (irreflexive, asymmetric, transitive), implying
// acyclicity. AcyclicPartialOrder is the marker that records this assertion.
// assert_acyclic() is the structural gate: only AcyclicWitness passes.
use wasm4pm_compat::powl::{assert_acyclic, AcyclicPartialOrder};

fn main() {
    let ok = assert_acyclic(AcyclicPartialOrder);
    assert!(ok);
}
