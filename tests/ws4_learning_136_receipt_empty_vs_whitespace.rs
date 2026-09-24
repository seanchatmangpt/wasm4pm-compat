use wasm4pm_compat::receipt::{Digest, ReceiptShape, ReplayHint};

#[test]
fn receipt_shape_distinguishes_empty_from_whitespace_witness() {
    let empty = ReceiptShape::new("", Digest::new("d"), ReplayHint::new("h"));
    let whitespace = ReceiptShape::new(" ", Digest::new("d"), ReplayHint::new("h"));
    assert!(!empty.is_well_shaped());
    assert!(whitespace.is_well_shaped());
}
