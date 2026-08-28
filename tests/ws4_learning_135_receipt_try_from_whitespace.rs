use wasm4pm_compat::receipt::{Digest, ReceiptEnvelope, ReplayHint};

#[test]
fn receipt_constructor_accepts_whitespace_only_fields_as_present() {
    let receipt = ReceiptEnvelope::try_from_parts(
        " ",
        " ",
        Digest::new(" "),
        ReplayHint::new(" "),
    )
    .expect("whitespace is structurally present at compat layer");
    assert!(receipt.is_well_shaped());
}
