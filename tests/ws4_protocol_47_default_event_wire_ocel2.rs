use wasm4pm_compat::prelude::*;
#[test]
fn capability_defaults_to_ocel2_event_wire() {
    let c = CapabilityContract::new(
        "cap",
        "https://e/c",
        "d",
        "in",
        "out",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "evt",
    );
    assert_eq!(c.event_wire, EventWireFormat::Ocel2);
}
