use wasm4pm_compat::prelude::*;
#[test]
fn missing_ocel_event_type_is_refused() {
    let c = CapabilityContract::new(
        "cap",
        "https://example.org/cap",
        "sem-1",
        "urn:in",
        "urn:out",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        " ",
    );
    assert!(c
        .validate()
        .contains(&ProtocolRefusal::MissingOcelEventType {
            capability_id: "cap".into()
        }));
}
