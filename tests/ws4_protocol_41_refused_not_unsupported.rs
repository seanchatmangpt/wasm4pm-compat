use wasm4pm_compat::prelude::*;
#[test]
fn refused_disposition_is_not_unsupported(){let r=SurfaceBinding::refused("cap",SurfaceKind::Cli,"d","DENY","policy");assert!(matches!(r.disposition,SurfaceDisposition::Refused{..}));assert!(!matches!(r.disposition,SurfaceDisposition::Unsupported{..}));}
