// COMPILE-PASS: ReceiptEnvelope constructs with a Digest and ReplayHint —
// covers the receipt envelope carrier beyond the bare ReceiptShape fixture.
//
// Law: Blue River Dam covenant — a ReceiptEnvelope extends ReceiptShape with
// a named subject: the thing being receipted. All four parts (subject, witness,
// digest, replay_hint) must be non-empty for the envelope to be well-shaped.
// This is structure-only: the envelope carries, never computes or verifies.
use wasm4pm_compat::receipt::{Digest, ReceiptEnvelope, ReceiptRefusal, ReplayHint};

fn main() {
    // Construct a well-shaped four-part envelope.
    let envelope = ReceiptEnvelope::new(
        "case-ocel-42",
        "OCEL-2.0-admission",
        Digest::new("blake3:deadbeef0123456789abcdef"),
        ReplayHint::new("rerun:plan#42"),
    );

    // All four fields are accessible.
    assert_eq!(envelope.subject, "case-ocel-42");
    assert_eq!(envelope.witness, "OCEL-2.0-admission");
    assert_eq!(envelope.digest.0, "blake3:deadbeef0123456789abcdef");
    assert_eq!(envelope.replay_hint.0, "rerun:plan#42");

    // A complete envelope is well-shaped.
    assert!(envelope.is_well_shaped());

    // Missing subject: not well-shaped.
    let no_subject = ReceiptEnvelope::new(
        "",
        "OCEL-2.0-admission",
        Digest::new("blake3:abc"),
        ReplayHint::new("rerun:plan#1"),
    );
    assert!(!no_subject.is_well_shaped());

    // Missing digest: not well-shaped.
    let no_digest = ReceiptEnvelope::new(
        "case-1",
        "wfnet",
        Digest::new(""),
        ReplayHint::new("rerun:plan#1"),
    );
    assert!(!no_digest.is_well_shaped());

    // Missing replay hint: not well-shaped.
    let no_hint = ReceiptEnvelope::new(
        "case-2",
        "xes-1849",
        Digest::new("sha3:ff"),
        ReplayHint::new(""),
    );
    assert!(!no_hint.is_well_shaped());

    // Digest and ReplayHint are repr(transparent) over String — wrapping is
    // the only construction path (no hashing, no replay).
    let d = Digest::new("sha3:cafebabe");
    assert_eq!(d.0, "sha3:cafebabe");

    let h = ReplayHint::new("rerun:plan#99");
    assert_eq!(h.0, "rerun:plan#99");

    // ReceiptRefusal names specific laws, not bare invalid-input.
    let _: ReceiptRefusal = ReceiptRefusal::MissingSubject;
    let _: ReceiptRefusal = ReceiptRefusal::MissingDigest;
    let _: ReceiptRefusal = ReceiptRefusal::MissingReplayHint;
    let _: ReceiptRefusal = ReceiptRefusal::UnreplayableClaim;
}
