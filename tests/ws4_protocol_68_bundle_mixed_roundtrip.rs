use wasm4pm_compat::prelude::*;
#[test]
fn mixed_surface_bundle_roundtrips_without_collapsing_dispositions() {
    let c = CapabilityContract::new(
        "cap",
        "https://public",
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
    let s = serde_json::to_string(&b).unwrap();
    let d: ProtocolBundle = serde_json::from_str(&s).unwrap();
    assert!(d.validate().is_empty());
    assert_eq!(d.surfaces.len(), 4);
}
