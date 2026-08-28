use wasm4pm_compat::prelude::*;
#[test]
fn whitespace_only_semantic_digest_is_missing() {
    let c = CapabilityContract::new(
        "cap",
        "https://p",
        " \t ",
        "in",
        "out",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "evt",
    );
    assert!(c
        .validate()
        .contains(&ProtocolRefusal::MissingSemanticDigest {
            capability_id: "cap".into()
        }));
}
