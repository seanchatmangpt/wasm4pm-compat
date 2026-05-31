// COMPILE-PASS: ReceiptVerdict Display — proves both Admitted and Refused(law)
// variants implement Display and render named-law diagnostics.
//
// Law: Blue River Dam covenant — ReceiptVerdict is a first-class named outcome
// for receipt shape-checks. Both variants produce human-readable diagnostics
// that name the structural law (Admitted or specific ReceiptRefusal). No bare
// error strings.
use wasm4pm_compat::receipt::{ReceiptRefusal, ReceiptVerdict};

fn main() {
    // Admitted variant renders as a structural verdict.
    let admitted = ReceiptVerdict::Admitted;
    let msg = format!("{}", admitted);
    assert!(msg.contains("Admitted"), "expected 'Admitted' in: {msg}");

    // Refused variant renders the named refusal law.
    let refused = ReceiptVerdict::Refused(ReceiptRefusal::MissingWitness);
    let msg2 = format!("{}", refused);
    assert!(msg2.contains("MissingWitness"), "expected 'MissingWitness' in: {msg2}");

    // Refused with a different named law.
    let refused2 = ReceiptVerdict::Refused(ReceiptRefusal::UnreplayableClaim);
    let msg3 = format!("{}", refused2);
    assert!(msg3.contains("UnreplayableClaim"), "expected 'UnreplayableClaim' in: {msg3}");

    // is_admitted / is_refused predicates are consistent with Display.
    assert!(admitted.is_admitted());
    assert!(!admitted.is_refused());
    assert!(!refused.is_admitted());
    assert!(refused.is_refused());
}
