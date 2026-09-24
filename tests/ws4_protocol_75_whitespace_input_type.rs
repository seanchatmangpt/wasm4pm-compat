use wasm4pm_compat::prelude::*;
#[test]
fn whitespace_only_input_type_is_missing() {
    let c = CapabilityContract::new(
        "cap",
        "https://p",
        "d",
        " \t ",
        "out",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "evt",
    );
    assert!(c.validate().contains(&ProtocolRefusal::MissingInputType {
        capability_id: "cap".into()
    }));
}
