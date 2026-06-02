use crate::receipt::C8Receipt;

/// An ordered, verifiable sequence of receipts.
///
/// A receipt chain enforces two invariants:
/// 1. **Contiguity**: Each receipt's post_state must equal the next receipt's pre_state.
/// 2. **Monotonicity**: Each receipt's causal_time must be strictly greater than the previous.
///
/// These invariants are checked by `verify()`.
///
/// A chain represents a lawful computation history: a series of state transitions
/// that could have produced the claimed final state from an initial state.
/// It does NOT prove that the deltas actually caused the transitions; it only
/// proves that the chain is internally consistent.
#[derive(Clone, Debug)]
pub struct ReceiptChain {
    receipts: Vec<C8Receipt>,
}

impl ReceiptChain {
    /// Creates an empty receipt chain.
    pub fn new() -> Self {
        ReceiptChain {
            receipts: Vec::new(),
        }
    }

    /// Appends a receipt to the chain.
    ///
    /// No validation is performed at append time. Validation happens in `verify()`.
    pub fn append(&mut self, receipt: C8Receipt) {
        self.receipts.push(receipt);
    }

    /// Verifies that all receipts in the chain are contiguous and monotonic.
    ///
    /// Returns `true` if:
    /// - For each pair (receipt_i, receipt_{i+1}):
    ///   - receipt_i.post_state == receipt_{i+1}.pre_state
    ///   - receipt_i.causal_time < receipt_{i+1}.causal_time
    ///
    /// An empty or single-receipt chain is trivially valid.
    pub fn verify(&self) -> bool {
        if self.receipts.len() <= 1 {
            return true;
        }

        for i in 0..self.receipts.len() - 1 {
            let curr = &self.receipts[i];
            let next = &self.receipts[i + 1];

            // Check contiguity: curr.post_state == next.pre_state
            if curr.post_state != next.pre_state {
                return false;
            }

            // Check monotonicity: curr.causal_time < next.causal_time
            if curr.causal_time >= next.causal_time {
                return false;
            }
        }

        true
    }

    /// Returns the number of receipts in the chain.
    pub fn len(&self) -> usize {
        self.receipts.len()
    }

    /// Returns `true` if the chain is empty.
    pub fn is_empty(&self) -> bool {
        self.receipts.is_empty()
    }

    /// Returns a reference to the receipt at the given index, if it exists.
    pub fn get(&self, index: usize) -> Option<&C8Receipt> {
        self.receipts.get(index)
    }

    /// Returns an iterator over all receipts in the chain.
    pub fn iter(&self) -> impl Iterator<Item = &C8Receipt> {
        self.receipts.iter()
    }

    /// Computes the hash chain: the sequence of hashes from each receipt.
    pub fn hash_chain(&self) -> Vec<crate::hash::ReceiptHash> {
        self.receipts.iter().map(|r| r.hash()).collect()
    }
}

impl Default for ReceiptChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_chain_verifies() {
        let chain = ReceiptChain::new();
        assert!(chain.verify());
        assert!(chain.is_empty());
    }

    #[test]
    fn single_receipt_chain_verifies() {
        let receipt = C8Receipt::new(b"v1".to_vec(), vec![1], b"v2".to_vec(), 1);
        let mut chain = ReceiptChain::new();
        chain.append(receipt);
        assert!(chain.verify());
        assert_eq!(chain.len(), 1);
    }

    #[test]
    fn contiguous_chain_verifies() {
        let mut chain = ReceiptChain::new();
        let r1 = C8Receipt::new(b"v1".to_vec(), vec![1], b"v2".to_vec(), 1);
        let r2 = C8Receipt::new(b"v2".to_vec(), vec![2], b"v3".to_vec(), 2);
        let r3 = C8Receipt::new(b"v3".to_vec(), vec![3], b"v4".to_vec(), 3);

        chain.append(r1);
        chain.append(r2);
        chain.append(r3);

        assert!(chain.verify());
    }

    #[test]
    fn non_contiguous_chain_fails() {
        let mut chain = ReceiptChain::new();
        let r1 = C8Receipt::new(b"v1".to_vec(), vec![1], b"v2".to_vec(), 1);
        let r2 = C8Receipt::new(b"v3".to_vec(), vec![2], b"v4".to_vec(), 2); // post != pre

        chain.append(r1);
        chain.append(r2);

        assert!(!chain.verify(), "Non-contiguous chain should fail verification");
    }

    #[test]
    fn non_monotonic_chain_fails() {
        let mut chain = ReceiptChain::new();
        let r1 = C8Receipt::new(b"v1".to_vec(), vec![1], b"v2".to_vec(), 2);
        let r2 = C8Receipt::new(b"v2".to_vec(), vec![2], b"v3".to_vec(), 1); // time goes backward

        chain.append(r1);
        chain.append(r2);

        assert!(!chain.verify(), "Non-monotonic chain should fail verification");
    }

    #[test]
    fn hash_chain_length() {
        let mut chain = ReceiptChain::new();
        chain.append(C8Receipt::new(b"v1".to_vec(), vec![1], b"v2".to_vec(), 1));
        chain.append(C8Receipt::new(b"v2".to_vec(), vec![2], b"v3".to_vec(), 2));
        chain.append(C8Receipt::new(b"v3".to_vec(), vec![3], b"v4".to_vec(), 3));

        let hashes = chain.hash_chain();
        assert_eq!(hashes.len(), 3);
    }
}
