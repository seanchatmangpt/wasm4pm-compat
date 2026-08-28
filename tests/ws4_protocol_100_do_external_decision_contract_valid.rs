use wasm4pm_compat::prelude::*;
#[test]
fn external_decision_do_contract_is_structurally_admissible() {
    let c = CapabilityContract::new(
        "cap",
        "https://p",
        "d",
        "in",
        "out",
        ConsequenceClass::Do,
        AuthorityMode::ExternalDecision,
        ReceiptPolicy::Required,
        "evt",
    );
    assert!(c.validate().is_empty());
    let a = AuthorityDecisionRef::new("authority", "cap", "sd", "decision");
    let rr = ReceiptRequirement::new("1", "blake3", "replay-v1");
    let e = DoEnvelope::try_new(&c, SubjectRef::new("s", "sd"), "id", a, rr).unwrap();
    assert_eq!(e.intent().consequence_class(), ConsequenceClass::Do);
    assert!(!e.intent().reversible());
}
