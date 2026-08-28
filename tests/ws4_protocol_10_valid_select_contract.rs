use wasm4pm_compat::prelude::*;
#[test]
fn valid_select_contract_has_no_refusals() {
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
    );
    assert!(c.validate().is_empty());
}
