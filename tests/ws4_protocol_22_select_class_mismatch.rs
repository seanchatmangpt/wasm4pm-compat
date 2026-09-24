use wasm4pm_compat::prelude::*;
#[test]
fn select_intent_refuses_construct_contract() {
    let c = CapabilityContract::new(
        "cap",
        "https://e/c",
        "d",
        "in",
        "out",
        ConsequenceClass::Construct,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "evt",
    );
    let e = ProtocolIntent::<SelectPhase>::try_new(&c, SubjectRef::new("s", "sd"), "input")
        .unwrap_err();
    assert!(e.contains(&ProtocolRefusal::ConsequenceClassMismatch {
        capability_id: "cap".into(),
        expected: ConsequenceClass::Select,
        actual: ConsequenceClass::Construct
    }));
}
