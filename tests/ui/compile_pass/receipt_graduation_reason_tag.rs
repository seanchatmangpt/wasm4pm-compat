// COMPILE-PASS: GraduationReceipt reason_tag is &'static str — proves that the
// reason_tag field carries a static string key (not a heap String), and that
// multiple reason tags can distinguish graduation declarations structurally.
//
// Law: Blue River Dam covenant — GraduationReceipt.reason_tag is &'static str
// to enable zero-cost key comparison at the boundary. The graduation crate uses
// stable tag keys; runtime strings are forbidden as reason tags.
use wasm4pm_compat::receipt::{Digest, GraduationReceipt, ReceiptEnvelope, ReplayHint};

fn describe_graduation(gr: &GraduationReceipt) -> &'static str {
    // reason_tag is &'static str — usable in static contexts.
    gr.reason_tag
}

fn main() {
    let make_env = |subject: &str| {
        ReceiptEnvelope::new(
            subject,
            "wasm4pm-bridge",
            Digest::new("blake3:grad"),
            ReplayHint::new("wasm4pm://intake/candidate"),
        )
    };

    // Different static reason tags produce distinct graduation receipts.
    let gr1 = GraduationReceipt::new(make_env("log-001"), "needs_discovery");
    let gr2 = GraduationReceipt::new(make_env("log-002"), "needs_replay");
    let gr3 = GraduationReceipt::new(make_env("log-003"), "needs_conformance");

    assert_eq!(describe_graduation(&gr1), "needs_discovery");
    assert_eq!(describe_graduation(&gr2), "needs_replay");
    assert_eq!(describe_graduation(&gr3), "needs_conformance");

    // reason_tag is &'static str — distinct from a heap-allocated String.
    let tag: &'static str = gr1.reason_tag;
    assert_eq!(tag, "needs_discovery");

    // is_well_shaped() requires both envelope well-shaped and reason_tag non-empty.
    assert!(gr1.is_well_shaped());
    assert!(gr2.is_well_shaped());

    // An empty reason_tag produces an ill-shaped graduation receipt.
    let bad = GraduationReceipt::new(make_env("log-bad"), "");
    assert!(!bad.is_well_shaped());
}
