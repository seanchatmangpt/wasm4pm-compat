// Law: ReceiptVerdictRefusedLaw — an ill-shaped ReceiptEnvelope (empty subject) produces ReceiptVerdict::Refused with a named ReceiptRefusal variant; bare strings are forbidden
// COMPILE-PASS: ReceiptVerdict::Refused — proves shape-check produces a named refusal law

use wasm4pm_compat::receipt::{Digest, ReceiptEnvelope, ReceiptRefusal, ReceiptVerdict, ReplayHint};

fn main() {
    let bad = ReceiptEnvelope::new("", "w", Digest::new("d"), ReplayHint::new("h"));
    let verdict =
        ReceiptVerdict::from_shape_check(bad.is_well_shaped(), Some(ReceiptRefusal::MissingSubject));
    assert!(!verdict.is_admitted());
    assert!(verdict.is_refused());
    assert_eq!(verdict.refusal(), Some(&ReceiptRefusal::MissingSubject));
}
