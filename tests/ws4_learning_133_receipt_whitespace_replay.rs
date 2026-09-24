use wasm4pm_compat::receipt::{Digest, ReceiptShape, ReplayHint};

#[test]
fn receipt_shape_treats_whitespace_replay_hint_as_present_structure() {
    let receipt = ReceiptShape::new("w", Digest::new("d"), ReplayHint::new("   "));
    assert!(receipt.is_well_shaped());
}
