use wasm4pm_compat::prelude::*;
#[test]
fn construct_phase_is_reversible() {
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
    assert!(i.reversible());
}
