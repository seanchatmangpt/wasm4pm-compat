//! Consuming behavioral witness for the `SealingAdmit` seam.
//!
//! The seam (`SealingAdmit::admit_sealed`) is the public entry point an external
//! consumer (e.g. affidavit) implements to fuse structural admission with a
//! BLAKE3 chain seal in ONE `Raw -> Admitted` transition. Without a consumer
//! that actually implements the trait and drives `admit_sealed`, the trait is
//! emitted-but-unconsumed — a dormant claim. This test IS that consumer: it
//! makes the seam load-bearing, transitioning it from OPEN to admitted.
//!
//! It implements `SealingAdmit` exactly as affidavit would — supplying its OWN
//! genesis-seeded rolling BLAKE3 fold as the chain rule (the rule never enters
//! the crate) — and proves, through the PUBLIC API only:
//! - the admit path yields a `SealedAdmission` whose seal locks the recomputed
//!   chain digest, and graduates to `Evidence<_, Admitted, AffidavitReceiptChain>`;
//! - a tampered claimed digest refuses BY NAME through the trait, never silently.

use wasm4pm_compat::admission::{
    recompute_and_match, ChainHashMismatch, Refusal, RuntimeSeal, SealedAdmission, SealingAdmit,
};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::state::Raw;
use wasm4pm_compat::witness::AffidavitReceiptChain;

/// A stand-in for affidavit's `Receipt`: an event byte-chain plus the consumer's
/// claimed chain digest (what affidavit's `chain_hash` would carry).
struct MockReceipt {
    events: Vec<Vec<u8>>,
    claimed: Digest,
}

/// The consumer's own chain rule — a genesis-seeded rolling BLAKE3 fold. It lives
/// entirely in the consumer; the crate only ever receives its `Digest` output.
fn chain_digest(events: &[Vec<u8>]) -> Digest {
    let mut acc = *blake3::hash(b"genesis").as_bytes();
    for ev in events {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&acc);
        hasher.update(ev);
        acc = *hasher.finalize().as_bytes();
    }
    Digest::new(format!("blake3:{}", blake3::Hash::from(acc).to_hex()))
}

/// The consumer's `SealingAdmit` boundary — what affidavit would write.
enum ReceiptAdmitter {}

impl SealingAdmit for ReceiptAdmitter {
    type Raw = MockReceipt;
    type Sealed = MockReceipt;
    type Reason = ChainHashMismatch;
    type Witness = AffidavitReceiptChain;

    fn admit_sealed(
        raw: Evidence<MockReceipt, Raw, AffidavitReceiptChain>,
    ) -> Result<
        SealedAdmission<MockReceipt, AffidavitReceiptChain>,
        Refusal<ChainHashMismatch, AffidavitReceiptChain>,
    > {
        let receipt = raw.value;
        let claimed = receipt.claimed.clone();
        // Recompute the chain via the consumer's own rule and match the claim.
        match recompute_and_match(&receipt.events, &claimed, |e| chain_digest(e)) {
            Ok(proof) => {
                let seal = RuntimeSeal::from_verified_chain(claimed, proof);
                Ok(SealedAdmission::seal(receipt, seal))
            }
            // Re-name the refusal under this boundary's witness.
            Err(_) => Err(Refusal::new(ChainHashMismatch)),
        }
    }
}

fn receipt(events: &[&[u8]]) -> MockReceipt {
    let events: Vec<Vec<u8>> = events.iter().map(|e| e.to_vec()).collect();
    let claimed = chain_digest(&events);
    MockReceipt { events, claimed }
}

#[test]
fn admit_sealed_seals_and_graduates_a_well_formed_receipt() {
    let r = receipt(&[b"e1", b"e2", b"e3"]);
    let expected = r.claimed.clone();

    let sealed = ReceiptAdmitter::admit_sealed(Evidence::raw(r))
        .expect("a self-consistent receipt must admit-and-seal");

    // The seal locks the recomputed chain digest the consumer claimed.
    assert_eq!(sealed.seal_ref().digest(), &expected);

    // It graduates through the typed one-way door to Admitted evidence,
    // tagged with the receipt-chain witness, and onward to Receipted.
    let admitted: Evidence<MockReceipt, wasm4pm_compat::state::Admitted, AffidavitReceiptChain> =
        sealed.into_evidence();
    let _receipted = admitted.into_receipted();
}

#[test]
fn admit_sealed_refuses_a_tampered_claim_by_name() {
    // A receipt whose claimed digest disagrees with its own event chain.
    let mut r = receipt(&[b"e1", b"e2"]);
    r.claimed = Digest::new("blake3:tampered");

    match ReceiptAdmitter::admit_sealed(Evidence::raw(r)) {
        Ok(_) => panic!("a tampered claim must be refused, not sealed"),
        Err(refusal) => assert_eq!(refusal.reason, ChainHashMismatch),
    }
}
