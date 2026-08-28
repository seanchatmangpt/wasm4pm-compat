use wasm4pm_compat::prelude::*;
#[test]
fn empty_protocol_version_is_refused() {
    let b = ProtocolBundle {
        protocol_id: "p".into(),
        version: " ".into(),
        capabilities: vec![],
        surfaces: vec![],
    };
    assert!(b
        .validate()
        .contains(&ProtocolRefusal::EmptyProtocolVersion));
}
