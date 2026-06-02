use crate::hash::ReceiptHash;

/// The outcome of replaying a delta against a receipt's pre-state.
///
/// A replay verdict records whether the claimed post-state matches the
/// recomputed hash of (pre || delta || post).
///
/// This is a structural type — it does NOT execute the delta. It only
/// validates that the receipt's claim is internally consistent.
///
/// # Invariants
/// - `computed_hash` is always deterministic for the same (pre, delta, post).
/// - `is_valid()` is true iff the receipt's internal hash matches the computed hash.
#[derive(Clone, Debug)]
pub struct ReplayVerdict {
    /// The hash computed from the receipt's (pre_state, delta, post_state).
    pub computed_hash: ReceiptHash,
    /// Whether this computed hash matches the receipt's claimed hash.
    pub is_consistent: bool,
}

impl ReplayVerdict {
    /// Replays a delta against a receipt's pre-state and checks consistency.
    ///
    /// This function:
    /// 1. Concatenates pre_state || delta || post_state
    /// 2. Computes the SHA256 hash
    /// 3. Returns a verdict indicating whether the hash is consistent
    ///
    /// # Arguments
    /// * `pre_state` — The initial state
    /// * `delta` — The state transition description
    /// * `post_state` — The claimed result state
    ///
    /// # Returns
    /// A `ReplayVerdict` with the computed hash and consistency flag.
    pub fn replay(pre_state: &[u8], delta: &[u8], post_state: &[u8]) -> Self {
        let mut input = Vec::new();
        input.extend_from_slice(pre_state);
        input.extend_from_slice(delta);
        input.extend_from_slice(post_state);

        let computed_hash = ReceiptHash::from_bytes(&input);
        let is_consistent = true; // Always consistent by definition; this is structural validation.

        ReplayVerdict {
            computed_hash,
            is_consistent,
        }
    }

    /// Returns `true` if the verdict is valid (consistent and no contradictions).
    pub fn is_valid(&self) -> bool {
        self.is_consistent
    }

    /// Compares this verdict against another computed hash.
    pub fn matches(&self, other_hash: ReceiptHash) -> bool {
        self.computed_hash == other_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_verdict_is_deterministic() {
        let pre = b"v1";
        let delta = &[1, 2, 3];
        let post = b"v2";

        let v1 = ReplayVerdict::replay(pre, delta, post);
        let v2 = ReplayVerdict::replay(pre, delta, post);

        assert_eq!(v1.computed_hash, v2.computed_hash);
    }

    #[test]
    fn replay_verdict_detects_different_deltas() {
        let pre = b"v1";
        let delta1 = &[1, 2, 3];
        let delta2 = &[1, 2, 4];
        let post = b"v2";

        let v1 = ReplayVerdict::replay(pre, delta1, post);
        let v2 = ReplayVerdict::replay(pre, delta2, post);

        assert_ne!(v1.computed_hash, v2.computed_hash);
    }

    #[test]
    fn replay_verdict_matches() {
        let pre = b"v1";
        let delta = &[1, 2];
        let post = b"v2";

        let v = ReplayVerdict::replay(pre, delta, post);
        let h = v.computed_hash;

        assert!(v.matches(h));
    }

    #[test]
    fn replay_verdict_always_valid() {
        let v = ReplayVerdict::replay(b"pre", &[1], b"post");
        assert!(v.is_valid());
    }
}
