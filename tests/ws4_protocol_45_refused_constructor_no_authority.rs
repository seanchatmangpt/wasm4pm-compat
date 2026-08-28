use wasm4pm_compat::prelude::*;
#[test]
fn refused_constructor_never_grants_ambient_authority(){let s=SurfaceBinding::refused("cap",SurfaceKind::A2a,"d","DENY","policy");assert!(!s.ambient_authority);match s.disposition{SurfaceDisposition::Refused{code,reason}=>{assert_eq!(code,"DENY");assert_eq!(reason,"policy");},other=>panic!("unexpected disposition: {other:?}")}}
