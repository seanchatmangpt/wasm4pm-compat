use wasm4pm_compat::prelude::*;
#[test]
fn empty_custom_semantic_is_refused() {
    let c = CapabilityContract::new(
        "cap",
        "https://example.org/cap",
        "sem-1",
        "urn:in",
        "urn:out",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "cap.selected",
    )
    .with_custom_semantic_iri(" ");
    assert!(c
        .validate()
        .contains(&ProtocolRefusal::EmptyCustomSemantic {
            capability_id: "cap".into()
        }));
}
