// COMPILE-PASS: ReceiptRefusal::MissingWitness — the named-law refusal variant
// for a receipt that claims to witness nothing.
//
// Law: Blue River Dam covenant — every receipt refusal must carry a specific
// named law. MissingWitness is the exact structural law for a receipt where no
// witness name is present. Bare "InvalidInput" is forbidden.
use wasm4pm_compat::receipt::{Digest, ReceiptEnvelope, ReceiptRefusal, ReplayHint};

fn main() {
    // MissingWitness is a named-law variant, not a string catch-all.
    let r: ReceiptRefusal = ReceiptRefusal::MissingWitness;
    assert_eq!(r, ReceiptRefusal::MissingWitness);
    assert_ne!(r, ReceiptRefusal::MissingSubject);

    // Display renders the named law — not a bare error string.
    let msg = format!("{}", ReceiptRefusal::MissingWitness);
    assert!(msg.contains("MissingWitness"));

    // try_from_parts produces MissingWitness when witness is empty.
    let result = ReceiptEnvelope::try_from_parts(
        "case-99",
        "",
        Digest::new("blake3:abc123"),
        ReplayHint::new("rerun:plan#1"),
    );
    assert_eq!(result, Err(ReceiptRefusal::MissingWitness));
}
