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
fn valid_select_intent_is_admitted() {
    let i =
        ProtocolIntent::<SelectPhase>::try_new(&c(), SubjectRef::new("s", "sd"), "input").unwrap();
    assert_eq!(i.consequence_class(), ConsequenceClass::Select);
}
