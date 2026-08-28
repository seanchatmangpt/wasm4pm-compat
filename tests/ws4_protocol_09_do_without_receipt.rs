use wasm4pm_compat::prelude::*;
#[test]
fn do_without_required_receipt_is_refused() {
    let c = CapabilityContract::new("cap", "https://example.org/cap", "sem-1", "urn:in", "urn:out", ConsequenceClass::Do, AuthorityMode::ExternalDecision, ReceiptPolicy::Optional, "cap.did");
    assert!(c.validate().contains(&ProtocolRefusal::DoWithoutRequiredReceipt { capability_id: "cap".into() }));
}
