use wasm4pm_compat::prelude::*;

#[test]
fn empty_capability_id_is_refused() {
    let c = CapabilityContract::new(
        "",
        "https://example.org/cap",
        "sem-1",
        "urn:in",
        "urn:out",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "cap.selected",
    );
    assert!(c.validate().contains(&ProtocolRefusal::EmptyCapabilityId));
}
