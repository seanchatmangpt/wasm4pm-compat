use wasm4pm_compat::prelude::*;
#[test]
fn brokered_do_contract_is_structurally_valid() {
    let c = CapabilityContract::new(
        "cap",
        "https://e/c",
        "d",
        "in",
        "out",
        ConsequenceClass::Do,
        AuthorityMode::Brokered,
        ReceiptPolicy::Required,
        "evt",
    );
    assert!(c.validate().is_empty());
}
