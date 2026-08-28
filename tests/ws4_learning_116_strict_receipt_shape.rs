use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

#[test]
fn receipt_claim_requires_receipt_shape() {
    let mut boundary = ProcessBoundary::fully_attested(ProcessBoundaryKind::ClaimsReceipt, "receipt-claim");
    boundary.has_receipt_shape = false;
    assert_eq!(boundary.check(), Err(vec![StrictViolation::MissingReceiptShape]));
}
