use wasm4pm_compat::prelude::*;
#[test]
fn a2a_surface_wire_name_is_stable() {
    assert_eq!(serde_json::to_string(&SurfaceKind::A2a).unwrap(), "\"a2a\"");
}
