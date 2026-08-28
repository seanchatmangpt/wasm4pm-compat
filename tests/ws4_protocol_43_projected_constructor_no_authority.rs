use wasm4pm_compat::prelude::*;
#[test]
fn projected_constructor_never_grants_ambient_authority() {
    let s = SurfaceBinding::projected("cap", SurfaceKind::Cli, "d", "in", "out");
    assert!(!s.ambient_authority);
}
