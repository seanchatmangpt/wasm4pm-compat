use wasm4pm_compat::prelude::*;
#[test]
fn cli_surface_wire_name_is_stable(){assert_eq!(serde_json::to_string(&SurfaceKind::Cli).unwrap(),"\"cli\"");}
