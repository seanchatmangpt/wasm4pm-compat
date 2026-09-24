use wasm4pm_compat::receipt::{Digest, ReceiptShape, ReplayHint};

#[test]
fn receipt_shape_treats_whitespace_digest_as_present_structure() {
    let receipt = ReceiptShape::new("w", Digest::new("   "), ReplayHint::new("h"));
    assert!(receipt.is_well_shaped());
}
