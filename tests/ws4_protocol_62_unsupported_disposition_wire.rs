use wasm4pm_compat::prelude::*;
#[test]
fn unsupported_disposition_serializes_reason() {
    let s = SurfaceBinding::unsupported("cap", SurfaceKind::Mcp, "d", "not available");
    let v = serde_json::to_value(s).unwrap();
    assert_eq!(v["disposition"]["disposition"], "unsupported");
    assert_eq!(v["disposition"]["reason"], "not available");
}
