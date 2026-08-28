use wasm4pm_compat::receipt::{Digest, ReceiptEnvelope, ReplayHint};

#[test]
fn receipt_envelope_treats_whitespace_subject_as_present_structure() {
    let receipt = ReceiptEnvelope::new("   ", "w", Digest::new("d"), ReplayHint::new("h"));
    assert!(receipt.is_well_shaped());
}
