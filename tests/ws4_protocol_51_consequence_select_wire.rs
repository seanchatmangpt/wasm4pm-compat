use wasm4pm_compat::prelude::*;
#[test]
fn select_wire_name_is_stable() {
    assert_eq!(
        serde_json::to_string(&ConsequenceClass::Select).unwrap(),
        "\"SELECT\""
    );
}
