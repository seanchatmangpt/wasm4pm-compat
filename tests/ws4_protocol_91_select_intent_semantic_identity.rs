use wasm4pm_compat::prelude::*;
#[test]
fn select_intent_binds_contract_semantic_digest() {
    let c = CapabilityContract::new(
        "cap",
        "https://p",
        "sem-d",
        "in",
        "out",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "evt",
    );
    let i = Intent::<SelectPhase>::try_new(&c, SubjectRef::new("s", "sd"), "id").unwrap();
    assert_eq!(i.semantic_digest(), "sem-d");
}
