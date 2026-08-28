use wasm4pm_compat::prelude::*;
#[test]
fn valid_construct_intent_is_admitted() {
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
    let i =
        ProtocolIntent::<ConstructPhase>::try_new(&c, SubjectRef::new("s", "sd"), "input").unwrap();
    assert_eq!(i.consequence_class(), ConsequenceClass::Construct);
}
