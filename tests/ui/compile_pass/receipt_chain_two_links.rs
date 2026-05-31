// COMPILE-PASS: ReceiptChain with two links — proves a multi-step provenance
// chain with exactly two well-shaped ReceiptEnvelope links constructs, that
// root() and tip() point to the correct links, and that chain_id is accessible.
//
// Law: Blue River Dam covenant — a receipt chain is structure-only; it carries
// links produced elsewhere and never links them cryptographically. The two-link
// case is the minimal multi-step provenance trail.
use wasm4pm_compat::receipt::{Digest, ReceiptChain, ReceiptEnvelope, ReplayHint};

fn main() {
    let root = ReceiptEnvelope::new(
        "case-root",
        "discovery-run",
        Digest::new("blake3:root000"),
        ReplayHint::new("rerun:plan#root"),
    );
    let tip = ReceiptEnvelope::new(
        "case-tip",
        "conformance-check",
        Digest::new("blake3:tip111"),
        ReplayHint::new("rerun:plan#tip"),
    );

    let chain = ReceiptChain::try_new("run-002", vec![root, tip]).unwrap();

    // Two-link chain has length 2.
    assert_eq!(chain.len(), 2);
    assert!(!chain.is_empty());

    // chain_id is accessible as a public field.
    assert_eq!(chain.chain_id, "run-002");

    // root() and tip() point to the correct ends.
    assert_eq!(chain.root().subject, "case-root");
    assert_eq!(chain.tip().subject, "case-tip");

    // Iterating yields both links in order.
    let subjects: Vec<&str> = chain.iter().map(|e| e.subject.as_str()).collect();
    assert_eq!(subjects, vec!["case-root", "case-tip"]);
}
