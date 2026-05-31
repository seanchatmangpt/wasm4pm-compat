// COMPILE-PASS: ReceiptChain root and tip — proves root() and tip() access the correct ends

use wasm4pm_compat::receipt::{Digest, ReceiptChain, ReceiptEnvelope, ReplayHint};

fn main() {
    let root = ReceiptEnvelope::new(
        "root-subj",
        "w",
        Digest::new("d0"),
        ReplayHint::new("h0"),
    );
    let tip = ReceiptEnvelope::new("tip-subj", "w", Digest::new("d1"), ReplayHint::new("h1"));
    let chain = ReceiptChain::try_new("run-x", vec![root, tip]).unwrap();
    assert_eq!(chain.root().subject, "root-subj");
    assert_eq!(chain.tip().subject, "tip-subj");
    assert_eq!(chain.len(), 2);
}
