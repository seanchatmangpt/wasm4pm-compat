// COMPILE-PASS: ReceiptVerdict::refusal accessor — proves refusal() returns Some on refused verdicts

use wasm4pm_compat::receipt::{ReceiptRefusal, ReceiptVerdict};

fn main() {
    let v = ReceiptVerdict::Refused(ReceiptRefusal::MissingDigest);
    assert_eq!(v.refusal(), Some(&ReceiptRefusal::MissingDigest));
    assert_eq!(ReceiptVerdict::Admitted.refusal(), None);
}
