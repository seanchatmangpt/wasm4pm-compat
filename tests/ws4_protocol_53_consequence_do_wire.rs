use wasm4pm_compat::prelude::*;
#[test]
fn do_wire_name_is_stable() {
    assert_eq!(
        serde_json::to_string(&ConsequenceClass::Do).unwrap(),
        "\"DO\""
    );
}
