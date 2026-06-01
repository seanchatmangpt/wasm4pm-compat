// COMPILE-FAIL: Receipt law — ReceiptShape cannot be passed where GraduationReceipt is required.
// Law: GraduationReceipt wraps a ReceiptEnvelope plus a reason_tag. A bare ReceiptShape
// lacks both the envelope wrapping and the graduation reason.
use wasm4pm_compat::receipt::{Digest, GraduationReceipt, ReplayHint, ReceiptShape};

fn requires_graduation_receipt(_r: GraduationReceipt) {}

fn main() {
    let shape = ReceiptShape::new("witness", Digest::new("d"), ReplayHint::new("h"));
    // This must fail: ReceiptShape is not GraduationReceipt.
    requires_graduation_receipt(shape);
}
