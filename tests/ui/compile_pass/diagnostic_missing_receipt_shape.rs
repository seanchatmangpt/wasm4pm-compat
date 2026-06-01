// COMPILE-PASS: CompatDiagnostic::MissingReceiptShape — proves the variant is
// constructible and used for surfaces where provenance-bearing evidence lacks
// a receipt envelope.
//
// Law: MissingReceiptShapeLaw — evidence that should be provenance-bearing must
// carry a Receipted state; a surface that skips the receipt envelope loses
// auditability.
use wasm4pm_compat::diagnostic::CompatDiagnostic;

fn check_receipt_shape(has_receipt: bool) -> Option<CompatDiagnostic> {
    if !has_receipt {
        Some(CompatDiagnostic::MissingReceiptShape)
    } else {
        None
    }
}

fn main() {
    let diag = check_receipt_shape(false);
    assert_eq!(diag, Some(CompatDiagnostic::MissingReceiptShape));

    let clean = check_receipt_shape(true);
    assert!(clean.is_none());
}
