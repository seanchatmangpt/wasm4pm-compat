use crate::hash::ReceiptHash;

/// A single state transition proof in a Construct8 computation.
///
/// A receipt captures: pre-state, delta, post-state, causal timestamp, and cryptographic hash.
/// The hash is computed over the concatenation of (pre_state || delta || post_state).
///
/// Invariants:
/// - Hash is deterministic: same (pre, delta, post) always produces same hash.
/// - Post-state is canonical: it is the claimed result of applying delta to pre-state.
/// - Causal time is monotonic: strictly increasing across a receipt chain.
///
/// This is a structure-only type. It does NOT execute delta against pre-state;
/// it only records what the caller claims. Verification is the caller's responsibility
/// (see `ReplayVerdict`).
#[derive(Clone, Debug)]
pub struct C8Receipt {
    pub pre_state: Vec<u8>,
    pub delta: Vec<u8>,
    pub post_state: Vec<u8>,
    pub causal_time: u64,
}

impl C8Receipt {
    /// Constructs a new receipt from raw state bytes and a causal timestamp.
    ///
    /// # Arguments
    /// * `pre_state` — The state before the transition.
    /// * `delta` — The description of the transition (event, mutation, observable).
    /// * `post_state` — The state after the transition.
    /// * `causal_time` — The timestamp (logical or real) of this transition.
    ///
    /// # Returns
    /// A new `C8Receipt` with all fields set. The internal hash is NOT computed
    /// until `hash()` is called.
    pub fn new(pre_state: Vec<u8>, delta: Vec<u8>, post_state: Vec<u8>, causal_time: u64) -> Self {
        C8Receipt {
            pre_state,
            delta,
            post_state,
            causal_time,
        }
    }

    /// Computes and returns the SHA256 hash of this receipt.
    ///
    /// The hash is computed over: pre_state || delta || post_state.
    /// This is the receipt's cryptographic identity proof.
    pub fn hash(&self) -> ReceiptHash {
        let mut input = Vec::new();
        input.extend_from_slice(&self.pre_state);
        input.extend_from_slice(&self.delta);
        input.extend_from_slice(&self.post_state);
        ReceiptHash::from_bytes(&input)
    }

    /// Returns the causal timestamp of this transition.
    pub fn causal_time(&self) -> u64 {
        self.causal_time
    }

    /// Verifies that this receipt's post_state is the expected output of applying delta to pre_state.
    ///
    /// This is a structural check only — it does NOT execute the delta. It compares the
    /// post-state against a caller-provided expected state.
    ///
    /// # Arguments
    /// * `expected_post` — The expected post-state.
    ///
    /// # Returns
    /// `true` if `self.post_state == expected_post`.
    pub fn verify_post_state(&self, expected_post: &[u8]) -> bool {
        self.post_state == expected_post
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receipt_new() {
        let pre = b"v1".to_vec();
        let delta = vec![1, 2];
        let post = b"v2".to_vec();
        let receipt = C8Receipt::new(pre.clone(), delta.clone(), post.clone(), 5);

        assert_eq!(receipt.pre_state, pre);
        assert_eq!(receipt.delta, delta);
        assert_eq!(receipt.post_state, post);
        assert_eq!(receipt.causal_time(), 5);
    }

    #[test]
    fn receipt_hash_is_deterministic() {
        let pre = b"v1".to_vec();
        let delta = vec![1, 2];
        let post = b"v2".to_vec();

        let r1 = C8Receipt::new(pre.clone(), delta.clone(), post.clone(), 1);
        let r2 = C8Receipt::new(pre.clone(), delta.clone(), post.clone(), 2);

        let h1 = r1.hash();
        let h2 = r2.hash();

        assert_eq!(
            h1, h2,
            "Hash should be deterministic; causal_time should not affect it"
        );
    }

    #[test]
    fn receipt_verify_post_state() {
        let receipt = C8Receipt::new(b"v1".to_vec(), vec![1], b"v2".to_vec(), 1);
        assert!(receipt.verify_post_state(b"v2"));
        assert!(!receipt.verify_post_state(b"v3"));
    }
}
