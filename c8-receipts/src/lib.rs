#![forbid(unsafe_code)]
#![doc = "Receipt, proof, and integrity sealing for Construct8 market physics.

This crate provides receipts (immutable, cryptographic proofs of computation state),
receipt chains (ordered, verifiable sequences), and replay verification.

The central types are:
- `C8Receipt` — A single state transition proof (pre, delta, post, causal_time, hash)
- `ReceiptChain` — A sequence of receipts with forward-chaining verification
- `ReceiptHash` — SHA256-based identity proof
- `ReplayVerdict` — Outcome of replaying a delta against a receipt state
- `BoundaryProof` — Proof that a computation boundary was not crossed
- `ImplementationReceipt` — Metadata capture for reproducibility

Receipts are intended to prove that code executed correctly, producing the claimed
delta from a known pre-state, and that the hash chain is unbroken. This module is
structure-only — engine logic (replay algorithms, conformance checking) graduates
to wasm4pm.
"]

mod receipt;
mod chain;
mod hash;
mod verdict;
mod proof;
mod implementation;

pub use receipt::C8Receipt;
pub use chain::ReceiptChain;
pub use hash::ReceiptHash;
pub use verdict::ReplayVerdict;
pub use proof::BoundaryProof;
pub use implementation::ImplementationReceipt;

/// Constructs a Construct8Delta for a single state transition.
///
/// A delta is the minimal description of what changed: event, mutation, observable.
/// This function is a no-op placeholder; real delta construction occurs in c8-market.
pub fn construct8_delta(
    event: &str,
    pre_state: &[u8],
    mutation: &str,
) -> Vec<u8> {
    let mut delta = Vec::new();
    delta.extend_from_slice(event.as_bytes());
    delta.push(0);
    delta.extend_from_slice(pre_state);
    delta.push(0);
    delta.extend_from_slice(mutation.as_bytes());
    delta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receipt_hash_changes() {
        let pre = b"state_v1".to_vec();
        let delta = vec![1, 2, 3];
        let post = b"state_v2".to_vec();
        let causal_time = 42;

        let receipt1 = C8Receipt::new(pre.clone(), delta.clone(), post.clone(), causal_time);
        let hash1 = receipt1.hash();

        let receipt2 = C8Receipt::new(pre.clone(), vec![1, 2, 4], post.clone(), causal_time);
        let hash2 = receipt2.hash();

        assert_ne!(hash1, hash2, "Different deltas should produce different hashes");
    }

    #[test]
    fn chain_verifies() {
        let mut chain = ReceiptChain::new();

        let pre1 = b"initial".to_vec();
        let delta1 = vec![1];
        let post1 = b"after_tick_1".to_vec();
        let receipt1 = C8Receipt::new(pre1, delta1, post1.clone(), 1);

        chain.append(receipt1);

        let delta2 = vec![2];
        let post2 = b"after_tick_2".to_vec();
        let receipt2 = C8Receipt::new(post1, delta2, post2, 2);

        chain.append(receipt2);

        assert!(chain.verify(), "Chain should verify when receipts are contiguous");
    }

    #[test]
    fn tampered_receipt_fails() {
        let pre = b"state".to_vec();
        let delta = vec![1, 2, 3];
        let post = b"next_state".to_vec();

        let mut receipt = C8Receipt::new(pre, delta, post, 1);
        let original_hash = receipt.hash();

        // Tamper with the post-state.
        receipt.post_state = b"corrupted_state".to_vec();

        let new_hash = receipt.hash();
        assert_ne!(
            original_hash, new_hash,
            "Tampering post-state should change the hash"
        );
    }

    #[test]
    fn replay_reproduces_hash() {
        let pre = b"v1".to_vec();
        let delta = construct8_delta("tick", b"v1", "market_update");
        let post = b"v2".to_vec();

        let receipt = C8Receipt::new(pre.clone(), delta.clone(), post.clone(), 10);
        let receipt_hash = receipt.hash();

        let verdict = ReplayVerdict::replay(&pre, &delta, &post);
        assert_eq!(
            verdict.computed_hash, receipt_hash,
            "Replay should reproduce the receipt hash"
        );
        assert!(verdict.is_valid(), "Replay should validate against receipt");
    }
}
