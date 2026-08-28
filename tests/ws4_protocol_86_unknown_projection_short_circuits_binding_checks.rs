use wasm4pm_compat::prelude::*;
#[test]
fn unknown_projection_short_circuits_binding_specific_checks() {
    let mut s = SurfaceBinding::projected("ghost", SurfaceKind::Cli, "wrong", "", "out");
    s.ambient_authority = true;
    let b = ProtocolBundle {
        protocol_id: "p".into(),
        version: "1".into(),
        capabilities: vec![],
        surfaces: vec![s],
    };
    let r = b.validate();
    assert!(r.contains(&ProtocolRefusal::UnknownCapabilityProjection {
        capability_id: "ghost".into(),
        surface: SurfaceKind::Cli
    }));
    assert!(!r.iter().any(|x| matches!(
        x,
        ProtocolRefusal::AmbientAuthorityOnSurface { .. }
            | ProtocolRefusal::ProjectionSemanticDrift { .. }
            | ProtocolRefusal::EmptySurfaceSchema { .. }
    )));
}
