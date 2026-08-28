use wasm4pm_compat::prelude::*;
fn c()->CapabilityContract{CapabilityContract::new("cap","https://e/c","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt")}
#[test]
fn duplicate_surface_binding_is_refused(){let s=SurfaceBinding::projected("cap",SurfaceKind::Cli,"d","in","out");let b=ProtocolBundle{protocol_id:"p".into(),version:"1".into(),capabilities:vec![c()],surfaces:vec![s.clone(),s]};assert!(b.validate().contains(&ProtocolRefusal::DuplicateSurfaceBinding{capability_id:"cap".into(),surface:SurfaceKind::Cli}));}
