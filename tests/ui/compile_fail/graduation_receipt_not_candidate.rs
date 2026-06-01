// COMPILE-FAIL: GraduationReceipt law — GraduationReceipt cannot be passed where
// ReceiptEnvelope is required. These are distinct types with distinct roles.
//
// Law: GraduationReceipt wraps a ReceiptEnvelope with a reason_tag. It is the
// graduation-specific receipt carrier. A plain ReceiptEnvelope does not carry a
// reason_tag and cannot substitute for a GraduationReceipt.
use wasm4pm_compat::receipt::{Digest, GraduationReceipt, ReplayHint, ReceiptEnvelope};

fn requires_envelope(_e: ReceiptEnvelope) {}

fn main() {
    let env = ReceiptEnvelope::new(
        "p2p-ocel-log",
        "wasm4pm-bridge",
        Digest::new("blake3:graduate"),
        ReplayHint::new("wasm4pm://intake/p2p-ocel-log"),
    );
    let receipt = GraduationReceipt::new(env, "needs_discovery");
    // This must fail: GraduationReceipt is not ReceiptEnvelope.
    requires_envelope(receipt);
}
