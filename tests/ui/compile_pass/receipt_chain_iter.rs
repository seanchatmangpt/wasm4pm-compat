// Law: ReceiptChainIterLaw — ReceiptChain::iter yields all links in order from root to tip; the chain is traversable without consuming it
// COMPILE-PASS: ReceiptChain::iter — proves iteration over chain links

use wasm4pm_compat::receipt::{Digest, ReceiptChain, ReceiptEnvelope, ReplayHint};

fn main() {
    let a = ReceiptEnvelope::new("a", "w", Digest::new("d1"), ReplayHint::new("h1"));
    let b = ReceiptEnvelope::new("b", "w", Digest::new("d2"), ReplayHint::new("h2"));
    let c = ReceiptEnvelope::new("c", "w", Digest::new("d3"), ReplayHint::new("h3"));
    let chain = ReceiptChain::try_new("run", vec![a, b, c]).unwrap();

    let subjects: Vec<&str> = chain.iter().map(|e| e.subject.as_str()).collect();
    assert_eq!(subjects, vec!["a", "b", "c"]);
}
