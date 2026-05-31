// COMPILE-PASS: ReceiptChain::extend_with — proves a well-shaped link extends the chain

use wasm4pm_compat::receipt::{Digest, ReceiptChain, ReceiptEnvelope, ReplayHint};

fn main() {
    let root = ReceiptEnvelope::new("root", "w", Digest::new("d0"), ReplayHint::new("h0"));
    let mut chain = ReceiptChain::try_new("run", vec![root]).unwrap();
    assert_eq!(chain.len(), 1);

    let next = ReceiptEnvelope::new("step-1", "w", Digest::new("d1"), ReplayHint::new("h1"));
    assert!(chain.extend_with(next).is_ok());
    assert_eq!(chain.len(), 2);
    assert_eq!(chain.tip().subject, "step-1");
}
