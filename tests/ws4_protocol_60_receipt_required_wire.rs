use wasm4pm_compat::prelude::*;
#[test]
fn required_receipt_wire_name_is_stable() {
    assert_eq!(
        serde_json::to_string(&ReceiptPolicy::Required).unwrap(),
        "\"required\""
    );
}
