use wasm4pm_compat::prelude::*;
#[test]
fn brokered_authority_wire_name_is_stable() {
    assert_eq!(
        serde_json::to_string(&AuthorityMode::Brokered).unwrap(),
        "\"brokered\""
    );
}
