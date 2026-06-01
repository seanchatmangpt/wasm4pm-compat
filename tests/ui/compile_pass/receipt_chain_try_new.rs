// Law: ReceiptChainTryNewLaw — ReceiptChain::try_new constructs a valid single-link chain; the chain_id and the single well-shaped link are both accessible
// COMPILE-PASS: ReceiptChain::try_new — proves a valid single-link chain constructs

use wasm4pm_compat::receipt::{Digest, ReceiptChain, ReceiptEnvelope, ReplayHint};

fn main() {
    let link = ReceiptEnvelope::new(
        "case-1",
        "discovery-run",
        Digest::new("blake3:aaa"),
        ReplayHint::new("rerun:plan#1"),
    );
    let chain = ReceiptChain::try_new("run-001", vec![link]).unwrap();
    assert_eq!(chain.len(), 1);
    assert!(!chain.is_empty());
    assert_eq!(chain.chain_id, "run-001");
}
