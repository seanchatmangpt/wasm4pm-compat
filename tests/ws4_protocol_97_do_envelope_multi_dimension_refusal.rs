use wasm4pm_compat::prelude::*;
#[test]
fn do_envelope_accumulates_contract_authority_and_receipt_failures() {
    let c = CapabilityContract::new(
        "cap",
        "https://p",
        "d",
        "in",
        "out",
        ConsequenceClass::Do,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "evt",
    );
    let a = AuthorityDecisionRef::new("", "other", "sd", "");
    let rr = ReceiptRequirement::new("", "", "");
    let r = DoEnvelope::try_new(&c, SubjectRef::new("s", "sd"), "id", a, rr).unwrap_err();
    assert!(r.contains(&ProtocolRefusal::DoWithoutAuthority {
        capability_id: "cap".into()
    }));
    assert!(r.contains(&ProtocolRefusal::DoWithoutRequiredReceipt {
        capability_id: "cap".into()
    }));
    assert!(r.contains(&ProtocolRefusal::MissingAuthorityId));
    assert!(r.contains(&ProtocolRefusal::MissingAuthorityDecisionDigest));
    assert!(r
        .iter()
        .any(|x| matches!(x, ProtocolRefusal::AuthorityCapabilityMismatch { .. })));
    assert!(r.contains(&ProtocolRefusal::MissingReceiptVersion));
    assert!(r.contains(&ProtocolRefusal::MissingReceiptDigestAlgorithm));
    assert!(r.contains(&ProtocolRefusal::MissingReplayContract));
}
