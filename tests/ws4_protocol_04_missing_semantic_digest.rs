use wasm4pm_compat::prelude::*;
#[test]
fn missing_semantic_digest_is_refused() {
    let c = CapabilityContract::new(
        "cap",
        "https://example.org/cap",
        " ",
        "urn:in",
        "urn:out",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "cap.selected",
    );
    assert!(c
        .validate()
        .contains(&ProtocolRefusal::MissingSemanticDigest {
            capability_id: "cap".into()
        }));
}
