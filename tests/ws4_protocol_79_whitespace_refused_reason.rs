use wasm4pm_compat::prelude::*;
#[test]
fn whitespace_only_refused_reason_is_refused() {
    let c = CapabilityContract::new(
        "cap",
        "https://p",
        "d",
        "in",
        "out",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "evt",
    );
    let b = ProtocolBundle {
        protocol_id: "p".into(),
        version: "1".into(),
        capabilities: vec![c],
        surfaces: vec![
            SurfaceBinding::refused("cap", SurfaceKind::Cli, "d", "POLICY", " \t "),
            SurfaceBinding::unsupported("cap", SurfaceKind::HttpApi, "d", "x"),
            SurfaceBinding::unsupported("cap", SurfaceKind::Mcp, "d", "x"),
            SurfaceBinding::unsupported("cap", SurfaceKind::A2a, "d", "x"),
        ],
    };
    assert!(b
        .validate()
        .contains(&ProtocolRefusal::EmptyDispositionReason {
            capability_id: "cap".into(),
            surface: SurfaceKind::Cli
        }));
}
