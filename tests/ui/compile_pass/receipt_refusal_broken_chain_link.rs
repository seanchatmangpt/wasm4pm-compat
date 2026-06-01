// Law: ReceiptChainBrokenLinkLaw — an ill-shaped ReceiptEnvelope (empty subject) as the first link is refused as BrokenChainLink(0); the named law identifies the position
// COMPILE-PASS: ReceiptChain broken link — proves BrokenChainLink(0) law for ill-shaped first link

use wasm4pm_compat::receipt::{Digest, ReceiptChain, ReceiptEnvelope, ReceiptRefusal, ReplayHint};

fn main() {
    // Empty subject is ill-shaped.
    let broken = ReceiptEnvelope::new("", "w", Digest::new("d"), ReplayHint::new("h"));
    let result = ReceiptChain::try_new("run-x", vec![broken]);
    assert_eq!(result, Err(ReceiptRefusal::BrokenChainLink(0)));
}
