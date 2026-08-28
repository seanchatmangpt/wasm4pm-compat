use wasm4pm_compat::prelude::*;

#[test]
fn missing_public_semantic_is_refused() {
    let c = CapabilityContract::new("cap", " ", "sem-1", "urn:in", "urn:out", ConsequenceClass::Select, AuthorityMode::None, ReceiptPolicy::Optional, "cap.selected");
    assert!(c.validate().contains(&ProtocolRefusal::MissingPublicSemantic { capability_id: "cap".into() }));
}
