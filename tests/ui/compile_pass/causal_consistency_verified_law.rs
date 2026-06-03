#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// Law: CausalConsistencyLaw — the Unknown → Consistent transition is one-way.
//
// COMPILE-PASS: VerifyCausalConsistency trait can be implemented by user code;
// ConsistencyVerified<T> is produced only via the trait; UnknownVerifier works
// as the compat-layer stand-in; is_consistent() and verdict() are correct.

use wasm4pm_compat::causality::{
    CausalConsistency,
    ConsistencyVerified,
    ConsistencyProof,
    UnknownVerifier,
    VerifyCausalConsistency,
};

// The pass-through verifier always emits Unknown (no algorithm).
fn check_unknown_verifier() {
    let verifier = UnknownVerifier;
    let result: ConsistencyVerified<u32> = verifier.verify(42u32);
    assert_eq!(result.verdict(), CausalConsistency::Unknown);
    assert!(!result.is_consistent());
    assert_eq!(result.inner, 42u32);
}

// A stub verifier that always claims Consistent — used in unit tests for
// wasm4pm where the real algorithm runs. In this test we verify only the
// contract surface compiles correctly.
struct AlwaysConsistentVerifier;

impl<T> VerifyCausalConsistency<T> for AlwaysConsistentVerifier {
    fn verify(&self, evidence: T) -> ConsistencyVerified<T> {
        // In real wasm4pm code this impl runs the Heuristics Miner +
        // cycle detection. Here we use the public mint path via the trait
        // return type — the ConsistencyProof is minted by the crate internals.
        //
        // IMPORTANT: we cannot directly construct ConsistencyVerified here
        // because ConsistencyProof::new() is pub(crate). Instead, we rely on
        // the blanket: the only way to return ConsistencyVerified<T> is to
        // call ConsistencyVerified::new() — which IS accessible inside the
        // crate (wasm4pm-compat). External impls of the trait must use the
        // public API surface, which means they cannot produce Consistent
        // without going through a crate-internal pathway. For this test we
        // are inside the test binary, which is an external crate — so we
        // intentionally use UnknownVerifier here and verify the public-only
        // surface compiles. The real Consistent path is tested by the
        // crate-internal oracle test in tests/causality_oracle.rs.
        let _ = evidence;
        let v = UnknownVerifier;
        // Satisfy the return type by delegation — type system validates the
        // shape is correct.
        v.verify(evidence)
    }
}

fn check_always_consistent_via_unknown_verifier() {
    // The outer verifier delegates to UnknownVerifier — it cannot forge
    // Consistent from outside the crate. This is the intended law.
    let verifier = AlwaysConsistentVerifier;
    let result: ConsistencyVerified<&str> = verifier.verify("log-evidence");
    // Verdict is Unknown (delegated), not Consistent — external impls
    // cannot produce Consistent without a crate-internal proof token.
    assert_eq!(result.verdict(), CausalConsistency::Unknown);
    assert_eq!(result.inner, "log-evidence");
}

fn check_verdict_variants_accessible() {
    // All CausalConsistency variants are still usable as plain values.
    // The law only restricts producing ConsistencyVerified with Consistent
    // without a proof token.
    assert_eq!(CausalConsistency::Consistent.to_string(), "causally-consistent");
    assert_eq!(CausalConsistency::HasCycles.to_string(), "has-causal-cycles");
    assert_eq!(CausalConsistency::HasContradictions.to_string(), "has-causal-contradictions");
    assert_eq!(CausalConsistency::Unknown.to_string(), "causal-consistency-unknown");
}

fn main() {
    check_unknown_verifier();
    check_always_consistent_via_unknown_verifier();
    check_verdict_variants_accessible();
}
