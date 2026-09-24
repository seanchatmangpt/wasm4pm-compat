use wasm4pm_compat::prelude::*;
fn c() -> CapabilityContract {
    CapabilityContract::new(
        "cap",
        "https://e/c",
        "d",
        "in",
        "out",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "evt",
    )
}
#[test]
fn missing_a2a_binding_is_refused() {
    let b = ProtocolBundle {
        protocol_id: "p".into(),
        version: "1".into(),
        capabilities: vec![c()],
        surfaces: vec![
            SurfaceBinding::projected("cap", SurfaceKind::Cli, "d", "in", "out"),
            SurfaceBinding::projected("cap", SurfaceKind::HttpApi, "d", "in", "out"),
            SurfaceBinding::projected("cap", SurfaceKind::Mcp, "d", "in", "out"),
        ],
    };
    assert!(b
        .validate()
        .contains(&ProtocolRefusal::MissingSurfaceBinding {
            capability_id: "cap".into(),
            surface: SurfaceKind::A2a
        }));
}
