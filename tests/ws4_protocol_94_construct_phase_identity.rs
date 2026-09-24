use wasm4pm_compat::prelude::*;
#[test]
fn construct_intent_reports_construct_phase() {
    let c = CapabilityContract::new(
        "cap",
        "https://p",
        "d",
        "in",
        "out",
        ConsequenceClass::Construct,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "evt",
    );
    let i = Intent::<ConstructPhase>::try_new(&c, SubjectRef::new("s", "sd"), "id").unwrap();
    assert_eq!(i.consequence_class(), ConsequenceClass::Construct);
    assert!(i.reversible());
}
