use wasm4pm_compat::prelude::*;
fn c() -> CapabilityContract {
    CapabilityContract::new(
        "cap",
        "https://e/c",
        "d",
        "in",
        "out",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "evt",
    )
}
#[test]
fn duplicate_capability_id_is_refused() {
    let b = ProtocolBundle {
        protocol_id: "p".into(),
        version: "1".into(),
        capabilities: vec![c(), c()],
        surfaces: vec![],
    };
    assert!(b
        .validate()
        .contains(&ProtocolRefusal::DuplicateCapabilityId {
            capability_id: "cap".into()
        }));
}
