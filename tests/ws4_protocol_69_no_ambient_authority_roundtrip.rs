use wasm4pm_compat::prelude::*;
#[test]
fn projected_surface_roundtrip_preserves_no_ambient_authority() {
    let s = SurfaceBinding::projected("cap", SurfaceKind::Cli, "d", "in", "out");
    let raw = serde_json::to_string(&s).unwrap();
    let d: SurfaceBinding = serde_json::from_str(&raw).unwrap();
    assert!(!d.ambient_authority);
}
