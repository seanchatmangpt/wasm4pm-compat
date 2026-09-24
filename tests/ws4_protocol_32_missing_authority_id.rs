use wasm4pm_compat::prelude::*;
fn c() -> CapabilityContract {
    CapabilityContract::new(
        "cap",
        "https://e/c",
        "sem",
        "in",
        "out",
        ConsequenceClass::Do,
        AuthorityMode::ExternalDecision,
        ReceiptPolicy::Required,
        "evt",
    )
}
#[test]
fn do_refuses_missing_authority_id() {
    let e = DoEnvelope::try_new(
        &c(),
        SubjectRef::new("s", "sd"),
        "input",
        AuthorityDecisionRef::new(" ", "cap", "sd", "decision"),
        ReceiptRequirement::new("1", "BLAKE3", "replay"),
    )
    .unwrap_err();
    assert!(e.contains(&ProtocolRefusal::MissingAuthorityId));
}
