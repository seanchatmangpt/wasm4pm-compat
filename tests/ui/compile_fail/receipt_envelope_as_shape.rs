// COMPILE-FAIL: Receipt shape law — ReceiptEnvelope cannot be passed where ReceiptShape is required.
// Law: ReceiptEnvelope (with subject, witness, digest, and replay hint) and ReceiptShape
// (witness, digest, replay hint only) are distinct types. An envelope is not a bare shape.
use wasm4pm_compat::receipt::{Digest, ReplayHint, ReceiptEnvelope, ReceiptShape};

fn requires_receipt_shape(_shape: ReceiptShape) {}

fn main() {
    let env = ReceiptEnvelope::new("subject-id", "witness-label", Digest::new("d"), ReplayHint::new("h"));
    // This must fail: ReceiptEnvelope is not ReceiptShape.
    requires_receipt_shape(env);
}
