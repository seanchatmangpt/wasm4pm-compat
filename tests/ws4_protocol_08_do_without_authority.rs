use wasm4pm_compat::prelude::*;
#[test]
fn do_without_authority_is_refused() {
    let c = CapabilityContract::new(
        "cap",
        "https://example.org/cap",
        "sem-1",
        "urn:in",
        "urn:out",
        ConsequenceClass::Do,
        AuthorityMode::None,
        ReceiptPolicy::Required,
        "cap.did",
    );
    assert!(c.validate().contains(&ProtocolRefusal::DoWithoutAuthority {
        capability_id: "cap".into()
    }));
}
