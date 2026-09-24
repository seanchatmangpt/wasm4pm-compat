use wasm4pm_compat::receipt::{Digest, ReceiptEnvelope, ReplayHint};

#[test]
fn receipt_envelope_new_is_a_carrier_not_an_admission_constructor() {
    let receipt = ReceiptEnvelope::new("", "w", Digest::new("d"), ReplayHint::new("h"));
    assert_eq!(receipt.subject, "");
    assert!(!receipt.is_well_shaped());
}
