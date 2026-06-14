//! Corpus-integrity test — every witness KEY across the entire bibliography is
//! unique. This is the runtime companion to the compile-time `const _: () = {…}`
//! proof in `src/witness_corpus.rs`: it runs in the fast `cargo make test` loop
//! and names the offending key on failure, where the const proof only halts the
//! build with a generic message.
//!
//! Why this matters: two distinct witness types may share a `KEY` string and
//! still compile (Rust only rejects duplicate type *names*). Such a collision
//! would print identical provenance for two different authorities — silently
//! reviving the authority-erasure problem the crate exists to prevent.

use std::collections::HashMap;
use wasm4pm_compat::witness_corpus::ALL_WITNESS_KEYS;

#[test]
fn all_witness_keys_are_unique() {
    let mut seen: HashMap<&str, usize> = HashMap::new();
    for (idx, key) in ALL_WITNESS_KEYS.iter().enumerate() {
        if let Some(prev) = seen.insert(key, idx) {
            panic!(
                "duplicate witness key {key:?} at corpus indices {prev} and {idx} \
                 — authority collision: two witnesses claim the same KEY"
            );
        }
    }
    assert_eq!(seen.len(), ALL_WITNESS_KEYS.len());
}

#[test]
fn corpus_is_non_empty() {
    // Guards against a broken ggen render producing an empty array (which would
    // make the uniqueness proof vacuously pass).
    assert!(
        ALL_WITNESS_KEYS.len() > 250,
        "witness corpus has only {} keys — expected the full bibliography; \
         did `ggen sync --rule witness-corpus` render correctly?",
        ALL_WITNESS_KEYS.len()
    );
}

#[test]
fn affidavit_receipt_chain_key_is_corpus_tracked() {
    // Durability guard for the hand-authored `AffidavitReceiptChain` witness.
    //
    // That witness is declared in `src/witness.rs` (hand-authored) and is NOT in
    // any ggen TTL, so a future `ggen sync --rule witness-corpus` would silently
    // DROP its key from ALL_WITNESS_KEYS — quietly weakening the compile-time
    // uniqueness proof for that key. This test makes that drop a loud CI failure
    // instead of a silent regression.
    //
    // Fix on failure: re-add `"affidavit-receipt-chain"` to the corpus (or add an
    // AffidavitReceiptChain WitnessMarker to the witness TTL the corpus query
    // reads), then `ggen sync --rule witness-corpus`.
    assert!(
        ALL_WITNESS_KEYS.contains(&"affidavit-receipt-chain"),
        "the hand-authored AffidavitReceiptChain witness key 'affidavit-receipt-chain' \
         is missing from ALL_WITNESS_KEYS — a `ggen sync` likely dropped it because the \
         witness is not in any TTL. Re-add it so the KEY-uniqueness proof still covers it."
    );
}
