// COMPILE-PASS: ReceiptEnvelope — a four-field provenance bearer with subject,
// witness, digest, and replay hint compiles and is well-shaped.
//
// Law: Blue River Dam covenant — a ReceiptEnvelope carries a named subject
// (the thing being receipted) alongside witness, digest, and replay hint.
// The digest field is carried, not computed; this boundary is structural-only.
use wasm4pm_compat::receipt::{Digest, ReceiptEnvelope, ReplayHint};

fn main() {
    // Construct a receipt envelope: subject, witness, digest, replay hint.
    let envelope = ReceiptEnvelope::new(
        "case-42",
        "OCEL-2.0-admission",
        Digest::new("blake3:deadbeef0123456789"),
        ReplayHint::new("rerun:plan#42"),
    );

    // All four fields are accessible.
    assert_eq!(envelope.subject, "case-42");
    assert_eq!(envelope.witness, "OCEL-2.0-admission");
    assert_eq!(envelope.digest.0, "blake3:deadbeef0123456789");
    assert_eq!(envelope.replay_hint.0, "rerun:plan#42");

    // A complete envelope is well-shaped.
    assert!(envelope.is_well_shaped());

    // An envelope with an empty subject is not well-shaped.
    let bad = ReceiptEnvelope::new(
        "",
        "OCEL-2.0-admission",
        Digest::new("blake3:deadbeef0123456789"),
        ReplayHint::new("rerun:plan#42"),
    );
    assert!(!bad.is_well_shaped());
}
