// COMPILE-PASS: ReceiptEnvelope::try_from_parts — proves named refusal for each missing field

use wasm4pm_compat::receipt::{Digest, ReceiptEnvelope, ReceiptRefusal, ReplayHint};

fn main() {
    // All fields present: ok.
    let ok = ReceiptEnvelope::try_from_parts(
        "case-7",
        "discovery-run",
        Digest::new("blake3:abc"),
        ReplayHint::new("rerun:plan#7"),
    );
    assert!(ok.is_ok());

    // Missing subject: MissingSubject.
    let bad_subj = ReceiptEnvelope::try_from_parts(
        "",
        "discovery-run",
        Digest::new("blake3:abc"),
        ReplayHint::new("rerun:plan#7"),
    );
    assert_eq!(bad_subj, Err(ReceiptRefusal::MissingSubject));

    // Missing witness: MissingWitness.
    let bad_wit = ReceiptEnvelope::try_from_parts(
        "case-7",
        "",
        Digest::new("blake3:abc"),
        ReplayHint::new("rerun:plan#7"),
    );
    assert_eq!(bad_wit, Err(ReceiptRefusal::MissingWitness));

    // Missing digest: MissingDigest.
    let bad_dig = ReceiptEnvelope::try_from_parts(
        "case-7",
        "w",
        Digest::new(""),
        ReplayHint::new("h"),
    );
    assert_eq!(bad_dig, Err(ReceiptRefusal::MissingDigest));
}
