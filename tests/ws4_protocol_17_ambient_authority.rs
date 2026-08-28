use wasm4pm_compat::prelude::*;
fn c()->CapabilityContract{CapabilityContract::new("cap","https://e/c","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt")}
#[test]
fn ambient_surface_authority_is_refused(){let mut s=SurfaceBinding::projected("cap",SurfaceKind::Cli,"d","in","out");s.ambient_authority=true;let b=ProtocolBundle{protocol_id:"p".into(),version:"1".into(),capabilities:vec![c()],surfaces:vec![s]};assert!(b.validate().contains(&ProtocolRefusal::AmbientAuthorityOnSurface{capability_id:"cap".into(),surface:SurfaceKind::Cli}));}
