use wasm4pm_compat::prelude::*;
#[test]
fn missing_output_type_is_refused() {
    let c = CapabilityContract::new(
        "cap",
        "https://example.org/cap",
        "sem-1",
        "urn:in",
        " ",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "cap.selected",
    );
    assert!(c.validate().contains(&ProtocolRefusal::MissingOutputType {
        capability_id: "cap".into()
    }));
}
