// Law: CarrierNonForgeabilityLaw — Evidence<T, Admitted, W> cannot be constructed
// via struct literal; the private `_seal` field prevents forging Admitted evidence
// outside the crate-internal `Evidence::sealed` / SealedAdmission flow.
//
// COMPILE-FAIL: Forging Admitted evidence directly by struct literal.
//
// THE REAL ATTACK PATH. From an external crate, attempt to mint
// `Evidence<String, Admitted, AffidavitReceiptChain>` by writing the struct
// literal directly, bypassing SealedAdmission / RuntimeSeal / ChainProof
// entirely. This must be a compile error because `Evidence` carries a private
// `_seal: ()` field: there is no struct-literal path to Admitted evidence from
// outside the crate.
//
// This is the bypass the original sealed_admission_unforgeable fixture missed:
// it only tested SealedAdmission / RuntimeSeal forgery and gave false assurance
// because the carrier itself was never proven unforgeable.
//
// Expected error: E0451 — field `_seal` of struct `Evidence` is private
// (and a missing-field error for `_seal`).
use core::marker::PhantomData;

use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Admitted;
use wasm4pm_compat::witness::AffidavitReceiptChain;

fn main() {
    let _forged: Evidence<String, Admitted, AffidavitReceiptChain> = Evidence {
        value: "r".to_string(),
        state: PhantomData,
        witness: PhantomData,
    };
}
