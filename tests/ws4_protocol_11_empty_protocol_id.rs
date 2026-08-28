use wasm4pm_compat::prelude::*;
fn cap() -> CapabilityContract { CapabilityContract::new("cap", "https://example.org/cap", "sem-1", "urn:in", "urn:out", ConsequenceClass::Select, AuthorityMode::None, ReceiptPolicy::Optional, "cap.selected") }
#[test]
fn empty_protocol_id_is_refused() {
    let b = ProtocolBundle { protocol_id: " ".into(), version: "1".into(), capabilities: vec![cap()], surfaces: vec![] };
    assert!(b.validate().contains(&ProtocolRefusal::EmptyProtocolId));
}
