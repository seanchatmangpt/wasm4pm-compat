// COMPILE-FAIL: ReceiptVerdict law — ReceiptVerdict::Refused wraps ReceiptRefusal,
// not ReceiptShape. Passing a ReceiptShape where ReceiptRefusal is required is a
// type error.
//
// Law: ReceiptVerdict::Refused carries a named ReceiptRefusal law, not a bare
// ReceiptShape or any other receipt type. The law names the structural violation;
// a shape is not a violation name.
use wasm4pm_compat::receipt::{Digest, ReplayHint, ReceiptShape, ReceiptVerdict};

fn main() {
    let shape = ReceiptShape::new("witness", Digest::new("d"), ReplayHint::new("h"));
    // This must fail: ReceiptVerdict::Refused requires a ReceiptRefusal, not a ReceiptShape.
    let _v = ReceiptVerdict::Refused(shape);
}
