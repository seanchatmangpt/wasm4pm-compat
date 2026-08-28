use wasm4pm_compat::prelude::*;
fn c() -> CapabilityContract {
    CapabilityContract::new(
        "cap",
        "https://e/c",
        "d1",
        "in",
        "out",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "evt",
    )
}
#[test]
fn projection_semantic_drift_is_refused() {
    let s = SurfaceBinding::projected("cap", SurfaceKind::Cli, "d2", "in", "out");
    let b = ProtocolBundle {
        protocol_id: "p".into(),
        version: "1".into(),
        capabilities: vec![c()],
        surfaces: vec![s],
    };
    assert!(b
        .validate()
        .contains(&ProtocolRefusal::ProjectionSemanticDrift {
            capability_id: "cap".into(),
            surface: SurfaceKind::Cli
        }));
}
