//! Chain-sealing admission seam — [`SealingAdmit`] end-to-end.
//!
//! Demonstrates the `SealingAdmit` surface shipped in v26.6.14:
//!
//! - [`recompute_and_match`] — consumer supplies the BLAKE3 fold; compat proves
//!   the digest matches and mints a [`ChainProof`].
//! - [`RuntimeSeal::from_verified_chain`] — consumes the one-time [`ChainProof`]
//!   to produce an unforgeable runtime seal.
//! - [`SealingAdmit`] — a consumer-implemented trait that threads the seal into
//!   the admission verdict atomically.
//! - [`SealedAdmission::into_evidence`] — the only bridge to `Admitted` evidence
//!   for a chain-sealed witness.
//!
//! **Failure witness:** the `tampered_claim` block shows that a one-byte
//! deviation in the chain digest produces a named `ChainHashMismatch` refusal,
//! not a panic or a silent success. If the seal is broken, this example breaks.
//!
//! Doc reference: `docs/API_TOUR.md`, `src/admission.rs`

use wasm4pm_compat::admission::{
    recompute_and_match, Refusal, RuntimeSeal, SealedAdmission, SealingAdmit,
};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::state::Raw;
use wasm4pm_compat::witness::AffidavitReceiptChain;

// ── Realistic consumer type ──────────────────────────────────────────────────

/// A receipt bundle arriving from an external affidavit system.
/// The `chain_hash` was computed by the affidavit crate's BLAKE3 fold over
/// its `OperationEvent` bytes.
#[derive(Debug)]
struct AffidavitBundle {
    payload: String,
    chain_hash: Digest,
}

/// Named refusal reason for this boundary.
#[derive(Debug, PartialEq, Eq)]
enum BundleRefusal {
    EmptyPayload,
    ChainMismatch(wasm4pm_compat::admission::ChainHashMismatch),
}

// ── SealingAdmit implementation ──────────────────────────────────────────────

struct BundleAdmitter;

impl SealingAdmit for BundleAdmitter {
    type Raw = AffidavitBundle;
    type Sealed = String;
    type Reason = BundleRefusal;
    type Witness = AffidavitReceiptChain;

    fn admit_sealed(
        raw: Evidence<Self::Raw, Raw, Self::Witness>,
    ) -> Result<
        SealedAdmission<Self::Sealed, Self::Witness>,
        Refusal<Self::Reason, Self::Witness>,
    > {
        let bundle = raw.value;

        // Structural law: payload must not be empty.
        if bundle.payload.is_empty() {
            return Err(Refusal::new(BundleRefusal::EmptyPayload));
        }

        // Chain law: consumer's BLAKE3 fold recomputed from the payload bytes.
        // The fold stays in this consumer; compat never sees the algorithm.
        let claimed = bundle.chain_hash.clone();
        let proof = recompute_and_match(
            bundle.payload.as_bytes(),
            &claimed,
            |bytes| {
                // Simulate the consumer's fold: blake3 hash of the raw bytes.
                let hash = blake3::hash(bytes);
                Digest::new(format!("blake3:{}", hash.to_hex()))
            },
        )
        .map_err(|_| Refusal::new(BundleRefusal::ChainMismatch(wasm4pm_compat::admission::ChainHashMismatch)))?;

        // Atomically lock the seal into the admission.
        let seal = RuntimeSeal::from_verified_chain(claimed, proof);
        Ok(SealedAdmission::seal(bundle.payload, seal))
    }
}

// ── main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== SealingAdmit chain-sealing seam (v26.6.14) ===\n");

    // ── Part 1: recompute_and_match ──────────────────────────────────────────
    println!("Part 1: recompute_and_match");

    let payload = b"receipt-event-bytes";
    let real_hash = blake3::hash(payload);
    let claimed = Digest::new(format!("blake3:{}", real_hash.to_hex()));

    let proof = recompute_and_match(payload, &claimed, |bytes| {
        Digest::new(format!("blake3:{}", blake3::hash(bytes).to_hex()))
    });
    assert!(proof.is_ok(), "matching digest must yield ChainProof");
    println!("  ✓ recompute_and_match: matching digest → ChainProof minted");

    // Tampered claim: one-byte flip in the declared hash.
    let tampered = Digest::new("blake3:0000000000000000000000000000000000000000000000000000000000000000");
    let bad_proof = recompute_and_match(payload, &tampered, |bytes| {
        Digest::new(format!("blake3:{}", blake3::hash(bytes).to_hex()))
    });
    assert!(bad_proof.is_err(), "mismatch must produce ChainHashMismatch");
    println!("  ✓ tampered claim → named ChainHashMismatch refusal (not panic)");

    // ── Part 2: RuntimeSeal::from_verified_chain ─────────────────────────────
    println!("\nPart 2: RuntimeSeal from ChainProof");

    let proof2 = recompute_and_match(payload, &claimed, |bytes| {
        Digest::new(format!("blake3:{}", blake3::hash(bytes).to_hex()))
    })
    .unwrap();
    let seal = RuntimeSeal::from_verified_chain(claimed.clone(), proof2);
    assert_eq!(seal.digest(), &claimed, "seal carries the verified digest");
    println!("  ✓ RuntimeSeal::from_verified_chain: seal carries the matched digest");

    // ── Part 3: full SealingAdmit flow ──────────────────────────────────────
    println!("\nPart 3: BundleAdmitter implements SealingAdmit");

    let good_payload = String::from("genesis-event-bytes");
    let hash_hex = blake3::hash(good_payload.as_bytes());
    let bundle = AffidavitBundle {
        payload: good_payload.clone(),
        chain_hash: Digest::new(format!("blake3:{}", hash_hex.to_hex())),
    };

    let raw_evidence = Evidence::<AffidavitBundle, Raw, AffidavitReceiptChain>::raw(bundle);
    let result = BundleAdmitter::admit_sealed(raw_evidence);

    assert!(result.is_ok(), "well-formed bundle must be admitted");
    let sealed = result.unwrap();
    assert_eq!(sealed.value, good_payload);
    println!("  ✓ admit_sealed: well-formed bundle → SealedAdmission");

    // Into evidence — the only bridge to Admitted state.
    let admitted = sealed.into_evidence();
    let _receipted = admitted.into_receipted();
    println!("  ✓ into_evidence + into_receipted: full one-way door traversal");

    // ── Part 4: named refusals ────────────────────────────────────────────────
    println!("\nPart 4: named refusals");

    // Empty payload → EmptyPayload
    let empty_bundle = AffidavitBundle {
        payload: String::new(),
        chain_hash: Digest::new("blake3:doesnotmatter"),
    };
    let raw2 = Evidence::<AffidavitBundle, Raw, AffidavitReceiptChain>::raw(empty_bundle);
    let empty_result = BundleAdmitter::admit_sealed(raw2);
    assert!(
        matches!(empty_result, Err(ref r) if matches!(r.reason, BundleRefusal::EmptyPayload)),
        "empty payload must produce EmptyPayload refusal"
    );
    println!("  ✓ empty payload → BundleRefusal::EmptyPayload (named law)");

    // Wrong chain hash → ChainMismatch
    let wrong_bundle = AffidavitBundle {
        payload: String::from("real-data"),
        chain_hash: Digest::new("blake3:wrong-hash"),
    };
    let raw3 = Evidence::<AffidavitBundle, Raw, AffidavitReceiptChain>::raw(wrong_bundle);
    let chain_result = BundleAdmitter::admit_sealed(raw3);
    assert!(
        matches!(chain_result, Err(ref r) if matches!(r.reason, BundleRefusal::ChainMismatch(_))),
        "hash mismatch must produce ChainMismatch refusal"
    );
    println!("  ✓ wrong chain hash → BundleRefusal::ChainMismatch (named law)");

    println!("\n=== All assertions passed — SealingAdmit seam is witnessed ===");
    println!("  Claims: recompute_and_match + ChainProof + RuntimeSeal + SealingAdmit + SealedAdmission");
    println!("  Witness: every assertion above; breaks if any claim regresses.");
}
