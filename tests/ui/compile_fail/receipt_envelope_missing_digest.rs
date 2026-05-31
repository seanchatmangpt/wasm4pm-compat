// COMPILE-FAIL: Rejects constructing ReceiptEnvelope without a Digest — proves
// receipt completeness is enforced structurally, not at runtime.
//
// Law: ReceiptEnvelope requires all four fields — subject, witness, digest, and
// replay_hint — to be present. Omitting `digest` from a struct literal is a
// compile error: the type system statically enforces receipt completeness. There
// is no runtime fallback, no Option, and no default. A digest-free envelope
// cannot exist.
use wasm4pm_compat::receipt::{ReplayHint, ReceiptEnvelope};

fn main() {
    // This must fail: `digest` is a required field of ReceiptEnvelope.
    // Omitting it from the struct literal produces a "missing field" error —
    // structural completeness is a compile-time law, not a runtime check.
    let _e = ReceiptEnvelope {
        subject: "case-42".to_string(),
        witness: "discovery-run".to_string(),
        // digest field intentionally omitted — must be rejected by compiler
        replay_hint: ReplayHint::new("rerun:plan#1"),
    };
}
