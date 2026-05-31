// COMPILE-PASS: ReplayHint field construction — proves that ReplayHint is a
// repr(transparent) String newtype, carries an opaque hint string, and is
// carried (not executed) by receipt types.
//
// Law: Blue River Dam covenant — ReplayHint is structure-only. It is never
// executed; it is carried to the wasm4pm engine for replay. No computation
// occurs inside this crate.
use wasm4pm_compat::receipt::{Digest, ReceiptEnvelope, ReceiptShape, ReplayHint};

fn main() {
    // ReplayHint wraps any &str or String — the only construction path.
    let h = ReplayHint::new("wasm4pm://intake/run-007");
    assert_eq!(h.0, "wasm4pm://intake/run-007");

    // ReplayHint carries arbitrary opaque hint strings.
    let h2 = ReplayHint::new("rerun:plan#42");
    assert_eq!(h2.0, "rerun:plan#42");

    // Clone semantics — ReplayHint is Clone.
    let h3 = h2.clone();
    assert_eq!(h3.0, "rerun:plan#42");

    // PartialEq — two hints with the same string are equal.
    assert_eq!(h2, h3);

    // A ReceiptShape carries a ReplayHint as a field — the field is accessible.
    let shape = ReceiptShape::new(
        "witness-label",
        Digest::new("blake3:cafebabe"),
        ReplayHint::new("rerun:plan#shape"),
    );
    assert_eq!(shape.replay_hint.0, "rerun:plan#shape");

    // A ReceiptEnvelope also carries a ReplayHint as a named field.
    let env = ReceiptEnvelope::new(
        "case-99",
        "ocel-2.0",
        Digest::new("blake3:deadbeef"),
        ReplayHint::new("wasm4pm://intake/case-99"),
    );
    assert_eq!(env.replay_hint.0, "wasm4pm://intake/case-99");

    // An empty ReplayHint makes an envelope ill-shaped.
    let bad = ReceiptEnvelope::new(
        "case-bad",
        "ocel-2.0",
        Digest::new("blake3:deadbeef"),
        ReplayHint::new(""),
    );
    assert!(!bad.is_well_shaped());
}
