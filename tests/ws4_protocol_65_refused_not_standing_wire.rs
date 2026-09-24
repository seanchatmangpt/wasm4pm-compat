use wasm4pm_compat::prelude::ProtocolStanding;
#[test]
fn refused_cannot_deserialize_as_standing() {
    assert!(serde_json::from_str::<ProtocolStanding>("\"REFUSED\"").is_err());
}
