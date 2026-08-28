use wasm4pm_compat::prelude::*;
#[test]
fn unsupported_constructor_never_grants_ambient_authority(){let s=SurfaceBinding::unsupported("cap",SurfaceKind::Mcp,"d","not implemented");assert!(!s.ambient_authority);assert!(matches!(s.disposition,SurfaceDisposition::Unsupported{..}));}
