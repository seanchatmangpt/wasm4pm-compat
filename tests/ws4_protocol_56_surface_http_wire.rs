use wasm4pm_compat::prelude::*;
#[test]
fn http_surface_wire_name_is_stable(){assert_eq!(serde_json::to_string(&SurfaceKind::HttpApi).unwrap(),"\"http_api\"");}
