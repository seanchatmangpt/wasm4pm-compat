use wasm4pm_compat::prelude::*;
#[test]
fn construct_intent_accumulates_subject_and_input_refusals() {
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
    let r = Intent::<ConstructPhase>::try_new(&c, SubjectRef::new("", ""), "").unwrap_err();
    assert!(r.contains(&ProtocolRefusal::MissingSubjectId));
    assert!(r.contains(&ProtocolRefusal::MissingSubjectDigest));
    assert!(r.contains(&ProtocolRefusal::MissingInputDigest));
}
