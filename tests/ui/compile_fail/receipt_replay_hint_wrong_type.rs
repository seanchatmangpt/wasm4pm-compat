// COMPILE-FAIL: ReplayHint law — ReceiptShape::new requires a typed ReplayHint
// value in the third position; a bare String is not accepted.
//
// Law: ReplayHint is a distinct newtype, not a String alias. Passing a plain
// String where ReplayHint is required is a compile error — the type system
// statically rejects replay-hint aliasing at zero runtime cost.
use wasm4pm_compat::receipt::{Digest, ReceiptShape};

fn main() {
    // This must fail: the third argument must be a ReplayHint, not a plain String.
    let _r = ReceiptShape::new(
        "discovery-run",
        Digest::new("blake3:abc123"),
        "rerun:plan#1".to_string(), // wrong type: String instead of ReplayHint
    );
}
