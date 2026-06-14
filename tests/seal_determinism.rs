//! Determinism + tamper-sensitivity receipts for the affidavit chain seal seam.
//!
//! Proves, through the PUBLIC flow only, that:
//! - the same value + same event byte sequence yields the SAME `RuntimeSeal`
//!   digest (determinism / golden),
//! - flipping a single input byte yields a DIFFERENT digest,
//! - `recompute_and_match` refuses BY NAME (`ChainHashMismatch`) when the
//!   claimed digest disagrees with the recomputed one (tamper-sensitivity).
//!
//! The chain rule lives in-test (a genesis-seeded rolling BLAKE3 fold), exactly
//! as an external consumer would supply it — the crate never sees it.

use wasm4pm_compat::admission::{
    recompute_and_match, ChainHashMismatch, RuntimeSeal, SealedAdmission,
};
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::AffidavitReceiptChain;

/// Genesis-seeded rolling BLAKE3 fold over an event byte sequence — a stand-in
/// for the affidavit consumer's `OperationEvent` chain rule.
fn chain_digest(events: &[&[u8]]) -> Digest {
    let mut acc = *blake3::hash(b"genesis").as_bytes();
    for ev in events {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&acc);
        hasher.update(ev);
        acc = *hasher.finalize().as_bytes();
    }
    Digest::new(format!("blake3:{}", blake3::Hash::from(acc).to_hex()))
}

/// Mints a `RuntimeSeal` over `events` through the public verified flow:
/// recompute, match against the consumer-claimed digest, then seal.
fn seal_over(events: &[&[u8]]) -> RuntimeSeal {
    let claimed = chain_digest(events);
    let proof = recompute_and_match(events, &claimed, |e| chain_digest(e))
        .expect("self-consistent claimed digest must match");
    RuntimeSeal::from_verified_chain(claimed, proof)
}

#[test]
fn same_value_and_chain_yields_same_seal() {
    let events: &[&[u8]] = &[b"e1", b"e2", b"e3"];

    let seal_a = seal_over(events);
    let seal_b = seal_over(events);

    // Determinism: identical value + chain => identical seal digest.
    assert_eq!(
        seal_a.digest(),
        seal_b.digest(),
        "the same event sequence must produce the same seal digest"
    );

    // And the full sealed-admission path is open with the matched seal.
    let sealed: SealedAdmission<&str, AffidavitReceiptChain> =
        SealedAdmission::seal("receipt", seal_a);
    let _admitted = sealed.into_evidence().into_receipted();
}

#[test]
fn flipping_one_byte_changes_the_digest() {
    let base: &[&[u8]] = &[b"e1", b"e2", b"e3"];
    let flipped: &[&[u8]] = &[b"e1", b"eX", b"e3"];

    assert_ne!(
        seal_over(base).digest(),
        seal_over(flipped).digest(),
        "a single flipped input byte must yield a different seal digest"
    );
}

#[test]
fn tampered_claim_refuses_by_name() {
    let events: &[&[u8]] = &[b"e1", b"e2"];
    // A claimed digest that disagrees with the recomputed chain digest.
    let lying_claim = Digest::new("blake3:tampered");

    match recompute_and_match(events, &lying_claim, |e| chain_digest(e)) {
        Ok(_) => panic!("a disagreeing claimed digest must be refused"),
        // The refusal is named, not a catch-all.
        Err(refusal) => assert_eq!(refusal.reason, ChainHashMismatch),
    }
}
