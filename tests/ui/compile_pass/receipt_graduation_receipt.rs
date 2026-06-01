// Law: GraduationReceiptLaw — GraduationReceipt carries a ReceiptEnvelope as the boundary crossing record; it is structure-only and never executes a replay
// COMPILE-PASS: GraduationReceipt — proves graduation boundary crossing receipt constructs

use wasm4pm_compat::receipt::{Digest, GraduationReceipt, ReceiptEnvelope, ReplayHint};

fn main() {
    let envelope = ReceiptEnvelope::new(
        "p2p-ocel-log",
        "wasm4pm-bridge",
        Digest::new("blake3:graduate"),
        ReplayHint::new("wasm4pm://intake/p2p-ocel-log"),
    );
    let gr = GraduationReceipt::new(envelope, "needs_discovery");
    assert_eq!(gr.reason_tag, "needs_discovery");
    assert!(gr.is_well_shaped());
    assert!(gr.envelope.is_well_shaped());
}
