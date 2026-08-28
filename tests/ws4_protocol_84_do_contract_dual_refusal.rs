use wasm4pm_compat::prelude::*;
#[test]
fn do_contract_accumulates_authority_and_receipt_refusals() {
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
    let r = c.validate();
    assert!(r.contains(&ProtocolRefusal::DoWithoutAuthority {
        capability_id: "cap".into()
    }));
    assert!(r.contains(&ProtocolRefusal::DoWithoutRequiredReceipt {
        capability_id: "cap".into()
    }));
}
