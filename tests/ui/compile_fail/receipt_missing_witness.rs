// COMPILE-FAIL: Receipt shape law — ReceiptEnvelope::new requires a typed
// Digest value in the third position; a bare String is not accepted.
//
// Law: Digest and ReplayHint are distinct newtypes — not String aliases — so
// swapping a plain String for a Digest is a compile error, not a runtime
// misrouting.
use wasm4pm_compat::receipt::{ReplayHint, ReceiptEnvelope};

fn main() {
    // This must fail: the third argument must be a Digest, not a plain String.
    let _e = ReceiptEnvelope::new(
        "case-42",
        "discovery-run",
        "blake3:deadbeef".to_string(), // wrong type: String instead of Digest
        ReplayHint::new("rerun:plan#1"),
    );
}
