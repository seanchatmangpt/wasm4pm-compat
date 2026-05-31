// COMPILE-PASS: ReceiptRefusal::MissingSubject — the named-law refusal variant
// for an envelope that names no subject compiles and is matchable.
//
// Law: Blue River Dam covenant — every receipt refusal must carry a specific
// named law. MissingSubject is the exact structural law for an envelope where
// what is being receipted is unknown. Bare "InvalidInput" is forbidden.
use wasm4pm_compat::receipt::ReceiptRefusal;

fn check_refusal(r: ReceiptRefusal) -> &'static str {
    match r {
        ReceiptRefusal::MissingSubject => "missing-subject",
        ReceiptRefusal::MissingWitness => "missing-witness",
        ReceiptRefusal::MissingDigest => "missing-digest",
        ReceiptRefusal::MissingReplayHint => "missing-replay-hint",
        ReceiptRefusal::UnreplayableClaim => "unreplayable-claim",
        _ => "unknown",
    }
}

fn main() {
    // MissingSubject names the specific structural law for a subjectless envelope.
    let refusal = ReceiptRefusal::MissingSubject;
    assert_eq!(check_refusal(refusal), "missing-subject");

    // ReceiptRefusal variants are comparable with PartialEq.
    assert_eq!(ReceiptRefusal::MissingSubject, ReceiptRefusal::MissingSubject);
    assert_ne!(ReceiptRefusal::MissingSubject, ReceiptRefusal::MissingWitness);

    // Display renders the named law.
    let msg = format!("{}", ReceiptRefusal::MissingSubject);
    assert!(msg.contains("MissingSubject"));
}
