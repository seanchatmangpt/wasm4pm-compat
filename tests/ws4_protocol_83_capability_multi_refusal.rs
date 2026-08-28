use wasm4pm_compat::prelude::*;
#[test]
fn capability_accumulates_independent_missing_fields() {
    let c = CapabilityContract::new(
        "",
        "",
        "",
        "",
        "",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "",
    );
    let r = c.validate();
    assert!(r.contains(&ProtocolRefusal::EmptyCapabilityId));
    assert!(r
        .iter()
        .any(|x| matches!(x, ProtocolRefusal::MissingPublicSemantic { .. })));
    assert!(r
        .iter()
        .any(|x| matches!(x, ProtocolRefusal::MissingSemanticDigest { .. })));
    assert!(r
        .iter()
        .any(|x| matches!(x, ProtocolRefusal::MissingInputType { .. })));
    assert!(r
        .iter()
        .any(|x| matches!(x, ProtocolRefusal::MissingOutputType { .. })));
    assert!(r
        .iter()
        .any(|x| matches!(x, ProtocolRefusal::MissingOcelEventType { .. })));
}
