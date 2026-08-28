use wasm4pm_compat::prelude::*;
#[test]
fn mcp_surface_wire_name_is_stable(){assert_eq!(serde_json::to_string(&SurfaceKind::Mcp).unwrap(),"\"mcp\"");}
