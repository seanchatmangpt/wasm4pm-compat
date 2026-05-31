// COMPILE-PASS: ReceiptShape — a provenance-bearing envelope with Digest and
// ReplayHint compiles.
//
// Law: Blue River Dam covenant — receipted evidence carries witness, digest, and
// replay hint as structural forms. The Digest is carried, not computed; the
// ReplayHint is carried, not executed. This boundary must be structural-only.
use wasm4pm_compat::receipt::{Digest, ReceiptShape, ReplayHint};

fn main() {
    // Build a receipt: witness label, digest, and replay hint.
    let receipt = ReceiptShape::new(
        "OCEL-2.0-admission",
        Digest::new("blake3:deadbeef0123456789"),
        ReplayHint::new("rerun:plan#42"),
    );

    assert_eq!(receipt.witness, "OCEL-2.0-admission");
    assert_eq!(receipt.digest.0, "blake3:deadbeef0123456789");
    assert_eq!(receipt.replay_hint.0, "rerun:plan#42");

    // Digest and ReplayHint are distinct transparent types.
    let _d: Digest = Digest::new("sha256:abc");
    let _h: ReplayHint = ReplayHint::new("replay:step=3");

    // A receipt is well-shaped when witness label and digest are non-empty.
    assert!(receipt.is_well_shaped());
}
