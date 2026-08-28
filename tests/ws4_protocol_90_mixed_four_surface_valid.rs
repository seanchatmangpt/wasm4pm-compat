use wasm4pm_compat::prelude::*;
#[test]
fn mixed_explicit_dispositions_close_bundle() {
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
            SurfaceBinding::projected("cap", SurfaceKind::Cli, "d", "in", "out"),
            SurfaceBinding::unsupported("cap", SurfaceKind::HttpApi, "d", "no"),
            SurfaceBinding::refused("cap", SurfaceKind::Mcp, "d", "POLICY", "deny"),
            SurfaceBinding::projected("cap", SurfaceKind::A2a, "d", "in", "out"),
        ],
    };
    assert!(b.validate().is_empty());
}
