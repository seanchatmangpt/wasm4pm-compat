use wasm4pm_compat::prelude::*;
#[test]
fn unknown_capability_projection_is_refused() {
    let s = SurfaceBinding::projected("ghost", SurfaceKind::Cli, "d", "in", "out");
    let b = ProtocolBundle {
        protocol_id: "p".into(),
        version: "1".into(),
        capabilities: vec![],
        surfaces: vec![s],
    };
    assert!(b
        .validate()
        .contains(&ProtocolRefusal::UnknownCapabilityProjection {
            capability_id: "ghost".into(),
            surface: SurfaceKind::Cli
        }));
}
