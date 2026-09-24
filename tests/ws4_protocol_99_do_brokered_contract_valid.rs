use wasm4pm_compat::prelude::*;
#[test]
fn brokered_do_contract_is_structurally_admissible() {
    let c = CapabilityContract::new(
        "cap",
        "https://p",
        "d",
        "in",
        "out",
        ConsequenceClass::Do,
        AuthorityMode::Brokered,
        ReceiptPolicy::Required,
        "evt",
    );
    assert!(c.validate().is_empty());
    let a = AuthorityDecisionRef::new("broker", "cap", "sd", "decision");
    let rr = ReceiptRequirement::new("1", "blake3", "replay-v1");
    assert!(DoEnvelope::try_new(&c, SubjectRef::new("s", "sd"), "id", a, rr).is_ok());
}
