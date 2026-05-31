// COMPILE-PASS: ReceiptChainConst<N> — proves const-generic arity chain constructs with type-level N

use wasm4pm_compat::receipt::{Digest, ReceiptChainConst, ReceiptEnvelope, ReplayHint};

fn main() {
    let a = ReceiptEnvelope::new("root", "w", Digest::new("d0"), ReplayHint::new("h0"));
    let b = ReceiptEnvelope::new("step", "w", Digest::new("d1"), ReplayHint::new("h1"));
    let chain = ReceiptChainConst::try_new("run-001", [a, b]).unwrap();
    assert_eq!(chain.arity(), 2);
    assert_eq!(chain.root().subject, "root");
    assert_eq!(chain.tip().subject, "step");
}
