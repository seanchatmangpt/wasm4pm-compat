use wasm4pm_compat::prelude::*;
#[test]
fn duplicate_unknown_projection_reports_topology_and_identity_failures(){let s=SurfaceBinding::projected("ghost",SurfaceKind::Cli,"d","in","out");let b=ProtocolBundle{protocol_id:"p".into(),version:"1".into(),capabilities:vec![],surfaces:vec![s.clone(),s]};let r=b.validate();assert!(r.contains(&ProtocolRefusal::DuplicateSurfaceBinding{capability_id:"ghost".into(),surface:SurfaceKind::Cli}));assert_eq!(r.iter().filter(|x|matches!(x,ProtocolRefusal::UnknownCapabilityProjection{..})).count(),2);}
