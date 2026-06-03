// COMPILE-FAIL: CausalConsistencyLaw — ConsistencyProof cannot be forged.
//
// External code cannot construct ConsistencyProof directly because the
// `_seal` field is pub(crate). Attempting to forge a ConsistencyVerified<T>
// by directly constructing ConsistencyProof must fail to compile.
//
// This test seals the one-way-door law: the only path to a Consistent
// verdict in ConsistencyVerified is through VerifyCausalConsistency impls
// that are internal to the wasm4pm-compat crate.

use wasm4pm_compat::causality::ConsistencyProof;

fn forge_proof() -> ConsistencyProof {
    // ERROR: `_seal` is private — cannot forge ConsistencyProof externally.
    ConsistencyProof { _seal: () }
}

fn main() {
    let _ = forge_proof();
}
