// Law: ReceiptVerdictAdmittedLaw — a well-shaped ReceiptEnvelope (non-empty subject, witness, digest, hint) produces ReceiptVerdict::Admitted via from_shape_check
// COMPILE-PASS: ReceiptVerdict::Admitted — proves shape-check produces Admitted for a well-shaped envelope

use wasm4pm_compat::receipt::{Digest, ReceiptEnvelope, ReceiptVerdict, ReplayHint};

fn main() {
    let env = ReceiptEnvelope::new(
        "case-1",
        "discovery-run",
        Digest::new("blake3:abc"),
        ReplayHint::new("rerun:plan#1"),
    );
    let verdict = ReceiptVerdict::from_shape_check(env.is_well_shaped(), None);
    assert_eq!(verdict, ReceiptVerdict::Admitted);
    assert!(verdict.is_admitted());
    assert!(!verdict.is_refused());
}
